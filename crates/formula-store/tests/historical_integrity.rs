use formula_store::{BlobStore, GenerationIndex};
use std::{fs, io};

#[test]
fn corrupt_older_history_cannot_be_hidden_by_valid_active_generation() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-older-corrupt-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    let next = store.put(b"next manifest").unwrap();

    index.publish(&store, 3, older).unwrap();
    index.publish(&store, 5, current).unwrap();

    // The active pointer and its own history remain consistent. An older
    // immutable history leaf is nevertheless corrupt and must fail replay.
    fs::write(root.join("index/generations/3.manifest"), b"corrupt").unwrap();
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 6, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/6.manifest").exists());
    assert_eq!(
        fs::read_to_string(root.join("index/active-generation")).unwrap(),
        format!("5{}{}", char::from(10), current.to_hex())
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn missing_older_manifest_blob_blocks_new_generation_publication() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-missing-older-blob-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    let next = store.put(b"next manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();

    // A canonical history pointer is insufficient when its immutable blob
    // disappeared. A successor must not publish over this broken chain.
    fs::remove_file(store.blob_path(older)).unwrap();
    assert_eq!(
        index.publish(&store, 3, next).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
    assert!(!root.join("index/generations/3.manifest").exists());
    assert_eq!(index.historical_manifest(2).unwrap(), current);
    fs::remove_dir_all(root).unwrap();
}
