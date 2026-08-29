use formula_check::{promotion::check_promotion_manifest, verdict::CheckVerdict};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
};

fn d(label: &[u8]) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label)
}

fn candidate(semantic: &[ArtifactDigest], judgements: &[ArtifactDigest]) -> FrozenCandidate {
    FrozenCandidate::new(
        "semantic-primitive".into(),
        semantic.to_vec(),
        d(b"world"),
        d(b"generation"),
        vec![d(b"dependency")],
        judgements.to_vec(),
        d(b"authority-contract"),
        d(b"observer"),
    )
}

#[test]
fn manifest_bound_to_exact_frozen_candidate_passes() {
    let semantic = d(b"semantic");
    let judgement = d(b"judgement");
    let candidate = candidate(&[semantic], &[judgement]);
    let evidence = [d(b"evidence-a"), d(b"evidence-b")];
    let parent = d(b"parent-generation");
    let manifest = PromotionManifest::new(
        parent,
        candidate.structural_digest(),
        evidence.to_vec(),
        vec![judgement, semantic],
        vec![d(b"authority-binding")],
    );

    assert_eq!(
        check_promotion_manifest(&manifest, &candidate, &evidence, parent),
        CheckVerdict::Pass
    );
}

#[test]
fn candidate_changed_after_certification_fails() {
    let certified = candidate(&[d(b"semantic-a")], &[d(b"judgement")]);
    let changed = candidate(&[d(b"semantic-b")], &[d(b"judgement")]);
    let evidence = [d(b"evidence")];
    let parent = d(b"parent-generation");
    let manifest = PromotionManifest::new(
        parent,
        certified.structural_digest(),
        evidence.to_vec(),
        vec![d(b"judgement")],
        vec![],
    );

    assert_ne!(
        check_promotion_manifest(&manifest, &changed, &evidence, parent),
        CheckVerdict::Pass
    );
}

#[test]
fn unreferenced_or_missing_evidence_fails() {
    let semantic = d(b"semantic");
    let candidate = candidate(&[semantic], &[]);
    let parent = d(b"parent-generation");
    let manifest = PromotionManifest::new(
        parent,
        candidate.structural_digest(),
        vec![d(b"evidence-a"), d(b"evidence-b")],
        vec![semantic],
        vec![],
    );

    assert_ne!(
        check_promotion_manifest(&manifest, &candidate, &[d(b"evidence-a")], parent),
        CheckVerdict::Pass
    );
    assert_ne!(
        check_promotion_manifest(
            &manifest,
            &candidate,
            &[d(b"evidence-a"), d(b"evidence-b"), d(b"evidence-c")],
            parent,
        ),
        CheckVerdict::Pass
    );
}

#[test]
fn wrong_parent_generation_fails_expected_binding() {
    let semantic = d(b"semantic");
    let candidate = candidate(&[semantic], &[]);
    let manifest = PromotionManifest::new(
        d(b"parent-a"),
        candidate.structural_digest(),
        vec![d(b"evidence")],
        vec![semantic],
        vec![],
    );

    assert_ne!(
        check_promotion_manifest(&manifest, &candidate, &[d(b"evidence")], d(b"parent-b"),),
        CheckVerdict::Pass
    );
}

#[test]
fn proposed_admission_not_covered_by_candidate_fails() {
    let candidate = candidate(&[d(b"semantic")], &[d(b"judgement")]);
    let parent = d(b"parent-generation");
    let manifest = PromotionManifest::new(
        parent,
        candidate.structural_digest(),
        vec![d(b"evidence")],
        vec![d(b"unrelated-admission")],
        vec![],
    );

    assert_ne!(
        check_promotion_manifest(&manifest, &candidate, &[d(b"evidence")], parent),
        CheckVerdict::Pass
    );
}
