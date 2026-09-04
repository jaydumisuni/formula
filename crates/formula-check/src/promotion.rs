use crate::verdict::{CheckFailure, CheckVerdict};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::{PromotionCandidate, QuarantineRecord},
};

fn normalized(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

pub fn check_promotion_manifest(
    manifest: &PromotionManifest,
    candidate: &FrozenCandidate,
    checked_evidence: &[ArtifactDigest],
    expected_parent_generation: ArtifactDigest,
) -> CheckVerdict {
    if manifest.frozen_candidate() != candidate.structural_digest() {
        return CheckVerdict::Fail(CheckFailure::FrozenCandidateMismatch);
    }

    if manifest.parent_generation() != expected_parent_generation {
        return CheckVerdict::Fail(CheckFailure::PromotionParentMismatch);
    }

    let expected_evidence = normalized(checked_evidence.to_vec());
    if manifest.evidence_envelopes() != expected_evidence.as_slice() {
        return CheckVerdict::Fail(CheckFailure::PromotionEvidenceMismatch);
    }

    let mut covered = candidate.semantic_artifacts().to_vec();
    covered.extend_from_slice(candidate.proposed_judgements());
    let covered = normalized(covered);
    if manifest
        .proposed_admissions()
        .iter()
        .any(|admission| covered.binary_search(admission).is_err())
    {
        return CheckVerdict::Fail(CheckFailure::PromotionAdmissionMismatch);
    }

    CheckVerdict::Pass
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromotionPolicyV1;

impl PromotionPolicyV1 {
    pub fn digest() -> ArtifactDigest {
        ArtifactDigest::of_bytes(b"formula-promotion-policy-v1")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PromotionPolicyFailure {
    Manifest(CheckFailure),
    FrozenCandidateMismatch,
    PromotionManifestMismatch,
    ParentGenerationMismatch,
    CandidateGenerationMismatch,
    ProofGenerationMismatch,
    DependencyNotAdmitted(ArtifactDigest),
    CandidateDependencyOutsideCone(ArtifactDigest),
    EvidenceBindingMismatch,
    SupersededArtifactNotAdmitted(ArtifactDigest),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionAuthorization {
    parent_generation: ArtifactDigest,
    frozen_candidate: ArtifactDigest,
    promotion_candidate: ArtifactDigest,
    proposed_admissions: Vec<ArtifactDigest>,
    authority_bindings: Vec<ArtifactDigest>,
    policy_digest: ArtifactDigest,
    supersedes: Vec<ArtifactDigest>,
}

impl PromotionAuthorization {
    pub fn parent_generation(&self) -> ArtifactDigest {
        self.parent_generation
    }

    pub fn frozen_candidate(&self) -> ArtifactDigest {
        self.frozen_candidate
    }

    pub fn promotion_candidate(&self) -> ArtifactDigest {
        self.promotion_candidate
    }

    pub fn proposed_admissions(&self) -> &[ArtifactDigest] {
        &self.proposed_admissions
    }

    pub fn authority_bindings(&self) -> &[ArtifactDigest] {
        &self.authority_bindings
    }

    pub fn policy_digest(&self) -> ArtifactDigest {
        self.policy_digest
    }

    pub fn supersedes(&self) -> &[ArtifactDigest] {
        &self.supersedes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PromotionDecision {
    Authorized(PromotionAuthorization),
    Quarantined(QuarantineRecord),
}

pub fn authorize_promotion_v1(
    manifest: &PromotionManifest,
    frozen: &FrozenCandidate,
    promotion: &PromotionCandidate,
    checked_evidence: &[ArtifactDigest],
    parent: &UniverseGeneration,
    conflicts: &[ArtifactDigest],
) -> Result<PromotionDecision, PromotionPolicyFailure> {
    let parent_digest = parent.digest();
    let frozen_digest = frozen.structural_digest();
    let promotion_digest = promotion.structural_digest();

    if promotion.frozen_candidate() != frozen_digest {
        return Err(PromotionPolicyFailure::FrozenCandidateMismatch);
    }
    if promotion.promotion_manifest() != manifest.structural_digest() {
        return Err(PromotionPolicyFailure::PromotionManifestMismatch);
    }
    if promotion.parent_generation() != parent_digest || manifest.parent_generation() != parent_digest {
        return Err(PromotionPolicyFailure::ParentGenerationMismatch);
    }
    if frozen.universe_generation() != parent_digest {
        return Err(PromotionPolicyFailure::CandidateGenerationMismatch);
    }
    if promotion.proof_generation() != parent_digest {
        return Err(PromotionPolicyFailure::ProofGenerationMismatch);
    }

    for dependency in promotion.dependency_cone() {
        if !parent.admitted().contains(dependency) {
            return Err(PromotionPolicyFailure::DependencyNotAdmitted(*dependency));
        }
    }
    for dependency in frozen.dependencies() {
        if promotion.dependency_cone().binary_search(dependency).is_err() {
            return Err(PromotionPolicyFailure::CandidateDependencyOutsideCone(*dependency));
        }
    }

    let evidence = normalized(checked_evidence.to_vec());
    if manifest.proposed_authority_bindings() != evidence.as_slice() {
        return Err(PromotionPolicyFailure::EvidenceBindingMismatch);
    }

    for superseded in promotion.supersedes() {
        if !parent.admitted().contains(superseded) {
            return Err(PromotionPolicyFailure::SupersededArtifactNotAdmitted(*superseded));
        }
    }

    if let CheckVerdict::Fail(failure) =
        check_promotion_manifest(manifest, frozen, &evidence, parent_digest)
    {
        return Err(PromotionPolicyFailure::Manifest(failure));
    }

    let conflicts = normalized(conflicts.to_vec());
    if !conflicts.is_empty() {
        return Ok(PromotionDecision::Quarantined(QuarantineRecord::new(
            promotion_digest,
            parent_digest,
            ArtifactDigest::of_bytes(b"formula-promotion-conflict-quarantine-v1"),
            conflicts,
        )));
    }

    Ok(PromotionDecision::Authorized(PromotionAuthorization {
        parent_generation: parent_digest,
        frozen_candidate: frozen_digest,
        promotion_candidate: promotion_digest,
        proposed_admissions: manifest.proposed_admissions().to_vec(),
        authority_bindings: evidence,
        policy_digest: PromotionPolicyV1::digest(),
        supersedes: promotion.supersedes().to_vec(),
    }))
}
