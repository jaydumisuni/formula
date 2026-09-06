use crate::federation::{
    FederationError, FederationMode, FederationRequest, validate_federation_adapter,
};
use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    federation::{BridgeContract, CertifiedFederationFact},
    theory::{
        CompositionClaim, CompositionClass, FactPolarity, FederationAdapterManifest, SharedFact,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CooperationError {
    CandidateOnlyCannotCertify,
    Adapter(FederationError),
    AdapterDigestMismatch,
    SemanticInputMismatch,
    SemanticOutputMismatch,
    EvidenceMismatch,
    MissingBridgeContract,
    BridgeSourcePackageMismatch,
    BridgeSourceSubjectMismatch,
    BridgeSourcePolarityMismatch,
    CompositionPackageMismatch,
    UnsafeCompositionClass,
    PolarityUpgrade,
}

#[allow(clippy::too_many_arguments)]
pub fn certify_federation_fact(
    adapter: &FederationAdapterManifest,
    mode: FederationMode,
    request: &FederationRequest,
    adapter_digest: ArtifactDigest,
    semantic_input: ArtifactDigest,
    checked_evidence: ArtifactDigest,
    fact: SharedFact,
) -> Result<CertifiedFederationFact, CooperationError> {
    if mode == FederationMode::CandidateOnly {
        return Err(CooperationError::CandidateOnlyCannotCertify);
    }

    validate_federation_adapter(adapter, mode, request).map_err(CooperationError::Adapter)?;

    if adapter.structural_digest() != adapter_digest {
        return Err(CooperationError::AdapterDigestMismatch);
    }
    if adapter
        .semantic_inputs()
        .binary_search(&semantic_input)
        .is_err()
    {
        return Err(CooperationError::SemanticInputMismatch);
    }
    if adapter
        .semantic_outputs()
        .binary_search(&fact.subject())
        .is_err()
    {
        return Err(CooperationError::SemanticOutputMismatch);
    }
    if fact.evidence() != checked_evidence {
        return Err(CooperationError::EvidenceMismatch);
    }

    let checker_route = request
        .checker_route()
        .ok_or(CooperationError::Adapter(FederationError::CheckerRouteMismatch))?;
    let translation = request
        .translation()
        .ok_or(CooperationError::Adapter(FederationError::TranslationMismatch))?;

    Ok(CertifiedFederationFact::new(
        fact,
        adapter.package(),
        adapter_digest,
        translation,
        checker_route,
        semantic_input,
        checked_evidence,
    ))
}

pub fn apply_bridge(
    source: &CertifiedFederationFact,
    bridge: Option<&BridgeContract>,
    composition: &CompositionClaim,
) -> Result<SharedFact, CooperationError> {
    let bridge = bridge.ok_or(CooperationError::MissingBridgeContract)?;

    if source.package() != bridge.source_package() {
        return Err(CooperationError::BridgeSourcePackageMismatch);
    }
    if source.fact().subject() != bridge.source_subject() {
        return Err(CooperationError::BridgeSourceSubjectMismatch);
    }
    if source.fact().polarity() != bridge.source_polarity() {
        return Err(CooperationError::BridgeSourcePolarityMismatch);
    }
    if !composition_matches_bridge(composition, bridge) {
        return Err(CooperationError::CompositionPackageMismatch);
    }
    if !safe_composition_class(composition.class()) {
        return Err(CooperationError::UnsafeCompositionClass);
    }
    if !polarity_flow_is_non_strengthening(bridge.source_polarity(), bridge.target_polarity()) {
        return Err(CooperationError::PolarityUpgrade);
    }

    let evidence = ArtifactDigest::of_bytes(
        format!(
            "{}\n{}\n{}",
            source.structural_digest().as_str(),
            bridge.structural_digest().as_str(),
            composition.structural_digest().as_str()
        )
        .as_bytes(),
    );

    Ok(SharedFact::new(
        source.fact().world(),
        bridge.target_subject(),
        source.fact().payload().clone(),
        bridge.target_polarity(),
        evidence,
    ))
}

fn composition_matches_bridge(composition: &CompositionClaim, bridge: &BridgeContract) -> bool {
    let source = bridge.source_package();
    let target = bridge.target_package();
    (composition.left_package() == source && composition.right_package() == target)
        || (composition.left_package() == target && composition.right_package() == source)
}

fn safe_composition_class(class: CompositionClass) -> bool {
    matches!(
        class,
        CompositionClass::CertifiedCombination
            | CompositionClass::ConservativeExtension
            | CompositionClass::SoundCooperation
    )
}

fn polarity_flow_is_non_strengthening(source: FactPolarity, target: FactPolarity) -> bool {
    source == FactPolarity::Exact || source == target
}
