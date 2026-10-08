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
