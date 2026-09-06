use crate::{
    artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest,
};
use std::collections::BTreeMap;

const P11_PROOF_SCHEMA_V1: &str = "formula-federation-breadth-proof-v1";

fn object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(P11_PROOF_SCHEMA_V1.into()),
        ),
    ])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FederationRouteKind {
    SatLrat,
    ExactArithmetic,
}

impl FederationRouteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SatLrat => "SAT_LRAT",
            Self::ExactArithmetic => "EXACT_ARITHMETIC",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationRouteProof {
    kind: FederationRouteKind,
    package: ArtifactDigest,
    adapter: ArtifactDigest,
    semantic_input: ArtifactDigest,
    checked_evidence: ArtifactDigest,
    certified_fact: ArtifactDigest,
}

impl FederationRouteProof {
    pub fn new(
        kind: FederationRouteKind,
        package: ArtifactDigest,
        adapter: ArtifactDigest,
        semantic_input: ArtifactDigest,
        checked_evidence: ArtifactDigest,
        certified_fact: ArtifactDigest,
    ) -> Self {
        Self {
            kind,
            package,
            adapter,
            semantic_input,
            checked_evidence,
            certified_fact,
        }
    }

    pub fn kind(&self) -> FederationRouteKind {
        self.kind
    }

    pub fn package(&self) -> ArtifactDigest {
        self.package
    }

    pub fn adapter(&self) -> ArtifactDigest {
        self.adapter
    }

    pub fn semantic_input(&self) -> ArtifactDigest {
        self.semantic_input
    }

    pub fn checked_evidence(&self) -> ArtifactDigest {
        self.checked_evidence
    }

    pub fn certified_fact(&self) -> ArtifactDigest {
        self.certified_fact
    }
}

impl StructuralIdentity for FederationRouteProof {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("FederationRouteProof");
        value.insert(
            "route_kind".into(),
            CanonicalValue::String(self.kind.as_str().into()),
        );
        value.insert("package".into(), CanonicalValue::Digest(self.package));
        value.insert("adapter".into(), CanonicalValue::Digest(self.adapter));
        value.insert(
            "semantic_input".into(),
            CanonicalValue::Digest(self.semantic_input),
        );
        value.insert(
            "checked_evidence".into(),
            CanonicalValue::Digest(self.checked_evidence),
        );
        value.insert(
            "certified_fact".into(),
            CanonicalValue::Digest(self.certified_fact),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FederationNegativeControl {
    CandidateOnlyAuthorityAttempt,
    ForgedLratHint,
    LratMissingEmptyClause,
    UnsupportedRatProofFailsClosed,
    WrongSatCheckerRoute,
    IncorrectExactArithmeticResult,
    MalformedExactArithmeticDecimal,
    WrongArithmeticTranslation,
    StaleSemanticInputDigest,
    SharedFactPolarityUpgrade,
    MissingBridgeContract,
    WrongBridgeDirection,
    UnsafeCompositionClass,
    ProducerIdentityCannotAuthorize,
}

impl FederationNegativeControl {
    pub const ALL: [Self; 14] = [
        Self::CandidateOnlyAuthorityAttempt,
        Self::ForgedLratHint,
        Self::LratMissingEmptyClause,
        Self::UnsupportedRatProofFailsClosed,
        Self::WrongSatCheckerRoute,
        Self::IncorrectExactArithmeticResult,
        Self::MalformedExactArithmeticDecimal,
        Self::WrongArithmeticTranslation,
        Self::StaleSemanticInputDigest,
        Self::SharedFactPolarityUpgrade,
        Self::MissingBridgeContract,
        Self::WrongBridgeDirection,
        Self::UnsafeCompositionClass,
        Self::ProducerIdentityCannotAuthorize,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CandidateOnlyAuthorityAttempt => "NC11-01_CANDIDATE_ONLY_AUTHORITY_ATTEMPT",
            Self::ForgedLratHint => "NC11-02_FORGED_LRAT_HINT",
            Self::LratMissingEmptyClause => "NC11-03_LRAT_MISSING_EMPTY_CLAUSE",
            Self::UnsupportedRatProofFailsClosed => "NC11-04_UNSUPPORTED_RAT_PROOF_FAILS_CLOSED",
            Self::WrongSatCheckerRoute => "NC11-05_WRONG_SAT_CHECKER_ROUTE",
            Self::IncorrectExactArithmeticResult => "NC11-06_INCORRECT_EXACT_ARITHMETIC_RESULT",
            Self::MalformedExactArithmeticDecimal => "NC11-07_MALFORMED_EXACT_ARITHMETIC_DECIMAL",
            Self::WrongArithmeticTranslation => "NC11-08_WRONG_ARITHMETIC_TRANSLATION",
            Self::StaleSemanticInputDigest => "NC11-09_STALE_SEMANTIC_INPUT_DIGEST",
            Self::SharedFactPolarityUpgrade => "NC11-10_SHARED_FACT_POLARITY_UPGRADE",
            Self::MissingBridgeContract => "NC11-11_MISSING_BRIDGE_CONTRACT",
            Self::WrongBridgeDirection => "NC11-12_WRONG_BRIDGE_DIRECTION",
            Self::UnsafeCompositionClass => "NC11-13_UNSAFE_COMPOSITION_CLASS",
            Self::ProducerIdentityCannotAuthorize => "NC11-14_PRODUCER_IDENTITY_CANNOT_AUTHORIZE",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationNegativeControlEvidence {
    control: FederationNegativeControl,
    evidence: ArtifactDigest,
}

impl FederationNegativeControlEvidence {
    pub fn new(control: FederationNegativeControl, evidence: ArtifactDigest) -> Self {
        Self { control, evidence }
    }

    pub fn control(&self) -> FederationNegativeControl {
        self.control
    }

    pub fn evidence(&self) -> ArtifactDigest {
        self.evidence
    }
}

impl StructuralIdentity for FederationNegativeControlEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("FederationNegativeControlEvidence");
        value.insert(
            "control".into(),
            CanonicalValue::String(self.control.as_str().into()),
        );
        value.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FederationNegativeControlManifestError {
    MissingDuplicateOrUnexpectedControl,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationNegativeControlManifest {
    controls: Vec<FederationNegativeControlEvidence>,
}

impl FederationNegativeControlManifest {
    pub fn new(
        mut controls: Vec<FederationNegativeControlEvidence>,
    ) -> Result<Self, FederationNegativeControlManifestError> {
        controls.sort_by_key(FederationNegativeControlEvidence::control);
        if controls.len() != FederationNegativeControl::ALL.len()
            || controls
                .iter()
                .map(FederationNegativeControlEvidence::control)
                .ne(FederationNegativeControl::ALL)
        {
            return Err(
                FederationNegativeControlManifestError::MissingDuplicateOrUnexpectedControl,
            );
        }
        Ok(Self { controls })
    }

    pub fn controls(&self) -> &[FederationNegativeControlEvidence] {
        &self.controls
    }

    pub fn is_complete(&self) -> bool {
        self.controls.len() == FederationNegativeControl::ALL.len()
            && self
                .controls
                .iter()
                .map(FederationNegativeControlEvidence::control)
                .eq(FederationNegativeControl::ALL)
    }
}

impl StructuralIdentity for FederationNegativeControlManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("FederationNegativeControlManifest");
        value.insert(
            "controls".into(),
            CanonicalValue::Array(
                self.controls
                    .iter()
                    .map(StructuralIdentity::canonical_value)
                    .collect(),
            ),
        );
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationBreadthProofManifest {
    source_commit: String,
    predecessor_p10: ArtifactDigest,
    world: ArtifactDigest,
    sat_route: FederationRouteProof,
    arithmetic_route: FederationRouteProof,
    bridge: ArtifactDigest,
    composition: ArtifactDigest,
    bridged_fact: ArtifactDigest,
    final_target: ArtifactDigest,
    negative_controls: FederationNegativeControlManifest,
    checker_identity: ArtifactDigest,
    verifier_identity: ArtifactDigest,
}

impl FederationBreadthProofManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_commit: String,
        predecessor_p10: ArtifactDigest,
        world: ArtifactDigest,
        sat_route: FederationRouteProof,
        arithmetic_route: FederationRouteProof,
        bridge: ArtifactDigest,
        composition: ArtifactDigest,
        bridged_fact: ArtifactDigest,
        final_target: ArtifactDigest,
        negative_controls: FederationNegativeControlManifest,
        checker_identity: ArtifactDigest,
        verifier_identity: ArtifactDigest,
    ) -> Self {
        Self {
            source_commit,
            predecessor_p10,
            world,
            sat_route,
            arithmetic_route,
            bridge,
            composition,
            bridged_fact,
            final_target,
            negative_controls,
            checker_identity,
            verifier_identity,
        }
    }

    pub fn source_commit(&self) -> &str {
        &self.source_commit
    }

    pub fn predecessor_p10(&self) -> ArtifactDigest {
        self.predecessor_p10
    }

    pub fn world(&self) -> ArtifactDigest {
        self.world
    }

    pub fn sat_route(&self) -> &FederationRouteProof {
        &self.sat_route
    }

    pub fn arithmetic_route(&self) -> &FederationRouteProof {
        &self.arithmetic_route
    }

    pub fn bridge(&self) -> ArtifactDigest {
        self.bridge
    }

    pub fn composition(&self) -> ArtifactDigest {
        self.composition
    }

    pub fn bridged_fact(&self) -> ArtifactDigest {
        self.bridged_fact
    }

    pub fn final_target(&self) -> ArtifactDigest {
        self.final_target
    }

    pub fn negative_controls(&self) -> &FederationNegativeControlManifest {
        &self.negative_controls
    }

    pub fn checker_identity(&self) -> ArtifactDigest {
        self.checker_identity
    }

    pub fn verifier_identity(&self) -> ArtifactDigest {
        self.verifier_identity
    }
}

impl StructuralIdentity for FederationBreadthProofManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("FederationBreadthProofManifest");
        value.insert(
            "source_commit".into(),
            CanonicalValue::String(self.source_commit.clone()),
        );
        value.insert(
            "predecessor_p10".into(),
            CanonicalValue::Digest(self.predecessor_p10),
        );
        value.insert("world".into(), CanonicalValue::Digest(self.world));
        value.insert("sat_route".into(), self.sat_route.canonical_value());
        value.insert(
            "arithmetic_route".into(),
            self.arithmetic_route.canonical_value(),
        );
        value.insert("bridge".into(), CanonicalValue::Digest(self.bridge));
        value.insert(
            "composition".into(),
            CanonicalValue::Digest(self.composition),
        );
        value.insert(
            "bridged_fact".into(),
            CanonicalValue::Digest(self.bridged_fact),
        );
        value.insert(
            "final_target".into(),
            CanonicalValue::Digest(self.final_target),
        );
        value.insert(
            "negative_controls".into(),
            self.negative_controls.canonical_value(),
        );
        value.insert(
            "checker_identity".into(),
            CanonicalValue::Digest(self.checker_identity),
        );
        value.insert(
            "verifier_identity".into(),
            CanonicalValue::Digest(self.verifier_identity),
        );
        CanonicalValue::Object(value)
    }
}
