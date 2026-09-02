use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

use crate::query::{ResourceContract, SideEffectPolicy};

const WORK_CELL_SCHEMA_V1: &str = "formula-work-cell-plan-v1";

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CheckpointPolicy {
    Never,
    AtStopBoundary,
}

impl CheckpointPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Never => "NEVER",
            Self::AtStopBoundary => "AT_STOP_BOUNDARY",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StopCondition {
    Satisfied,
    Refuted,
    CertifiedBound,
    SemanticUnknown,
    ResourceBoundedUnknown,
    BlockedByAuthority,
}

impl StopCondition {
    fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "SATISFIED",
            Self::Refuted => "REFUTED",
            Self::CertifiedBound => "CERTIFIED_BOUND",
            Self::SemanticUnknown => "SEMANTIC_UNKNOWN",
            Self::ResourceBoundedUnknown => "RESOURCE_BOUNDED_UNKNOWN",
            Self::BlockedByAuthority => "BLOCKED_BY_AUTHORITY",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkCellPlan {
    obligation_digest: ArtifactDigest,
    semantic_inputs: Vec<ArtifactDigest>,
    allowed_packages: Vec<ArtifactDigest>,
    allowed_capabilities: Vec<ArtifactDigest>,
    evidence_requirement: ArtifactDigest,
    required_authority: ArtifactDigest,
    resource_budget: ResourceContract,
    deterministic_replay_key: ArtifactDigest,
    checkpoint_policy: CheckpointPolicy,
    side_effect_limits: SideEffectPolicy,
    stop_conditions: Vec<StopCondition>,
}

impl WorkCellPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        obligation_digest: ArtifactDigest,
        semantic_inputs: Vec<ArtifactDigest>,
        allowed_packages: Vec<ArtifactDigest>,
        allowed_capabilities: Vec<ArtifactDigest>,
        evidence_requirement: ArtifactDigest,
        required_authority: ArtifactDigest,
        resource_budget: ResourceContract,
        deterministic_replay_key: ArtifactDigest,
        checkpoint_policy: CheckpointPolicy,
        side_effect_limits: SideEffectPolicy,
        mut stop_conditions: Vec<StopCondition>,
    ) -> Self {
        stop_conditions.sort_unstable();
        stop_conditions.dedup();
        Self {
            obligation_digest,
            semantic_inputs: sorted_digests(semantic_inputs),
            allowed_packages: sorted_digests(allowed_packages),
            allowed_capabilities: sorted_digests(allowed_capabilities),
            evidence_requirement,
            required_authority,
            resource_budget,
            deterministic_replay_key,
            checkpoint_policy,
            side_effect_limits,
            stop_conditions,
        }
    }

    pub fn obligation_digest(&self) -> ArtifactDigest {
        self.obligation_digest
    }

    pub fn required_authority(&self) -> ArtifactDigest {
        self.required_authority
    }

    pub fn side_effect_limits(&self) -> SideEffectPolicy {
        self.side_effect_limits
    }

    pub fn deterministic_replay_key(&self) -> ArtifactDigest {
        self.deterministic_replay_key
    }

    pub fn with_obligation(mut self, value: ArtifactDigest) -> Self {
        self.obligation_digest = value;
        self
    }

    pub fn with_semantic_inputs(mut self, values: Vec<ArtifactDigest>) -> Self {
        self.semantic_inputs = sorted_digests(values);
        self
    }

    pub fn with_allowed_packages(mut self, values: Vec<ArtifactDigest>) -> Self {
        self.allowed_packages = sorted_digests(values);
        self
    }

    pub fn with_allowed_capabilities(mut self, values: Vec<ArtifactDigest>) -> Self {
        self.allowed_capabilities = sorted_digests(values);
        self
    }

    pub fn with_evidence_requirement(mut self, value: ArtifactDigest) -> Self {
        self.evidence_requirement = value;
        self
    }

    pub fn with_required_authority(mut self, value: ArtifactDigest) -> Self {
        self.required_authority = value;
        self
    }

    pub fn with_resource_budget(mut self, value: ResourceContract) -> Self {
        self.resource_budget = value;
        self
    }

    pub fn with_deterministic_replay_key(mut self, value: ArtifactDigest) -> Self {
        self.deterministic_replay_key = value;
        self
    }

    pub fn with_checkpoint_policy(mut self, value: CheckpointPolicy) -> Self {
        self.checkpoint_policy = value;
        self
    }

    pub fn with_side_effect_limits(mut self, value: SideEffectPolicy) -> Self {
        self.side_effect_limits = value;
        self
    }

    pub fn with_stop_conditions(mut self, mut values: Vec<StopCondition>) -> Self {
        values.sort_unstable();
        values.dedup();
        self.stop_conditions = values;
        self
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "allowed_capabilities".into(),
                digest_array(&self.allowed_capabilities),
            ),
            (
                "allowed_packages".into(),
                digest_array(&self.allowed_packages),
            ),
            (
                "checkpoint_policy".into(),
                CanonicalValue::String(self.checkpoint_policy.as_str().into()),
            ),
            (
                "deterministic_replay_key".into(),
                CanonicalValue::Digest(self.deterministic_replay_key),
            ),
            (
                "evidence_requirement".into(),
                CanonicalValue::Digest(self.evidence_requirement),
            ),
            (
                "obligation_digest".into(),
                CanonicalValue::Digest(self.obligation_digest),
            ),
            (
                "required_authority".into(),
                CanonicalValue::Digest(self.required_authority),
            ),
            (
                "resource_budget".into(),
                CanonicalValue::String(format!("{:?}", self.resource_budget)),
            ),
            (
                "schema".into(),
                CanonicalValue::String(WORK_CELL_SCHEMA_V1.into()),
            ),
            (
                "semantic_inputs".into(),
                digest_array(&self.semantic_inputs),
            ),
            (
                "side_effect_limits".into(),
                CanonicalValue::String(format!("{:?}", self.side_effect_limits)),
            ),
            (
                "stop_conditions".into(),
                CanonicalValue::Array(
                    self.stop_conditions
                        .iter()
                        .map(|condition| CanonicalValue::String(condition.as_str().into()))
                        .collect(),
                ),
            ),
        ]))
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_value().to_canonical_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
