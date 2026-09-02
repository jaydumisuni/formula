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
fn rational128_normalizes_exactly() {
    assert_eq!(Rational128::new(6, -8).unwrap(), Rational128::new(-3, 4).unwrap());
    assert!(Rational128::new(1, 0).is_err());
}

#[test]
fn exact_samples_refine_an_affine_family_not_one_enumerated_candidate() {
    let mut space = AffinePolynomialSpace::new(context(), 2);
    assert_eq!(space.affine_dimension().unwrap(), 3);

    space.add_exact_sample(0, 3).unwrap();
    assert_eq!(space.affine_dimension().unwrap(), 2);
    space.add_exact_sample(1, 5).unwrap();
    assert_eq!(space.affine_dimension().unwrap(), 1);
    space.add_exact_sample(2, 7).unwrap();
    assert_eq!(space.affine_dimension().unwrap(), 0);

    let candidate = space.extract_min_degree_unique().unwrap();
    assert_eq!(candidate.coefficients(), &[Rational128::integer(3), Rational128::integer(2)]);
}

#[test]
fn degree_restriction_can_empty_the_space() {
    let mut space = AffinePolynomialSpace::new(context(), 2);
    space.add_exact_sample(0, 0).unwrap();
    space.add_exact_sample(1, 1).unwrap();
    space.add_exact_sample(2, 4).unwrap();
    assert!(!space.empty().unwrap());
    space.restrict_degree(1).unwrap();
    assert!(space.empty().unwrap());
}

#[test]
fn sample_order_is_non_semantic_for_freeze_identity() {
    let mut a = AffinePolynomialSpace::new(context(), 2);
    let mut b = AffinePolynomialSpace::new(context(), 2);
    for (x, y) in [(0, 3), (1, 5), (2, 7)] {
        a.add_exact_sample(x, y).unwrap();
    }
    for (x, y) in [(2, 7), (0, 3), (1, 5)] {
        b.add_exact_sample(x, y).unwrap();
    }
    assert_eq!(a.freeze().digest(), b.freeze().digest());
    assert_eq!(a.extract_min_degree_unique().unwrap().digest(), b.extract_min_degree_unique().unwrap().digest());
}
