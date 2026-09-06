use formula_check::bootstrap::BootstrapValidationAuthorization;
use formula_core::{
    artifacts::StructuralIdentity,
    bootstrap::{BootstrapBytecode, BootstrapGenerationId, BootstrapSeedManifest},
    digest::{ArtifactDigest, DigestError},
};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::{error::Error, fmt, fs, path::Path};

#[derive(Debug)]
pub enum BootstrapStoreError {
    Io(std::io::Error),
    Sqlite(rusqlite::Error),
    Digest(DigestError),
    BootstrapRootAlreadyExists,
    NoActiveBootstrapGeneration,
    BootstrapGenerationNotFound(BootstrapGenerationId),
    PredecessorMismatch {
        expected: BootstrapGenerationId,
        actual: BootstrapGenerationId,
    },
    SuccessorOrdinalMismatch {
        expected: u64,
        actual: u64,
    },
    GeneratorEqualsValidator,
    SeedIdentityMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    CandidateArtifactMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    StoredGenerationDigestMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    CandidateReplayMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    GenerationOrdinalOverflow,
}

impl fmt::Display for BootstrapStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "bootstrap-store I/O error: {error}"),
            Self::Sqlite(error) => write!(f, "bootstrap-store SQLite error: {error}"),
            Self::Digest(error) => write!(f, "bootstrap-store digest error: {error}"),
            Self::BootstrapRootAlreadyExists => f.write_str("bootstrap root already exists"),
            Self::NoActiveBootstrapGeneration => {
                f.write_str("no active bootstrap generation exists")
            }
            Self::BootstrapGenerationNotFound(id) => write!(
                f,
                "bootstrap generation not found: T{} {}",
                id.ordinal(),
                id.digest().as_str()
            ),
            Self::PredecessorMismatch { expected, actual } => write!(
                f,
                "bootstrap predecessor mismatch: expected T{} {}, got T{} {}",
                expected.ordinal(),
                expected.digest().as_str(),
                actual.ordinal(),
                actual.digest().as_str()
            ),
            Self::SuccessorOrdinalMismatch { expected, actual } => write!(
                f,
                "bootstrap successor ordinal mismatch: expected {expected}, got {actual}"
            ),
            Self::GeneratorEqualsValidator => {
                f.write_str("bootstrap generator and validator identities must differ")
            }
            Self::SeedIdentityMismatch { expected, actual } => write!(
                f,
                "bootstrap seed identity mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::CandidateArtifactMismatch { expected, actual } => write!(
                f,
                "bootstrap candidate artifact mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::StoredGenerationDigestMismatch { expected, actual } => write!(
                f,
                "stored bootstrap generation digest mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::CandidateReplayMismatch { expected, actual } => write!(
                f,
                "bootstrap candidate replay mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::GenerationOrdinalOverflow => {
                f.write_str("bootstrap generation ordinal exceeds SQLite INTEGER range")
            }
        }
    }
}

impl Error for BootstrapStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Sqlite(error) => Some(error),
            Self::Digest(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for BootstrapStoreError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for BootstrapStoreError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

impl From<DigestError> for BootstrapStoreError {
    fn from(value: DigestError) -> Self {
        Self::Digest(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBootstrapGeneration {
    id: BootstrapGenerationId,
    predecessor: Option<BootstrapGenerationId>,
    seed_identity: ArtifactDigest,
    rebuild_manifest: Option<ArtifactDigest>,
    source_digest: Option<ArtifactDigest>,
    candidate_artifact: Option<ArtifactDigest>,
    generator_identity: Option<ArtifactDigest>,
    validator_identity: Option<ArtifactDigest>,
    semantic_evidence: Option<ArtifactDigest>,
    candidate: Option<BootstrapBytecode>,
}

impl AdmittedBootstrapGeneration {
    pub fn id(&self) -> BootstrapGenerationId {
        self.id
    }

    pub fn predecessor(&self) -> Option<BootstrapGenerationId> {
        self.predecessor
    }

    pub fn seed_identity(&self) -> ArtifactDigest {
        self.seed_identity
    }

    pub fn rebuild_manifest(&self) -> Option<ArtifactDigest> {
        self.rebuild_manifest
    }

    pub fn source_digest(&self) -> Option<ArtifactDigest> {
        self.source_digest
    }

    pub fn candidate_artifact(&self) -> Option<ArtifactDigest> {
        self.candidate_artifact
    }

    pub fn generator_identity(&self) -> Option<ArtifactDigest> {
        self.generator_identity
    }

    pub fn validator_identity(&self) -> Option<ArtifactDigest> {
        self.validator_identity
    }

    pub fn semantic_evidence(&self) -> Option<ArtifactDigest> {
        self.semantic_evidence
    }

    pub fn candidate(&self) -> Option<&BootstrapBytecode> {
        self.candidate.as_ref()
    }
}

pub struct BootstrapAuthorityStore {
    connection: Connection,
}

impl BootstrapAuthorityStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, BootstrapStoreError> {
        let root = root.as_ref();
        fs::create_dir_all(root)?;
        let connection = Connection::open(root.join("bootstrap-authority.sqlite"))?;
        connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS bootstrap_meta (
               key TEXT PRIMARY KEY,
               value TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS bootstrap_generations (
               ordinal INTEGER PRIMARY KEY,
               digest TEXT NOT NULL UNIQUE,
               predecessor_ordinal INTEGER,
               predecessor_digest TEXT,
               seed_identity TEXT NOT NULL,
               rebuild_manifest TEXT,
               source_digest TEXT,
               candidate_artifact TEXT,
               generator_identity TEXT,
               validator_identity TEXT,
               semantic_evidence TEXT,
               candidate_bytes BLOB,
               FOREIGN KEY (predecessor_ordinal) REFERENCES bootstrap_generations(ordinal)
             );",
        )?;
        Ok(Self { connection })
    }

    pub fn create_bootstrap_root(
        &mut self,
        seed: &BootstrapSeedManifest,
    ) -> Result<BootstrapGenerationId, BootstrapStoreError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let count: i64 = transaction.query_row(
            "SELECT COUNT(*) FROM bootstrap_generations",
            [],
            |row| row.get(0),
        )?;
        if count != 0 || active_generation_in(&transaction)?.is_some() {
            return Err(BootstrapStoreError::BootstrapRootAlreadyExists);
        }

        let seed_identity = seed.structural_digest();
        let root = BootstrapGenerationId::new(0, seed_identity);
        transaction.execute(
            "INSERT INTO bootstrap_generations (
               ordinal, digest, predecessor_ordinal, predecessor_digest,
               seed_identity, rebuild_manifest, source_digest, candidate_artifact,
               generator_identity, validator_identity, semantic_evidence, candidate_bytes
             ) VALUES (?1, ?2, NULL, NULL, ?3, NULL, NULL, NULL, NULL, NULL, NULL, NULL)",
            params![0_i64, root.digest().as_str(), seed_identity.as_str()],
        )?;
        set_active_ordinal(&transaction, 0)?;
        transaction.commit()?;
        Ok(root)
    }

    pub fn admit_bootstrap_successor(
        &mut self,
        authorization: &BootstrapValidationAuthorization,
        candidate: &BootstrapBytecode,
    ) -> Result<BootstrapGenerationId, BootstrapStoreError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let active = active_generation_in(&transaction)?
            .ok_or(BootstrapStoreError::NoActiveBootstrapGeneration)?;
        let predecessor = authorization.predecessor();
        if active != predecessor {
            return Err(BootstrapStoreError::PredecessorMismatch {
                expected: active,
                actual: predecessor,
            });
        }

        let successor = authorization.successor();
        let expected_ordinal = active
            .ordinal()
            .checked_add(1)
            .ok_or(BootstrapStoreError::GenerationOrdinalOverflow)?;
        if successor.ordinal() != expected_ordinal {
            return Err(BootstrapStoreError::SuccessorOrdinalMismatch {
                expected: expected_ordinal,
                actual: successor.ordinal(),
            });
        }
        if authorization.generator_identity() == authorization.validator_identity() {
            return Err(BootstrapStoreError::GeneratorEqualsValidator);
        }

        let root_seed = root_seed_identity_in(&transaction)?;
        if authorization.seed_identity() != root_seed {
            return Err(BootstrapStoreError::SeedIdentityMismatch {
                expected: root_seed,
                actual: authorization.seed_identity(),
            });
        }

        let candidate_artifact = candidate.structural_digest();
        if candidate_artifact != authorization.candidate_artifact() {
            return Err(BootstrapStoreError::CandidateArtifactMismatch {
                expected: authorization.candidate_artifact(),
                actual: candidate_artifact,
            });
        }

        let successor_ordinal = i64::try_from(successor.ordinal())
            .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
        let predecessor_ordinal = i64::try_from(predecessor.ordinal())
            .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
        transaction.execute(
            "INSERT INTO bootstrap_generations (
               ordinal, digest, predecessor_ordinal, predecessor_digest,
               seed_identity, rebuild_manifest, source_digest, candidate_artifact,
               generator_identity, validator_identity, semantic_evidence, candidate_bytes
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                successor_ordinal,
                successor.digest().as_str(),
                predecessor_ordinal,
                predecessor.digest().as_str(),
                authorization.seed_identity().as_str(),
                authorization.rebuild_manifest().as_str(),
                authorization.source_digest().as_str(),
                authorization.candidate_artifact().as_str(),
                authorization.generator_identity().as_str(),
                authorization.validator_identity().as_str(),
                authorization.semantic_evidence().as_str(),
                candidate.bytes(),
            ],
        )?;
        set_active_ordinal(&transaction, successor_ordinal)?;
        transaction.commit()?;
        Ok(successor)
    }

    pub fn active_bootstrap_generation(
        &self,
    ) -> Result<BootstrapGenerationId, BootstrapStoreError> {
        active_generation_in(&self.connection)?
            .ok_or(BootstrapStoreError::NoActiveBootstrapGeneration)
    }

    pub fn replay_bootstrap_generation(
        &self,
        id: BootstrapGenerationId,
    ) -> Result<AdmittedBootstrapGeneration, BootstrapStoreError> {
        let ordinal = i64::try_from(id.ordinal())
            .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
        let row = self
            .connection
            .query_row(
                "SELECT digest, predecessor_ordinal, predecessor_digest, seed_identity,
                        rebuild_manifest, source_digest, candidate_artifact,
                        generator_identity, validator_identity, semantic_evidence, candidate_bytes
                 FROM bootstrap_generations WHERE ordinal = ?1",
                params![ordinal],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<i64>>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, Option<String>>(7)?,
                        row.get::<_, Option<String>>(8)?,
                        row.get::<_, Option<String>>(9)?,
                        row.get::<_, Option<Vec<u8>>>(10)?,
                    ))
                },
            )
            .optional()?
            .ok_or(BootstrapStoreError::BootstrapGenerationNotFound(id))?;

        let stored_digest = ArtifactDigest::parse(&row.0)?;
        if stored_digest != id.digest() {
            return Err(BootstrapStoreError::StoredGenerationDigestMismatch {
                expected: id.digest(),
                actual: stored_digest,
            });
        }

        let predecessor = match (row.1, row.2) {
            (Some(predecessor_ordinal), Some(predecessor_digest)) => {
                let predecessor_ordinal = u64::try_from(predecessor_ordinal)
                    .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
                Some(BootstrapGenerationId::new(
                    predecessor_ordinal,
                    ArtifactDigest::parse(&predecessor_digest)?,
                ))
            }
            (None, None) => None,
            _ => return Err(BootstrapStoreError::NoActiveBootstrapGeneration),
        };

        let seed_identity = ArtifactDigest::parse(&row.3)?;
        let rebuild_manifest = parse_optional_digest(row.4)?;
        let source_digest = parse_optional_digest(row.5)?;
        let candidate_artifact = parse_optional_digest(row.6)?;
        let generator_identity = parse_optional_digest(row.7)?;
        let validator_identity = parse_optional_digest(row.8)?;
        let semantic_evidence = parse_optional_digest(row.9)?;
        let candidate = row.10.map(BootstrapBytecode::new);

        if let (Some(expected), Some(candidate)) = (candidate_artifact, candidate.as_ref()) {
            let actual = candidate.structural_digest();
            if actual != expected {
                return Err(BootstrapStoreError::CandidateReplayMismatch { expected, actual });
            }
        }

        Ok(AdmittedBootstrapGeneration {
            id,
            predecessor,
            seed_identity,
            rebuild_manifest,
            source_digest,
            candidate_artifact,
            generator_identity,
            validator_identity,
            semantic_evidence,
            candidate,
        })
    }

    pub fn select_bootstrap_generation(
        &mut self,
        id: BootstrapGenerationId,
    ) -> Result<(), BootstrapStoreError> {
        self.replay_bootstrap_generation(id)?;
        let ordinal = i64::try_from(id.ordinal())
            .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        set_active_ordinal(&transaction, ordinal)?;
        transaction.commit()?;
        Ok(())
    }
}

fn parse_optional_digest(
    value: Option<String>,
) -> Result<Option<ArtifactDigest>, BootstrapStoreError> {
    value
        .map(|value| ArtifactDigest::parse(&value).map_err(BootstrapStoreError::from))
        .transpose()
}

fn active_generation_in(
    connection: &Connection,
) -> Result<Option<BootstrapGenerationId>, BootstrapStoreError> {
    let active: Option<String> = connection
        .query_row(
            "SELECT value FROM bootstrap_meta WHERE key = 'active_ordinal'",
            [],
            |row| row.get(0),
        )
        .optional()?;
    let Some(active) = active else {
        return Ok(None);
    };
    let ordinal_i64: i64 = active
        .parse()
        .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
    let ordinal = u64::try_from(ordinal_i64)
        .map_err(|_| BootstrapStoreError::GenerationOrdinalOverflow)?;
    let digest: String = connection.query_row(
        "SELECT digest FROM bootstrap_generations WHERE ordinal = ?1",
        params![ordinal_i64],
        |row| row.get(0),
    )?;
    Ok(Some(BootstrapGenerationId::new(
        ordinal,
        ArtifactDigest::parse(&digest)?,
    )))
}

fn root_seed_identity_in(connection: &Connection) -> Result<ArtifactDigest, BootstrapStoreError> {
    let seed: String = connection.query_row(
        "SELECT seed_identity FROM bootstrap_generations WHERE ordinal = 0",
        [],
        |row| row.get(0),
    )?;
    Ok(ArtifactDigest::parse(&seed)?)
}

fn set_active_ordinal(
    connection: &Connection,
    ordinal: i64,
) -> Result<(), BootstrapStoreError> {
    connection.execute(
        "INSERT INTO bootstrap_meta(key, value) VALUES('active_ordinal', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![ordinal.to_string()],
    )?;
    Ok(())
}
