use crate::{artifacts::StructuralIdentity, canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const P10_PROOF_SCHEMA_V1: &str = "formula-self-expansion-proof-v1";

fn object(kind: &str) -> BTreeMap<String, CanonicalValue> {
    BTreeMap::from([
        ("kind".into(), CanonicalValue::String(kind.into())),
        (
            "schema".into(),
            CanonicalValue::String(P10_PROOF_SCHEMA_V1.into()),
        ),
    ])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SelfExpansionNegativeControl {
    WrongBasePromotion,
    ForbiddenClassEffect,
    UnadmittedStructureWitness,
    UnboundStructureEvidence,
    UnscopedAutomaticNogood,
    RouteMissingPreservationEvidence,
    GrammarGenerationMismatch,
    UngatedAutomaticMetaprimitive,
    NonConservativeSilentTransport,
    UnauthorizedProofRepairOrTransport,
    RealizationUpgradeSemanticAdmission,
    RollbackHistoryRewrite,
}

impl SelfExpansionNegativeControl {
    pub const ALL: [Self; 12] = [
        Self::WrongBasePromotion,
        Self::ForbiddenClassEffect,
        Self::UnadmittedStructureWitness,
        Self::UnboundStructureEvidence,
        Self::UnscopedAutomaticNogood,
        Self::RouteMissingPreservationEvidence,
        Self::GrammarGenerationMismatch,
        Self::UngatedAutomaticMetaprimitive,
        Self::NonConservativeSilentTransport,
        Self::UnauthorizedProofRepairOrTransport,
        Self::RealizationUpgradeSemanticAdmission,
        Self::RollbackHistoryRewrite,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WrongBasePromotion => "NC10-01_WRONG_BASE_PROMOTION",
            Self::ForbiddenClassEffect => "NC10-02_FORBIDDEN_CLASS_EFFECT",
            Self::UnadmittedStructureWitness => "NC10-03_UNADMITTED_STRUCTURE_WITNESS",
            Self::UnboundStructureEvidence => "NC10-04_UNBOUND_STRUCTURE_EVIDENCE",
            Self::UnscopedAutomaticNogood => "NC10-05_UNSCOPED_AUTOMATIC_NOGOOD",
            Self::RouteMissingPreservationEvidence => "NC10-06_ROUTE_MISSING_PRESERVATION_EVIDENCE",
            Self::GrammarGenerationMismatch => "NC10-07_GRAMMAR_GENERATION_MISMATCH",
            Self::UngatedAutomaticMetaprimitive => "NC10-08_UNGATED_AUTOMATIC_METAPRIMITIVE",
            Self::NonConservativeSilentTransport => "NC10-09_NON_CONSERVATIVE_SILENT_TRANSPORT",
            Self::UnauthorizedProofRepairOrTransport => {
                "NC10-10_UNAUTHORIZED_PROOF_REPAIR_OR_TRANSPORT"
            }
            Self::RealizationUpgradeSemanticAdmission => {
                "NC10-11_REALIZATION_UPGRADE_SEMANTIC_ADMISSION"
            }
            Self::RollbackHistoryRewrite => "NC10-12_ROLLBACK_HISTORY_REWRITE",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfExpansionNegativeControlEvidence {
    control: SelfExpansionNegativeControl,
    evidence: ArtifactDigest,
}

impl SelfExpansionNegativeControlEvidence {
    pub fn new(control: SelfExpansionNegativeControl, evidence: ArtifactDigest) -> Self {
        Self { control, evidence }
    }

    pub fn control(&self) -> SelfExpansionNegativeControl {
        self.control
    }

    pub fn evidence(&self) -> ArtifactDigest {
        self.evidence
    }
}

impl StructuralIdentity for SelfExpansionNegativeControlEvidence {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("SelfExpansionNegativeControlEvidence");
        value.insert(
            "control".into(),
            CanonicalValue::String(self.control.as_str().into()),
        );
        value.insert("evidence".into(), CanonicalValue::Digest(self.evidence));
        CanonicalValue::Object(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelfExpansionNegativeControlManifestError {
    MissingDuplicateOrUnexpectedControl,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfExpansionNegativeControlManifest {
    controls: Vec<SelfExpansionNegativeControlEvidence>,
}

impl SelfExpansionNegativeControlManifest {
    pub fn new(
        mut controls: Vec<SelfExpansionNegativeControlEvidence>,
    ) -> Result<Self, SelfExpansionNegativeControlManifestError> {
        controls.sort_by_key(SelfExpansionNegativeControlEvidence::control);
        if controls.len() != SelfExpansionNegativeControl::ALL.len()
            || controls
                .iter()
                .map(SelfExpansionNegativeControlEvidence::control)
                .ne(SelfExpansionNegativeControl::ALL)
        {
            return Err(
                SelfExpansionNegativeControlManifestError::MissingDuplicateOrUnexpectedControl,
            );
        }
        Ok(Self { controls })
    }

    pub fn controls(&self) -> &[SelfExpansionNegativeControlEvidence] {
        &self.controls
    }

    pub fn is_complete(&self) -> bool {
        self.controls.len() == SelfExpansionNegativeControl::ALL.len()
            && self
                .controls
                .iter()
                .map(SelfExpansionNegativeControlEvidence::control)
                .eq(SelfExpansionNegativeControl::ALL)
    }
}

impl StructuralIdentity for SelfExpansionNegativeControlManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("SelfExpansionNegativeControlManifest");
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
pub struct SelfExpansionProofManifest {
    source_commit: String,
    predecessor_p9: ArtifactDigest,
    source_generation: ArtifactDigest,
    expanded_generation: ArtifactDigest,
    world: ArtifactDigest,
    registry_digest: ArtifactDigest,
    rational_package_before: ArtifactDigest,
    rational_package_after: ArtifactDigest,
    closure_before: ArtifactDigest,
    closure_after: ArtifactDigest,
    closure_delta: ArtifactDigest,
    unlocked_capability: ArtifactDigest,
    structure_witness: ArtifactDigest,
    base_promotion: ArtifactDigest,
    expansion_authorization: ArtifactDigest,
    lambda_before: ArtifactDigest,
    lambda_after: ArtifactDigest,
    nogood_proof: ArtifactDigest,
    route_proof: ArtifactDigest,
    shadow_metaprimitive: ArtifactDigest,
    semantic_change: ArtifactDigest,
    proof_evolution: ArtifactDigest,
    realization_upgrade: ArtifactDigest,
    realization_rollback: ArtifactDigest,
    negative_controls: SelfExpansionNegativeControlManifest,
    checker_identity: ArtifactDigest,
    verifier_identity: ArtifactDigest,
}

impl SelfExpansionProofManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_commit: String,
        predecessor_p9: ArtifactDigest,
        source_generation: ArtifactDigest,
        expanded_generation: ArtifactDigest,
        world: ArtifactDigest,
        registry_digest: ArtifactDigest,
        rational_package_before: ArtifactDigest,
        rational_package_after: ArtifactDigest,
        closure_before: ArtifactDigest,
        closure_after: ArtifactDigest,
        closure_delta: ArtifactDigest,
        unlocked_capability: ArtifactDigest,
        structure_witness: ArtifactDigest,
        base_promotion: ArtifactDigest,
        expansion_authorization: ArtifactDigest,
        lambda_before: ArtifactDigest,
        lambda_after: ArtifactDigest,
        nogood_proof: ArtifactDigest,
        route_proof: ArtifactDigest,
        shadow_metaprimitive: ArtifactDigest,
        semantic_change: ArtifactDigest,
        proof_evolution: ArtifactDigest,
        realization_upgrade: ArtifactDigest,
        realization_rollback: ArtifactDigest,
        negative_controls: SelfExpansionNegativeControlManifest,
        checker_identity: ArtifactDigest,
        verifier_identity: ArtifactDigest,
    ) -> Self {
        Self {
            source_commit,
            predecessor_p9,
            source_generation,
            expanded_generation,
            world,
            registry_digest,
            rational_package_before,
            rational_package_after,
            closure_before,
            closure_after,
            closure_delta,
            unlocked_capability,
            structure_witness,
            base_promotion,
            expansion_authorization,
            lambda_before,
            lambda_after,
            nogood_proof,
            route_proof,
            shadow_metaprimitive,
            semantic_change,
            proof_evolution,
            realization_upgrade,
            realization_rollback,
            negative_controls,
            checker_identity,
            verifier_identity,
        }
    }

    pub fn source_commit(&self) -> &str {
        &self.source_commit
    }
    pub fn predecessor_p9(&self) -> ArtifactDigest {
        self.predecessor_p9
    }
    pub fn source_generation(&self) -> ArtifactDigest {
        self.source_generation
    }
    pub fn expanded_generation(&self) -> ArtifactDigest {
        self.expanded_generation
    }
    pub fn world(&self) -> ArtifactDigest {
        self.world
    }
    pub fn registry_digest(&self) -> ArtifactDigest {
        self.registry_digest
    }
    pub fn rational_package_before(&self) -> ArtifactDigest {
        self.rational_package_before
    }
    pub fn rational_package_after(&self) -> ArtifactDigest {
        self.rational_package_after
    }
    pub fn closure_before(&self) -> ArtifactDigest {
        self.closure_before
    }
    pub fn closure_after(&self) -> ArtifactDigest {
        self.closure_after
    }
    pub fn closure_delta(&self) -> ArtifactDigest {
        self.closure_delta
    }
    pub fn unlocked_capability(&self) -> ArtifactDigest {
        self.unlocked_capability
    }
    pub fn structure_witness(&self) -> ArtifactDigest {
        self.structure_witness
    }
    pub fn base_promotion(&self) -> ArtifactDigest {
        self.base_promotion
    }
    pub fn expansion_authorization(&self) -> ArtifactDigest {
        self.expansion_authorization
    }
    pub fn lambda_before(&self) -> ArtifactDigest {
        self.lambda_before
    }
    pub fn lambda_after(&self) -> ArtifactDigest {
        self.lambda_after
    }
    pub fn nogood_proof(&self) -> ArtifactDigest {
        self.nogood_proof
    }
    pub fn route_proof(&self) -> ArtifactDigest {
        self.route_proof
    }
    pub fn shadow_metaprimitive(&self) -> ArtifactDigest {
        self.shadow_metaprimitive
    }
    pub fn semantic_change(&self) -> ArtifactDigest {
        self.semantic_change
    }
    pub fn proof_evolution(&self) -> ArtifactDigest {
        self.proof_evolution
    }
    pub fn realization_upgrade(&self) -> ArtifactDigest {
        self.realization_upgrade
    }
    pub fn realization_rollback(&self) -> ArtifactDigest {
        self.realization_rollback
    }
    pub fn negative_controls(&self) -> &SelfExpansionNegativeControlManifest {
        &self.negative_controls
    }
    pub fn checker_identity(&self) -> ArtifactDigest {
        self.checker_identity
    }
    pub fn verifier_identity(&self) -> ArtifactDigest {
        self.verifier_identity
    }
}

impl StructuralIdentity for SelfExpansionProofManifest {
    fn canonical_value(&self) -> CanonicalValue {
        let mut value = object("SelfExpansionProofManifest");
        value.insert(
            "source_commit".into(),
            CanonicalValue::String(self.source_commit.clone()),
        );
        value.insert(
            "predecessor_p9".into(),
            CanonicalValue::Digest(self.predecessor_p9),
        );
        value.insert(
            "source_generation".into(),
            CanonicalValue::Digest(self.source_generation),
        );
        value.insert(
            "expanded_generation".into(),
            CanonicalValue::Digest(self.expanded_generation),
        );
        value.insert("world".into(), CanonicalValue::Digest(self.world));
        value.insert(
            "registry_digest".into(),
            CanonicalValue::Digest(self.registry_digest),
        );
        value.insert(
            "rational_package_before".into(),
            CanonicalValue::Digest(self.rational_package_before),
        );
        value.insert(
            "rational_package_after".into(),
            CanonicalValue::Digest(self.rational_package_after),
        );
        value.insert(
            "closure_before".into(),
            CanonicalValue::Digest(self.closure_before),
        );
        value.insert(
            "closure_after".into(),
            CanonicalValue::Digest(self.closure_after),
        );
        value.insert(
            "closure_delta".into(),
            CanonicalValue::Digest(self.closure_delta),
        );
        value.insert(
            "unlocked_capability".into(),
            CanonicalValue::Digest(self.unlocked_capability),
        );
        value.insert(
            "structure_witness".into(),
            CanonicalValue::Digest(self.structure_witness),
        );
        value.insert(
            "base_promotion".into(),
            CanonicalValue::Digest(self.base_promotion),
        );
        value.insert(
            "expansion_authorization".into(),
            CanonicalValue::Digest(self.expansion_authorization),
        );
        value.insert(
            "lambda_before".into(),
            CanonicalValue::Digest(self.lambda_before),
        );
        value.insert(
            "lambda_after".into(),
            CanonicalValue::Digest(self.lambda_after),
        );
        value.insert(
            "nogood_proof".into(),
            CanonicalValue::Digest(self.nogood_proof),
        );
        value.insert(
            "route_proof".into(),
            CanonicalValue::Digest(self.route_proof),
        );
        value.insert(
            "shadow_metaprimitive".into(),
            CanonicalValue::Digest(self.shadow_metaprimitive),
        );
        value.insert(
            "semantic_change".into(),
            CanonicalValue::Digest(self.semantic_change),
        );
        value.insert(
            "proof_evolution".into(),
            CanonicalValue::Digest(self.proof_evolution),
        );
        value.insert(
            "realization_upgrade".into(),
            CanonicalValue::Digest(self.realization_upgrade),
        );
        value.insert(
            "realization_rollback".into(),
            CanonicalValue::Digest(self.realization_rollback),
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
