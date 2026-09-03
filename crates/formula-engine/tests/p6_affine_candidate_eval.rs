use formula_core::digest::ArtifactDigest;
use formula_engine::{
    affine_polynomial::{AffinePolynomialSpace, Rational128},
    candidate_space::CandidateSpaceContext,
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn context() -> CandidateSpaceContext {
    CandidateSpaceContext::new(d(1), d(2), d(3), d(4), d(5), d(6))
}

#[test]
fn frozen_affine_candidate_supports_exact_read_only_evaluation() {
    let mut space = AffinePolynomialSpace::new(context(), 1);
    space.add_exact_sample(0, 4).unwrap();
    space.add_exact_sample(1, 7).unwrap();
    let candidate = space.extract_min_degree_unique().unwrap();

    assert_eq!(
        candidate.evaluate_integer(2).unwrap(),
        Rational128::integer(10)
    );
    assert_eq!(candidate.coefficients(), &[Rational128::integer(4), Rational128::integer(3)]);
}
