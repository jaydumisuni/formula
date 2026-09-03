use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use formula_engine::candidate_space::FrozenCandidate;
use std::collections::BTreeMap;

const BLINDNESS_MANIFEST_SCHEMA_V1: &str = "formula-p6-blindness-manifest-v1";
const FROZEN_SUBMISSION_SCHEMA_V1: &str = "formula-p6-frozen-submission-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FirstLightTarget {
    FlA,
    FlB,
    FlC,
}

impl FirstLightTarget {
    fn as_str(self) -> &'static str {
        match self {
            Self::FlA => "FL-A",
            Self::FlB => "FL-B",
            Self::FlC => "FL-C",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlindnessManifest {
    target: FirstLightTarget,
    sealed_target_digest: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    query_digest: ArtifactDigest,
    grammar_or_routes_digest: ArtifactDigest,
    package_set_digest: ArtifactDigest,
    oracle_contract_digest: ArtifactDigest,
}

impl BlindnessManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target: FirstLightTarget,
        sealed_target_digest: ArtifactDigest,
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        query_digest: ArtifactDigest,
        grammar_or_routes_digest: ArtifactDigest,
        package_set_digest: ArtifactDigest,
        oracle_contract_digest: ArtifactDigest,
    ) -> Self {
        Self {
            target,
            sealed_target_digest,
            universe_generation,
            world,
            query_digest,
            grammar_or_routes_digest,
            package_set_digest,
            oracle_contract_digest,
        }
    }

    pub fn target(&self) -> FirstLightTarget {
        self.target
    }

    pub fn sealed_target_digest(&self) -> ArtifactDigest {
        self.sealed_target_digest
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn query_digest(&self) -> ArtifactDigest {
        self.query_digest
    }

    pub fn grammar_or_routes_digest(&self) -> ArtifactDigest {
        self.grammar_or_routes_digest
    }

    pub fn package_set_digest(&self) -> ArtifactDigest {
        self.package_set_digest
    }

    pub fn oracle_contract_digest(&self) -> ArtifactDigest {
        self.oracle_contract_digest
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "grammar_or_routes_digest".into(),
                CanonicalValue::Digest(self.grammar_or_routes_digest),
            ),
            (
                "oracle_contract_digest".into(),
                CanonicalValue::Digest(self.oracle_contract_digest),
            ),
            (
                "package_set_digest".into(),
                CanonicalValue::Digest(self.package_set_digest),
            ),
            (
                "query_digest".into(),
                CanonicalValue::Digest(self.query_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(BLINDNESS_MANIFEST_SCHEMA_V1.into()),
            ),
            (
                "sealed_target_digest".into(),
                CanonicalValue::Digest(self.sealed_target_digest),
            ),
            (
                "target".into(),
                CanonicalValue::String(self.target.as_str().into()),
            ),
            (
                "universe_generation".into(),
                CanonicalValue::Digest(self.universe_generation),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenSubmission {
    target: FirstLightTarget,
    candidate: FrozenCandidate,
}

impl FrozenSubmission {
    pub fn new(target: FirstLightTarget, candidate: FrozenCandidate) -> Self {
        Self { target, candidate }
    }

    pub fn target(&self) -> FirstLightTarget {
        self.target
    }

    pub fn candidate(&self) -> &FrozenCandidate {
        &self.candidate
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "candidate_digest".into(),
                CanonicalValue::Digest(self.candidate.digest()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(FROZEN_SUBMISSION_SCHEMA_V1.into()),
            ),
            (
                "target".into(),
                CanonicalValue::String(self.target.as_str().into()),
            ),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
