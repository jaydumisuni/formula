use formula_core::digest::ArtifactDigest;
use formula_engine::candidate_space::FrozenCandidate;
use formula_first_light::manifest::{BlindnessManifest, FirstLightTarget, FrozenSubmission};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn manifest(target: FirstLightTarget) -> BlindnessManifest {
    BlindnessManifest::new(target, d(1), d(2), d(3), d(4), d(5), d(6), d(7))
}

#[test]
fn blindness_manifest_identity_binds_every_semantic_input() {
    let baseline = manifest(FirstLightTarget::FlA);
    let same = manifest(FirstLightTarget::FlA);
    assert_eq!(baseline.digest(), same.digest());

    let variants = [
        BlindnessManifest::new(
            FirstLightTarget::FlB,
            d(1),
            d(2),
            d(3),
            d(4),
            d(5),
            d(6),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(8),
            d(2),
            d(3),
            d(4),
            d(5),
            d(6),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(1),
            d(8),
            d(3),
            d(4),
            d(5),
            d(6),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(1),
            d(2),
            d(8),
            d(4),
            d(5),
            d(6),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(1),
            d(2),
            d(3),
            d(8),
            d(5),
            d(6),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(1),
            d(2),
            d(3),
            d(4),
            d(8),
            d(6),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(1),
            d(2),
            d(3),
            d(4),
            d(5),
            d(8),
            d(7),
        ),
        BlindnessManifest::new(
            FirstLightTarget::FlA,
            d(1),
            d(2),
            d(3),
            d(4),
            d(5),
            d(6),
            d(8),
        ),
    ];

    for variant in variants {
        assert_ne!(baseline.digest(), variant.digest());
    }

    assert_eq!(baseline.target(), FirstLightTarget::FlA);
    assert_eq!(baseline.sealed_target_digest(), d(1));
    assert_eq!(baseline.universe_generation(), d(2));
    assert_eq!(baseline.world(), d(3));
    assert_eq!(baseline.query_digest(), d(4));
    assert_eq!(baseline.grammar_or_routes_digest(), d(5));
    assert_eq!(baseline.package_set_digest(), d(6));
    assert_eq!(baseline.oracle_contract_digest(), d(7));
}

#[test]
fn frozen_submission_binds_target_and_frozen_candidate() {
    let candidate = FrozenCandidate::new(d(20), d(21), 9);
    let submission = FrozenSubmission::new(FirstLightTarget::FlC, candidate.clone());
    let same = FrozenSubmission::new(FirstLightTarget::FlC, candidate.clone());
    let other_target = FrozenSubmission::new(FirstLightTarget::FlA, candidate.clone());
    let other_candidate = FrozenSubmission::new(
        FirstLightTarget::FlC,
        FrozenCandidate::new(d(20), d(22), 9),
    );

    assert_eq!(submission.digest(), same.digest());
    assert_ne!(submission.digest(), other_target.digest());
    assert_ne!(submission.digest(), other_candidate.digest());
    assert_eq!(submission.target(), FirstLightTarget::FlC);
    assert_eq!(submission.candidate(), &candidate);
}
