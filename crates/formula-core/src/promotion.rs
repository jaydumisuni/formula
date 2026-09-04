use crate::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
};
use std::collections::BTreeMap;

const PROMOTION_SCHEMA_V1: &str = "formula-promotion-v1";

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PromotionState {
    Certified,
    Admitted,
    Activated,
    Quarantined,
}

impl PromotionState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "CERTIFIED",
            Self::Admitted => "ADMITTED",
            Self::Activated => "ACTIVATED",
            Self::Quarantined => "QUARANTINED",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionCandidate {
    frozen_candidate: ArtifactDigest,
    promotion_manifest: ArtifactDigest,
    parent_generation: ArtifactDigest,
    proof_generation: ArtifactDigest,
    dependency_cone: Vec<ArtifactDigest>,
    supersedes: Vec<ArtifactDigest>,
}

impl PromotionCandidate {
    pub fn new(
        frozen_candidate: ArtifactDigest,
        promotion_manifest: ArtifactDigest,
        parent_generation: ArtifactDigest,
        proof_generation: ArtifactDigest,
        dependency_cone: Vec<ArtifactDigest>,
        supersedes: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            frozen_candidate,
            promotion_manifest,
            parent_generation,
            proof_generation,
            dependency_cone: sorted_digests(dependency_cone),
            supersedes: sorted_digests(supersedes),
        }
    }

    pub fn frozen_candidate(&self) -> ArtifactDigest {
        self.frozen_candidate
    }

    pub fn promotion_manifest(&self) -> ArtifactDigest {
        self.promotion_manifest
    }

    pub fn parent_generation(&self) -> ArtifactDigest {
        self.parent_generation
    }

    pub fn proof_generation(&self) -> ArtifactDigest {
        self.proof_generation
    }

    pub fn dependency_cone(&self) -> &[ArtifactDigest] {
        &self.dependency_cone
    }

    pub fn supersedes(&self) -> &[ArtifactDigest] {
        &self.supersedes
    }
}

impl StructuralIdentity for PromotionCandidate {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "dependency_cone".into(),
                digest_array(&self.dependency_cone),
            ),
            (
                "frozen_candidate".into(),
                CanonicalValue::Digest(self.frozen_candidate),
            ),
            (
                "kind".into(),
                CanonicalValue::String("PromotionCandidate".into()),
            ),
            (
                "parent_generation".into(),
                CanonicalValue::Digest(self.parent_generation),
            ),
            (
                "promotion_manifest".into(),
                CanonicalValue::Digest(self.promotion_manifest),
            ),
            (
                "proof_generation".into(),
                CanonicalValue::Digest(self.proof_generation),
            ),
            (
                "schema".into(),
                CanonicalValue::String(PROMOTION_SCHEMA_V1.into()),
            ),
            ("supersedes".into(), digest_array(&self.supersedes)),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionRecord {
    candidate: ArtifactDigest,
    state: PromotionState,
    generation: ArtifactDigest,
    policy: ArtifactDigest,
    evidence: Vec<ArtifactDigest>,
    semantic_artifacts: Vec<ArtifactDigest>,
}

impl PromotionRecord {
    pub fn new(
        candidate: ArtifactDigest,
        state: PromotionState,
        generation: ArtifactDigest,
        policy: ArtifactDigest,
        evidence: Vec<ArtifactDigest>,
        semantic_artifacts: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            candidate,
            state,
            generation,
            policy,
            evidence: sorted_digests(evidence),
            semantic_artifacts: sorted_digests(semantic_artifacts),
        }
    }

    pub fn candidate(&self) -> ArtifactDigest {
        self.candidate
    }

    pub fn state(&self) -> PromotionState {
        self.state
    }

    pub fn generation(&self) -> ArtifactDigest {
        self.generation
    }

    pub fn policy(&self) -> ArtifactDigest {
        self.policy
    }

    pub fn evidence(&self) -> &[ArtifactDigest] {
        &self.evidence
    }

    pub fn semantic_artifacts(&self) -> &[ArtifactDigest] {
        &self.semantic_artifacts
    }
}

impl StructuralIdentity for PromotionRecord {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "candidate".into(),
                CanonicalValue::Digest(self.candidate),
            ),
            ("evidence".into(), digest_array(&self.evidence)),
            (
                "generation".into(),
                CanonicalValue::Digest(self.generation),
            ),
            (
                "kind".into(),
                CanonicalValue::String("PromotionRecord".into()),
            ),
            ("policy".into(), CanonicalValue::Digest(self.policy)),
            (
                "schema".into(),
                CanonicalValue::String(PROMOTION_SCHEMA_V1.into()),
            ),
            (
                "semantic_artifacts".into(),
                digest_array(&self.semantic_artifacts),
            ),
            (
                "state".into(),
                CanonicalValue::String(self.state.as_str().into()),
            ),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuarantineRecord {
    candidate: ArtifactDigest,
    parent_generation: ArtifactDigest,
    reason: ArtifactDigest,
    conflicts: Vec<ArtifactDigest>,
}

impl QuarantineRecord {
    pub fn new(
        candidate: ArtifactDigest,
        parent_generation: ArtifactDigest,
        reason: ArtifactDigest,
        conflicts: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            candidate,
            parent_generation,
            reason,
            conflicts: sorted_digests(conflicts),
        }
    }

    pub fn candidate(&self) -> ArtifactDigest {
        self.candidate
    }

    pub fn parent_generation(&self) -> ArtifactDigest {
        self.parent_generation
    }

    pub fn reason(&self) -> ArtifactDigest {
        self.reason
    }

    pub fn conflicts(&self) -> &[ArtifactDigest] {
        &self.conflicts
    }
}

impl StructuralIdentity for QuarantineRecord {
    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            ("candidate".into(), CanonicalValue::Digest(self.candidate)),
            ("conflicts".into(), digest_array(&self.conflicts)),
            (
                "kind".into(),
                CanonicalValue::String("QuarantineRecord".into()),
            ),
            (
                "parent_generation".into(),
                CanonicalValue::Digest(self.parent_generation),
            ),
            ("reason".into(), CanonicalValue::Digest(self.reason)),
            (
                "schema".into(),
                CanonicalValue::String(PROMOTION_SCHEMA_V1.into()),
            ),
        ]))
    }
}
