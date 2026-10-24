//! Optional neural-learning request to the existing SUP product Director.
//! Formula owns its mathematical truth, self-expansion, and admission.
//! SUP only advises on neural capability; this module runs no model/training.

use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SupHandoffError {
    MissingInstruction,
    InvalidSupGeneration,
    MissingCapability,
    MissingAcceptanceCriteria,
    MissingRegressionCriteria,
    MissingBaseline,
    OversizedRequest,
}

/// Caller-grounded, explicit intent. Formula never invents a neural
/// baseline, source qualification, method, or training authorization.
#[derive(Clone, Debug)]
pub struct SupNeuralIntent<'a> {
    pub instruction: &'a str,
    pub capability: &'a str,
    pub baseline_metric: &'a str,
    pub baseline_evidence: ArtifactDigest,
    pub acceptance_criteria: &'a [String],
    pub forbidden_regressions: &'a [String],
}

/// Explicitly chosen specialist request. Never inferred from search failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupLearningRequest {
    bytes: Vec<u8>,
    digest: ArtifactDigest,
    sup_generation: ArtifactDigest,
}

impl SupLearningRequest {
    /// None means no neural task was justified. Ordinary Formula mathematical
    /// discovery never requires SUP to be reachable.
    pub fn optional_neural(
        neural_needed: bool,
        sup_generation: &str,
        intent: &SupNeuralIntent<'_>,
    ) -> Result<Option<Self>, SupHandoffError> {
        if !neural_needed {
            return Ok(None);
        }
        let SupNeuralIntent {
            instruction,
            capability,
            baseline_metric,
            baseline_evidence,
            acceptance_criteria,
            forbidden_regressions,
        } = intent;
        if instruction.trim().is_empty() {
            return Err(SupHandoffError::MissingInstruction);
        }
        if capability.trim().is_empty() {
            return Err(SupHandoffError::MissingCapability);
        }
        if baseline_metric.trim().is_empty() {
            return Err(SupHandoffError::MissingBaseline);
        }
        if acceptance_criteria.is_empty() || acceptance_criteria.iter().any(|s| s.trim().is_empty())
        {
            return Err(SupHandoffError::MissingAcceptanceCriteria);
        }
        if forbidden_regressions.is_empty()
            || forbidden_regressions.iter().any(|s| s.trim().is_empty())
        {
            return Err(SupHandoffError::MissingRegressionCriteria);
        }
        let generation = ArtifactDigest::parse(sup_generation)
            .map_err(|_| SupHandoffError::InvalidSupGeneration)?;
        // SUP's existing G4 production-director schema. Keep the contract
        // small: without externally qualified learning source SUP will
        // correctly return AWAIT_SOURCE rather than invent neural authority.
        let interpretation = CanonicalValue::Object(BTreeMap::from([
            (
                "capability_need".into(),
                CanonicalValue::String("neural".into()),
            ),
            (
                "capability".into(),
                CanonicalValue::String((*capability).to_owned()),
            ),
            (
                "acceptance_criteria".into(),
                CanonicalValue::Array(
                    acceptance_criteria
                        .iter()
                        .cloned()
                        .map(CanonicalValue::String)
                        .collect(),
                ),
            ),
            (
                "baseline".into(),
                CanonicalValue::Object(BTreeMap::from([
                    (
                        "metric".into(),
                        CanonicalValue::String((*baseline_metric).into()),
                    ),
                    (
                        "evidence_digest".into(),
                        CanonicalValue::Digest(*baseline_evidence),
                    ),
                ])),
            ),
            (
                "forbidden_regressions".into(),
                CanonicalValue::Array(
                    forbidden_regressions
                        .iter()
                        .cloned()
                        .map(CanonicalValue::String)
                        .collect(),
                ),
            ),
        ]));
        let canonical = CanonicalValue::Object(BTreeMap::from([
            ("subject".into(), CanonicalValue::String("SUP".into())),
            (
                "instruction".into(),
                CanonicalValue::String((*instruction).to_owned()),
            ),
            (
                "target_product".into(),
                CanonicalValue::String("Formula".into()),
            ),
            ("sup_generation".into(), CanonicalValue::Digest(generation)),
            ("autonomous_deploy".into(), CanonicalValue::Bool(false)),
            ("interpretation".into(), interpretation),
        ]));
        let bytes = canonical.to_canonical_bytes();
        if bytes.len() > 24 * 1024 || instruction.len() > 20_000 {
            return Err(SupHandoffError::OversizedRequest);
        }
        Ok(Some(Self {
            digest: ArtifactDigest::of_bytes(&bytes),
            bytes,
            sup_generation: generation,
        }))
    }

    pub fn request_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn digest(&self) -> ArtifactDigest {
        self.digest
    }
    pub fn sup_generation(&self) -> ArtifactDigest {
        self.sup_generation
    }
}

/// SUP's returned bytes are evidence/advice only. A matching hash is NOT a
/// source qualification, neural artifact admission, mathematical certificate
/// or a training authorization; SUP and Formula retain their own Directors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UntrustedSupAdvice {
    source_request: ArtifactDigest,
    bytes: Vec<u8>,
    digest: ArtifactDigest,
}
impl UntrustedSupAdvice {
    pub fn observe(
        source: &SupLearningRequest,
        claimed_request: ArtifactDigest,
        bytes: Vec<u8>,
        claimed_digest: ArtifactDigest,
    ) -> Option<Self> {
        if source.digest != claimed_request || ArtifactDigest::of_bytes(&bytes) != claimed_digest {
            return None;
        }
        Some(Self {
            source_request: claimed_request,
            bytes,
            digest: claimed_digest,
        })
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn digest(&self) -> ArtifactDigest {
        self.digest
    }
    pub fn source_request(&self) -> ArtifactDigest {
        self.source_request
    }
}
