#![cfg(target_os = "linux")]

use formula_core::ArtifactDigest;
use formula_store::BlobStore;
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileTypeExt;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn fifo_blob_leaf_is_rejected_without_blocking_or_replacement() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root =
        std::env::temp_dir().join(format!("formula-fifo-blob-{}-{nonce}", std::process::id()));
    let store = BlobStore::new(&root);
    let bytes = b"immutable artifact cannot be a fifo";
    let digest = ArtifactDigest::sha256(bytes);
    let leaf = store.blob_path(digest);
    fs::create_dir_all(leaf.parent().unwrap()).unwrap();

    let name = CString::new(leaf.as_os_str().as_bytes()).unwrap();
    // SAFETY: name is a live, NUL-terminated pathname for mkfifo.
    assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);

    for result in [
        store.get(digest).map(|_| ()),
        store.contains(digest).map(|_| ()),
        store.put(bytes).map(|_| ()),
    ] {
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }
    assert!(fs::symlink_metadata(&leaf).unwrap().file_type().is_fifo());
    fs::remove_dir_all(root).unwrap();
}
