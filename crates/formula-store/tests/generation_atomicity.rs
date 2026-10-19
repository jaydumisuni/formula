use formula_store::{BlobStore, GenerationIndex};
use std::{fs, io};

#[test]
fn stale_pointer_temp_does_not_block_atomic_publication() {
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
        .join(format!("active-generation.tmp-{}-0", std::process::id()));
    fs::write(&pending, b"simulated interrupted write").unwrap();
    assert_eq!(index.active_generation().unwrap(), (1, old));

    // The stale temp remains untouched while a fresh slot publishes atomically.
    index.publish(&store, 2, next).unwrap();
    assert_eq!(fs::read(&pending).unwrap(), b"simulated interrupted write");
    assert_eq!(index.historical_manifest(2).unwrap(), next);
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
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&history, fs::Permissions::from_mode(0o600)).unwrap();
    }
    assert_eq!(
        index.publish(&store, 2, second).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );

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

#[cfg(unix)]
#[test]
fn symlinked_history_entry_rejected_for_replay_and_publication() {
    use std::os::unix::fs::symlink;

    let root = std::env::temp_dir().join(format!("formula-history-symlink-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let digest = store.put(b"manifest").unwrap();
    index.publish(&store, 1, digest).unwrap();

    let history = root.join("index/generations/1.manifest");
    let external = root.join("external-history");
    fs::write(&external, digest.to_hex()).unwrap();
    fs::remove_file(&history).unwrap();
    symlink(&external, &history).unwrap();

    assert_eq!(
        index.historical_manifest(1).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 1, digest).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(fs::read_to_string(external).unwrap(), digest.to_hex());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn nonregular_history_entry_rejected() {
    let root = std::env::temp_dir().join(format!("formula-history-dir-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let digest = store.put(b"manifest").unwrap();
    index.publish(&store, 1, digest).unwrap();
    let history = root.join("index/generations/1.manifest");
    fs::remove_file(&history).unwrap();
    fs::create_dir(&history).unwrap();
    assert_eq!(
        index.historical_manifest(1).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 1, digest).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn symlinked_generation_index_root_cannot_publish_or_replay() {
    use std::os::unix::fs::symlink;

    let root = std::env::temp_dir().join(format!(
        "formula-generation-symlink-root-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("outside/generations")).unwrap();
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current").unwrap();
    let next = store.put(b"next").unwrap();
    fs::write(
        root.join("outside/generations/1.manifest"),
        current.to_hex(),
    )
    .unwrap();
    fs::write(
        root.join("outside/active-generation"),
        format!("1\n{}", current.to_hex()),
    )
    .unwrap();
    symlink(root.join("outside"), root.join("index")).unwrap();

    assert_eq!(
        index.historical_manifest(1).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("outside/generations/2.manifest").exists());
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn symlinked_history_directory_cannot_publish_or_replay() {
    use std::os::unix::fs::symlink;

    let root = std::env::temp_dir().join(format!(
        "formula-generation-symlink-directory-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let second = store.put(b"second").unwrap();
    index.publish(&store, 1, first).unwrap();

    let history_dir = root.join("index/generations");
    let external_dir = root.join("external-history");
    fs::rename(&history_dir, &external_dir).unwrap();
    symlink(&external_dir, &history_dir).unwrap();

    assert_eq!(
        index.historical_manifest(1).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, second).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!external_dir.join("2.manifest").exists());
    assert_eq!(
        fs::read_to_string(external_dir.join("1.manifest")).unwrap(),
        first.to_hex()
    );
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn hardlinked_history_entry_cannot_publish_or_replay() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-hardlink-history-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let second = store.put(b"second").unwrap();
    index.publish(&store, 1, first).unwrap();

    let history = root.join("index/generations/1.manifest");
    let external = root.join("external-history.manifest");
    fs::hard_link(&history, &external).unwrap();

    assert_eq!(
        index.historical_manifest(1).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, second).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/2.manifest").exists());
    assert_eq!(fs::read_to_string(external).unwrap(), first.to_hex());
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn hardlinked_active_pointer_cannot_publish_or_replay() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-hardlink-active-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let second = store.put(b"second").unwrap();
    index.publish(&store, 1, first).unwrap();

    let pointer = root.join("index/active-generation");
    let external = root.join("external-active");
    fs::hard_link(&pointer, &external).unwrap();

    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, second).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/2.manifest").exists());
    assert_eq!(
        fs::read_to_string(external).unwrap(),
        format!("1\n{}", first.to_hex())
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn stale_history_temp_does_not_publish_partial_digest() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-history-temp-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let second = store.put(b"second").unwrap();
    index.publish(&store, 1, first).unwrap();

    let pending = root
        .join("index/generations")
        .join(format!("2.manifest.tmp-{}-0", std::process::id()));
    fs::write(&pending, b"interrupted partial history").unwrap();
    assert!(!root.join("index/generations/2.manifest").exists());

    index.publish(&store, 2, second).unwrap();
    assert_eq!(fs::read(&pending).unwrap(), b"interrupted partial history");
    assert_eq!(index.historical_manifest(2).unwrap(), second);
    assert_eq!(index.active_generation().unwrap(), (2, second));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn corrupt_future_history_cannot_mask_stale_active_generation() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-future-corrupt-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    index.publish(&store, 1, first).unwrap();

    // Atomic history publication cannot produce this leaf. Treat an injected
    // malformed higher generation as an integrity failure, not permission to
    // replay an older active pointer.
    fs::write(root.join("index/generations/2.manifest"), b"corrupt").unwrap();
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 1, first).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn concurrent_identical_history_publication_preserves_one_binding() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let root = std::env::temp_dir().join(format!(
        "formula-generation-history-concurrent-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = Arc::new(BlobStore::new(root.join("store")));
    let index = Arc::new(GenerationIndex::new(root.join("index")));
    let digest = store.put(b"concurrent manifest").unwrap();
    let barrier = Arc::new(Barrier::new(2));

    let workers: Vec<_> = (0..2)
        .map(|_| {
            let store = Arc::clone(&store);
            let index = Arc::clone(&index);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                index.publish(&store, 1, digest)
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap().unwrap();
    }

    assert_eq!(index.historical_manifest(1).unwrap(), digest);
    assert_eq!(index.active_generation().unwrap(), (1, digest));
    assert_eq!(
        fs::read_to_string(root.join("index/generations/1.manifest")).unwrap(),
        digest.to_hex()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn concurrent_conflicting_history_publication_cannot_rebind_generation() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let root = std::env::temp_dir().join(format!(
        "formula-generation-history-conflict-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = Arc::new(BlobStore::new(root.join("store")));
    let index = Arc::new(GenerationIndex::new(root.join("index")));
    let first = store.put(b"first candidate").unwrap();
    let second = store.put(b"second candidate").unwrap();
    let barrier = Arc::new(Barrier::new(2));

    let workers: Vec<_> = [first, second]
        .into_iter()
        .map(|digest| {
            let store = Arc::clone(&store);
            let index = Arc::clone(&index);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                index.publish(&store, 1, digest)
            })
        })
        .collect();
    let results: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();

    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| result
                .as_ref()
                .is_err_and(|error| error.kind() == io::ErrorKind::AlreadyExists))
            .count(),
        1
    );
    let bound = index.historical_manifest(1).unwrap();
    assert!(bound == first || bound == second);
    assert_eq!(index.active_generation().unwrap(), (1, bound));
    assert_eq!(
        fs::read_to_string(root.join("index/generations/1.manifest")).unwrap(),
        bound.to_hex()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn oversized_history_leaf_blocks_public_replay_and_publication() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-oversized-history-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 1, first).unwrap();

    let history = root.join("index/generations/1.manifest");
    let oversized = vec![b'a'; 129];
    fs::write(&history, &oversized).unwrap();

    assert_eq!(
        index.historical_manifest(1).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/2.manifest").exists());
    assert_eq!(fs::read(history).unwrap(), oversized);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn oversized_active_pointer_blocks_public_replay_and_publication() {
    let root = std::env::temp_dir().join(format!(
        "formula-generation-oversized-active-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 1, first).unwrap();

    let pointer = root.join("index/active-generation");
    let oversized = vec![b'a'; 129];
    fs::write(&pointer, &oversized).unwrap();

    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/2.manifest").exists());
    assert_eq!(fs::read(pointer).unwrap(), oversized);
    fs::remove_dir_all(root).unwrap();
}
