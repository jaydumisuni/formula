use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

use crate::query::ResourceContract;

const OBLIGATION_SCHEMA_V1: &str = "formula-obligation-ir-v1";
const OBLIGATION_OUTCOME_SCHEMA_V1: &str = "formula-obligation-outcome-v1";

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TerminalState {
    Satisfied,
    Refuted,
    CertifiedBound,
    SemanticUnknown,
    ResourceBoundedUnknown,
    UndecidableGeneralClass,
    Superseded,
    BlockedByAuthority,
}

impl TerminalState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "SATISFIED",
            Self::Refuted => "REFUTED",
            Self::CertifiedBound => "CERTIFIED_BOUND",
            Self::SemanticUnknown => "SEMANTIC_UNKNOWN",
            Self::ResourceBoundedUnknown => "RESOURCE_BOUNDED_UNKNOWN",
            Self::UndecidableGeneralClass => "UNDECIDABLE_GENERAL_CLASS",
            Self::Superseded => "SUPERSEDED",
            Self::BlockedByAuthority => "BLOCKED_BY_AUTHORITY",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObligationIR {
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    semantic_prerequisites: Vec<ArtifactDigest>,
    target_family: String,
    observer: ArtifactDigest,
    required_authority: ArtifactDigest,
    admissible_capabilities: Vec<ArtifactDigest>,
    dependencies: Vec<ArtifactDigest>,
    resource_contract: ResourceContract,
    terminal_state_policy: Vec<TerminalState>,
}

impl ObligationIR {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        universe_generation: ArtifactDigest,
        world: ArtifactDigest,
        semantic_prerequisites: Vec<ArtifactDigest>,
        target_family: impl Into<String>,
        observer: ArtifactDigest,
        required_authority: ArtifactDigest,
        admissible_capabilities: Vec<ArtifactDigest>,
        dependencies: Vec<ArtifactDigest>,
        resource_contract: ResourceContract,
        mut terminal_state_policy: Vec<TerminalState>,
    ) -> Self {
        terminal_state_policy.sort_unstable();
        terminal_state_policy.dedup();
        Self {
            universe_generation,
            world,
            semantic_prerequisites: sorted_digests(semantic_prerequisites),
            target_family: target_family.into(),
            observer,
            required_authority,
            admissible_capabilities: sorted_digests(admissible_capabilities),
            dependencies: sorted_digests(dependencies),
            resource_contract,
            terminal_state_policy,
        }
    }

    pub fn universe_generation(&self) -> ArtifactDigest {
        self.universe_generation
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn observer(&self) -> ArtifactDigest {
        self.observer
    }

    pub fn required_authority(&self) -> ArtifactDigest {
        self.required_authority
    }

    pub fn admissible_capabilities(&self) -> &[ArtifactDigest] {
        &self.admissible_capabilities
    }

    pub fn dependencies(&self) -> &[ArtifactDigest] {
        &self.dependencies
    }

    pub fn resource_contract(&self) -> ResourceContract {
        self.resource_contract
    }

    pub fn terminal_state_policy(&self) -> &[TerminalState] {
        &self.terminal_state_policy
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "admissible_capabilities".into(),
                digest_array(&self.admissible_capabilities),
            ),
            ("dependencies".into(), digest_array(&self.dependencies)),
            ("observer".into(), CanonicalValue::Digest(self.observer)),
            (
                "required_authority".into(),
                CanonicalValue::Digest(self.required_authority),
            ),
            (
                "resource_contract".into(),
                CanonicalValue::String(format!("{:?}", self.resource_contract)),
            ),
            (
                "schema".into(),
                CanonicalValue::String(OBLIGATION_SCHEMA_V1.into()),
            ),
            (
                "semantic_prerequisites".into(),
                digest_array(&self.semantic_prerequisites),
            ),
            (
                "target_family".into(),
                CanonicalValue::String(self.target_family.clone()),
            ),
            (
                "terminal_state_policy".into(),
                CanonicalValue::Array(
                    self.terminal_state_policy
                        .iter()
                        .map(|state| CanonicalValue::String(state.as_str().into()))
                        .collect(),
                ),
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
pub struct ObligationOutcome {
    obligation: ArtifactDigest,
    state: TerminalState,
}

impl ObligationOutcome {
    pub fn new(obligation: ArtifactDigest, state: TerminalState) -> Self {
        Self { obligation, state }
    }

    pub fn resource_exhausted(obligation: ArtifactDigest) -> Self {
        Self::new(obligation, TerminalState::ResourceBoundedUnknown)
    }

    pub fn obligation(&self) -> ArtifactDigest {
        self.obligation
    }

    pub fn state(&self) -> TerminalState {
        self.state
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        CanonicalValue::Object(BTreeMap::from([
            ("obligation".into(), CanonicalValue::Digest(self.obligation)),
            (
                "schema".into(),
                CanonicalValue::String(OBLIGATION_OUTCOME_SCHEMA_V1.into()),
            ),
            (
                "state".into(),
                CanonicalValue::String(self.state.as_str().into()),
            ),
        ]))
        .to_canonical_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        ArtifactDigest::of_bytes(&self.canonical_bytes())
    }
}
