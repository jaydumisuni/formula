#![cfg(target_os = "linux")]

use formula_store::{BlobStore, GenerationIndex};
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileTypeExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn test_root(label: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "formula-generation-{label}-{}-{nonce}",
        std::process::id()
    ))
}

fn replace_with_fifo(path: &std::path::Path) {
    fs::remove_file(path).unwrap();
    let name = CString::new(path.as_os_str().as_bytes()).unwrap();
    // SAFETY: name is a live, NUL-terminated pathname.
    assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
}

#[test]
fn fifo_active_pointer_cannot_block_replay_or_publish() {
    let root = test_root("fifo-active");
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 1, first).unwrap();

    let pointer = root.join("index/active-generation");
    replace_with_fifo(&pointer);
    assert_eq!(
        index.active_generation().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert_eq!(
        index.publish(&store, 2, next).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(!root.join("index/generations/2.manifest").exists());
    assert!(
        fs::symlink_metadata(&pointer)
            .unwrap()
            .file_type()
            .is_fifo()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn fifo_history_leaf_cannot_block_replay_or_publish() {
    let root = test_root("fifo-history");
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let first = store.put(b"first").unwrap();
    let next = store.put(b"next").unwrap();
    index.publish(&store, 1, first).unwrap();

    let history = root.join("index/generations/1.manifest");
    replace_with_fifo(&history);
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
    assert!(
        fs::symlink_metadata(&history)
            .unwrap()
            .file_type()
            .is_fifo()
    );
    fs::remove_dir_all(root).unwrap();
}
