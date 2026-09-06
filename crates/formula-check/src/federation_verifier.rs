use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    federation_proof::{
        FederationBreadthProofManifest, FederationNegativeControlManifest, FederationRouteKind,
        FederationRouteProof,
    },
};

pub const P11_CANONICAL_MARKERS: [&str; 9] = [
    "PASS P11_SAT_LRAT_CHECKED",
    "PASS P11_EXACT_ARITHMETIC_CHECKED",
    "PASS P11_FEDERATION_PROVENANCE_BOUND",
    "PASS P11_SHARED_FACT_POLARITY_PRESERVED",
    "PASS P11_BRIDGE_CONTRACT_ENFORCED",
    "PASS P11_HETEROGENEOUS_COOPERATION",
    "PASS P11_PRODUCER_IDENTITY_UNTRUSTED",
    "PASS P11_NEGATIVE_CONTROLS",
    "PASS FEDERATION_BREADTH_PROVED",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FederationReplayClaims {
    sat_lrat_checked: bool,
    exact_arithmetic_checked: bool,
    federation_provenance_bound: bool,
    shared_fact_polarity_preserved: bool,
    bridge_contract_enforced: bool,
    heterogeneous_cooperation: bool,
    producer_identity_untrusted: bool,
}

impl FederationReplayClaims {
    pub fn all_proved() -> Self {
        Self {
            sat_lrat_checked: true,
            exact_arithmetic_checked: true,
            federation_provenance_bound: true,
            shared_fact_polarity_preserved: true,
            bridge_contract_enforced: true,
            heterogeneous_cooperation: true,
            producer_identity_untrusted: true,
        }
    }

    pub fn none_proved() -> Self {
        Self {
            sat_lrat_checked: false,
            exact_arithmetic_checked: false,
            federation_provenance_bound: false,
            shared_fact_polarity_preserved: false,
            bridge_contract_enforced: false,
            heterogeneous_cooperation: false,
            producer_identity_untrusted: false,
        }
    }

    fn all(self) -> bool {
        self.sat_lrat_checked
            && self.exact_arithmetic_checked
            && self.federation_provenance_bound
            && self.shared_fact_polarity_preserved
            && self.bridge_contract_enforced
            && self.heterogeneous_cooperation
            && self.producer_identity_untrusted
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FederationReplayEvidence {
    manifest_digest: ArtifactDigest,
    predecessor_p10: ArtifactDigest,
    world: ArtifactDigest,
    sat_route: FederationRouteProof,
    arithmetic_route: FederationRouteProof,
    bridge: ArtifactDigest,
    composition: ArtifactDigest,
    bridged_fact: ArtifactDigest,
    final_target: ArtifactDigest,
    arithmetic_contribution: ArtifactDigest,
    bridge_source_package: ArtifactDigest,
    bridge_target_package: ArtifactDigest,
    negative_controls: FederationNegativeControlManifest,
    checker_identity: ArtifactDigest,
    verifier_identity: ArtifactDigest,
    claims: FederationReplayClaims,
}

impl FederationReplayEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest_digest: ArtifactDigest,
        predecessor_p10: ArtifactDigest,
        world: ArtifactDigest,
        sat_route: FederationRouteProof,
        arithmetic_route: FederationRouteProof,
        bridge: ArtifactDigest,
        composition: ArtifactDigest,
        bridged_fact: ArtifactDigest,
        final_target: ArtifactDigest,
        arithmetic_contribution: ArtifactDigest,
        bridge_source_package: ArtifactDigest,
        bridge_target_package: ArtifactDigest,
        negative_controls: FederationNegativeControlManifest,
        checker_identity: ArtifactDigest,
        verifier_identity: ArtifactDigest,
        claims: FederationReplayClaims,
    ) -> Self {
        Self {
            manifest_digest,
            predecessor_p10,
            world,
            sat_route,
            arithmetic_route,
            bridge,
            composition,
            bridged_fact,
            final_target,
            arithmetic_contribution,
            bridge_source_package,
            bridge_target_package,
            negative_controls,
            checker_identity,
            verifier_identity,
            claims,
        }
    }

    #[doc(hidden)]
    pub fn set_arithmetic_evidence_for_test(&mut self, value: ArtifactDigest) {
        self.arithmetic_route = FederationRouteProof::new(
            self.arithmetic_route.kind(),
            self.arithmetic_route.package(),
            self.arithmetic_route.adapter(),
            self.arithmetic_route.semantic_input(),
            value,
            self.arithmetic_route.certified_fact(),
        );
    }

    #[doc(hidden)]
    pub fn set_bridge_target_package_for_test(&mut self, value: ArtifactDigest) {
        self.bridge_target_package = value;
    }

    #[doc(hidden)]
    pub fn set_claims_for_test(&mut self, claims: FederationReplayClaims) {
        self.claims = claims;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FederationVerificationFailure {
    ReplayBindingMismatch,
    RouteIdentityMismatch,
    HeterogeneousCooperationNotProved,
    NegativeControlsIncomplete,
    FederationClaimNotProved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FederationVerificationResult;

impl FederationVerificationResult {
    pub fn markers(&self) -> &[&'static str; 9] {
        &P11_CANONICAL_MARKERS
    }
}

pub fn verify_federation_breadth_manifest(
    manifest: &FederationBreadthProofManifest,
    evidence: &FederationReplayEvidence,
) -> Result<FederationVerificationResult, FederationVerificationFailure> {
    if manifest.structural_digest() != evidence.manifest_digest
        || manifest.predecessor_p10() != evidence.predecessor_p10
        || manifest.world() != evidence.world
        || manifest.sat_route() != &evidence.sat_route
        || manifest.arithmetic_route() != &evidence.arithmetic_route
        || manifest.bridge() != evidence.bridge
        || manifest.composition() != evidence.composition
        || manifest.bridged_fact() != evidence.bridged_fact
        || manifest.final_target() != evidence.final_target
        || manifest.negative_controls() != &evidence.negative_controls
        || manifest.checker_identity() != evidence.checker_identity
        || manifest.verifier_identity() != evidence.verifier_identity
    {
        return Err(FederationVerificationFailure::ReplayBindingMismatch);
    }

    if evidence.sat_route.kind() != FederationRouteKind::SatLrat
        || evidence.arithmetic_route.kind() != FederationRouteKind::ExactArithmetic
        || evidence.arithmetic_contribution != evidence.arithmetic_route.certified_fact()
    {
        return Err(FederationVerificationFailure::RouteIdentityMismatch);
    }

    if evidence.bridge_source_package != evidence.sat_route.package()
        || evidence.bridge_target_package != evidence.arithmetic_route.package()
        || evidence.bridge_source_package == evidence.bridge_target_package
        || evidence.sat_route.package() == evidence.arithmetic_route.package()
        || evidence.sat_route.certified_fact() == evidence.arithmetic_route.certified_fact()
        || evidence.final_target == evidence.bridged_fact
        || evidence.final_target == evidence.arithmetic_contribution
    {
        return Err(FederationVerificationFailure::HeterogeneousCooperationNotProved);
    }

    if !evidence.negative_controls.is_complete() {
        return Err(FederationVerificationFailure::NegativeControlsIncomplete);
    }

    if !evidence.claims.all() {
        return Err(FederationVerificationFailure::FederationClaimNotProved);
    }

    Ok(FederationVerificationResult)
}
