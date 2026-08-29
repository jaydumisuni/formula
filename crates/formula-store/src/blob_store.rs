use formula_core::digest::ArtifactDigest;
use std::{
    error::Error,
    fmt, fs,
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum BlobStoreError {
    Io(io::Error),
    DigestMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    ExistingBlobConflict {
        digest: ArtifactDigest,
    },
}

impl fmt::Display for BlobStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "blob-store I/O error: {error}"),
            Self::DigestMismatch { expected, actual } => write!(
                f,
                "blob digest mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::ExistingBlobConflict { digest } => write!(
                f,
                "existing blob bytes conflict at content address {}",
                digest.as_str()
            ),
        }
    }
}

impl Error for BlobStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::DigestMismatch { .. } | Self::ExistingBlobConflict { .. } => None,
        }
    }
}

impl From<io::Error> for BlobStoreError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Clone, Debug)]
pub struct BlobStore {
    root: PathBuf,
}

impl BlobStore {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub fn path_for(&self, digest: ArtifactDigest) -> PathBuf {
        let hex = digest.hex();
        self.root
            .join("objects")
            .join("sha256")
            .join(&hex[..2])
            .join(&hex[2..])
    }

    pub fn put(&self, bytes: &[u8]) -> Result<ArtifactDigest, BlobStoreError> {
        let digest = ArtifactDigest::of_bytes(bytes);
        let target = self.path_for(digest);

        if target.exists() {
            self.verify_existing(&target, digest, Some(bytes))?;
            return Ok(digest);
        }

        let parent = target.parent().expect("blob path always has a parent");
        fs::create_dir_all(parent)?;
        let temporary = self.create_staged_file(parent, &target, bytes)?;

        match fs::hard_link(&temporary, &target) {
            Ok(()) => {
                fs::remove_file(&temporary)?;
                self.verify_existing(&target, digest, Some(bytes))?;
                Ok(digest)
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let _ = fs::remove_file(&temporary);
                self.verify_existing(&target, digest, Some(bytes))?;
                Ok(digest)
            }
            Err(error) => {
                let _ = fs::remove_file(&temporary);
                Err(BlobStoreError::Io(error))
            }
        }
    }

    pub fn get(&self, digest: ArtifactDigest) -> Result<Vec<u8>, BlobStoreError> {
        let path = self.path_for(digest);
        let bytes = fs::read(&path)?;
        let actual = ArtifactDigest::of_bytes(&bytes);
        if actual != digest {
            return Err(BlobStoreError::DigestMismatch {
                expected: digest,
                actual,
            });
        }
        Ok(bytes)
    }

    pub fn contains(&self, digest: ArtifactDigest) -> Result<bool, BlobStoreError> {
        let path = self.path_for(digest);
        if !path.exists() {
            return Ok(false);
        }
        self.get(digest)?;
        Ok(true)
    }

    fn verify_existing(
        &self,
        path: &Path,
        expected_digest: ArtifactDigest,
        expected_bytes: Option<&[u8]>,
    ) -> Result<(), BlobStoreError> {
        let existing = fs::read(path)?;
        let actual = ArtifactDigest::of_bytes(&existing);
        if actual != expected_digest {
            return Err(BlobStoreError::DigestMismatch {
                expected: expected_digest,
                actual,
            });
        }
        if expected_bytes.is_some_and(|bytes| bytes != existing) {
            return Err(BlobStoreError::ExistingBlobConflict {
                digest: expected_digest,
            });
        }
        Ok(())
    }

    fn create_staged_file(
        &self,
        parent: &Path,
        target: &Path,
        bytes: &[u8],
    ) -> Result<PathBuf, BlobStoreError> {
        let target_name = target
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("blob");

        loop {
            let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
            let temporary = parent.join(format!(
                ".{target_name}.tmp-{}-{counter}",
                std::process::id()
            ));
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
            {
                Ok(mut file) => {
                    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
                        let _ = fs::remove_file(&temporary);
                        return Err(BlobStoreError::Io(error));
                    }
                    return Ok(temporary);
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(BlobStoreError::Io(error)),
            }
        }
    }
}
