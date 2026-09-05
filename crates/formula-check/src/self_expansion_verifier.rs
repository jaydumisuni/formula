use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    self_expansion_proof::{SelfExpansionNegativeControlManifest, SelfExpansionProofManifest},
};

pub const P10_CANONICAL_MARKERS: [&str; 13] = [
    "PASS P10_PROMOTION_CLASS_REGISTRY",
    "PASS P10_STRUCTURE_WITNESS_PROMOTION",
    "PASS P10_NON_PRIMITIVE_CAPABILITY_UNLOCK",
    "PASS P10_NOGOOD_SCOPE_ENFORCED",
    "PASS P10_ROUTE_PROMOTION_GATED",
    "PASS P10_GRAMMAR_GENERATION_BOUND",
    "PASS P10_METAPRIMITIVE_SHADOW_GATE",
    "PASS P10_SEMANTIC_CHANGE_REVALIDATION",
    "PASS P10_PROOF_TRANSPORT_REPAIR_GATED",
    "PASS P10_REALIZATION_ONLY_UPGRADE",
    "PASS P10_ROLLBACK_HISTORY_PRESERVED",
    "PASS P10_NEGATIVE_CONTROLS",
    "PASS SELF_EXPANSION_HARDENED",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelfExpansionReplayClaims {
    promotion_class_registry: bool,
    structure_witness_promotion: bool,
    non_primitive_capability_unlock: bool,
    nogood_scope_enforced: bool,
    route_promotion_gated: bool,
    grammar_generation_bound: bool,
    metaprimitive_shadow_gate: bool,
    semantic_change_revalidation: bool,
    proof_transport_repair_gated: bool,
    realization_only_upgrade: bool,
    rollback_history_preserved: bool,
}

impl SelfExpansionReplayClaims {
    pub fn all_proved() -> Self {
        Self {
            promotion_class_registry: true,
            structure_witness_promotion: true,
            non_primitive_capability_unlock: true,
            nogood_scope_enforced: true,
            route_promotion_gated: true,
            grammar_generation_bound: true,
            metaprimitive_shadow_gate: true,
            semantic_change_revalidation: true,
            proof_transport_repair_gated: true,
            realization_only_upgrade: true,
            rollback_history_preserved: true,
        }
    }

    pub fn none_proved() -> Self {
        Self {
            promotion_class_registry: false,
            structure_witness_promotion: false,
            non_primitive_capability_unlock: false,
            nogood_scope_enforced: false,
            route_promotion_gated: false,
            grammar_generation_bound: false,
            metaprimitive_shadow_gate: false,
            semantic_change_revalidation: false,
            proof_transport_repair_gated: false,
            realization_only_upgrade: false,
            rollback_history_preserved: false,
        }
    }

    fn all(self) -> bool {
        self.promotion_class_registry
            && self.structure_witness_promotion
            && self.non_primitive_capability_unlock
            && self.nogood_scope_enforced
            && self.route_promotion_gated
            && self.grammar_generation_bound
            && self.metaprimitive_shadow_gate
            && self.semantic_change_revalidation
            && self.proof_transport_repair_gated
            && self.realization_only_upgrade
            && self.rollback_history_preserved
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfExpansionReplayEvidence {
    manifest_digest: ArtifactDigest,
    source_generation: ArtifactDigest,
    expanded_generation: ArtifactDigest,
    expanded_parent: ArtifactDigest,
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
    claims: SelfExpansionReplayClaims,
}

impl SelfExpansionReplayEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest_digest: ArtifactDigest,
        source_generation: ArtifactDigest,
        expanded_generation: ArtifactDigest,
        expanded_parent: ArtifactDigest,
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
        claims: SelfExpansionReplayClaims,
    ) -> Self {
        Self {
            manifest_digest,
            source_generation,
            expanded_generation,
            expanded_parent,
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
            claims,
        }
    }

    #[doc(hidden)]
    pub fn set_rational_package_after_for_test(&mut self, value: ArtifactDigest) {
        self.rational_package_after = value;
    }

    #[doc(hidden)]
    pub fn set_claims_for_test(&mut self, claims: SelfExpansionReplayClaims) {
        self.claims = claims;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelfExpansionVerificationFailure {
    ReplayBindingMismatch,
    UniverseTransitionMismatch,
    PackageMutationDetected,
    CapabilityUnlockNotProved,
    GrammarGenerationNotChanged,
    NegativeControlsIncomplete,
    HardeningClaimNotProved,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfExpansionVerificationResult;

impl SelfExpansionVerificationResult {
    pub fn markers(&self) -> &[&'static str; 13] {
        &P10_CANONICAL_MARKERS
    }
}

pub fn verify_self_expansion_manifest(
    manifest: &SelfExpansionProofManifest,
    evidence: &SelfExpansionReplayEvidence,
) -> Result<SelfExpansionVerificationResult, SelfExpansionVerificationFailure> {
    if manifest.structural_digest() != evidence.manifest_digest
        || manifest.source_generation() != evidence.source_generation
        || manifest.expanded_generation() != evidence.expanded_generation
        || manifest.registry_digest() != evidence.registry_digest
        || manifest.rational_package_before() != evidence.rational_package_before
        || manifest.rational_package_after() != evidence.rational_package_after
        || manifest.closure_before() != evidence.closure_before
        || manifest.closure_after() != evidence.closure_after
        || manifest.closure_delta() != evidence.closure_delta
        || manifest.unlocked_capability() != evidence.unlocked_capability
        || manifest.structure_witness() != evidence.structure_witness
        || manifest.base_promotion() != evidence.base_promotion
        || manifest.expansion_authorization() != evidence.expansion_authorization
        || manifest.lambda_before() != evidence.lambda_before
        || manifest.lambda_after() != evidence.lambda_after
        || manifest.nogood_proof() != evidence.nogood_proof
        || manifest.route_proof() != evidence.route_proof
        || manifest.shadow_metaprimitive() != evidence.shadow_metaprimitive
        || manifest.semantic_change() != evidence.semantic_change
        || manifest.proof_evolution() != evidence.proof_evolution
        || manifest.realization_upgrade() != evidence.realization_upgrade
        || manifest.realization_rollback() != evidence.realization_rollback
        || manifest.negative_controls() != &evidence.negative_controls
        || manifest.checker_identity() != evidence.checker_identity
        || manifest.verifier_identity() != evidence.verifier_identity
    {
        return Err(SelfExpansionVerificationFailure::ReplayBindingMismatch);
    }

    if evidence.expanded_parent != evidence.source_generation
        || evidence.source_generation == evidence.expanded_generation
    {
        return Err(SelfExpansionVerificationFailure::UniverseTransitionMismatch);
    }
    if evidence.rational_package_before != evidence.rational_package_after {
        return Err(SelfExpansionVerificationFailure::PackageMutationDetected);
    }
    if evidence.closure_before == evidence.closure_after
        || evidence.closure_delta == evidence.closure_before
        || evidence.closure_delta == evidence.closure_after
    {
        return Err(SelfExpansionVerificationFailure::CapabilityUnlockNotProved);
    }
    if evidence.lambda_before == evidence.lambda_after {
        return Err(SelfExpansionVerificationFailure::GrammarGenerationNotChanged);
    }
    if !evidence.negative_controls.is_complete() {
        return Err(SelfExpansionVerificationFailure::NegativeControlsIncomplete);
    }
    if !evidence.claims.all() {
        return Err(SelfExpansionVerificationFailure::HardeningClaimNotProved);
    }

    Ok(SelfExpansionVerificationResult)
}
