use formula_store::{BlobStore, GenerationIndex};
use std::{fs, io};

#[test]
fn interrupted_pointer_update_preserves_active_generation() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-atomicity-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let old = store.put(b"old").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 1, old).unwrap();

    let pending = root
        .join("index")
        .join(format!("active-generation.tmp-{}", std::process::id()));
    fs::write(&pending, b"simulated interrupted write").unwrap();
    assert_eq!(
        index.publish(&store, 2, next).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
    assert_eq!(index.historical_manifest(2).unwrap(), next);
    assert_eq!(index.active_generation().unwrap(), (1, old));

    fs::remove_file(pending).unwrap();
    index.publish(&store, 2, next).unwrap();
    assert_eq!(index.active_generation().unwrap(), (2, next));
    assert_eq!(index.historical_manifest(1).unwrap(), old);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn corrupt_historical_manifest_cannot_activate_new_generation() {
    let root = std::env::temp_dir().join(format!("formula-corrupt-history-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let second = store.put(b"second").unwrap();
    index.publish(&store, 1, first).unwrap();

    let history = root.join("index/generations/2.manifest");
    fs::write(&history, format!("{}\n", second.to_hex())).unwrap();
    assert_eq!(
        index.publish(&store, 2, second).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
    assert_eq!(index.active_generation().unwrap(), (1, first));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn older_generation_cannot_replace_active_generation() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-rollback-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older").unwrap();
    let current = store.put(b"current").unwrap();
    let newer = store.put(b"newer").unwrap();

    index.publish(&store, 5, current).unwrap();
    assert_eq!(
        index.publish(&store, 4, older).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(!root.join("index/generations/4.manifest").exists());
    assert_eq!(index.active_generation().unwrap(), (5, current));

    index.publish(&store, 6, newer).unwrap();
    assert_eq!(index.active_generation().unwrap(), (6, newer));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn corrupt_active_pointer_blocks_new_publication() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-corrupt-active-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current").unwrap();
    let newer = store.put(b"newer").unwrap();

    index.publish(&store, 5, current).unwrap();
    fs::write(root.join("index/active-generation"), b"corrupt").unwrap();
    assert_eq!(
        index.publish(&store, 6, newer).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/6.manifest").exists());
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn symlinked_active_pointer_cannot_publish_or_replay() {
    use std::os::unix::fs::symlink;

    let root = std::env::temp_dir().join(format!(
        "formula-generation-symlink-active-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 5, current).unwrap();

    let pointer = root.join("index/active-generation");
    let external = root.join("external-pointer");
    fs::write(&external, format!("5\n{}", current.to_hex())).unwrap();
    fs::remove_file(&pointer).unwrap();
    symlink(&external, &pointer).unwrap();

    assert_eq!(
        index.publish(&store, 6, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/6.manifest").exists());
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    fs::remove_dir_all(root).unwrap();
}
