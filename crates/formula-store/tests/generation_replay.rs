use formula_core::{digest::ArtifactDigest, generation::UniverseGeneration};
use formula_store::authority_store::AuthorityStore;
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
    assert_eq!(store.replay_generation(u0_digest).unwrap().digest(), u0_digest);
}

#[test]
fn wrong_parent_cannot_publish() {
    let dir = tempdir().unwrap();
    let mut store = AuthorityStore::open(dir.path()).unwrap();
    let u0 = UniverseGeneration::new(0, None, vec![], vec![]);
    let u0_digest = store.initialize_genesis(&u0).unwrap();

    let bad = UniverseGeneration::new(
        1,
        Some(d(b"not-active")),
        vec![d(b"x")],
        vec![],
    );
    assert!(store.publish_generation(&bad).is_err());
    assert_eq!(store.active_generation().unwrap(), Some(u0_digest));
}
