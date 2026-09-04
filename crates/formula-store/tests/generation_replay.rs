use formula_check::promotion::{PromotionAuthorization, PromotionDecision, authorize_promotion_v1};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
};
use formula_store::{authority_store::AuthorityStore, blob_store::BlobStore};
use std::fs;
use tempfile::tempdir;

fn d(value: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(value)
}

fn authorization(
    parent: &UniverseGeneration,
    primitive: ArtifactDigest,
    evidence: ArtifactDigest,
) -> PromotionAuthorization {
    let parent_digest = parent.digest();
    let frozen = FrozenCandidate::new(
        "replay-test-primitive".into(),
        vec![primitive],
        d(b"world"),
        parent_digest,
        vec![],
        vec![],
        d(b"authority-contract"),
        d(b"observer"),
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
        vec![],
        vec![],
    );
    let decision = authorize_promotion_v1(
        &manifest,
        &frozen,
        &candidate,
        &[evidence],
        parent,
        &[],
    )
    .unwrap();
    let PromotionDecision::Authorized(authorization) = decision else {
        panic!("replay-test promotion was quarantined")
    };
    authorization
}

#[test]
fn successful_generation_publish_moves_active_root_once() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d(b"base")], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));

    let authorization = authorization(&u0, d(b"new"), d(b"proof"));
    let outcome = store.promote(&authorization).unwrap();
    let u1_digest = outcome.new_generation();
    assert_eq!(store.active_generation().unwrap(), Some(u1_digest));
    assert_eq!(
        store.replay_generation(u0_digest).unwrap().digest(),
        u0_digest
    );
}

#[test]
fn stale_parent_authorization_cannot_publish() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let first = authorization(&u0, d(b"first"), d(b"proof-first"));
    let stale = authorization(&u0, d(b"stale"), d(b"proof-stale"));

    let u1_digest = store.promote(&first).unwrap().new_generation();
    assert!(store.promote(&stale).is_err());
    assert_eq!(store.active_generation().unwrap(), Some(u1_digest));
    assert_eq!(store.replay_generation(u0_digest).unwrap().digest(), u0_digest);
}

#[test]
fn historical_roots_replay_after_fresh_store_open() {
    let dir = tempdir().unwrap();
    let (u0_digest, u1_digest) = {
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        let u0 = UniverseGeneration::new(0, None, vec![d(b"a")], vec![]);
        let u0_digest = store.initialize_genesis(&u0).unwrap();
        let authorization = authorization(&u0, d(b"b"), d(b"proof-b"));
        let u1_digest = store.promote(&authorization).unwrap().new_generation();
        (u0_digest, u1_digest)
    };

    let reopened = AuthorityStore::open(dir.path()).unwrap();
    assert_eq!(reopened.active_generation().unwrap(), Some(u1_digest));
    assert_eq!(
        reopened.replay_generation(u0_digest).unwrap().digest(),
        u0_digest
    );
    assert_eq!(
        reopened.replay_generation(u1_digest).unwrap().digest(),
        u1_digest
    );
}

#[test]
fn tampered_generation_manifest_is_rejected_during_replay() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d(b"a")], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let manifest = dir
        .path()
        .join("generations")
        .join(format!("{}.json", u0_digest.hex()));
    fs::write(manifest, b"tampered-generation-manifest").unwrap();

    assert!(store.replay_generation(u0_digest).is_err());
}

#[test]
fn tampered_manifest_blob_is_rejected_during_replay() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d(b"a")], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    let blobs = BlobStore::new(dir.path());
    fs::write(blobs.path_for(u0_digest), b"tampered-manifest-blob").unwrap();

    assert!(store.replay_generation(u0_digest).is_err());
}
