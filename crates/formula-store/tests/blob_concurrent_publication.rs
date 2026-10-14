use formula_core::ArtifactDigest;
use formula_store::BlobStore;
use std::fs;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[test]
fn competing_destination_is_never_overwritten_by_blob_publication() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("formula-blob-race-{}-{nonce}", std::process::id()));
    let store_root = root.join("store");
    let bytes = vec![0x5a; 32 * 1024 * 1024];
    let digest = ArtifactDigest::sha256(&bytes);
    let leaf = BlobStore::new(&store_root).blob_path(digest);
    let parent = leaf.parent().unwrap();
    fs::create_dir_all(parent).unwrap();

    let writer = thread::spawn(move || BlobStore::new(store_root).put(&bytes));
    let start = Instant::now();
    let mut observed_temp = false;
    while start.elapsed() < Duration::from_secs(10) {
        if fs::read_dir(parent)
            .unwrap()
            .flatten()
            .any(|entry| entry.file_name().to_string_lossy().contains(".tmp-"))
        {
            observed_temp = true;
            break;
        }
        thread::yield_now();
    }
    assert!(
        observed_temp,
        "writer never exposed its temporary publication stage"
    );
    fs::write(&leaf, b"competing-writer-evidence").unwrap();
    let outcome = writer.join().unwrap();
    assert!(
        outcome.is_err(),
        "publication must reject a conflicting destination"
    );
    assert_eq!(fs::read(&leaf).unwrap(), b"competing-writer-evidence");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn simultaneous_identical_writers_reuse_one_verified_immutable_blob() {
    use std::sync::{Arc, Barrier};

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "formula-blob-identical-race-{}-{nonce}",
        std::process::id()
    ));
    let store_root = root.join("store");
    let payload = Arc::new(vec![0xa7; 512 * 1024]);
    let digest = ArtifactDigest::sha256(payload.as_slice());
    let barrier = Arc::new(Barrier::new(5));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let store_root = store_root.clone();
            let payload = Arc::clone(&payload);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                BlobStore::new(store_root).put(payload.as_slice())
            })
        })
        .collect();
    barrier.wait();
    for handle in handles {
        assert_eq!(handle.join().unwrap().unwrap(), digest);
    }

    let store = BlobStore::new(&store_root);
    assert_eq!(store.get(digest).unwrap(), payload.as_slice());
    let leaf = store.blob_path(digest);
    assert!(
        !fs::read_dir(leaf.parent().unwrap())
            .unwrap()
            .flatten()
            .any(|entry| entry.file_name().to_string_lossy().contains(".tmp-")),
        "successful concurrent publication must not leave owned temp slots"
    );
    fs::remove_dir_all(root).unwrap();
}
