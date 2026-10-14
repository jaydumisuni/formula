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
#[test]
fn exhausted_blob_temp_slots_fail_closed_without_overwriting_prior_writers() {
    let root = std::env::temp_dir().join(format!("formula-exhausted-temp-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let bytes = b"fresh immutable payload";
    let digest = formula_core::ArtifactDigest::sha256(bytes);
    let leaf = store.blob_path(digest);
    fs::create_dir_all(leaf.parent().unwrap()).unwrap();
    let slots: Vec<_> = (0..32)
        .map(|suffix| {
            let path = leaf.with_extension(format!("tmp-{}-{suffix}", std::process::id()));
            fs::write(&path, b"interrupted-writer-evidence").unwrap();
            path
        })
        .collect();
    assert_eq!(
        store.put(bytes).unwrap_err().kind(),
        std::io::ErrorKind::AlreadyExists
    );
    assert!(!leaf.exists());
    for slot in &slots {
        assert_eq!(fs::read(slot).unwrap(), b"interrupted-writer-evidence");
    }
    fs::remove_file(&slots[0]).unwrap();
    assert_eq!(store.put(bytes).unwrap(), digest);
    assert_eq!(store.get(digest).unwrap(), bytes);
    for slot in slots.iter().skip(1) {
        assert_eq!(fs::read(slot).unwrap(), b"interrupted-writer-evidence");
    }
    fs::remove_dir_all(root).unwrap();
}
