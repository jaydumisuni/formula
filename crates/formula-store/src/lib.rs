//! Content-addressed immutable blob storage for Formula P1.
//!
//! This slice stores exact bytes under their SHA-256 digest and verifies bytes
//! again on read. It does not publish generations or grant mathematical authority.

use formula_core::ArtifactDigest;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct BlobStore {
    root: PathBuf,
}

impl BlobStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Store exact bytes immutably and return their content digest.
    ///
    /// An existing digest path is accepted only when its bytes still hash to
    /// the same digest; mutation/corruption therefore fails closed.
    pub fn put(&self, bytes: &[u8]) -> io::Result<ArtifactDigest> {
        let digest = ArtifactDigest::sha256(bytes);
        let path = self.blob_path(digest);
        fs::create_dir_all(path.parent().expect("blob path has parent"))?;

        if path.exists() {
            self.verify_existing(&path, digest)?;
            return Ok(digest);
        }

        let temp = path.with_extension(format!("tmp-{}", std::process::id()));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        if let Err(error) = (|| -> io::Result<()> {
            file.write_all(bytes)?;
            file.sync_all()?;
            fs::rename(&temp, &path)?;
            Ok(())
        })() {
            let _ = fs::remove_file(&temp);
            return Err(error);
        }
        Ok(digest)
    }

    /// Read exact bytes only when their content still matches the requested digest.
    pub fn get(&self, digest: ArtifactDigest) -> io::Result<Vec<u8>> {
        let path = self.blob_path(digest);
        let bytes = fs::read(path)?;
        if ArtifactDigest::sha256(&bytes) != digest {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "blob digest mismatch",
            ));
        }
        Ok(bytes)
    }

    #[must_use]
    pub fn blob_path(&self, digest: ArtifactDigest) -> PathBuf {
        let hex = digest.to_hex();
        self.root.join("blobs").join(&hex[..2]).join(hex)
    }

    fn verify_existing(&self, path: &Path, expected: ArtifactDigest) -> io::Result<()> {
        let bytes = fs::read(path)?;
        if ArtifactDigest::sha256(&bytes) != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "existing blob digest mismatch",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("formula-store-{name}-{}", std::process::id()))
    }

    #[test]
    fn put_and_get_are_content_addressed() {
        let root = test_root("roundtrip");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let digest = store.put(b"canonical artifact").unwrap();
        assert_eq!(digest, ArtifactDigest::sha256(b"canonical artifact"));
        assert_eq!(store.get(digest).unwrap(), b"canonical artifact");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn duplicate_put_reuses_same_immutable_blob() {
        let root = test_root("duplicate");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let first = store.put(b"same bytes").unwrap();
        let second = store.put(b"same bytes").unwrap();
        assert_eq!(first, second);
        assert_eq!(store.get(first).unwrap(), b"same bytes");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn mutated_blob_is_rejected_on_read_and_reput() {
        let root = test_root("mutation");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let digest = store.put(b"original").unwrap();
        fs::write(store.blob_path(digest), b"tampered").unwrap();
        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.put(b"original").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }
}
