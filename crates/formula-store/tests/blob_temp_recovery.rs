use formula_store::BlobStore;
use std::fs;

#[test]
fn stale_blob_temp_does_not_block_new_immutable_put() {
    let root = std::env::temp_dir().join(format!("formula-stale-blob-temp-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let bytes = b"immutable after interrupted temp write";
    let digest = formula_core::ArtifactDigest::sha256(bytes);
    let leaf = store.blob_path(digest);
    fs::create_dir_all(leaf.parent().unwrap()).unwrap();
    let stale = leaf.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&stale, b"previous interrupted blob write").unwrap();
    let stale_slot = leaf.with_extension(format!("tmp-{}-0", std::process::id()));
    fs::write(&stale_slot, b"previous interrupted slot zero").unwrap();

    assert_eq!(store.put(bytes).unwrap(), digest);
    assert_eq!(store.get(digest).unwrap(), bytes);
    assert_eq!(fs::read(stale).unwrap(), b"previous interrupted blob write");
    assert_eq!(
        fs::read(stale_slot).unwrap(),
        b"previous interrupted slot zero"
    );
    fs::remove_dir_all(root).unwrap();
}
