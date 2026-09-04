use formula_core::digest::ArtifactDigest;
use formula_engine::{
    candidate_space::CandidateSpaceContext,
    observational::{FrozenExprCandidate, ObservationalExprSpace, U8BoolGrammar},
};
use formula_first_light::{
    fl_c::{fl_c_oracle, FlCOracle},
    manifest::{BlindnessManifest, FirstLightTarget},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn manifest(sealed_target: ArtifactDigest) -> BlindnessManifest {
    BlindnessManifest::new(
        FirstLightTarget::FlC,
        sealed_target,
        d(2),
        d(3),
        d(4),
        d(5),
        d(6),
        d(7),
    )
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(11), d(12), d(13), d(14), d(15), d(16))
}

fn expected_nonzero_domain(input: u8) -> bool {
    matches!(input, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128)
}

#[test]
fn blindness_manifest_tampering_changes_identity() {
    let baseline = manifest(d(1));
    let same = manifest(d(1));
    let tampered = manifest(d(8));

    assert_eq!(baseline, same);
    assert_eq!(baseline.digest(), same.digest());
    assert_ne!(baseline, tampered);
    assert_ne!(baseline.digest(), tampered.digest());
}

#[test]
fn hidden_fl_c_comparison_is_typed_to_frozen_candidates() {
    let _: fn(&FlCOracle, &FrozenExprCandidate) -> Option<(u8, bool)> =
        FlCOracle::first_counterexample;
}

#[test]
fn public_observations_can_leave_a_plausible_candidate_that_hidden_oracle_rejects() {
    let mut space = ObservationalExprSpace::new(context(), U8BoolGrammar::minimal(), 6);
    for input in 1_u8..=u8::MAX {
        space.restrict_exact_sample(input, expected_nonzero_domain(input));
    }
    let frozen_candidate = space.extract_min_cost().expect("publicly plausible candidate");

    assert_eq!(
        fl_c_oracle().first_counterexample(&frozen_candidate),
        Some((0, false))
    );
}
