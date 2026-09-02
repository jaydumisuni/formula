use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const CONTEXT_SCHEMA_V1: &str = "formula-candidate-space-context-v1";
const SPACE_SCHEMA_V1: &str = "formula-frozen-candidate-space-v1";
const CANDIDATE_SCHEMA_V1: &str = "formula-frozen-candidate-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CandidatePolarity {
    Exact,
    SoundOverApproximation,
    SoundUnderApproximation,
    HeuristicProposal,
}

impl CandidatePolarity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "EXACT",
            Self::SoundOverApproximation => "SOUND_OVER_APPROXIMATION",
            Self::SoundUnderApproximation => "SOUND_UNDER_APPROXIMATION",
            Self::HeuristicProposal => "HEURISTIC_PROPOSAL",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompletenessClass {
    CompleteWithinBound,
    Incomplete,
}

impl CompletenessClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::CompleteWithinBound => "COMPLETE_WITHIN_BOUND",
            Self::Incomplete => "INCOMPLETE",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchAuthority {
    CandidateOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateSpaceContext {
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    query_digest: ArtifactDigest,
    obligation_digest: ArtifactDigest,
    grammar_or_routes_digest: ArtifactDigest,
    policy_digest: ArtifactDigest,
}

impl CandidateSpaceContext {
    pub fn new(
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        query_digest: ArtifactDigest,
        obligation_digest: ArtifactDigest,
        grammar_or_routes_digest: ArtifactDigest,
        policy_digest: ArtifactDigest,
    ) -> Self {
        Self {
            universe_generation,
            world,
            query_digest,
            obligation_digest,
            grammar_or_routes_digest,
            policy_digest,
        }
    }

    pub fn with_generation(mut self, value: ArtifactDigest) -> Self {
        self.universe_generation = value;
        self
    }

    pub fn with_world(mut self, value: ArtifactDigest) -> Self {
        self.world = value;
        self
    }

    pub fn with_query(mut self, value: ArtifactDigest) -> Self {
        self.query_digest = value;
        self
    }

    pub fn with_grammar_or_routes(mut self, value: ArtifactDigest) -> Self {
        self.grammar_or_routes_digest = value;
        self
    }

    pub fn with_policy(mut self, value: ArtifactDigest) -> Self {
        self.policy_digest = value;
        self
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

    pub fn obligation_digest(&self) -> ArtifactDigest {
        self.obligation_digest
    }

    pub fn grammar_or_routes_digest(&self) -> ArtifactDigest {
        self.grammar_or_routes_digest
    }

    pub fn policy_digest(&self) -> ArtifactDigest {
        self.policy_digest
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "grammar_or_routes_digest".into(),
                CanonicalValue::Digest(self.grammar_or_routes_digest),
            ),
            (
                "obligation_digest".into(),
                CanonicalValue::Digest(self.obligation_digest),
            ),
            (
                "policy_digest".into(),
                CanonicalValue::Digest(self.policy_digest),
            ),
            (
                "query_digest".into(),
                CanonicalValue::Digest(self.query_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(CONTEXT_SCHEMA_V1.into()),
            ),
            (
                "universe_generation".into(),
                CanonicalValue::Digest(self.universe_generation),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_value().to_canonical_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCandidateSpace {
    context: CandidateSpaceContext,
    backend_kind: String,
    polarity: CandidatePolarity,
    completeness: CompletenessClass,
    state_digest: ArtifactDigest,
}

impl FrozenCandidateSpace {
    pub fn new(
        context: CandidateSpaceContext,
        backend_kind: impl Into<String>,
        polarity: CandidatePolarity,
        completeness: CompletenessClass,
        state_digest: ArtifactDigest,
    ) -> Self {
        Self {
            context,
            backend_kind: backend_kind.into(),
            polarity,
            completeness,
            state_digest,
        }
    }

    pub fn context(&self) -> &CandidateSpaceContext {
        &self.context
    }

    pub fn polarity(&self) -> CandidatePolarity {
        self.polarity
    }

    pub fn completeness(&self) -> CompletenessClass {
        self.completeness
    }

    pub fn state_digest(&self) -> ArtifactDigest {
        self.state_digest
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "backend_kind".into(),
                CanonicalValue::String(self.backend_kind.clone()),
            ),
            (
                "completeness".into(),
                CanonicalValue::String(self.completeness.as_str().into()),
            ),
            (
                "context_digest".into(),
                CanonicalValue::Digest(self.context.digest()),
            ),
            (
                "polarity".into(),
                CanonicalValue::String(self.polarity.as_str().into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(SPACE_SCHEMA_V1.into()),
            ),
            (
                "state_digest".into(),
                CanonicalValue::Digest(self.state_digest),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenCandidate {
    space_digest: ArtifactDigest,
    candidate_digest: ArtifactDigest,
    cost: u64,
}

impl FrozenCandidate {
    pub fn new(space_digest: ArtifactDigest, candidate_digest: ArtifactDigest, cost: u64) -> Self {
        Self {
            space_digest,
            candidate_digest,
            cost,
        }
    }

    pub fn space_digest(&self) -> ArtifactDigest {
        self.space_digest
    }

    pub fn candidate_digest(&self) -> ArtifactDigest {
        self.candidate_digest
    }

    pub fn cost(&self) -> u64 {
        self.cost
    }

    pub fn authority(&self) -> SearchAuthority {
        SearchAuthority::CandidateOnly
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "candidate_digest".into(),
                CanonicalValue::Digest(self.candidate_digest),
            ),
            ("cost".into(), CanonicalValue::U64(self.cost)),
            (
                "schema".into(),
                CanonicalValue::String(CANDIDATE_SCHEMA_V1.into()),
            ),
            (
                "space_digest".into(),
                CanonicalValue::Digest(self.space_digest),
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
