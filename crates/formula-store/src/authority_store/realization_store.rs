use super::{AuthorityStore, AuthorityStoreError};
use formula_check::realization::RealizationAuthorization;
use formula_core::{digest::ArtifactDigest, realization::RealizationDispatchContext};
use rusqlite::{OptionalExtension, params};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedRealization {
    manifest_digest: ArtifactDigest,
    semantic_target: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    specialization_digest: ArtifactDigest,
    source_digest: ArtifactDigest,
    toolchain_digest: ArtifactDigest,
    binary_digest: ArtifactDigest,
    binary_bytes: Vec<u8>,
}

#[derive(Debug)]
struct RealizationRow {
    manifest_digest: String,
    semantic_target: String,
    universe_generation: String,
    world: String,
    authority_contract: String,
    observer: String,
    specialization_digest: String,
    source_digest: String,
    toolchain_digest: String,
    binary_digest: String,
}

impl AdmittedRealization {
    pub fn manifest_digest(&self) -> ArtifactDigest {
        self.manifest_digest
    }

    pub fn semantic_target(&self) -> ArtifactDigest {
        self.semantic_target
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn authority_contract(&self) -> ArtifactDigest {
        self.authority_contract
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn specialization_digest(&self) -> ArtifactDigest {
        self.specialization_digest
    }

    pub fn source_digest(&self) -> ArtifactDigest {
        self.source_digest
    }

    pub fn toolchain_digest(&self) -> ArtifactDigest {
        self.toolchain_digest
    }

    pub fn binary_digest(&self) -> ArtifactDigest {
        self.binary_digest
    }

    pub fn binary_bytes(&self) -> &[u8] {
        &self.binary_bytes
    }
}

impl AuthorityStore {
    pub fn admit_realization(
        &mut self,
        authorization: &RealizationAuthorization,
        binary_bytes: &[u8],
    ) -> Result<AdmittedRealization, AuthorityStoreError> {
        ensure_realization_table(self)?;

        let active = self
            .active_generation()?
            .ok_or(AuthorityStoreError::NoActiveGeneration)?;
        if active != authorization.universe_generation() {
            return Err(AuthorityStoreError::RealizationGenerationMismatch {
                expected: active,
                actual: authorization.universe_generation(),
            });
        }

        let actual_binary = ArtifactDigest::of_bytes(binary_bytes);
        if actual_binary != authorization.binary_digest() {
            return Err(AuthorityStoreError::RealizationBinaryDigestMismatch {
                expected: authorization.binary_digest(),
                actual: actual_binary,
            });
        }

        let stored_binary = self.blobs.put(binary_bytes)?;
        if stored_binary != authorization.binary_digest() {
            return Err(AuthorityStoreError::RealizationBinaryDigestMismatch {
                expected: authorization.binary_digest(),
                actual: stored_binary,
            });
        }

        self.connection.execute(
            "INSERT INTO realizations (
                 manifest_digest, semantic_target, universe_generation, world,
                 authority_contract, observer, specialization_digest, source_digest,
                 toolchain_digest, binary_digest
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                authorization.realization_manifest().as_str(),
                authorization.semantic_target().as_str(),
                authorization.universe_generation().as_str(),
                authorization.world().as_str(),
                authorization.authority_contract().as_str(),
                authorization.observer().as_str(),
                authorization.specialization_digest().as_str(),
                authorization.source_digest().as_str(),
                authorization.toolchain_digest().as_str(),
                authorization.binary_digest().as_str(),
            ],
        )?;

        Ok(admitted_from_authorization(
            authorization,
            binary_bytes.to_vec(),
        ))
    }

    pub fn resolve_realization(
        &self,
        context: &RealizationDispatchContext,
    ) -> Result<Option<AdmittedRealization>, AuthorityStoreError> {
        ensure_realization_table(self)?;

        let row: Option<RealizationRow> = self
            .connection
            .query_row(
                "SELECT manifest_digest, semantic_target, universe_generation, world,
                        authority_contract, observer, specialization_digest, source_digest,
                        toolchain_digest, binary_digest
                 FROM realizations
                 WHERE semantic_target = ?1
                   AND universe_generation = ?2
                   AND world = ?3
                   AND authority_contract = ?4
                   AND observer = ?5",
                params![
                    context.semantic_target().as_str(),
                    context.universe_generation().as_str(),
                    context.world().as_str(),
                    context.authority_contract().as_str(),
                    context.observer().as_str(),
                ],
                |row| {
                    Ok(RealizationRow {
                        manifest_digest: row.get(0)?,
                        semantic_target: row.get(1)?,
                        universe_generation: row.get(2)?,
                        world: row.get(3)?,
                        authority_contract: row.get(4)?,
                        observer: row.get(5)?,
                        specialization_digest: row.get(6)?,
                        source_digest: row.get(7)?,
                        toolchain_digest: row.get(8)?,
                        binary_digest: row.get(9)?,
                    })
                },
            )
            .optional()?;

        let Some(row) = row else {
            return Ok(None);
        };

        let binary_digest = ArtifactDigest::parse(&row.binary_digest)?;
        let binary_bytes = self.blobs.get(binary_digest)?;

        Ok(Some(AdmittedRealization {
            manifest_digest: ArtifactDigest::parse(&row.manifest_digest)?,
            semantic_target: ArtifactDigest::parse(&row.semantic_target)?,
            universe_generation: ArtifactDigest::parse(&row.universe_generation)?,
            world: ArtifactDigest::parse(&row.world)?,
            authority_contract: ArtifactDigest::parse(&row.authority_contract)?,
            observer: ArtifactDigest::parse(&row.observer)?,
            specialization_digest: ArtifactDigest::parse(&row.specialization_digest)?,
            source_digest: ArtifactDigest::parse(&row.source_digest)?,
            toolchain_digest: ArtifactDigest::parse(&row.toolchain_digest)?,
            binary_digest,
            binary_bytes,
        }))
    }
}

fn ensure_realization_table(store: &AuthorityStore) -> Result<(), AuthorityStoreError> {
    store.connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS realizations (
           manifest_digest TEXT PRIMARY KEY,
           semantic_target TEXT NOT NULL,
           universe_generation TEXT NOT NULL,
           world TEXT NOT NULL,
           authority_contract TEXT NOT NULL,
           observer TEXT NOT NULL,
           specialization_digest TEXT NOT NULL,
           source_digest TEXT NOT NULL,
           toolchain_digest TEXT NOT NULL,
           binary_digest TEXT NOT NULL,
           UNIQUE (semantic_target, universe_generation, world, authority_contract, observer),
           FOREIGN KEY (universe_generation) REFERENCES generations(digest)
         );",
    )?;
    Ok(())
}

fn admitted_from_authorization(
    authorization: &RealizationAuthorization,
    binary_bytes: Vec<u8>,
) -> AdmittedRealization {
    AdmittedRealization {
        manifest_digest: authorization.realization_manifest(),
        semantic_target: authorization.semantic_target(),
        universe_generation: authorization.universe_generation(),
        world: authorization.world(),
        authority_contract: authorization.authority_contract(),
        observer: authorization.observer(),
        specialization_digest: authorization.specialization_digest(),
        source_digest: authorization.source_digest(),
        toolchain_digest: authorization.toolchain_digest(),
        binary_digest: authorization.binary_digest(),
        binary_bytes,
    }
}
