use formula_core::digest::ArtifactDigest;
use formula_store::blob_store::{BlobStore, BlobStoreError};
use std::fs;
use tempfile::tempdir;

#[test]
fn blob_path_matches_d3_layout() {
    let dir = tempdir().unwrap();
    let store = BlobStore::new(dir.path());
    let digest = ArtifactDigest::of_bytes(b"abc");
    let hex = digest.hex();
    assert_eq!(
        store.path_for(digest),
        dir.path()
            .join("objects/sha256")
            .join(&hex[..2])
            .join(&hex[2..])
    );
}

#[test]
fn put_is_idempotent_and_read_is_verified() {
    let dir = tempdir().unwrap();
    let store = BlobStore::new(dir.path());
    let first = store.put(b"authority bytes").unwrap();
    let second = store.put(b"authority bytes").unwrap();
    assert_eq!(first, second);
    assert!(store.contains(first).unwrap());
    assert_eq!(store.get(first).unwrap(), b"authority bytes");
}

#[test]
fn mutated_blob_is_rejected() {
    let dir = tempdir().unwrap();
    let store = BlobStore::new(dir.path());
    let digest = store.put(b"authority bytes").unwrap();
    fs::write(store.path_for(digest), b"tampered").unwrap();
    assert!(matches!(
        store.get(digest),
        Err(BlobStoreError::DigestMismatch { .. })
    ));
}
