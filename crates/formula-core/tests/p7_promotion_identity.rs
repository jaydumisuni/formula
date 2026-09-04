use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    promotion::{PromotionCandidate, PromotionRecord, PromotionState, QuarantineRecord},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn sorted(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

#[test]
fn promotion_candidate_normalizes_set_like_inputs() {
    let a = PromotionCandidate::new(
        d("frozen"),
        d("manifest"),
        d("u0"),
        d("u0"),
        vec![d("dep-b"), d("dep-a"), d("dep-a")],
        vec![d("old-b"), d("old-a"), d("old-a")],
    );
    let b = PromotionCandidate::new(
        d("frozen"),
        d("manifest"),
        d("u0"),
        d("u0"),
        vec![d("dep-a"), d("dep-b")],
        vec![d("old-a"), d("old-b")],
    );

    assert_eq!(a.structural_digest(), b.structural_digest());
    assert_eq!(a.dependency_cone(), sorted(vec![d("dep-a"), d("dep-b")]).as_slice());
    assert_eq!(a.supersedes(), sorted(vec![d("old-a"), d("old-b")]).as_slice());
}

#[test]
fn promotion_candidate_binds_parent_proof_and_manifest_identity() {
    let base = PromotionCandidate::new(
        d("frozen"),
        d("manifest"),
        d("u0"),
        d("u0"),
        vec![d("dep")],
        vec![],
    );
    let changed_parent = PromotionCandidate::new(
        d("frozen"),
        d("manifest"),
        d("u1"),
        d("u0"),
        vec![d("dep")],
        vec![],
    );
    let changed_proof = PromotionCandidate::new(
        d("frozen"),
        d("manifest"),
        d("u0"),
        d("other-proof-generation"),
        vec![d("dep")],
        vec![],
    );
    let changed_manifest = PromotionCandidate::new(
        d("frozen"),
        d("other-manifest"),
        d("u0"),
        d("u0"),
        vec![d("dep")],
        vec![],
    );

    assert_ne!(base.structural_digest(), changed_parent.structural_digest());
    assert_ne!(base.structural_digest(), changed_proof.structural_digest());
    assert_ne!(base.structural_digest(), changed_manifest.structural_digest());
}

#[test]
fn lifecycle_states_are_structurally_distinct() {
    let candidate = d("promotion-candidate");
    let generation = d("generation");
    let policy = d("policy");
    let evidence = vec![d("proof")];
    let artifacts = vec![d("primitive")];

    let certified = PromotionRecord::new(
        candidate,
        PromotionState::Certified,
        generation,
        policy,
        evidence.clone(),
        artifacts.clone(),
    );
    let admitted = PromotionRecord::new(
        candidate,
        PromotionState::Admitted,
        generation,
        policy,
        evidence.clone(),
        artifacts.clone(),
    );
    let activated = PromotionRecord::new(
        candidate,
        PromotionState::Activated,
        generation,
        policy,
        evidence.clone(),
        artifacts.clone(),
    );
    let quarantined = PromotionRecord::new(
        candidate,
        PromotionState::Quarantined,
        generation,
        policy,
        evidence,
        artifacts,
    );

    let digests = [
        certified.structural_digest(),
        admitted.structural_digest(),
        activated.structural_digest(),
        quarantined.structural_digest(),
    ];
    for left in 0..digests.len() {
        for right in (left + 1)..digests.len() {
            assert_ne!(digests[left], digests[right]);
        }
    }
}

#[test]
fn quarantine_record_normalizes_conflicts_and_binds_reason() {
    let a = QuarantineRecord::new(
        d("candidate"),
        d("u0"),
        d("reason"),
        vec![d("conflict-b"), d("conflict-a"), d("conflict-a")],
    );
    let b = QuarantineRecord::new(
        d("candidate"),
        d("u0"),
        d("reason"),
        vec![d("conflict-a"), d("conflict-b")],
    );
    let changed_reason = QuarantineRecord::new(
        d("candidate"),
        d("u0"),
        d("other-reason"),
        vec![d("conflict-a"), d("conflict-b")],
    );

    assert_eq!(a.structural_digest(), b.structural_digest());
    assert_eq!(
        a.conflicts(),
        sorted(vec![d("conflict-a"), d("conflict-b")]).as_slice()
    );
    assert_ne!(a.structural_digest(), changed_reason.structural_digest());
}
