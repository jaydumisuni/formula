//! P1 D2-P02: independent publishers must not rebind one generation.
//! This is construction evidence only, not a Formula authority admission.

#[cfg(target_os = "linux")]
#[test]
fn competing_generation_publishers_admit_exactly_one_manifest() {
    use formula_store::{BlobStore, GenerationIndex};
    use std::fs;
    use std::io;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "formula-p1-competing-publishers-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&root).expect("create isolated publication fixture");
    let store = Arc::new(BlobStore::new(root.join("store")));
    let index = Arc::new(GenerationIndex::new(root.join("index")));
    let first = store
        .put(b"first independent manifest")
        .expect("first blob");
    let second = store
        .put(b"second independent manifest")
        .expect("second blob");
    assert_ne!(first, second);

    let gate = Arc::new(Barrier::new(3));
    let publishers: Vec<_> = [first, second]
        .into_iter()
        .map(|manifest| {
            let gate = Arc::clone(&gate);
            let store = Arc::clone(&store);
            let index = Arc::clone(&index);
            thread::spawn(move || {
                gate.wait();
                index.publish(&store, 42, manifest)
            })
        })
        .collect();
    gate.wait();

    let results: Vec<_> = publishers
        .into_iter()
        .map(|publisher| publisher.join().expect("publisher thread panicked"))
        .collect();
    assert_eq!(
        results.iter().filter(|result| result.is_ok()).count(),
        1,
        "only one distinct manifest may bind generation 42"
    );
    for rejected in results.iter().filter_map(|result| result.as_ref().err()) {
        assert_eq!(rejected.kind(), io::ErrorKind::AlreadyExists);
    }
    let winner = if results[0].is_ok() { first } else { second };
    assert_eq!(index.historical_manifest(42).unwrap(), winner);
    assert_eq!(
        index.verified_active_generation(&store).unwrap(),
        (42, winner)
    );
    fs::remove_dir_all(root).expect("remove isolated fixture");
}
