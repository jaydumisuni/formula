use formula_check::promotion::{PromotionDecision, authorize_promotion_v1};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionCandidate, PromotionRecord, PromotionState},
};
use formula_store::authority_store::{AuthorityStore, AuthorityStoreError};
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn promote_one(
    store: &mut AuthorityStore,
    u0: &UniverseGeneration,
    primitive: ArtifactDigest,
    evidence: ArtifactDigest,
) -> (ArtifactDigest, PromotionRecord) {
    let u0_digest = u0.digest();
    let frozen = FrozenCandidate::new(
        "p9-semantic-primitive".into(),
        vec![primitive],
        d("p9-world"),
        u0_digest,
        vec![],
        vec![],
        d("p9-authority"),
        d("p9-observer"),
    );
    let manifest = PromotionManifest::new(
        u0_digest,
        frozen.structural_digest(),
        vec![evidence],
        vec![primitive],
        vec![evidence],
    );
    let promotion = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        u0_digest,
        u0_digest,
        vec![],
        vec![],
    );
    let decision =
        authorize_promotion_v1(&manifest, &frozen, &promotion, &[evidence], u0, &[]).unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("valid test promotion must authorize")
    };
    let outcome = store.promote(&authorization).unwrap();
    let activated = PromotionRecord::new(
        promotion.structural_digest(),
        PromotionState::Activated,
        outcome.new_generation(),
        authorization.policy_digest(),
        vec![evidence],
        vec![primitive],
    );
    (outcome.new_generation(), activated)
}

#[test]
fn activated_semantic_primitive_is_persisted_and_replayed() {
    let dir = tempdir().unwrap();
    let primitive = d("p9-primitive");
    let evidence = d("p9-evidence");
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);

    let activation_digest = {
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        store.initialize_genesis(&u0).unwrap();
        let (u1, activated) = promote_one(&mut store, &u0, primitive, evidence);

        let admitted = store
            .admit_semantic_activation(&activated, primitive)
            .unwrap();
        assert_eq!(admitted.structural_digest(), activated.structural_digest());

        let resolved = store
            .resolve_semantic_activation(u1, primitive)
            .unwrap()
            .expect("activation must resolve before reopen");
        assert_eq!(resolved.structural_digest(), activated.structural_digest());
        activated.structural_digest()
    };

    let reopened = AuthorityStore::open(dir.path()).unwrap();
    let active = reopened.active_generation().unwrap().unwrap();
    let resolved = reopened
        .resolve_semantic_activation(active, primitive)
        .unwrap()
        .expect("activation must survive reopen");
    assert_eq!(resolved.structural_digest(), activation_digest);
}

#[test]
fn semantic_activation_rejects_non_activated_state() {
    let dir = tempdir().unwrap();
    let primitive = d("p9-primitive-state");
    let evidence = d("p9-evidence-state");
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    store.initialize_genesis(&u0).unwrap();
    let (u1, activated) = promote_one(&mut store, &u0, primitive, evidence);
    let admitted = PromotionRecord::new(
        activated.candidate(),
        PromotionState::Admitted,
        u1,
        activated.policy(),
        activated.evidence().to_vec(),
        activated.semantic_artifacts().to_vec(),
    );

    let error = store
        .admit_semantic_activation(&admitted, primitive)
        .unwrap_err();
    assert!(matches!(
        error,
        AuthorityStoreError::SemanticActivationStateMismatch
    ));
}

#[test]
fn semantic_activation_rejects_unadmitted_primitive_and_unbound_evidence() {
    let dir = tempdir().unwrap();
    let primitive = d("p9-primitive-bindings");
    let evidence = d("p9-evidence-bindings");
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    store.initialize_genesis(&u0).unwrap();
    let (u1, activated) = promote_one(&mut store, &u0, primitive, evidence);

    let wrong_primitive = d("p9-not-admitted");
    let wrong_primitive_record = PromotionRecord::new(
        activated.candidate(),
        PromotionState::Activated,
        u1,
        activated.policy(),
        activated.evidence().to_vec(),
        vec![wrong_primitive],
    );
    let error = store
        .admit_semantic_activation(&wrong_primitive_record, wrong_primitive)
        .unwrap_err();
    assert!(matches!(
        error,
        AuthorityStoreError::SemanticActivationPrimitiveNotAdmitted(value) if value == wrong_primitive
    ));

    let unbound = d("p9-unbound-evidence");
    let wrong_evidence_record = PromotionRecord::new(
        activated.candidate(),
        PromotionState::Activated,
        u1,
        activated.policy(),
        vec![unbound],
        vec![primitive],
    );
    let error = store
        .admit_semantic_activation(&wrong_evidence_record, primitive)
        .unwrap_err();
    assert!(matches!(
        error,
        AuthorityStoreError::SemanticActivationEvidenceNotAuthorityBound(value) if value == unbound
    ));
}
