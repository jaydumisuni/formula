use crate::{
    identity::CheckerDescriptor,
    verdict::{AuthorityMatch, CheckFailure},
};
use formula_core::{
    artifacts::{AuthorityContract, Observer, StructuralIdentity},
    certification::{CertificateEnvelope, FrozenCandidate},
    digest::ArtifactDigest,
};

pub struct CheckRequest<'a> {
    envelope: &'a CertificateEnvelope,
    frozen_candidate: &'a FrozenCandidate,
    expected_target: ArtifactDigest,
    expected_generation: ArtifactDigest,
    expected_world: ArtifactDigest,
    expected_dependencies: &'a [ArtifactDigest],
    authority_contract: &'a AuthorityContract,
    observer: &'a Observer,
    certificate_body: &'a [u8],
    expected_checker_trust_root: ArtifactDigest,
}

impl<'a> CheckRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        envelope: &'a CertificateEnvelope,
        frozen_candidate: &'a FrozenCandidate,
        expected_target: ArtifactDigest,
        expected_generation: ArtifactDigest,
        expected_world: ArtifactDigest,
        expected_dependencies: &'a [ArtifactDigest],
        authority_contract: &'a AuthorityContract,
        observer: &'a Observer,
        certificate_body: &'a [u8],
        expected_checker_trust_root: ArtifactDigest,
    ) -> Self {
        Self {
            envelope,
            frozen_candidate,
            expected_target,
            expected_generation,
            expected_world,
            expected_dependencies,
            authority_contract,
            observer,
            certificate_body,
            expected_checker_trust_root,
        }
    }
}

pub fn validate_envelope(request: &CheckRequest<'_>) -> Result<AuthorityMatch, CheckFailure> {
    let envelope = request.envelope;

    if ArtifactDigest::of_bytes(request.certificate_body) != envelope.certificate_body_digest() {
        return Err(CheckFailure::CertificateBodyDigestMismatch);
    }

    if request.frozen_candidate.structural_digest() != envelope.frozen_candidate() {
        return Err(CheckFailure::FrozenCandidateMismatch);
    }

    if envelope.target_judgement() != request.expected_target {
        return Err(CheckFailure::TargetMismatch);
    }
    if envelope.universe_generation() != request.expected_generation {
        return Err(CheckFailure::GenerationMismatch);
    }
    if envelope.world() != request.expected_world {
        return Err(CheckFailure::WorldMismatch);
    }

    if request.frozen_candidate.universe_generation() != request.expected_generation {
        return Err(CheckFailure::GenerationMismatch);
    }
    if request.frozen_candidate.world() != request.expected_world {
        return Err(CheckFailure::WorldMismatch);
    }

    let mut expected_dependencies = request.expected_dependencies.to_vec();
    expected_dependencies.sort_unstable();
    expected_dependencies.dedup();
    if envelope.dependencies() != expected_dependencies.as_slice()
        || request.frozen_candidate.dependencies() != expected_dependencies.as_slice()
    {
        return Err(CheckFailure::DependencyMismatch);
    }

    let authority_digest = request.authority_contract.structural_digest();
    if envelope.authority_contract() != authority_digest
        || request.frozen_candidate.authority_contract() != authority_digest
    {
        return Err(CheckFailure::AuthorityContractMismatch);
    }

    let observer_digest = request.observer.structural_digest();
    if envelope.observer() != observer_digest
        || request.frozen_candidate.observer() != observer_digest
    {
        return Err(CheckFailure::ObserverMismatch);
    }

    let descriptor = CheckerDescriptor::current();
    if envelope.checker() != descriptor.identity() {
        return Err(CheckFailure::CheckerIdentityMismatch);
    }
    if envelope.checker_trust_root() != request.expected_checker_trust_root {
        return Err(CheckFailure::CheckerTrustRootMismatch);
    }
    if !descriptor.supports_family_name(envelope.certificate_family()) {
        return Err(CheckFailure::UnsupportedCertificateFamily);
    }
    if !descriptor.supports_family(
        envelope.certificate_family(),
        envelope.certificate_family_version(),
    ) {
        return Err(CheckFailure::UnsupportedCertificateFamilyVersion);
    }

    if !request
        .authority_contract
        .allowed_evidence_families()
        .iter()
        .any(|family| family == envelope.certificate_family())
    {
        return Err(CheckFailure::AuthorityInsufficient);
    }

    let exactness = request.authority_contract.exactness_requirement();
    let authoritative_exact_mode = matches!(
        envelope.verification_mode(),
        "FOUNDATIONAL_PROOF" | "INDEPENDENT_CERTIFICATE" | "EXACT_RECOMPUTATION" | "EXHAUSTIVE"
    );
    if request.authority_contract.requested_authority_class() != "deterministic-proof"
        || exactness != "exact"
        || envelope.outcome_class() != "PROVED"
        || !authoritative_exact_mode
    {
        return Err(CheckFailure::AuthorityInsufficient);
    }

    Ok(AuthorityMatch::new(
        envelope.certificate_family().to_owned(),
        exactness.to_owned(),
        envelope.verification_mode().to_owned(),
    ))
}
