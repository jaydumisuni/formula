use formula_check::promotion::{PromotionDecision, authorize_promotion_v1};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
    self_expansion::{
        ActivationMode, ExpansionActivationRecord, PromotionClass, SupersessionKind,
        SupersessionRecord,
    },
};
use formula_store::authority_store::{AuthorityStore, AuthorityStoreError};
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn promote_one(
    store: &mut AuthorityStore,
    parent: &UniverseGeneration,
    artifact: ArtifactDigest,
    evidence: ArtifactDigest,
) -> ArtifactDigest {
    let parent_digest = parent.digest();
    let frozen = FrozenCandidate::new(
        format!("p10-store-{}", artifact.hex()),
        vec![artifact],
        d("p10-store-world"),
        parent_digest,
        vec![],
        vec![],
        d("p10-store-authority"),
        d("p10-store-observer"),
    );
    let manifest = PromotionManifest::new(
        parent_digest,
        frozen.structural_digest(),
        vec![evidence],
        vec![artifact],
        vec![evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        parent_digest,
        parent_digest,
        vec![],
        vec![],
    );
    let decision = authorize_promotion_v1(
        &manifest,
        &frozen,
        &promotion,
        &[evidence],
        parent,
        &[],
    )
    .unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("valid store promotion must authorize")
    };
    store.promote(&authorization).unwrap().new_generation()
}

#[test]
fn expansion_activation_round_trips_exact_structural_identity() {
    let dir = tempdir().unwrap();
    let subject = d("p10-expansion-subject");
    let evidence = d("p10-expansion-evidence");
    let scope = d("p10-expansion-scope");
    let world = d("p10-expansion-world");
    let generation = UniverseGeneration::new(0, None, vec![subject], vec![evidence]);
    let generation_digest = generation.digest();
    let record = ExpansionActivationRecord::new(
        subject,
        PromotionClass::StructureWitness,
        generation_digest,
        world,
        ActivationMode::DefaultAutomatic,
        vec![evidence],
        vec![scope],
    );
    let record_digest = record.structural_digest();

    {
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        store.initialize_genesis(&generation).unwrap();
        let persisted = store.record_expansion_activation(&record).unwrap();
        assert_eq!(persisted.structural_digest(), record_digest);

        let resolved = store
            .resolve_expansion_activation(
                generation_digest,
                subject,
                PromotionClass::StructureWitness,
            )
            .unwrap()
            .expect("expansion activation must resolve before reopen");
        assert_eq!(resolved, record);
        assert_eq!(resolved.structural_digest(), record_digest);
    }

    let reopened = AuthorityStore::open(dir.path()).unwrap();
    let resolved = reopened
        .resolve_expansion_activation(
            generation_digest,
            subject,
            PromotionClass::StructureWitness,
        )
        .unwrap()
        .expect("expansion activation must survive reopen");
    assert_eq!(resolved, record);
    assert_eq!(resolved.structural_digest(), record_digest);
}

#[test]
fn activation_rejects_unadmitted_subject_or_unbound_evidence() {
    let dir = tempdir().unwrap();
    let admitted = d("p10-admitted-subject");
    let bound = d("p10-bound-evidence");
    let generation = UniverseGeneration::new(0, None, vec![admitted], vec![bound]);
    let generation_digest = generation.digest();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    store.initialize_genesis(&generation).unwrap();

    let missing_subject = d("p10-missing-subject");
    let bad_subject = ExpansionActivationRecord::new(
        missing_subject,
        PromotionClass::StructureWitness,
        generation_digest,
        d("world"),
        ActivationMode::DefaultAutomatic,
        vec![bound],
        vec![],
    );
    assert!(matches!(
        store.record_expansion_activation(&bad_subject),
        Err(AuthorityStoreError::ExpansionActivationSubjectNotAdmitted(value))
            if value == missing_subject
    ));

    let unbound = d("p10-unbound-evidence");
    let bad_evidence = ExpansionActivationRecord::new(
        admitted,
        PromotionClass::StructureWitness,
        generation_digest,
        d("world"),
        ActivationMode::DefaultAutomatic,
        vec![unbound],
        vec![],
    );
    assert!(matches!(
        store.record_expansion_activation(&bad_evidence),
        Err(AuthorityStoreError::ExpansionActivationEvidenceNotAuthorityBound(value))
            if value == unbound
    ));
}

#[test]
fn supersession_ledger_round_trips_without_rewriting_history() {
    let dir = tempdir().unwrap();
    let subject = d("p10-superseded-subject");
    let successor = d("p10-successor");
    let evidence = d("p10-supersession-evidence");
    let scope = d("p10-supersession-scope");
    let generation = UniverseGeneration::new(
        0,
        None,
        vec![subject, successor],
        vec![evidence],
    );
    let generation_digest = generation.digest();
    let record = SupersessionRecord::new(
        subject,
        successor,
        SupersessionKind::SupersededBy,
        generation_digest,
        vec![scope],
        vec![evidence],
    );
    let record_digest = record.structural_digest();

    {
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        store.initialize_genesis(&generation).unwrap();
        let persisted = store.record_supersession(&record).unwrap();
        assert_eq!(persisted.structural_digest(), record_digest);
        let records = store.supersessions_for(subject).unwrap();
        assert_eq!(records, vec![record.clone()]);
    }

    let reopened = AuthorityStore::open(dir.path()).unwrap();
    let records = reopened.supersessions_for(subject).unwrap();
    assert_eq!(records, vec![record]);
    assert_eq!(records[0].structural_digest(), record_digest);
    assert_eq!(reopened.replay_generation(generation_digest).unwrap(), generation);
}

#[test]
fn rollback_reselects_historical_generation_without_deleting_newer_history() {
    let dir = tempdir().unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = u0.digest();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    store.initialize_genesis(&u0).unwrap();

    let u1_digest = promote_one(&mut store, &u0, d("p10-u1-artifact"), d("p10-u1-proof"));
    let u1 = store.replay_generation(u1_digest).unwrap();
    let u2_digest = promote_one(&mut store, &u1, d("p10-u2-artifact"), d("p10-u2-proof"));
    let u2 = store.replay_generation(u2_digest).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u2_digest));

    assert_eq!(store.select_active_generation(u0_digest).unwrap(), u0_digest);
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));
    assert_eq!(store.replay_generation(u1_digest).unwrap(), u1);
    assert_eq!(store.replay_generation(u2_digest).unwrap(), u2);

    assert_eq!(store.select_active_generation(u2_digest).unwrap(), u2_digest);
    assert_eq!(store.active_generation().unwrap(), Some(u2_digest));
    assert_eq!(store.replay_generation(u2_digest).unwrap().digest(), u2_digest);
}
