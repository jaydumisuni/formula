use formula_check::promotion::{PromotionDecision, authorize_promotion_v1};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionCandidate, PromotionState},
};
use formula_store::authority_store::{AuthorityStore, AuthorityStoreError};
use tempfile::tempdir;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

struct Fixture {
    parent: UniverseGeneration,
    primitive: ArtifactDigest,
    evidence: ArtifactDigest,
    authorization: formula_check::promotion::PromotionAuthorization,
}

fn fixture() -> Fixture {
    let dependency = d("admitted-dependency");
    let superseded = d("older-primitive");
    let primitive = d("fl-c-primitive");
    let evidence = d("checked-certificate");
    let parent = UniverseGeneration::new(0, None, vec![dependency, superseded], vec![]);
    let parent_digest = parent.digest();
    let frozen = FrozenCandidate::new(
        "fl-c-semantic-primitive".into(),
        vec![primitive],
        d("world"),
        parent_digest,
        vec![dependency],
        vec![],
        d("authority-contract"),
        d("observer"),
    );
    let manifest = PromotionManifest::new(
        parent_digest,
        frozen.structural_digest(),
        vec![evidence],
        vec![primitive],
        vec![evidence],
    );
    let candidate = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        parent_digest,
        parent_digest,
        vec![dependency],
        vec![superseded],
    );
    let decision =
        authorize_promotion_v1(&manifest, &frozen, &candidate, &[evidence], &parent, &[]).unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("valid promotion was quarantined")
    };
    Fixture {
        parent,
        primitive,
        evidence,
        authorization,
    }
}

#[test]
fn checked_authorization_advances_u0_to_u1_without_mutating_u0() {
    let f = fixture();
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = store.initialize_genesis(&f.parent).unwrap();
    let u0_bytes = f.parent.canonical_bytes();

    let outcome = store.promote(&f.authorization).unwrap();
    let u1 = outcome.new_generation();

    assert_eq!(outcome.parent_generation(), u0);
    assert_eq!(store.active_generation().unwrap(), Some(u1));

    let replayed_u0 = store.replay_generation(u0).unwrap();
    assert_eq!(replayed_u0.canonical_bytes(), u0_bytes);
    assert_eq!(replayed_u0.digest(), u0);
    assert!(!replayed_u0.admitted().contains(&f.primitive));
    assert!(!replayed_u0.authority_bindings().contains(&f.evidence));

    let replayed_u1 = store.replay_generation(u1).unwrap();
    assert_eq!(replayed_u1.generation_number(), 1);
    assert_eq!(replayed_u1.parent(), Some(u0));
    assert!(replayed_u1.admitted().contains(&f.primitive));
    assert!(replayed_u1.authority_bindings().contains(&f.evidence));

    let admitted = outcome.admitted_record();
    assert_eq!(admitted.state(), PromotionState::Admitted);
    assert_eq!(admitted.generation(), u1);
    assert_eq!(admitted.candidate(), f.authorization.promotion_candidate());
    assert_eq!(admitted.policy(), f.authorization.policy_digest());
    assert_eq!(admitted.evidence(), f.authorization.authority_bindings());
    assert_eq!(
        admitted.semantic_artifacts(),
        f.authorization.proposed_admissions()
    );
}

#[test]
fn stale_authorization_cannot_advance_after_parent_has_changed() {
    let first = fixture();
    let stale = fixture();
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = store.initialize_genesis(&first.parent).unwrap();
    let first_outcome = store.promote(&first.authorization).unwrap();
    let u1 = first_outcome.new_generation();

    let error = store.promote(&stale.authorization).unwrap_err();
    assert!(matches!(
        error,
        AuthorityStoreError::ParentMismatch {
            expected: Some(expected),
            actual: Some(actual),
        } if expected == u1 && actual == u0
    ));
    assert_eq!(store.active_generation().unwrap(), Some(u1));
    assert_eq!(store.replay_generation(u0).unwrap().digest(), u0);
    assert_eq!(store.replay_generation(u1).unwrap().digest(), u1);
}
