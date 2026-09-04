use super::{AuthorityStore, AuthorityStoreError};
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    promotion::{PromotionRecord, PromotionState},
};
use rusqlite::{OptionalExtension, params};

#[derive(Debug)]
struct ActivationRow {
    activation_digest: String,
    generation_digest: String,
    candidate_digest: String,
    policy_digest: String,
}

impl AuthorityStore {
    pub fn admit_semantic_activation(
        &mut self,
        record: &PromotionRecord,
        primitive: ArtifactDigest,
    ) -> Result<PromotionRecord, AuthorityStoreError> {
        ensure_activation_tables(self)?;

        if record.state() != PromotionState::Activated {
            return Err(AuthorityStoreError::SemanticActivationStateMismatch);
        }

        let active = self
            .active_generation()?
            .ok_or(AuthorityStoreError::NoActiveGeneration)?;
        if active != record.generation() {
            return Err(AuthorityStoreError::SemanticActivationGenerationMismatch {
                expected: active,
                actual: record.generation(),
            });
        }

        let generation = self.replay_generation(record.generation())?;
        if !generation.admitted().contains(&primitive) {
            return Err(AuthorityStoreError::SemanticActivationPrimitiveNotAdmitted(
                primitive,
            ));
        }
        if !record.semantic_artifacts().contains(&primitive) {
            return Err(AuthorityStoreError::SemanticActivationPrimitiveNotRecorded(
                primitive,
            ));
        }
        for evidence in record.evidence() {
            if !generation.authority_bindings().contains(evidence) {
                return Err(
                    AuthorityStoreError::SemanticActivationEvidenceNotAuthorityBound(*evidence),
                );
            }
        }

        let activation_digest = record.structural_digest();
        self.connection.execute(
            "INSERT INTO semantic_activations (
                 activation_digest, generation_digest, primitive_digest,
                 candidate_digest, policy_digest
             ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                activation_digest.as_str(),
                record.generation().as_str(),
                primitive.as_str(),
                record.candidate().as_str(),
                record.policy().as_str(),
            ],
        )?;
        for evidence in record.evidence() {
            self.connection.execute(
                "INSERT INTO semantic_activation_evidence (activation_digest, evidence_digest)
                 VALUES (?1, ?2)",
                params![activation_digest.as_str(), evidence.as_str()],
            )?;
        }
        for artifact in record.semantic_artifacts() {
            self.connection.execute(
                "INSERT INTO semantic_activation_artifacts (activation_digest, artifact_digest)
                 VALUES (?1, ?2)",
                params![activation_digest.as_str(), artifact.as_str()],
            )?;
        }

        Ok(record.clone())
    }

    pub fn resolve_semantic_activation(
        &self,
        generation: ArtifactDigest,
        primitive: ArtifactDigest,
    ) -> Result<Option<PromotionRecord>, AuthorityStoreError> {
        ensure_activation_tables(self)?;

        let row: Option<ActivationRow> = self
            .connection
            .query_row(
                "SELECT activation_digest, generation_digest, candidate_digest, policy_digest
                 FROM semantic_activations
                 WHERE generation_digest = ?1 AND primitive_digest = ?2",
                params![generation.as_str(), primitive.as_str()],
                |row| {
                    Ok(ActivationRow {
                        activation_digest: row.get(0)?,
                        generation_digest: row.get(1)?,
                        candidate_digest: row.get(2)?,
                        policy_digest: row.get(3)?,
                    })
                },
            )
            .optional()?;

        let Some(row) = row else {
            return Ok(None);
        };

        let stored_digest = ArtifactDigest::parse(&row.activation_digest)?;
        let evidence = read_activation_digests(
            self,
            "SELECT evidence_digest FROM semantic_activation_evidence
             WHERE activation_digest = ?1 ORDER BY evidence_digest",
            stored_digest,
        )?;
        let semantic_artifacts = read_activation_digests(
            self,
            "SELECT artifact_digest FROM semantic_activation_artifacts
             WHERE activation_digest = ?1 ORDER BY artifact_digest",
            stored_digest,
        )?;
        let record = PromotionRecord::new(
            ArtifactDigest::parse(&row.candidate_digest)?,
            PromotionState::Activated,
            ArtifactDigest::parse(&row.generation_digest)?,
            ArtifactDigest::parse(&row.policy_digest)?,
            evidence,
            semantic_artifacts,
        );
        let reconstructed = record.structural_digest();
        if reconstructed != stored_digest {
            return Err(AuthorityStoreError::SemanticActivationDigestMismatch {
                stored: stored_digest,
                reconstructed,
            });
        }

        Ok(Some(record))
    }
}

fn ensure_activation_tables(store: &AuthorityStore) -> Result<(), AuthorityStoreError> {
    store.connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS semantic_activations (
           activation_digest TEXT PRIMARY KEY,
           generation_digest TEXT NOT NULL,
           primitive_digest TEXT NOT NULL,
           candidate_digest TEXT NOT NULL,
           policy_digest TEXT NOT NULL,
           UNIQUE (generation_digest, primitive_digest),
           FOREIGN KEY (generation_digest) REFERENCES generations(digest)
         );
         CREATE TABLE IF NOT EXISTS semantic_activation_evidence (
           activation_digest TEXT NOT NULL,
           evidence_digest TEXT NOT NULL,
           PRIMARY KEY (activation_digest, evidence_digest),
           FOREIGN KEY (activation_digest) REFERENCES semantic_activations(activation_digest)
         );
         CREATE TABLE IF NOT EXISTS semantic_activation_artifacts (
           activation_digest TEXT NOT NULL,
           artifact_digest TEXT NOT NULL,
           PRIMARY KEY (activation_digest, artifact_digest),
           FOREIGN KEY (activation_digest) REFERENCES semantic_activations(activation_digest)
         );",
    )?;
    Ok(())
}

fn read_activation_digests(
    store: &AuthorityStore,
    sql: &str,
    activation_digest: ArtifactDigest,
) -> Result<Vec<ArtifactDigest>, AuthorityStoreError> {
    let mut statement = store.connection.prepare(sql)?;
    let rows = statement.query_map(params![activation_digest.as_str()], |row| {
        row.get::<_, String>(0)
    })?;
    let mut values = Vec::new();
    for row in rows {
        values.push(ArtifactDigest::parse(&row?)?);
    }
    Ok(values)
}
