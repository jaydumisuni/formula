use super::{AuthorityStore, AuthorityStoreError};
use formula_check::realization::RealizationAuthorization;
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    realization::RealizationDispatchContext,
    self_expansion::{
        RealizationUpgrade, SemanticChangeClass, SupersessionKind, SupersessionRecord,
    },
};
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
pub enum RealizationUpgradeError {
    Store(AuthorityStoreError),
    ClassMismatch,
    GenerationMismatch {
        expected: ArtifactDigest,
        actual: ArtifactDigest,
    },
    SemanticNotAdmitted(ArtifactDigest),
    EvidenceNotAuthorityBound(ArtifactDigest),
    VariantNotFound(ArtifactDigest),
    VariantContextMismatch,
}

impl From<AuthorityStoreError> for RealizationUpgradeError {
    fn from(value: AuthorityStoreError) -> Self {
        Self::Store(value)
    }
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
            "INSERT INTO realizations_v2 (
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
        let row = query_context_row(self, context)?;
        row.map(|row| materialize_row(self, row)).transpose()
    }

    pub fn preferred_realization(
        &self,
        context: &RealizationDispatchContext,
    ) -> Result<Option<AdmittedRealization>, RealizationUpgradeError> {
        ensure_realization_table(self)?;

        let selected: Option<String> = self
            .connection
            .query_row(
                "SELECT manifest_digest FROM realization_selections
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
                |row| row.get(0),
            )
            .optional()
            .map_err(AuthorityStoreError::from)?;

        if let Some(selected) = selected {
            let manifest = ArtifactDigest::parse(&selected).map_err(AuthorityStoreError::from)?;
            let realization = self
                .realization_by_manifest(manifest)?
                .ok_or(RealizationUpgradeError::VariantNotFound(manifest))?;
            validate_context(&realization, context)?;
            return Ok(Some(realization));
        }

        let row = query_context_row(self, context)?;
        row.map(|row| materialize_row(self, row))
            .transpose()
            .map_err(RealizationUpgradeError::from)
    }

    pub fn realization_by_manifest(
        &self,
        manifest: ArtifactDigest,
    ) -> Result<Option<AdmittedRealization>, AuthorityStoreError> {
        ensure_realization_table(self)?;
        let row = query_manifest_row(self, manifest)?;
        row.map(|row| materialize_row(self, row)).transpose()
    }

    pub fn select_realization(
        &mut self,
        context: &RealizationDispatchContext,
        manifest: ArtifactDigest,
    ) -> Result<ArtifactDigest, RealizationUpgradeError> {
        ensure_realization_table(self)?;
        let realization = self
            .realization_by_manifest(manifest)?
            .ok_or(RealizationUpgradeError::VariantNotFound(manifest))?;
        validate_context(&realization, context)?;

        self.connection
            .execute(
                "INSERT INTO realization_selections (
                     semantic_target, universe_generation, world, authority_contract,
                     observer, manifest_digest
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(semantic_target, universe_generation, world, authority_contract, observer)
                 DO UPDATE SET manifest_digest = excluded.manifest_digest",
                params![
                    context.semantic_target().as_str(),
                    context.universe_generation().as_str(),
                    context.world().as_str(),
                    context.authority_contract().as_str(),
                    context.observer().as_str(),
                    manifest.as_str(),
                ],
            )
            .map_err(AuthorityStoreError::from)?;
        Ok(manifest)
    }

    pub fn record_realization_upgrade(
        &mut self,
        upgrade: &RealizationUpgrade,
    ) -> Result<RealizationUpgrade, RealizationUpgradeError> {
        ensure_realization_table(self)?;
        if upgrade.semantic_change_class() != SemanticChangeClass::RealizationOnly {
            return Err(RealizationUpgradeError::ClassMismatch);
        }

        let active = self
            .active_generation()?
            .ok_or(AuthorityStoreError::NoActiveGeneration)?;
        if active != upgrade.universe_generation() {
            return Err(RealizationUpgradeError::GenerationMismatch {
                expected: active,
                actual: upgrade.universe_generation(),
            });
        }
        let generation = self.replay_generation(active)?;
        if !generation.admitted().contains(&upgrade.semantic_artifact()) {
            return Err(RealizationUpgradeError::SemanticNotAdmitted(
                upgrade.semantic_artifact(),
            ));
        }
        for evidence in upgrade.validation_evidence() {
            if !generation.authority_bindings().contains(evidence) {
                return Err(RealizationUpgradeError::EvidenceNotAuthorityBound(
                    *evidence,
                ));
            }
        }

        let old = self
            .realization_by_manifest(upgrade.old_realization())?
            .ok_or(RealizationUpgradeError::VariantNotFound(
                upgrade.old_realization(),
            ))?;
        let new = self
            .realization_by_manifest(upgrade.new_realization())?
            .ok_or(RealizationUpgradeError::VariantNotFound(
                upgrade.new_realization(),
            ))?;

        if old.semantic_target() != upgrade.semantic_artifact()
            || new.semantic_target() != upgrade.semantic_artifact()
            || old.universe_generation() != active
            || new.universe_generation() != active
            || old.world() != new.world()
            || old.authority_contract() != new.authority_contract()
            || old.observer() != new.observer()
        {
            return Err(RealizationUpgradeError::VariantContextMismatch);
        }

        let upgrade_digest = upgrade.structural_digest();
        self.connection
            .execute(
                "INSERT INTO realization_upgrades (
                     upgrade_digest, semantic_target, universe_generation,
                     old_realization, new_realization, selection_policy
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    upgrade_digest.as_str(),
                    upgrade.semantic_artifact().as_str(),
                    active.as_str(),
                    upgrade.old_realization().as_str(),
                    upgrade.new_realization().as_str(),
                    upgrade.selection_policy().as_str(),
                ],
            )
            .map_err(AuthorityStoreError::from)?;
        for evidence in upgrade.validation_evidence() {
            self.connection
                .execute(
                    "INSERT INTO realization_upgrade_evidence (upgrade_digest, evidence_digest)
                     VALUES (?1, ?2)",
                    params![upgrade_digest.as_str(), evidence.as_str()],
                )
                .map_err(AuthorityStoreError::from)?;
        }

        let supersession = SupersessionRecord::new(
            upgrade.old_realization(),
            upgrade.new_realization(),
            SupersessionKind::ReplacedRealizationBy,
            active,
            vec![
                upgrade.semantic_artifact(),
                old.world(),
                old.authority_contract(),
                old.observer(),
                upgrade.selection_policy(),
            ],
            upgrade.validation_evidence().to_vec(),
        );
        self.record_supersession(&supersession)?;
        let context = RealizationDispatchContext::new(
            upgrade.semantic_artifact(),
            active,
            old.world(),
            old.authority_contract(),
            old.observer(),
        );
        self.select_realization(&context, upgrade.new_realization())?;
        Ok(upgrade.clone())
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
         );
         CREATE TABLE IF NOT EXISTS realizations_v2 (
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
           FOREIGN KEY (universe_generation) REFERENCES generations(digest)
         );
         CREATE INDEX IF NOT EXISTS realizations_v2_context
           ON realizations_v2 (
             semantic_target, universe_generation, world, authority_contract, observer
           );
         CREATE TABLE IF NOT EXISTS realization_selections (
           semantic_target TEXT NOT NULL,
           universe_generation TEXT NOT NULL,
           world TEXT NOT NULL,
           authority_contract TEXT NOT NULL,
           observer TEXT NOT NULL,
           manifest_digest TEXT NOT NULL,
           PRIMARY KEY (
             semantic_target, universe_generation, world, authority_contract, observer
           ),
           FOREIGN KEY (manifest_digest) REFERENCES realizations_v2(manifest_digest),
           FOREIGN KEY (universe_generation) REFERENCES generations(digest)
         );
         CREATE TABLE IF NOT EXISTS realization_upgrades (
           upgrade_digest TEXT PRIMARY KEY,
           semantic_target TEXT NOT NULL,
           universe_generation TEXT NOT NULL,
           old_realization TEXT NOT NULL,
           new_realization TEXT NOT NULL,
           selection_policy TEXT NOT NULL,
           FOREIGN KEY (universe_generation) REFERENCES generations(digest),
           FOREIGN KEY (old_realization) REFERENCES realizations_v2(manifest_digest),
           FOREIGN KEY (new_realization) REFERENCES realizations_v2(manifest_digest)
         );
         CREATE TABLE IF NOT EXISTS realization_upgrade_evidence (
           upgrade_digest TEXT NOT NULL,
           evidence_digest TEXT NOT NULL,
           PRIMARY KEY (upgrade_digest, evidence_digest),
           FOREIGN KEY (upgrade_digest) REFERENCES realization_upgrades(upgrade_digest)
         );
         INSERT OR IGNORE INTO realizations_v2 (
           manifest_digest, semantic_target, universe_generation, world,
           authority_contract, observer, specialization_digest, source_digest,
           toolchain_digest, binary_digest
         ) SELECT
           manifest_digest, semantic_target, universe_generation, world,
           authority_contract, observer, specialization_digest, source_digest,
           toolchain_digest, binary_digest
         FROM realizations;",
    )?;
    Ok(())
}

fn query_context_row(
    store: &AuthorityStore,
    context: &RealizationDispatchContext,
) -> Result<Option<RealizationRow>, AuthorityStoreError> {
    store
        .connection
        .query_row(
            "SELECT manifest_digest, semantic_target, universe_generation, world,
                    authority_contract, observer, specialization_digest, source_digest,
                    toolchain_digest, binary_digest
             FROM realizations_v2
             WHERE semantic_target = ?1
               AND universe_generation = ?2
               AND world = ?3
               AND authority_contract = ?4
               AND observer = ?5
             ORDER BY manifest_digest
             LIMIT 1",
            params![
                context.semantic_target().as_str(),
                context.universe_generation().as_str(),
                context.world().as_str(),
                context.authority_contract().as_str(),
                context.observer().as_str(),
            ],
            row_from_sql,
        )
        .optional()
        .map_err(AuthorityStoreError::from)
}

fn query_manifest_row(
    store: &AuthorityStore,
    manifest: ArtifactDigest,
) -> Result<Option<RealizationRow>, AuthorityStoreError> {
    store
        .connection
        .query_row(
            "SELECT manifest_digest, semantic_target, universe_generation, world,
                    authority_contract, observer, specialization_digest, source_digest,
                    toolchain_digest, binary_digest
             FROM realizations_v2
             WHERE manifest_digest = ?1",
            params![manifest.as_str()],
            row_from_sql,
        )
        .optional()
        .map_err(AuthorityStoreError::from)
}

fn row_from_sql(row: &rusqlite::Row<'_>) -> rusqlite::Result<RealizationRow> {
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
}

fn materialize_row(
    store: &AuthorityStore,
    row: RealizationRow,
) -> Result<AdmittedRealization, AuthorityStoreError> {
    let binary_digest = ArtifactDigest::parse(&row.binary_digest)?;
    let binary_bytes = store.blobs.get(binary_digest)?;
    Ok(AdmittedRealization {
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
    })
}

fn validate_context(
    realization: &AdmittedRealization,
    context: &RealizationDispatchContext,
) -> Result<(), RealizationUpgradeError> {
    if realization.semantic_target() != context.semantic_target()
        || realization.universe_generation() != context.universe_generation()
        || realization.world() != context.world()
        || realization.authority_contract() != context.authority_contract()
        || realization.observer() != context.observer()
    {
        return Err(RealizationUpgradeError::VariantContextMismatch);
    }
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
