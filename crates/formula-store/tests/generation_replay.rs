use formula_core::{digest::ArtifactDigest, generation::UniverseGeneration};
use formula_store::{authority_store::AuthorityStore, blob_store::BlobStore};
use std::fs;
use tempfile::tempdir;

fn d(value: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(value)
}

#[test]
fn successful_generation_publish_moves_active_root_once() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![d(b"base")], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));

    let u1 = UniverseGeneration::new(
        1,
        Some(u0_digest),
        vec![d(b"base"), d(b"new")],
        vec![d(b"proof")],
    );
    let u1_digest = store.publish_generation(&u1).unwrap();
    assert_eq!(store.active_generation().unwrap(), Some(u1_digest));
    assert_eq!(
        store.replay_generation(u0_digest).unwrap().digest(),
        u0_digest
    );
}

#[test]
fn wrong_parent_cannot_publish() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();

    let bad = UniverseGeneration::new(1, Some(d(b"not-active")), vec![d(b"x")], vec![]);
    assert!(store.publish_generation(&bad).is_err());
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));
}

#[test]
fn historical_roots_replay_after_fresh_store_open() {
    let dir = tempdir().unwrap();
    let (u0_digest, u1_digest) = {
        let mut store = AuthorityStore::open(dir.path()).unwrap();
        let u0 = UniverseGeneration::new(0, None, vec![d(b"a")], vec![]);
        let u0_digest = store.initialize_genesis(&u0).unwrap();
        let u1 = UniverseGeneration::new(
            1,
            Some(u0_digest),
            vec![d(b"a"), d(b"b")],
            vec![d(b"proof-b")],
        );
        let u1_digest = store.publish_generation(&u1).unwrap();
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
