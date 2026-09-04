mod realization_store;

pub use realization_store::AdmittedRealization;

use crate::blob_store::{BlobStore, BlobStoreError};
use formula_core::{
    digest::{ArtifactDigest, DigestError},
    generation::UniverseGeneration,
};
use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use std::{
    error::Error,
    fmt, fs,
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static MANIFEST_TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PublishFailpoint {
    None,
    AfterRowsBeforeActive,
    AfterActiveBeforeCommit,
}

#[derive(Debug)]
pub enum AuthorityStoreError {
    Io(io::Error),
    Sqlite(rusqlite::Error),
    Blob(BlobStoreError),
    Digest(DigestError),
    ActiveGenerationAlreadyExists,
    NoActiveGeneration,
    ParentMismatch {
        expected: Option<ArtifactDigest>,
        actual: Option<ArtifactDigest>,
    },
    GenerationNumberMismatch {
        expected: u64,
        actual: u64,
    },
    GenerationNumberOverflow,
    GenerationNotFound(ArtifactDigest),
    ReplayedDigestMismatch {
        requested: ArtifactDigest,
        reconstructed: ArtifactDigest,
    },
    ManifestFileMismatch(ArtifactDigest),
    ManifestBlobBindingMismatch {
        generation: ArtifactDigest,
        blob: ArtifactDigest,
    },
    ManifestBlobBytesMismatch(ArtifactDigest),
    RealizationGenerationMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    RealizationBinaryDigestMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    InjectedPublishFailure(&'static str),
}

impl fmt::Display for AuthorityStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "authority-store I/O error: {error}"),
            Self::Sqlite(error) => write!(f, "authority-store SQLite error: {error}"),
            Self::Blob(error) => write!(f, "authority-store blob error: {error}"),
            Self::Digest(error) => write!(f, "authority-store digest error: {error}"),
            Self::ActiveGenerationAlreadyExists => f.write_str("active generation already exists"),
            Self::NoActiveGeneration => f.write_str("no active generation exists"),
            Self::ParentMismatch { expected, actual } => write!(
                f,
                "generation parent mismatch: expected {:?}, got {:?}",
                expected.map(|digest| digest.as_str()),
                actual.map(|digest| digest.as_str())
            ),
            Self::GenerationNumberMismatch { expected, actual } => write!(
                f,
                "generation number mismatch: expected {expected}, got {actual}"
            ),
            Self::GenerationNumberOverflow => {
                f.write_str("generation number exceeds SQLite INTEGER range")
            }
            Self::GenerationNotFound(digest) => {
                write!(f, "generation not found: {}", digest.as_str())
            }
            Self::ReplayedDigestMismatch {
                requested,
                reconstructed,
            } => write!(
                f,
                "replayed generation digest mismatch: requested {}, reconstructed {}",
                requested.as_str(),
                reconstructed.as_str()
            ),
            Self::ManifestFileMismatch(digest) => write!(
                f,
                "generation manifest file bytes do not match canonical replay for {}",
                digest.as_str()
            ),
            Self::ManifestBlobBindingMismatch { generation, blob } => write!(
                f,
                "generation {} is bound to unexpected manifest blob {}",
                generation.as_str(),
                blob.as_str()
            ),
            Self::ManifestBlobBytesMismatch(digest) => write!(
                f,
                "generation manifest blob bytes do not match canonical replay for {}",
                digest.as_str()
            ),
            Self::RealizationGenerationMismatch { expected, actual } => write!(
                f,
                "realization generation mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::RealizationBinaryDigestMismatch { expected, actual } => write!(
                f,
                "realization binary digest mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::InjectedPublishFailure(point) => {
                write!(f, "injected generation-publication failure at {point}")
            }
        }
    }
}

impl Error for AuthorityStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Sqlite(error) => Some(error),
            Self::Blob(error) => Some(error),
            Self::Digest(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for AuthorityStoreError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for AuthorityStoreError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<BlobStoreError> for AuthorityStoreError {
    fn from(value: BlobStoreError) -> Self {
        Self::Blob(value)
    }
}

impl From<DigestError> for AuthorityStoreError {
    fn from(value: DigestError) -> Self {
        Self::Digest(value)
    }
}

pub struct AuthorityStore {
    root: PathBuf,
    connection: Connection,
    blobs: BlobStore,
}

impl AuthorityStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, AuthorityStoreError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("generations"))?;
        let connection = Connection::open(root.join("authority.sqlite"))?;
        connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS meta (
               key TEXT PRIMARY KEY,
               value TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS generations (
               digest TEXT PRIMARY KEY,
               generation_number INTEGER NOT NULL,
               parent_digest TEXT,
               manifest_blob_digest TEXT NOT NULL UNIQUE
             );
             CREATE TABLE IF NOT EXISTS generation_admitted (
               generation_digest TEXT NOT NULL,
               artifact_digest TEXT NOT NULL,
               PRIMARY KEY (generation_digest, artifact_digest),
               FOREIGN KEY (generation_digest) REFERENCES generations(digest)
             );
             CREATE TABLE IF NOT EXISTS generation_authority_bindings (
               generation_digest TEXT NOT NULL,
               evidence_digest TEXT NOT NULL,
               PRIMARY KEY (generation_digest, evidence_digest),
               FOREIGN KEY (generation_digest) REFERENCES generations(digest)
             );",
        )?;
        let blobs = BlobStore::new(&root);
        Ok(Self {
            root,
            connection,
            blobs,
        })
    }

    pub fn initialize_genesis(
        &mut self,
        generation: &UniverseGeneration,
    ) -> Result<ArtifactDigest, AuthorityStoreError> {
        if generation.parent().is_some() || generation.generation_number() != 0 {
            return Err(AuthorityStoreError::GenerationNumberMismatch {
                expected: 0,
                actual: generation.generation_number(),
            });
        }

        let digest = self.persist_manifest(generation)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        if active_generation_in(&transaction)?.is_some() {
            return Err(AuthorityStoreError::ActiveGenerationAlreadyExists);
        }
        insert_generation_rows(&transaction, generation, digest)?;
        set_active_generation(&transaction, digest)?;
        transaction.commit()?;
        Ok(digest)
    }

    pub(crate) fn publish_generation_inner(
        &mut self,
        generation: &UniverseGeneration,
        failpoint: PublishFailpoint,
    ) -> Result<ArtifactDigest, AuthorityStoreError> {
        let digest = self.persist_manifest(generation)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;

        let active =
            active_generation_in(&transaction)?.ok_or(AuthorityStoreError::NoActiveGeneration)?;
        if generation.parent() != Some(active) {
            return Err(AuthorityStoreError::ParentMismatch {
                expected: Some(active),
                actual: generation.parent(),
            });
        }

        let parent_number = generation_number_in(&transaction, active)?;
        let expected_number = parent_number
            .checked_add(1)
            .ok_or(AuthorityStoreError::GenerationNumberOverflow)?;
        if generation.generation_number() != expected_number {
            return Err(AuthorityStoreError::GenerationNumberMismatch {
                expected: expected_number,
                actual: generation.generation_number(),
            });
        }

        insert_generation_rows(&transaction, generation, digest)?;
        if failpoint == PublishFailpoint::AfterRowsBeforeActive {
            return Err(AuthorityStoreError::InjectedPublishFailure(
                "after-rows-before-active",
            ));
        }

        set_active_generation(&transaction, digest)?;
        if failpoint == PublishFailpoint::AfterActiveBeforeCommit {
            return Err(AuthorityStoreError::InjectedPublishFailure(
                "after-active-before-commit",
            ));
        }

        transaction.commit()?;
        Ok(digest)
    }

    pub fn active_generation(&self) -> Result<Option<ArtifactDigest>, AuthorityStoreError> {
        let value: Option<String> = self
            .connection
            .query_row(
                "SELECT value FROM meta WHERE key = 'active_generation'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        value
            .map(|value| ArtifactDigest::parse(&value).map_err(AuthorityStoreError::from))
            .transpose()
    }

    pub fn replay_generation(
        &self,
        digest: ArtifactDigest,
    ) -> Result<UniverseGeneration, AuthorityStoreError> {
        let digest_string = digest.as_str();
        let row: Option<(i64, Option<String>, String)> = self
            .connection
            .query_row(
                "SELECT generation_number, parent_digest, manifest_blob_digest
                 FROM generations WHERE digest = ?1",
                params![digest_string],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()?;
        let (number, parent, manifest_blob) =
            row.ok_or(AuthorityStoreError::GenerationNotFound(digest))?;
        let number =
            u64::try_from(number).map_err(|_| AuthorityStoreError::GenerationNumberOverflow)?;
        let parent = parent
            .map(|value| ArtifactDigest::parse(&value).map_err(AuthorityStoreError::from))
            .transpose()?;
        let admitted = read_digest_column(
            &self.connection,
            "SELECT artifact_digest FROM generation_admitted WHERE generation_digest = ?1 ORDER BY artifact_digest",
            &digest_string,
        )?;
        let bindings = read_digest_column(
            &self.connection,
            "SELECT evidence_digest FROM generation_authority_bindings WHERE generation_digest = ?1 ORDER BY evidence_digest",
            &digest_string,
        )?;

        let generation = UniverseGeneration::new(number, parent, admitted, bindings);
        let reconstructed = generation.digest();
        if reconstructed != digest {
            return Err(AuthorityStoreError::ReplayedDigestMismatch {
                requested: digest,
                reconstructed,
            });
        }

        let canonical_bytes = generation.canonical_bytes();
        let manifest_path = self
            .root
            .join("generations")
            .join(format!("{}.json", digest.hex()));
        if fs::read(manifest_path)? != canonical_bytes {
            return Err(AuthorityStoreError::ManifestFileMismatch(digest));
        }

        let manifest_blob = ArtifactDigest::parse(&manifest_blob)?;
        if manifest_blob != digest {
            return Err(AuthorityStoreError::ManifestBlobBindingMismatch {
                generation: digest,
                blob: manifest_blob,
            });
        }
        if self.blobs.get(manifest_blob)? != canonical_bytes {
            return Err(AuthorityStoreError::ManifestBlobBytesMismatch(digest));
        }

        Ok(generation)
    }

    fn persist_manifest(
        &self,
        generation: &UniverseGeneration,
    ) -> Result<ArtifactDigest, AuthorityStoreError> {
        let bytes = generation.canonical_bytes();
        let digest = generation.digest();
        let blob_digest = self.blobs.put(&bytes)?;
        if blob_digest != digest {
            return Err(AuthorityStoreError::ReplayedDigestMismatch {
                requested: digest,
                reconstructed: blob_digest,
            });
        }
        let path = self
            .root
            .join("generations")
            .join(format!("{}.json", digest.hex()));
        write_immutable_manifest(&path, &bytes)?;
        Ok(digest)
    }
}

fn active_generation_in(
    transaction: &Transaction<'_>,
) -> Result<Option<ArtifactDigest>, AuthorityStoreError> {
    let value: Option<String> = transaction
        .query_row(
            "SELECT value FROM meta WHERE key = 'active_generation'",
            [],
            |row| row.get(0),
        )
        .optional()?;
    value
        .map(|value| ArtifactDigest::parse(&value).map_err(AuthorityStoreError::from))
        .transpose()
}

fn generation_number_in(
    transaction: &Transaction<'_>,
    digest: ArtifactDigest,
) -> Result<u64, AuthorityStoreError> {
    let number: Option<i64> = transaction
        .query_row(
            "SELECT generation_number FROM generations WHERE digest = ?1",
            params![digest.as_str()],
            |row| row.get(0),
        )
        .optional()?;
    let number = number.ok_or(AuthorityStoreError::GenerationNotFound(digest))?;
    u64::try_from(number).map_err(|_| AuthorityStoreError::GenerationNumberOverflow)
}

fn insert_generation_rows(
    transaction: &Transaction<'_>,
    generation: &UniverseGeneration,
    digest: ArtifactDigest,
) -> Result<(), AuthorityStoreError> {
    let generation_number = i64::try_from(generation.generation_number())
        .map_err(|_| AuthorityStoreError::GenerationNumberOverflow)?;
    transaction.execute(
        "INSERT INTO generations (digest, generation_number, parent_digest, manifest_blob_digest)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            digest.as_str(),
            generation_number,
            generation.parent().map(|value| value.as_str()),
            digest.as_str(),
        ],
    )?;
    for artifact in generation.admitted() {
        transaction.execute(
            "INSERT INTO generation_admitted (generation_digest, artifact_digest) VALUES (?1, ?2)",
            params![digest.as_str(), artifact.as_str()],
        )?;
    }
    for evidence in generation.authority_bindings() {
        transaction.execute(
            "INSERT INTO generation_authority_bindings (generation_digest, evidence_digest) VALUES (?1, ?2)",
            params![digest.as_str(), evidence.as_str()],
        )?;
    }
    Ok(())
}

fn set_active_generation(
    transaction: &Transaction<'_>,
    digest: ArtifactDigest,
) -> Result<(), AuthorityStoreError> {
    transaction.execute(
        "INSERT INTO meta (key, value) VALUES ('active_generation', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![digest.as_str()],
    )?;
    Ok(())
}

fn read_digest_column(
    connection: &Connection,
    sql: &str,
    generation_digest: &str,
) -> Result<Vec<ArtifactDigest>, AuthorityStoreError> {
    let mut statement = connection.prepare(sql)?;
    let rows = statement.query_map(params![generation_digest], |row| row.get::<_, String>(0))?;
    let mut values = Vec::new();
    for row in rows {
        values.push(ArtifactDigest::parse(&row?)?);
    }
    Ok(values)
}

fn write_immutable_manifest(path: &Path, bytes: &[u8]) -> Result<(), AuthorityStoreError> {
    if path.exists() {
        let existing = fs::read(path)?;
        if existing == bytes {
            return Ok(());
        }
        return Err(AuthorityStoreError::Io(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "generation manifest exists with different bytes",
        )));
    }

    let parent = path.parent().expect("generation manifest path has parent");
    loop {
        let counter = MANIFEST_TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let temporary = parent.join(format!(".manifest.tmp-{}-{counter}", std::process::id()));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(mut file) => {
                if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
                    let _ = fs::remove_file(&temporary);
                    return Err(AuthorityStoreError::Io(error));
                }
                match fs::hard_link(&temporary, path) {
                    Ok(()) => {
                        fs::remove_file(&temporary)?;
                        return Ok(());
                    }
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                        let _ = fs::remove_file(&temporary);
                        let existing = fs::read(path)?;
                        if existing == bytes {
                            return Ok(());
                        }
                        return Err(AuthorityStoreError::Io(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            "generation manifest race produced different bytes",
                        )));
                    }
                    Err(error) => {
                        let _ = fs::remove_file(&temporary);
                        return Err(AuthorityStoreError::Io(error));
                    }
                }
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(AuthorityStoreError::Io(error)),
        }
    }
}
