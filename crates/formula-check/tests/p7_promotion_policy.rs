use formula_check::promotion::{
    PromotionDecision, PromotionPolicyFailure, PromotionPolicyV1, authorize_promotion_v1,
};
use formula_core::{
    artifacts::StructuralIdentity,
    certification::{FrozenCandidate, PromotionManifest},
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    promotion::PromotionCandidate,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

struct Fixture {
    parent: UniverseGeneration,
    frozen: FrozenCandidate,
    manifest: PromotionManifest,
    candidate: PromotionCandidate,
    evidence: Vec<ArtifactDigest>,
    primitive: ArtifactDigest,
    dependency: ArtifactDigest,
    superseded: ArtifactDigest,
}

fn fixture() -> Fixture {
    let dependency = d("admitted-dependency");
    let superseded = d("older-primitive");
    let primitive = d("fl-c-primitive");
    let parent = UniverseGeneration::new(0, None, vec![dependency, superseded], vec![]);
    let parent_digest = parent.digest();
    let frozen = FrozenCandidate::new(
        "fl-c-semantic-primitive".into(),
        vec![primitive],
        d("world"),
        parent_digest,
        vec![dependency],
        vec![],
        d("authority-contract"),
        d("observer"),
    );
    let evidence = vec![d("checked-certificate")];
    let manifest = PromotionManifest::new(
        parent_digest,
        frozen.structural_digest(),
        evidence.clone(),
        vec![primitive],
        evidence.clone(),
    );
    let candidate = PromotionCandidate::new(
        frozen.structural_digest(),
        manifest.structural_digest(),
        parent_digest,
        parent_digest,
        vec![dependency],
        vec![superseded],
    );
    Fixture {
        parent,
        frozen,
        manifest,
        candidate,
        evidence,
        primitive,
        dependency,
        superseded,
    }
}

#[test]
fn exact_fresh_candidate_receives_opaque_authorization() {
    let f = fixture();
    let decision = authorize_promotion_v1(
        &f.manifest,
        &f.frozen,
        &f.candidate,
        &f.evidence,
        &f.parent,
        &[],
    )
    .unwrap();

    let PromotionDecision::Authorized(auth) = decision else {
        panic!("valid promotion was quarantined")
    };
    assert_eq!(auth.parent_generation(), f.parent.digest());
    assert_eq!(auth.frozen_candidate(), f.frozen.structural_digest());
    assert_eq!(auth.promotion_candidate(), f.candidate.structural_digest());
    assert_eq!(auth.proposed_admissions(), &[f.primitive]);
    assert_eq!(auth.authority_bindings(), f.evidence.as_slice());
    assert_eq!(auth.policy_digest(), PromotionPolicyV1::digest());
    assert_eq!(auth.supersedes(), &[f.superseded]);
}

#[test]
fn stale_or_mismatched_generation_fails_closed() {
    let f = fixture();
    let stale = PromotionCandidate::new(
        f.frozen.structural_digest(),
        f.manifest.structural_digest(),
        f.parent.digest(),
        d("stale-proof-generation"),
        vec![f.dependency],
        vec![f.superseded],
    );
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &f.frozen,
            &stale,
            &f.evidence,
            &f.parent,
            &[],
        ),
        Err(PromotionPolicyFailure::ProofGenerationMismatch)
    );

    let other_parent = UniverseGeneration::new(0, None, vec![f.dependency, f.superseded], vec![d("binding")]);
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &f.frozen,
            &f.candidate,
            &f.evidence,
            &other_parent,
            &[],
        ),
        Err(PromotionPolicyFailure::ParentGenerationMismatch)
    );
}

#[test]
fn dependency_cone_and_candidate_dependencies_must_be_admitted() {
    let f = fixture();
    let missing = d("not-admitted");
    let candidate = PromotionCandidate::new(
        f.frozen.structural_digest(),
        f.manifest.structural_digest(),
        f.parent.digest(),
        f.parent.digest(),
        vec![f.dependency, missing],
        vec![f.superseded],
    );
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &f.frozen,
            &candidate,
            &f.evidence,
            &f.parent,
            &[],
        ),
        Err(PromotionPolicyFailure::DependencyNotAdmitted(missing))
    );

    let narrow = PromotionCandidate::new(
        f.frozen.structural_digest(),
        f.manifest.structural_digest(),
        f.parent.digest(),
        f.parent.digest(),
        vec![],
        vec![f.superseded],
    );
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &f.frozen,
            &narrow,
            &f.evidence,
            &f.parent,
            &[],
        ),
        Err(PromotionPolicyFailure::CandidateDependencyOutsideCone(f.dependency))
    );
}

#[test]
fn evidence_binding_changed_candidate_and_invalid_supersession_fail() {
    let f = fixture();
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &f.frozen,
            &f.candidate,
            &[d("different-proof")],
            &f.parent,
            &[],
        ),
        Err(PromotionPolicyFailure::EvidenceBindingMismatch)
    );

    let changed_frozen = FrozenCandidate::new(
        "changed".into(),
        vec![f.primitive],
        d("world"),
        f.parent.digest(),
        vec![f.dependency],
        vec![],
        d("authority-contract"),
        d("observer"),
    );
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &changed_frozen,
            &f.candidate,
            &f.evidence,
            &f.parent,
            &[],
        ),
        Err(PromotionPolicyFailure::FrozenCandidateMismatch)
    );

    let missing = d("missing-superseded-artifact");
    let invalid_supersession = PromotionCandidate::new(
        f.frozen.structural_digest(),
        f.manifest.structural_digest(),
        f.parent.digest(),
        f.parent.digest(),
        vec![f.dependency],
        vec![missing],
    );
    assert_eq!(
        authorize_promotion_v1(
            &f.manifest,
            &f.frozen,
            &invalid_supersession,
            &f.evidence,
            &f.parent,
            &[],
        ),
        Err(PromotionPolicyFailure::SupersededArtifactNotAdmitted(missing))
    );
}

#[test]
fn conflicts_quarantine_instead_of_authorizing() {
    let f = fixture();
    let conflict = d("conflicting-authority");
    let decision = authorize_promotion_v1(
        &f.manifest,
        &f.frozen,
        &f.candidate,
        &f.evidence,
        &f.parent,
        &[conflict],
    )
    .unwrap();

    let PromotionDecision::Quarantined(record) = decision else {
        panic!("conflicted promotion was authorized")
    };
    assert_eq!(record.candidate(), f.candidate.structural_digest());
    assert_eq!(record.parent_generation(), f.parent.digest());
    assert_eq!(record.conflicts(), &[conflict]);
}
