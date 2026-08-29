use formula_check::{
    polynomial::{IntegerPolynomial, check_polynomial_identity},
    verdict::CheckVerdict,
};

fn seventh_difference() -> IntegerPolynomial {
    // (n + 1)^7 - n^7, ascending coefficient order.
    IntegerPolynomial::from_i64(&[1, 7, 21, 35, 35, 21, 7])
}

#[test]
fn expanded_difference_of_seventh_powers_passes() {
    let expected = seventh_difference();
    let candidate = IntegerPolynomial::from_i64(&[1, 7, 21, 35, 35, 21, 7]);
    assert_eq!(
        check_polynomial_identity(&expected, &candidate),
        CheckVerdict::Pass
    );
}

#[test]
fn sample_fitting_near_miss_fails_universal_identity() {
    // Add n(n-1)...(n-6): this agrees at samples 0..=6 but is not the same polynomial.
    let expected = seventh_difference();
    let near_miss = IntegerPolynomial::from_i64(&[1, 727, -1743, 1659, -700, 196, -14, 1]);
    assert_ne!(
        check_polynomial_identity(&expected, &near_miss),
        CheckVerdict::Pass
    );
}

#[test]
fn trailing_zero_coefficients_normalize() {
    let expected = seventh_difference();
    let candidate = IntegerPolynomial::from_i64(&[1, 7, 21, 35, 35, 21, 7, 0, 0]);
    assert_eq!(
        check_polynomial_identity(&expected, &candidate),
        CheckVerdict::Pass
    );
}

#[test]
fn coefficient_change_fails() {
    let expected = seventh_difference();
    let changed = IntegerPolynomial::from_i64(&[1, 7, 21, 36, 35, 21, 7]);
    assert_ne!(
        check_polynomial_identity(&expected, &changed),
        CheckVerdict::Pass
    );
}
