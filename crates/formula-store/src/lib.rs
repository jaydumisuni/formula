//! Content-addressed immutable blob storage for Formula P1.
//!
//! This slice stores exact bytes under their SHA-256 digest and verifies bytes
//! again on read. It does not publish generations or grant mathematical authority.

use formula_core::{ArtifactDigest, UniverseGeneration};
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
        fs::File::open(path.parent().expect("blob path has parent"))?.sync_all()?;
        Ok(digest)
    }

    /// Persist one canonical universe-generation manifest as an immutable blob.
    ///
    /// This records manifest bytes only; it does not activate or publish the generation.
    pub fn put_generation_manifest(
        &self,
        generation: &UniverseGeneration,
    ) -> io::Result<ArtifactDigest> {
        self.put(&generation.canonical_bytes())
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

    /// Return whether an exact immutable blob is present and still valid.
    pub fn contains(&self, digest: ArtifactDigest) -> io::Result<bool> {
        let path = self.blob_path(digest);
        if !path.exists() {
            return Ok(false);
        }
        self.verify_existing(&path, digest)?;
        Ok(true)
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

/// Local generation index with an atomically replaced active-generation pointer.
///
/// Publication requires the manifest blob to exist and verify first. Historical
/// generation roots are append-only files; the active pointer is replaced only
/// after that durable history entry is synced.
#[derive(Debug)]
pub struct GenerationIndex {
    root: PathBuf,
}

impl GenerationIndex {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn publish(
        &self,
        store: &BlobStore,
        generation: u64,
        manifest: ArtifactDigest,
    ) -> io::Result<()> {
        if !store.contains(manifest)? {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "generation manifest blob missing",
            ));
        }
        let generations = self.root.join("generations");
        fs::create_dir_all(&generations)?;
        let history = generations.join(format!("{generation}.manifest"));
        let manifest_hex = manifest.to_hex();
        if history.exists() {
            if fs::read_to_string(&history)? != manifest_hex {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "generation already bound to another manifest",
                ));
            }
        } else {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&history)?;
            file.write_all(manifest_hex.as_bytes())?;
            file.sync_all()?;
            fs::File::open(&generations)?.sync_all()?;
        }

        let active = self.root.join("active-generation");
        let temp = self
            .root
            .join(format!("active-generation.tmp-{}", std::process::id()));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        if let Err(error) = (|| -> io::Result<()> {
            write!(file, "{generation}\n{manifest_hex}")?;
            file.sync_all()?;
            fs::rename(&temp, &active)?;
            fs::File::open(&self.root)?.sync_all()?;
            Ok(())
        })() {
            let _ = fs::remove_file(&temp);
            return Err(error);
        }
        Ok(())
    }

    pub fn historical_manifest(&self, generation: u64) -> io::Result<ArtifactDigest> {
        let hex = fs::read_to_string(
            self.root
                .join("generations")
                .join(format!("{generation}.manifest")),
        )?;
        ArtifactDigest::from_hex(hex.trim()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid historical manifest digest",
            )
        })
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
    fn generation_manifest_roundtrips_as_canonical_immutable_bytes() {
        let root = test_root("generation-manifest");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let generation = UniverseGeneration::new(
            7,
            Some(ArtifactDigest::sha256(b"parent")),
            ArtifactDigest::sha256(b"world"),
            ArtifactDigest::sha256(b"authority"),
            vec![ArtifactDigest::sha256(b"evidence")],
            vec![ArtifactDigest::sha256(b"realization")],
        );
        let expected = generation.canonical_bytes();
        let digest = store.put_generation_manifest(&generation).unwrap();
        assert_eq!(digest, ArtifactDigest::sha256(&expected));
        assert_eq!(store.get(digest).unwrap(), expected);
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
    fn contains_distinguishes_missing_valid_and_corrupt_blobs() {
        let root = test_root("contains");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let missing = ArtifactDigest::sha256(b"missing");
        assert!(!store.contains(missing).unwrap());
        let digest = store.put(b"present").unwrap();
        assert!(store.contains(digest).unwrap());
        fs::write(store.blob_path(digest), b"corrupt").unwrap();
        assert_eq!(
            store.contains(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generation_publication_requires_present_manifest_and_preserves_history() {
        let root = test_root("generation-index");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let missing = ArtifactDigest::sha256(b"missing");
        assert_eq!(
            index.publish(&store, 1, missing).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
        assert!(!root.join("index/active-generation").exists());

        let manifest = store.put(b"generation-one").unwrap();
        index.publish(&store, 1, manifest).unwrap();
        assert_eq!(index.historical_manifest(1).unwrap(), manifest);
        assert_eq!(
            fs::read_to_string(root.join("index/active-generation")).unwrap(),
            format!("1\n{}", manifest.to_hex())
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generation_history_cannot_be_rebound() {
        let root = test_root("generation-rebind");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let first = store.put(b"first").unwrap();
        let second = store.put(b"second").unwrap();
        index.publish(&store, 4, first).unwrap();
        assert_eq!(
            index.publish(&store, 4, second).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(index.historical_manifest(4).unwrap(), first);
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
