use super::{AuthorityStore, AuthorityStoreError, set_active_generation};
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    self_expansion::{
        ActivationMode, ExpansionActivationRecord, PromotionClass, PromotionClassRegistryV1,
        SupersessionKind, SupersessionRecord,
    },
};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};

#[derive(Debug)]
struct ExpansionActivationRow {
    activation_digest: String,
    generation_digest: String,
    subject_digest: String,
    promotion_class: String,
    world_digest: String,
    activation_mode: String,
}

#[derive(Debug)]
struct SupersessionRow {
    supersession_digest: String,
    subject_digest: String,
    successor_digest: String,
    kind: String,
    source_generation_digest: String,
}

impl AuthorityStore {
    pub fn record_expansion_activation(
        &mut self,
        record: &ExpansionActivationRecord,
    ) -> Result<ExpansionActivationRecord, AuthorityStoreError> {
        ensure_expansion_tables(self)?;

        let active = self
            .active_generation()?
            .ok_or(AuthorityStoreError::NoActiveGeneration)?;
        if record.generation() != active {
            return Err(AuthorityStoreError::ExpansionActivationGenerationMismatch {
                expected: active,
                actual: record.generation(),
            });
        }

        let generation = self.replay_generation(record.generation())?;
        let policy = PromotionClassRegistryV1::policy(record.promotion_class());
        let realization_only_derived =
            !policy.may_change_universe() && policy.may_change_realization_selection();
        if !realization_only_derived && !generation.admitted().contains(&record.subject()) {
            return Err(AuthorityStoreError::ExpansionActivationSubjectNotAdmitted(
                record.subject(),
            ));
        }
        for evidence in record.evidence() {
            if !generation.authority_bindings().contains(evidence) {
                return Err(
                    AuthorityStoreError::ExpansionActivationEvidenceNotAuthorityBound(*evidence),
                );
            }
        }
        if record.mode() == ActivationMode::Quarantined {
            return Err(AuthorityStoreError::ExpansionActivationQuarantined);
        }

        let activation_digest = record.structural_digest();
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            "INSERT INTO expansion_activations (
                 activation_digest, generation_digest, subject_digest,
                 promotion_class, world_digest, activation_mode
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                activation_digest.as_str(),
                record.generation().as_str(),
                record.subject().as_str(),
                record.promotion_class().as_str(),
                record.world().as_str(),
                record.mode().as_str(),
            ],
        )?;
        for evidence in record.evidence() {
            transaction.execute(
                "INSERT INTO expansion_activation_evidence (activation_digest, evidence_digest)
                 VALUES (?1, ?2)",
                params![activation_digest.as_str(), evidence.as_str()],
            )?;
        }
        for scope in record.scope() {
            transaction.execute(
                "INSERT INTO expansion_activation_scope (activation_digest, scope_digest)
                 VALUES (?1, ?2)",
                params![activation_digest.as_str(), scope.as_str()],
            )?;
        }
        transaction.commit()?;

        Ok(record.clone())
    }

    pub fn resolve_expansion_activation(
        &self,
        generation: ArtifactDigest,
        subject: ArtifactDigest,
        promotion_class: PromotionClass,
    ) -> Result<Option<ExpansionActivationRecord>, AuthorityStoreError> {
        ensure_expansion_tables(self)?;

        let row: Option<ExpansionActivationRow> = self
            .connection
            .query_row(
                "SELECT activation_digest, generation_digest, subject_digest,
                        promotion_class, world_digest, activation_mode
                 FROM expansion_activations
                 WHERE generation_digest = ?1
                   AND subject_digest = ?2
                   AND promotion_class = ?3",
                params![generation.as_str(), subject.as_str(), promotion_class.as_str()],
                |row| {
                    Ok(ExpansionActivationRow {
                        activation_digest: row.get(0)?,
                        generation_digest: row.get(1)?,
                        subject_digest: row.get(2)?,
                        promotion_class: row.get(3)?,
                        world_digest: row.get(4)?,
                        activation_mode: row.get(5)?,
                    })
                },
            )
            .optional()?;

        let Some(row) = row else {
            return Ok(None);
        };

        let stored_digest = ArtifactDigest::parse(&row.activation_digest)?;
        let evidence = read_child_digests(
            &self.connection,
            "SELECT evidence_digest FROM expansion_activation_evidence
             WHERE activation_digest = ?1 ORDER BY evidence_digest",
            stored_digest,
        )?;
        let scope = read_child_digests(
            &self.connection,
            "SELECT scope_digest FROM expansion_activation_scope
             WHERE activation_digest = ?1 ORDER BY scope_digest",
            stored_digest,
        )?;
        let record = ExpansionActivationRecord::new(
            ArtifactDigest::parse(&row.subject_digest)?,
            parse_promotion_class(&row.promotion_class)?,
            ArtifactDigest::parse(&row.generation_digest)?,
            ArtifactDigest::parse(&row.world_digest)?,
            parse_activation_mode(&row.activation_mode)?,
            evidence,
            scope,
        );
        let reconstructed = record.structural_digest();
        if reconstructed != stored_digest {
            return Err(AuthorityStoreError::ExpansionActivationDigestMismatch {
                stored: stored_digest,
                reconstructed,
            });
        }

        Ok(Some(record))
    }

    pub fn record_supersession(
        &mut self,
        record: &SupersessionRecord,
    ) -> Result<SupersessionRecord, AuthorityStoreError> {
        ensure_expansion_tables(self)?;

        let generation = self.replay_generation(record.source_generation())?;
        for evidence in record.evidence() {
            if !generation.authority_bindings().contains(evidence) {
                return Err(AuthorityStoreError::SupersessionEvidenceNotAuthorityBound(
                    *evidence,
                ));
            }
        }

        let supersession_digest = record.structural_digest();
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            "INSERT INTO supersessions (
                 supersession_digest, subject_digest, successor_digest,
                 kind, source_generation_digest
             ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                supersession_digest.as_str(),
                record.subject().as_str(),
                record.successor().as_str(),
                record.kind().as_str(),
                record.source_generation().as_str(),
            ],
        )?;
        for scope in record.selection_scope() {
            transaction.execute(
                "INSERT INTO supersession_scope (supersession_digest, scope_digest)
                 VALUES (?1, ?2)",
                params![supersession_digest.as_str(), scope.as_str()],
            )?;
        }
        for evidence in record.evidence() {
            transaction.execute(
                "INSERT INTO supersession_evidence (supersession_digest, evidence_digest)
                 VALUES (?1, ?2)",
                params![supersession_digest.as_str(), evidence.as_str()],
            )?;
        }
        transaction.commit()?;

        Ok(record.clone())
    }

    pub fn supersessions_for(
        &self,
        subject: ArtifactDigest,
    ) -> Result<Vec<SupersessionRecord>, AuthorityStoreError> {
        ensure_expansion_tables(self)?;

        let rows = {
            let mut statement = self.connection.prepare(
                "SELECT supersession_digest, subject_digest, successor_digest,
                        kind, source_generation_digest
                 FROM supersessions
                 WHERE subject_digest = ?1
                 ORDER BY supersession_digest",
            )?;
            let mapped = statement.query_map(params![subject.as_str()], |row| {
                Ok(SupersessionRow {
                    supersession_digest: row.get(0)?,
                    subject_digest: row.get(1)?,
                    successor_digest: row.get(2)?,
                    kind: row.get(3)?,
                    source_generation_digest: row.get(4)?,
                })
            })?;
            let mut rows = Vec::new();
            for row in mapped {
                rows.push(row?);
            }
            rows
        };

        let mut records = Vec::with_capacity(rows.len());
        for row in rows {
            let stored_digest = ArtifactDigest::parse(&row.supersession_digest)?;
            let selection_scope = read_child_digests(
                &self.connection,
                "SELECT scope_digest FROM supersession_scope
                 WHERE supersession_digest = ?1 ORDER BY scope_digest",
                stored_digest,
            )?;
            let evidence = read_child_digests(
                &self.connection,
                "SELECT evidence_digest FROM supersession_evidence
                 WHERE supersession_digest = ?1 ORDER BY evidence_digest",
                stored_digest,
            )?;
            let record = SupersessionRecord::new(
                ArtifactDigest::parse(&row.subject_digest)?,
                ArtifactDigest::parse(&row.successor_digest)?,
                parse_supersession_kind(&row.kind)?,
                ArtifactDigest::parse(&row.source_generation_digest)?,
                selection_scope,
                evidence,
            );
            let reconstructed = record.structural_digest();
            if reconstructed != stored_digest {
                return Err(AuthorityStoreError::SupersessionDigestMismatch {
                    stored: stored_digest,
                    reconstructed,
                });
            }
            records.push(record);
        }

        Ok(records)
    }

    pub fn select_active_generation(
        &mut self,
        target: ArtifactDigest,
    ) -> Result<ArtifactDigest, AuthorityStoreError> {
        let replayed = self.replay_generation(target)?;
        let reconstructed = replayed.digest();
        if reconstructed != target {
            return Err(AuthorityStoreError::ReplayedDigestMismatch {
                requested: target,
                reconstructed,
            });
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        set_active_generation(&transaction, target)?;
        transaction.commit()?;
        Ok(target)
    }
}

fn ensure_expansion_tables(store: &AuthorityStore) -> Result<(), AuthorityStoreError> {
    store.connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS expansion_activations (
           activation_digest TEXT PRIMARY KEY,
           generation_digest TEXT NOT NULL,
           subject_digest TEXT NOT NULL,
           promotion_class TEXT NOT NULL,
           world_digest TEXT NOT NULL,
           activation_mode TEXT NOT NULL,
           UNIQUE (generation_digest, subject_digest, promotion_class),
           FOREIGN KEY (generation_digest) REFERENCES generations(digest)
         );
         CREATE TABLE IF NOT EXISTS expansion_activation_evidence (
           activation_digest TEXT NOT NULL,
           evidence_digest TEXT NOT NULL,
           PRIMARY KEY (activation_digest, evidence_digest),
           FOREIGN KEY (activation_digest) REFERENCES expansion_activations(activation_digest)
         );
         CREATE TABLE IF NOT EXISTS expansion_activation_scope (
           activation_digest TEXT NOT NULL,
           scope_digest TEXT NOT NULL,
           PRIMARY KEY (activation_digest, scope_digest),
           FOREIGN KEY (activation_digest) REFERENCES expansion_activations(activation_digest)
         );
         CREATE TABLE IF NOT EXISTS supersessions (
           supersession_digest TEXT PRIMARY KEY,
           subject_digest TEXT NOT NULL,
           successor_digest TEXT NOT NULL,
           kind TEXT NOT NULL,
           source_generation_digest TEXT NOT NULL,
           FOREIGN KEY (source_generation_digest) REFERENCES generations(digest)
         );
         CREATE TABLE IF NOT EXISTS supersession_scope (
           supersession_digest TEXT NOT NULL,
           scope_digest TEXT NOT NULL,
           PRIMARY KEY (supersession_digest, scope_digest),
           FOREIGN KEY (supersession_digest) REFERENCES supersessions(supersession_digest)
         );
         CREATE TABLE IF NOT EXISTS supersession_evidence (
           supersession_digest TEXT NOT NULL,
           evidence_digest TEXT NOT NULL,
           PRIMARY KEY (supersession_digest, evidence_digest),
           FOREIGN KEY (supersession_digest) REFERENCES supersessions(supersession_digest)
         );",
    )?;
    Ok(())
}

fn read_child_digests(
    connection: &Connection,
    sql: &str,
    parent_digest: ArtifactDigest,
) -> Result<Vec<ArtifactDigest>, AuthorityStoreError> {
    let mut statement = connection.prepare(sql)?;
    let rows = statement.query_map(params![parent_digest.as_str()], |row| {
        row.get::<_, String>(0)
    })?;
    let mut values = Vec::new();
    for row in rows {
        values.push(ArtifactDigest::parse(&row?)?);
    }
    Ok(values)
}

fn parse_promotion_class(value: &str) -> Result<PromotionClass, AuthorityStoreError> {
    match value {
        "THEOREM_JUDGEMENT" => Ok(PromotionClass::TheoremJudgement),
        "STRUCTURE_WITNESS" => Ok(PromotionClass::StructureWitness),
        "COUNTEREXAMPLE_NOGOOD" => Ok(PromotionClass::CounterexampleNogood),
        "INVARIANT_CERTIFIED_BOUND" => Ok(PromotionClass::InvariantCertifiedBound),
        "REPRESENTATION" => Ok(PromotionClass::Representation),
        "REDUCTION" => Ok(PromotionClass::Reduction),
        "MORPHISM_THEORY_INTERPRETATION" => Ok(PromotionClass::MorphismTheoryInterpretation),
        "DECOMPOSITION_SUFFICIENT_SUMMARY" => Ok(PromotionClass::DecompositionSufficientSummary),
        "SEMANTIC_PRIMITIVE" => Ok(PromotionClass::SemanticPrimitive),
        "CAPABILITY" => Ok(PromotionClass::Capability),
        "METAPRIMITIVE_SEARCH_METHOD" => Ok(PromotionClass::MetaprimitiveSearchMethod),
        "REALIZATION" => Ok(PromotionClass::Realization),
        "PACKAGE_THEORY_EXTENSION" => Ok(PromotionClass::PackageTheoryExtension),
        "TOOLCHAIN_CHECKER_REALIZATION" => Ok(PromotionClass::ToolchainCheckerRealization),
        _ => Err(AuthorityStoreError::ExpansionActivationUnknownPromotionClass(
            value.to_owned(),
        )),
    }
}

fn parse_activation_mode(value: &str) -> Result<ActivationMode, AuthorityStoreError> {
    match value {
        "MANUAL_ONLY" => Ok(ActivationMode::ManualOnly),
        "SHADOW_ONLY" => Ok(ActivationMode::ShadowOnly),
        "BOUNDED_AUTOMATIC" => Ok(ActivationMode::BoundedAutomatic),
        "DEFAULT_AUTOMATIC" => Ok(ActivationMode::DefaultAutomatic),
        "SUPERSEDED" => Ok(ActivationMode::Superseded),
        "QUARANTINED" => Ok(ActivationMode::Quarantined),
        _ => Err(AuthorityStoreError::ExpansionActivationUnknownMode(
            value.to_owned(),
        )),
    }
}

fn parse_supersession_kind(value: &str) -> Result<SupersessionKind, AuthorityStoreError> {
    match value {
        "SUPERSEDED_BY" => Ok(SupersessionKind::SupersededBy),
        "REFUTED_BY" => Ok(SupersessionKind::RefutedBy),
        "REPLACED_REALIZATION_BY" => Ok(SupersessionKind::ReplacedRealizationBy),
        "WITHDRAWN_FROM_DEFAULT_ACTIVATION" => Ok(SupersessionKind::WithdrawnFromDefaultActivation),
        _ => Err(AuthorityStoreError::SupersessionUnknownKind(value.to_owned())),
    }
}
