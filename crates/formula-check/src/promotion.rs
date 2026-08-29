use crate::verdict::{CheckFailure, CheckVerdict};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
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
