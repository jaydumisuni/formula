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

#[test]
fn verified_replay_rejects_corrupt_older_manifest_blob() {
    let root = std::env::temp_dir().join(format!(
        "formula-verified-replay-corrupt-older-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();
    assert_eq!(
        index.verified_active_generation(&store).unwrap(),
        (2, current)
    );

    fs::write(store.blob_path(older), b"tampered").unwrap();
    assert_eq!(index.active_generation().unwrap(), (2, current));
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn verified_replay_rejects_missing_older_manifest_blob() {
    let root = std::env::temp_dir().join(format!(
        "formula-verified-replay-missing-older-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();

    fs::remove_file(store.blob_path(older)).unwrap();
    assert_eq!(index.active_generation().unwrap(), (2, current));
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn verified_replay_rejects_missing_active_manifest_blob() {
    let root = std::env::temp_dir().join(format!(
        "formula-verified-replay-missing-active-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let active = store.put(b"active manifest").unwrap();
    index.publish(&store, 3, active).unwrap();
    fs::remove_file(store.blob_path(active)).unwrap();
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn noncanonical_history_alias_cannot_be_replayed_or_published() {
    let root = std::env::temp_dir().join(format!("formula-history-alias-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current manifest").unwrap();
    let next = store.put(b"next manifest").unwrap();
    index.publish(&store, 2, current).unwrap();

    // A second spelling of the same generation is an ambiguous authority
    // identity even when its digest bytes match the canonical entry.
    fs::write(root.join("index/generations/02.manifest"), current.to_hex()).unwrap();
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 3, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/3.manifest").exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invisible_history_name_alias_blocks_successor_publication() {
    let root = std::env::temp_dir().join(format!(
        "formula-history-invisible-alias-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current manifest").unwrap();
    let next = store.put(b"next manifest").unwrap();
    index.publish(&store, 1, current).unwrap();

    for alias in [
        "1.0.manifest",
        "1e0.manifest",
        "0x1.manifest",
        "1_0.manifest",
        "1+0.manifest",
        "1..manifest",
        "1\u{200b}.manifest",
        "1\u{200a}.manifest",
        "1\u{2009}.manifest",
        "1\u{2007}.manifest",
        "1\u{200c}.manifest",
        "1\u{200d}.manifest",
        "1\u{00ad}.manifest",
        "1\u{202f}.manifest",
        "1\u{2060}.manifest",
        "1\u{2063}.manifest",
        "1\u{2064}.manifest",
        "1\u{e0020}.manifest",
        "1\u{e007f}.manifest",
        "1\u{fe0f}.manifest",
        "1\u{e0100}.manifest",
        "1\u{034f}.manifest",
        "1\u{180e}.manifest",
        "1\u{202a}.manifest",
        "1\u{202e}.manifest",
        "1\u{2066}.manifest",
        "1\u{2069}.manifest",
        "１.manifest",
        "١.manifest",  // Arabic-Indic digit one must not alias ASCII authority
        "۱.manifest",  // Extended Arabic-Indic digit one
        "१.manifest",  // Devanagari digit one
        "๑.manifest",  // Thai digit one must not alias ASCII authority
        "一.manifest", // CJK numeral one is not an ASCII generation
    ] {
        let path = root.join("index/generations").join(alias);
        fs::write(&path, current.to_hex()).unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.verified_active_generation(&store).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 2, next).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("index/generations/2.manifest").exists());
        fs::remove_file(path).unwrap();
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn uppercase_historical_digest_cannot_replay_or_authorize_successor() {
    let root = std::env::temp_dir().join(format!(
        "formula-history-uppercase-digest-{}",
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

    // A parseable alternate digest spelling is not canonical history authority.
    // The active pointer stays untouched; an older leaf alone must fail closed.
    fs::write(
        root.join("index/generations/1.manifest"),
        older.to_hex().to_uppercase(),
    )
    .unwrap();
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 3, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/3.manifest").exists());
    assert_eq!(
        fs::read_to_string(root.join("index/active-generation")).unwrap(),
        format!("2\n{}", current.to_hex())
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn historical_digest_with_trailing_newline_cannot_authorize_replay_or_successor() {
    let root = std::env::temp_dir().join(format!(
        "formula-history-trailing-newline-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    let successor = store.put(b"successor manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();

    // The historical leaf is an exact digest record, not a line-oriented
    // text file. The current active pointer must not conceal its mutation.
    let history = root.join("index/generations/1.manifest");
    fs::write(&history, format!("{}\n", older.to_hex())).unwrap();
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 3, successor).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/3.manifest").exists());
    assert_eq!(
        fs::read_to_string(root.join("index/active-generation")).unwrap(),
        format!("2\n{}", current.to_hex())
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn historical_digest_with_trailing_space_cannot_authorize_replay_or_successor() {
    let root = std::env::temp_dir().join(format!(
        "formula-history-trailing-space-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    let successor = store.put(b"successor manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();

    // The historical leaf is an exact digest record, not a whitespace-trimmed
    // token. The current active pointer must not conceal its mutation.
    let history = root.join("index/generations/1.manifest");
    fs::write(&history, format!("{} ", older.to_hex())).unwrap();
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 3, successor).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/3.manifest").exists());
    assert_eq!(
        fs::read_to_string(root.join("index/active-generation")).unwrap(),
        format!("2\n{}", current.to_hex())
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn historical_digest_control_byte_aliases_cannot_authorize_successor() {
    let root = std::env::temp_dir().join(format!(
        "formula-history-control-byte-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    let successor = store.put(b"successor manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();

    let history = root.join("index/generations/1.manifest");
    for suffix in ["\r", "\r\n", "\0", "\t", "\u{000b}", "\u{000c}"] {
        fs::write(&history, format!("{}{}", older.to_hex(), suffix)).unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.verified_active_generation(&store).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 3, successor).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("index/generations/3.manifest").exists());
        assert_eq!(
            fs::read_to_string(root.join("index/active-generation")).unwrap(),
            format!("2\n{}", current.to_hex())
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn historical_digest_unicode_format_aliases_fail_closed() {
    let root = std::env::temp_dir().join(format!(
        "formula-history-format-alias-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let older = store.put(b"older manifest").unwrap();
    let current = store.put(b"current manifest").unwrap();
    let successor = store.put(b"successor manifest").unwrap();
    index.publish(&store, 1, older).unwrap();
    index.publish(&store, 2, current).unwrap();
    let history = root.join("index/generations/1.manifest");
    for suffix in ["\u{feff}", "\u{2060}", "\u{200b}", "\u{fe0f}", "\u{e0100}"] {
        fs::write(&history, format!("{}{}", older.to_hex(), suffix)).unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.verified_active_generation(&store).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 3, successor).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("index/generations/3.manifest").exists());
        assert_eq!(
            fs::read_to_string(root.join("index/active-generation")).unwrap(),
            format!("2\n{}", current.to_hex())
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn noncanonical_active_pointer_spellings_fail_closed() {
    let root = std::env::temp_dir().join(format!("formula-active-spelling-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current manifest").unwrap();
    let next = store.put(b"next manifest").unwrap();
    index.publish(&store, 1, current).unwrap();

    let pointer = root.join("index/active-generation");
    let canonical = format!("1\n{}", current.to_hex());
    let alternatives = [
        format!("01\n{}", current.to_hex()),
        format!("+1\n{}", current.to_hex()),
        format!("-1\n{}", current.to_hex()),
        format!(" 1\n{}", current.to_hex()),
        format!("1.0\n{}", current.to_hex()),
        format!("1e0\n{}", current.to_hex()),
        format!("0x1\n{}", current.to_hex()),
        format!("1\n{} ", current.to_hex()),

        format!("1 \n{}", current.to_hex()),
        format!("{canonical}\n"),
        format!("1\r\n{}", current.to_hex()),
        format!("1\n{}\r", current.to_hex()),
        format!("1\n{}\0", current.to_hex()),
        format!("1\t\n{}", current.to_hex()),
        format!("1\n\t{}", current.to_hex()),
        format!("1\u{000b}\n{}", current.to_hex()),
        format!("1\u{000c}\n{}", current.to_hex()),
        format!("１\n{}", current.to_hex()),
        format!("\u{0661}\n{}", current.to_hex()),
        format!("\u{1d7cf}\n{}", current.to_hex()),
        format!("1\0\n{}", current.to_hex()),
        format!("1\u{2028}{}", current.to_hex()),
        format!("1\u{2029}{}", current.to_hex()),
        format!("1\n\u{200b}{}", current.to_hex()),
        format!("1\n\u{200c}{}", current.to_hex()),
        format!("1\n\u{200d}{}", current.to_hex()),
        format!("1\u{fe0f}\n{}", current.to_hex()),
        format!("1\u{e0100}\n{}", current.to_hex()),
        format!("1\n\u{fe0f}{}", current.to_hex()),
        format!("1\n\u{e0100}{}", current.to_hex()),
        format!("\u{feff}1\n{}", current.to_hex()),
        format!("1\n\u{feff}{}", current.to_hex()),
        format!("1\n\u{2060}{}", current.to_hex()),
        format!("1\n\u{2066}{}", current.to_hex()),
        format!("1\n\u{2067}{}", current.to_hex()),
        format!("1\n\u{2068}{}", current.to_hex()),
        format!("1\n\u{2069}{}", current.to_hex()),
        format!("1\n\u{202a}{}", current.to_hex()),
        format!("1\n\u{202b}{}", current.to_hex()),
        format!("1\n\u{202c}{}", current.to_hex()),
        format!("1\n\u{202d}{}", current.to_hex()),
        format!("1\n\u{202e}{}", current.to_hex()),
        format!("1\n\u{061c}{}", current.to_hex()),
        format!("1\n\u{200e}{}", current.to_hex()),
        format!("1\n\u{200f}{}", current.to_hex()),
        // Digest identity is lowercase ASCII hex, not merely a parseable hash.
        format!("1\n{}", current.to_hex().to_uppercase()),
        format!("1\n{}", {
            let mut hex = current.to_hex();
            if let Some((i, ch)) = hex.char_indices().find(|(_, ch)| ch.is_ascii_alphabetic()) {
                hex.replace_range(i..i + 1, &ch.to_ascii_uppercase().to_string());
            }
            hex
        }),
    ];
    for alternative in alternatives {
        fs::write(&pointer, &alternative).unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 2, next).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.verified_active_generation(&store).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(fs::read_to_string(&pointer).unwrap(), alternative);
        assert!(!root.join("index/generations/2.manifest").exists());
    }
    fs::remove_dir_all(root).unwrap();
}

#[cfg(target_os = "linux")]
#[test]
fn publication_waits_for_exclusive_index_directory_lock() {
    use std::os::fd::AsRawFd;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let root = std::env::temp_dir().join(format!(
        "formula-generation-publish-lock-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store_root = root.join("store");
    let index_root = root.join("index");
    let store = BlobStore::new(&store_root);
    let index = GenerationIndex::new(&index_root);
    let current = store.put(b"current manifest").unwrap();
    let next = store.put(b"next manifest").unwrap();
    index.publish(&store, 1, current).unwrap();

    // A second publisher must not inspect a stale active pointer and later
    // overwrite a newer committed generation. The index directory is the
    // stable serialization boundary; no disposable lock file is needed.
    let directory = fs::File::open(&index_root).unwrap();
    assert_eq!(
        unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_EX) },
        0
    );
    let (started_tx, started_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        started_tx.send(()).unwrap();
        let result = GenerationIndex::new(index_root).publish(&BlobStore::new(store_root), 2, next);
        done_tx.send(result).unwrap();
    });
    started_rx.recv_timeout(Duration::from_secs(2)).unwrap();
    assert!(
        done_rx.recv_timeout(Duration::from_millis(250)).is_err(),
        "publication bypassed the exclusive index directory lock"
    );
    assert_eq!(
        unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_UN) },
        0
    );
    worker.join().unwrap();
    done_rx
        .recv_timeout(Duration::from_secs(3))
        .unwrap()
        .unwrap();
    assert_eq!(index.active_generation().unwrap(), (2, next));
    fs::remove_dir_all(root).unwrap();
}

#[cfg(target_os = "linux")]
#[test]
fn concurrent_successor_publishers_preserve_monotonic_authority() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let root = std::env::temp_dir().join(format!(
        "formula-concurrent-successors-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let initial = store.put(b"initial generation").unwrap();
    let older = store.put(b"successor two").unwrap();
    let newer = store.put(b"successor three").unwrap();
    index.publish(&store, 1, initial).unwrap();

    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for (generation, digest) in [(2, older), (3, newer)] {
        let barrier = Arc::clone(&barrier);
        let root = root.clone();
        workers.push(thread::spawn(move || {
            barrier.wait();
            GenerationIndex::new(root.join("index")).publish(
                &BlobStore::new(root.join("store")),
                generation,
                digest,
            )
        }));
    }
    barrier.wait();
    let older_result = workers.remove(0).join().unwrap();
    let newer_result = workers.remove(0).join().unwrap();
    newer_result.unwrap();
    if let Err(error) = older_result {
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(!root.join("index/generations/2.manifest").exists());
    } else {
        assert_eq!(index.historical_manifest(2).unwrap(), older);
    }
    assert_eq!(index.historical_manifest(1).unwrap(), initial);
    assert_eq!(index.historical_manifest(3).unwrap(), newer);
    assert_eq!(
        index.verified_active_generation(&store).unwrap(),
        (3, newer)
    );
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn non_utf8_history_name_blocks_replay_and_successor_publication() {
    use std::os::unix::ffi::OsStringExt;
    let root = std::env::temp_dir().join(format!("formula-history-nonutf8-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let current = store.put(b"current").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 1, current).unwrap();
    // Invalid UTF-8 must fail closed whether it precedes or follows a
    // plausible generation number; neither spelling can be silently skipped.
    for bytes in [b"2\xff.manifest".as_slice(), b"\xff2.manifest".as_slice()] {
        let name = std::ffi::OsString::from_vec(bytes.to_vec());
        fs::write(root.join("index/generations").join(name), current.to_hex()).unwrap();
    }
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.verified_active_generation(&store).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/2.manifest").exists());
    fs::remove_dir_all(root).unwrap();
}
