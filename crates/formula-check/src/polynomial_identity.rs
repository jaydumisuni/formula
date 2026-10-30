//! Bounded exact integer-polynomial identity preflight (P2 preparation only).
//! Coefficient vectors use ascending powers; no sample-based admission.
//! This module does not issue certificates or admit a trust root.

// Wire v0: one-byte coefficient count (1..=16), followed by that many
// signed big-endian i64 coefficients, repeated for the right-hand side.
// Coefficients are low-degree first; the zero polynomial is encoded [0].
// This is a bounded exact subset of Z[n], not a complete polynomial solver.
const MAX_COEFFICIENTS: usize = 16;
const MAX_FRAME_BYTES: usize = 2 + 2 * MAX_COEFFICIENTS * 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialIdentityError {
    InvalidFrame,
    NonCanonicalLeft,
    NonCanonicalRight,
    CoefficientMismatch { power: u8, left: i64, right: i64 },
}

fn read_coefficients(body: &[u8], cursor: &mut usize) -> Option<Vec<i64>> {
    let count = usize::from(*body.get(*cursor)?);
    if !(1..=MAX_COEFFICIENTS).contains(&count) {
        return None;
    }
    *cursor += 1;
    let byte_count = count * 8;
    let encoded = body.get(*cursor..*cursor + byte_count)?;
    *cursor += byte_count;
    let mut coefficients = Vec::with_capacity(count);
    for bytes in encoded.as_chunks::<8>().0 {
        coefficients.push(i64::from_be_bytes(*bytes));
    }
    Some(coefficients)
}

/// Compare two canonical, frozen integer coefficient vectors in Z[n].
///
/// Equality of coefficients is a complete identity decision within this
/// bounded representation, unlike agreement at finitely many sample points.
/// The caller must independently establish semantic judgement binding,
/// certificate policy and checker trust before granting any authority.
pub fn check_frozen_polynomial_identity(body: &[u8]) -> Result<(), PolynomialIdentityError> {
    if body.len() > MAX_FRAME_BYTES {
        return Err(PolynomialIdentityError::InvalidFrame);
    }
    let mut cursor = 0;
    let left = read_coefficients(body, &mut cursor).ok_or(PolynomialIdentityError::InvalidFrame)?;
    let right =
        read_coefficients(body, &mut cursor).ok_or(PolynomialIdentityError::InvalidFrame)?;
    if cursor != body.len() {
        return Err(PolynomialIdentityError::InvalidFrame);
    }
    if left.len() > 1 && left.last() == Some(&0) {
        return Err(PolynomialIdentityError::NonCanonicalLeft);
    }
    if right.len() > 1 && right.last() == Some(&0) {
        return Err(PolynomialIdentityError::NonCanonicalRight);
    }
    for power in 0..left.len().max(right.len()) {
        let a = left.get(power).copied().unwrap_or(0);
        let b = right.get(power).copied().unwrap_or(0);
        if a != b {
            return Err(PolynomialIdentityError::CoefficientMismatch {
                power: power as u8,
                left: a,
                right: b,
            });
        }
    }
    Ok(())
}

// Wire v0 product witness: three canonical coefficient vectors (factor A,
// factor B, claimed product). The bound is deliberately small and never
// delegates arithmetic or authority to the producer.
const MAX_PRODUCT_FRAME_BYTES: usize = 3 + 3 * MAX_COEFFICIENTS * 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialProductError {
    InvalidFrame,
    NonCanonicalFactor,
    NonCanonicalProduct,
    DegreeOutOfBounds,
    ArithmeticOverflow,
    CoefficientOutOfRange,
    CoefficientMismatch {
        power: u8,
        expected: i64,
        actual: i64,
    },
}

/// Independently check a bounded Z[n] multiplication witness with exact
/// integer arithmetic. No samples, wrapping, certificate issuance, or
/// promotion authority are involved.
pub fn check_frozen_polynomial_product(body: &[u8]) -> Result<(), PolynomialProductError> {
    if body.len() > MAX_PRODUCT_FRAME_BYTES {
        return Err(PolynomialProductError::InvalidFrame);
    }
    let mut cursor = 0;
    let a = read_coefficients(body, &mut cursor).ok_or(PolynomialProductError::InvalidFrame)?;
    let b = read_coefficients(body, &mut cursor).ok_or(PolynomialProductError::InvalidFrame)?;
    let claimed =
        read_coefficients(body, &mut cursor).ok_or(PolynomialProductError::InvalidFrame)?;
    if cursor != body.len() {
        return Err(PolynomialProductError::InvalidFrame);
    }
    if (a.len() > 1 && a.last() == Some(&0)) || (b.len() > 1 && b.last() == Some(&0)) {
        return Err(PolynomialProductError::NonCanonicalFactor);
    }
    if claimed.len() > 1 && claimed.last() == Some(&0) {
        return Err(PolynomialProductError::NonCanonicalProduct);
    }
    let zero = a == [0] || b == [0];
    let expected_len = if zero { 1 } else { a.len() + b.len() - 1 };
    if expected_len > MAX_COEFFICIENTS {
        return Err(PolynomialProductError::DegreeOutOfBounds);
    }
    let mut coefficients = vec![0_i128; expected_len];
    for (i, &left) in a.iter().enumerate() {
        for (j, &right) in b.iter().enumerate() {
            if left == 0 || right == 0 {
                continue;
            }
            let term = i128::from(left)
                .checked_mul(i128::from(right))
                .ok_or(PolynomialProductError::ArithmeticOverflow)?;
            coefficients[i + j] = coefficients[i + j]
                .checked_add(term)
                .ok_or(PolynomialProductError::ArithmeticOverflow)?;
        }
    }
    if claimed.len() != expected_len {
        return Err(PolynomialProductError::NonCanonicalProduct);
    }
    for (power, (&exact, &actual)) in coefficients.iter().zip(&claimed).enumerate() {
        let expected =
            i64::try_from(exact).map_err(|_| PolynomialProductError::CoefficientOutOfRange)?;
        if expected != actual {
            return Err(PolynomialProductError::CoefficientMismatch {
                power: power as u8,
                expected,
                actual,
            });
        }
    }
    Ok(())
}

// Wire v0 sum witness: three canonical vectors (left, right, claimed sum).
// This is exact Z[n] addition, not a proof of the enclosing Judgement.
const MAX_SUM_FRAME_BYTES: usize = 3 + 3 * MAX_COEFFICIENTS * 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialSumError {
    InvalidFrame,
    NonCanonicalOperand,
    NonCanonicalSum,
    CoefficientOutOfRange,
    CoefficientMismatch {
        power: u8,
        expected: i64,
        actual: i64,
    },
}

/// Check a frozen, bounded polynomial-addition witness independently of its
/// producer. i128 intermediates prevent signed i64 wrapping; cancellation
/// must yield a canonical zero or trimmed-degree polynomial. This does not
/// certify a theorem, admit a checker trust root, or grant promotion.
pub fn check_frozen_polynomial_sum(body: &[u8]) -> Result<(), PolynomialSumError> {
    if body.len() > MAX_SUM_FRAME_BYTES {
        return Err(PolynomialSumError::InvalidFrame);
    }
    let mut cursor = 0;
    let left = read_coefficients(body, &mut cursor).ok_or(PolynomialSumError::InvalidFrame)?;
    let right = read_coefficients(body, &mut cursor).ok_or(PolynomialSumError::InvalidFrame)?;
    let claimed = read_coefficients(body, &mut cursor).ok_or(PolynomialSumError::InvalidFrame)?;
    if cursor != body.len() {
        return Err(PolynomialSumError::InvalidFrame);
    }
    if (left.len() > 1 && left.last() == Some(&0)) || (right.len() > 1 && right.last() == Some(&0))
    {
        return Err(PolynomialSumError::NonCanonicalOperand);
    }
    if claimed.len() > 1 && claimed.last() == Some(&0) {
        return Err(PolynomialSumError::NonCanonicalSum);
    }

    let mut expected = Vec::with_capacity(left.len().max(right.len()));
    for power in 0..left.len().max(right.len()) {
        let exact = i128::from(left.get(power).copied().unwrap_or(0))
            + i128::from(right.get(power).copied().unwrap_or(0));
        expected.push(i64::try_from(exact).map_err(|_| PolynomialSumError::CoefficientOutOfRange)?);
    }
    while expected.len() > 1 && expected.last() == Some(&0) {
        expected.pop();
    }
    for power in 0..expected.len().max(claimed.len()) {
        let exact = expected.get(power).copied().unwrap_or(0);
        let actual = claimed.get(power).copied().unwrap_or(0);
        if exact != actual {
            return Err(PolynomialSumError::CoefficientMismatch {
                power: power as u8,
                expected: exact,
                actual,
            });
        }
    }
    Ok(())
}

// Wire v0 difference witness: canonical left, right, and claimed left-right.
// The signed arithmetic is exact over Z[n], not wrapping i64 subtraction.
const MAX_DIFFERENCE_FRAME_BYTES: usize = 3 + 3 * MAX_COEFFICIENTS * 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialDifferenceError {
    InvalidFrame,
    NonCanonicalOperand,
    NonCanonicalDifference,
    CoefficientOutOfRange,
    CoefficientMismatch {
        power: u8,
        expected: i64,
        actual: i64,
    },
}

/// Check a frozen bounded subtraction witness, including signed boundaries,
/// cancellation, and canonical degree. This is non-authoritative P2 preparation;
/// it cannot certify that the enclosing Judgement denotes these polynomials.
pub fn check_frozen_polynomial_difference(body: &[u8]) -> Result<(), PolynomialDifferenceError> {
    if body.len() > MAX_DIFFERENCE_FRAME_BYTES {
        return Err(PolynomialDifferenceError::InvalidFrame);
    }
    let mut cursor = 0;
    let left =
        read_coefficients(body, &mut cursor).ok_or(PolynomialDifferenceError::InvalidFrame)?;
    let right =
        read_coefficients(body, &mut cursor).ok_or(PolynomialDifferenceError::InvalidFrame)?;
    let claimed =
        read_coefficients(body, &mut cursor).ok_or(PolynomialDifferenceError::InvalidFrame)?;
    if cursor != body.len() {
        return Err(PolynomialDifferenceError::InvalidFrame);
    }
    if (left.len() > 1 && left.last() == Some(&0)) || (right.len() > 1 && right.last() == Some(&0))
    {
        return Err(PolynomialDifferenceError::NonCanonicalOperand);
    }
    if claimed.len() > 1 && claimed.last() == Some(&0) {
        return Err(PolynomialDifferenceError::NonCanonicalDifference);
    }

    let mut expected = Vec::with_capacity(left.len().max(right.len()));
    for power in 0..left.len().max(right.len()) {
        let exact = i128::from(left.get(power).copied().unwrap_or(0))
            - i128::from(right.get(power).copied().unwrap_or(0));
        expected.push(
            i64::try_from(exact).map_err(|_| PolynomialDifferenceError::CoefficientOutOfRange)?,
        );
    }
    while expected.len() > 1 && expected.last() == Some(&0) {
        expected.pop();
    }
    for power in 0..expected.len().max(claimed.len()) {
        let exact = expected.get(power).copied().unwrap_or(0);
        let actual = claimed.get(power).copied().unwrap_or(0);
        if exact != actual {
            return Err(PolynomialDifferenceError::CoefficientMismatch {
                power: power as u8,
                expected: exact,
                actual,
            });
        }
    }
    Ok(())
}

// Wire v0 scalar witness: signed big-endian i64 scalar followed by two
// canonical coefficient vectors (operand and claimed scalar product).
// This bounded Z[n] check is preparation, never certificate authority.
const MAX_SCALE_FRAME_BYTES: usize = 8 + 2 + 2 * MAX_COEFFICIENTS * 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialScaleError {
    InvalidFrame,
    NonCanonicalOperand,
    NonCanonicalResult,
    CoefficientOutOfRange,
    CoefficientMismatch {
        power: u8,
        expected: i64,
        actual: i64,
    },
}

/// Check exact scalar multiplication against the frozen bytes, not samples.
/// The enclosing Judgement, trust root, and promotion remain unqualified.
pub fn check_frozen_polynomial_scale(body: &[u8]) -> Result<(), PolynomialScaleError> {
    if body.len() > MAX_SCALE_FRAME_BYTES {
        return Err(PolynomialScaleError::InvalidFrame);
    }
    let scalar_bytes: [u8; 8] = body
        .get(..8)
        .ok_or(PolynomialScaleError::InvalidFrame)?
        .try_into()
        .map_err(|_| PolynomialScaleError::InvalidFrame)?;
    let scalar = i128::from(i64::from_be_bytes(scalar_bytes));
    let mut cursor = 8;
    let operand = read_coefficients(body, &mut cursor).ok_or(PolynomialScaleError::InvalidFrame)?;
    let claimed = read_coefficients(body, &mut cursor).ok_or(PolynomialScaleError::InvalidFrame)?;
    if cursor != body.len() {
        return Err(PolynomialScaleError::InvalidFrame);
    }
    if operand.len() > 1 && operand.last() == Some(&0) {
        return Err(PolynomialScaleError::NonCanonicalOperand);
    }
    if claimed.len() > 1 && claimed.last() == Some(&0) {
        return Err(PolynomialScaleError::NonCanonicalResult);
    }
    let mut expected = Vec::with_capacity(operand.len());
    for coefficient in operand {
        let exact = scalar * i128::from(coefficient);
        expected
            .push(i64::try_from(exact).map_err(|_| PolynomialScaleError::CoefficientOutOfRange)?);
    }
    while expected.len() > 1 && expected.last() == Some(&0) {
        expected.pop();
    }
    for power in 0..expected.len().max(claimed.len()) {
        let expected_coefficient = expected.get(power).copied().unwrap_or(0);
        let actual = claimed.get(power).copied().unwrap_or(0);
        if expected_coefficient != actual {
            return Err(PolynomialScaleError::CoefficientMismatch {
                power: power as u8,
                expected: expected_coefficient,
                actual,
            });
        }
    }
    Ok(())
}

// Wire v0 formal derivative witness: two canonical coefficient vectors
// (operand and claimed derivative). Coefficients are ascending powers.
// This checks exact bounded Z[n] arithmetic, not a theorem certificate.
const MAX_DERIVATIVE_FRAME_BYTES: usize = 2 + 2 * MAX_COEFFICIENTS * 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialDerivativeError {
    InvalidFrame,
    NonCanonicalOperand,
    NonCanonicalDerivative,
    CoefficientOutOfRange,
    CoefficientMismatch {
        power: u8,
        expected: i64,
        actual: i64,
    },
}

/// Independently check the formal derivative d/dn of a frozen polynomial.
/// Reject overflow instead of wrapping or accepting a producer's sample values.
/// Judgement binding, checker trust and certificate authority remain separate.
pub fn check_frozen_polynomial_derivative(body: &[u8]) -> Result<(), PolynomialDerivativeError> {
    if body.len() > MAX_DERIVATIVE_FRAME_BYTES {
        return Err(PolynomialDerivativeError::InvalidFrame);
    }
    let mut cursor = 0;
    let operand =
        read_coefficients(body, &mut cursor).ok_or(PolynomialDerivativeError::InvalidFrame)?;
    let claimed =
        read_coefficients(body, &mut cursor).ok_or(PolynomialDerivativeError::InvalidFrame)?;
    if cursor != body.len() {
        return Err(PolynomialDerivativeError::InvalidFrame);
    }
    if operand.len() > 1 && operand.last() == Some(&0) {
        return Err(PolynomialDerivativeError::NonCanonicalOperand);
    }
    if claimed.len() > 1 && claimed.last() == Some(&0) {
        return Err(PolynomialDerivativeError::NonCanonicalDerivative);
    }
    let mut expected = Vec::with_capacity(operand.len().saturating_sub(1).max(1));
    if operand.len() == 1 {
        expected.push(0);
    } else {
        for (power, &coefficient) in operand.iter().enumerate().skip(1) {
            let exact = i128::from(coefficient) * power as i128;
            expected.push(
                i64::try_from(exact)
                    .map_err(|_| PolynomialDerivativeError::CoefficientOutOfRange)?,
            );
        }
    }
    for power in 0..expected.len().max(claimed.len()) {
        let correct = expected.get(power).copied().unwrap_or(0);
        let actual = claimed.get(power).copied().unwrap_or(0);
        if correct != actual {
            return Err(PolynomialDerivativeError::CoefficientMismatch {
                power: power as u8,
                expected: correct,
                actual,
            });
        }
    }
    Ok(())
}

// Wire v0 evaluation witness: signed big-endian i64 evaluation point,
// one canonical bounded Z[x] coefficient vector, and a claimed signed i64
// value. Exact evaluation of one point never establishes polynomial identity.
const MAX_EVALUATION_FRAME_BYTES: usize = 8 + 1 + MAX_COEFFICIENTS * 8 + 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialEvaluationError {
    InvalidFrame,
    NonCanonicalPolynomial,
    ArithmeticOverflow,
    ValueOutOfRange,
    ValueMismatch { expected: i64, actual: i64 },
}

/// Independently evaluate a frozen polynomial at one integer point using
/// checked Horner arithmetic. The bounded i128 intermediate path fails closed
/// on overflow, and the claimed result must fit the signed i64 wire contract.
/// Agreement at finitely many points is NOT a polynomial identity certificate;
/// no Judgement, trust-root, or promotion authority is granted here.
pub fn check_frozen_polynomial_evaluation(body: &[u8]) -> Result<(), PolynomialEvaluationError> {
    if body.len() > MAX_EVALUATION_FRAME_BYTES {
        return Err(PolynomialEvaluationError::InvalidFrame);
    }
    let point_bytes: [u8; 8] = body
        .get(..8)
        .ok_or(PolynomialEvaluationError::InvalidFrame)?
        .try_into()
        .map_err(|_| PolynomialEvaluationError::InvalidFrame)?;
    let point = i128::from(i64::from_be_bytes(point_bytes));
    let mut cursor = 8;
    let coefficients =
        read_coefficients(body, &mut cursor).ok_or(PolynomialEvaluationError::InvalidFrame)?;
    let claimed_bytes: [u8; 8] = body
        .get(cursor..)
        .ok_or(PolynomialEvaluationError::InvalidFrame)?
        .try_into()
        .map_err(|_| PolynomialEvaluationError::InvalidFrame)?;
    if coefficients.len() > 1 && coefficients.last() == Some(&0) {
        return Err(PolynomialEvaluationError::NonCanonicalPolynomial);
    }
    let actual = i64::from_be_bytes(claimed_bytes);
    let mut exact = 0_i128;
    for coefficient in coefficients.iter().rev() {
        exact = exact
            .checked_mul(point)
            .and_then(|value| value.checked_add(i128::from(*coefficient)))
            .ok_or(PolynomialEvaluationError::ArithmeticOverflow)?;
    }
    let expected = i64::try_from(exact).map_err(|_| PolynomialEvaluationError::ValueOutOfRange)?;
    if expected != actual {
        return Err(PolynomialEvaluationError::ValueMismatch { expected, actual });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(left: &[i64], right: &[i64]) -> Vec<u8> {
        let mut out = Vec::new();
        for coefficients in [left, right] {
            out.push(coefficients.len() as u8);
            for coefficient in coefficients {
                out.extend_from_slice(&coefficient.to_be_bytes());
            }
        }
        out
    }

    fn product_frame(a: &[i64], b: &[i64], product: &[i64]) -> Vec<u8> {
        let mut out = Vec::new();
        for coefficients in [a, b, product] {
            out.push(coefficients.len() as u8);
            for coefficient in coefficients {
                out.extend_from_slice(&coefficient.to_be_bytes());
            }
        }
        out
    }

    #[test]
    fn frozen_product_witness_uses_exact_convolution_not_samples() {
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[2, -3], &[4, 5], &[8, -2, -15])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[0], &[1, 2], &[0])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[i64::MAX], &[1], &[i64::MAX])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[2, -3], &[4, 5], &[8, -2, -14])),
            Err(PolynomialProductError::CoefficientMismatch {
                power: 2,
                expected: -15,
                actual: -14,
            })
        );
    }

    #[test]
    fn frozen_product_witness_rejects_overflow_and_noncanonical_frames() {
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[i64::MAX], &[2], &[0])),
            Err(PolynomialProductError::CoefficientOutOfRange)
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(
                &[i64::MAX; 3],
                &[i64::MAX; 3],
                &[0, 0, 0, 0, 1],
            )),
            Err(PolynomialProductError::ArithmeticOverflow)
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[1; 16], &[1, 1], &[1])),
            Err(PolynomialProductError::DegreeOutOfBounds)
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[1, 0], &[1], &[1])),
            Err(PolynomialProductError::NonCanonicalFactor)
        );
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&[1], &[1], &[1, 0])),
            Err(PolynomialProductError::NonCanonicalProduct)
        );
        let valid = product_frame(&[1, -2], &[2, 1], &[2, -3, -2]);
        for end in 0..valid.len() {
            assert_eq!(
                check_frozen_polynomial_product(&valid[..end]),
                Err(PolynomialProductError::InvalidFrame),
                "truncated at {end}"
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_product(&trailing),
            Err(PolynomialProductError::InvalidFrame)
        );
    }

    #[test]
    fn exact_coefficients_and_distinct_degrees() {
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[1, -2, 3], &[1, -2, 3])),
            Ok(())
        );
        assert_eq!(check_frozen_polynomial_identity(&frame(&[0], &[0])), Ok(()));
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[1, -2, 3], &[1, -2, 4])),
            Err(PolynomialIdentityError::CoefficientMismatch {
                power: 2,
                left: 3,
                right: 4,
            })
        );
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[1, 2, 3], &[1, 2, 3, 1])),
            Err(PolynomialIdentityError::CoefficientMismatch {
                power: 3,
                left: 0,
                right: 1,
            })
        );
    }

    // Eight coincident sample values are not an identity certificate.
    // This degree-eight polynomial vanishes at x=0..7 but is not zero in Z[x].
    #[test]
    fn finite_sample_agreement_cannot_substitute_for_exact_polynomial_identity() {
        let vanishing = [0, -5040, 13068, -13132, 6769, -1960, 322, -28, 1];
        for x in 0_i128..8 {
            let evaluated = vanishing.iter().rev().fold(0_i128, |acc, &coefficient| {
                acc * x + i128::from(coefficient)
            });
            assert_eq!(evaluated, 0, "counterexample fixture must vanish at x={x}");
        }
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&vanishing, &vanishing)),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[0], &vanishing)),
            Err(PolynomialIdentityError::CoefficientMismatch {
                power: 1,
                left: 0,
                right: -5040,
            })
        );
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&vanishing, &[0])),
            Err(PolynomialIdentityError::CoefficientMismatch {
                power: 1,
                left: -5040,
                right: 0,
            })
        );
    }

    // A degree-eight polynomial can vanish at eight sampled inputs while
    // remaining nonzero. None of the independent arithmetic witnesses may
    // substitute that finite agreement for exact coefficient custody.
    #[test]
    fn finite_sample_collision_cannot_forge_polynomial_arithmetic_witnesses() {
        let vanishing = [0, -5040, 13068, -13132, 6769, -1960, 322, -28, 1];
        for x in 0_i128..8 {
            assert_eq!(
                vanishing.iter().rev().fold(0_i128, |acc, &coefficient| {
                    acc * x + i128::from(coefficient)
                }),
                0,
                "fixture must vanish at sampled x={x}"
            );
        }

        // All four correct frozen witnesses are admissible as bounded
        // arithmetic checks; no judgement/certificate authority follows.
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&vanishing, &[1], &vanishing)),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&vanishing, &[0], &vanishing)),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&vanishing, &[0], &vanishing)),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(1, &vanishing, &vanishing)),
            Ok(())
        );

        // A producer claiming the zero polynomial would pass all eight
        // samples, but each exact checker must reject the same forged claim.
        assert_eq!(
            check_frozen_polynomial_product(&product_frame(&vanishing, &[1], &[0])),
            Err(PolynomialProductError::NonCanonicalProduct)
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&vanishing, &[0], &[0])),
            Err(PolynomialSumError::CoefficientMismatch {
                power: 1,
                expected: -5040,
                actual: 0
            })
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&vanishing, &[0], &[0])),
            Err(PolynomialDifferenceError::CoefficientMismatch {
                power: 1,
                expected: -5040,
                actual: 0
            })
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(1, &vanishing, &[0])),
            Err(PolynomialScaleError::CoefficientMismatch {
                power: 1,
                expected: -5040,
                actual: 0
            })
        );
    }

    #[test]
    fn malformed_and_noncanonical_frozen_frames_fail_closed() {
        for invalid in [vec![], vec![0], vec![1], vec![1, 0], vec![17], vec![255]] {
            assert_eq!(
                check_frozen_polynomial_identity(&invalid),
                Err(PolynomialIdentityError::InvalidFrame)
            );
        }
        let mut trailing = frame(&[1], &[1]);
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_identity(&trailing),
            Err(PolynomialIdentityError::InvalidFrame)
        );
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[1, 0], &[1])),
            Err(PolynomialIdentityError::NonCanonicalLeft)
        );
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[1], &[1, 0])),
            Err(PolynomialIdentityError::NonCanonicalRight)
        );
    }

    #[test]
    fn signed_boundaries_and_maximum_degree_are_exact() {
        let mut coefficients = vec![0; 16];
        coefficients[0] = i64::MIN;
        coefficients[15] = i64::MAX;
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&coefficients, &coefficients)),
            Ok(())
        );
        let mut altered = coefficients.clone();
        altered[15] = i64::MAX - 1;
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&coefficients, &altered)),
            Err(PolynomialIdentityError::CoefficientMismatch {
                power: 15,
                left: i64::MAX,
                right: i64::MAX - 1,
            })
        );
        assert_eq!(
            check_frozen_polynomial_identity(&frame(&[1; 17], &[1])),
            Err(PolynomialIdentityError::InvalidFrame)
        );
    }

    #[test]
    fn frozen_difference_witness_checks_signed_cancellation_and_exact_bounds() {
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[5, -3], &[2, -5], &[3, 2])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[1, 2], &[1, 2], &[0])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[i64::MIN], &[0], &[i64::MIN])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[0], &[i64::MIN], &[0])),
            Err(PolynomialDifferenceError::CoefficientOutOfRange)
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[1, 2], &[1, 1], &[1, 1])),
            Err(PolynomialDifferenceError::CoefficientMismatch {
                power: 0,
                expected: 0,
                actual: 1,
            })
        );
    }

    #[test]
    fn frozen_difference_rejects_noncanonical_and_malformed_frames() {
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[1, 0], &[1], &[0])),
            Err(PolynomialDifferenceError::NonCanonicalOperand)
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[1], &[0], &[1, 0])),
            Err(PolynomialDifferenceError::NonCanonicalDifference)
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[1; 17], &[0], &[1])),
            Err(PolynomialDifferenceError::InvalidFrame)
        );
        assert_eq!(
            check_frozen_polynomial_difference(&product_frame(&[0], &[1], &[-1])),
            Ok(())
        );
        let valid = product_frame(&[1, -2], &[2, 1], &[-1, -3]);
        for end in 0..valid.len() {
            assert_eq!(
                check_frozen_polynomial_difference(&valid[..end]),
                Err(PolynomialDifferenceError::InvalidFrame),
                "truncated at {end}"
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_difference(&trailing),
            Err(PolynomialDifferenceError::InvalidFrame)
        );
    }

    fn scale_frame(scalar: i64, operand: &[i64], claimed: &[i64]) -> Vec<u8> {
        let mut out = scalar.to_be_bytes().to_vec();
        for coefficients in [operand, claimed] {
            out.push(coefficients.len() as u8);
            for coefficient in coefficients {
                out.extend_from_slice(&coefficient.to_be_bytes());
            }
        }
        out
    }

    #[test]
    fn frozen_scalar_witness_is_exact_and_canonical() {
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(-2, &[2, -3], &[-4, 6])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(0, &[1, 2], &[0])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(1, &[i64::MIN], &[i64::MIN])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(-2, &[2, -3], &[-4, 5])),
            Err(PolynomialScaleError::CoefficientMismatch {
                power: 1,
                expected: 6,
                actual: 5
            })
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(-1, &[i64::MIN], &[0])),
            Err(PolynomialScaleError::CoefficientOutOfRange)
        );
    }

    #[test]
    fn frozen_scalar_witness_rejects_noncanonical_and_malformed_frames() {
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(1, &[1, 0], &[1])),
            Err(PolynomialScaleError::NonCanonicalOperand)
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(0, &[1], &[0, 0])),
            Err(PolynomialScaleError::NonCanonicalResult)
        );
        assert_eq!(
            check_frozen_polynomial_scale(&scale_frame(2, &[1; 17], &[2])),
            Err(PolynomialScaleError::InvalidFrame)
        );
        let valid = scale_frame(-2, &[1, -2], &[-2, 4]);
        for end in 0..valid.len() {
            assert_eq!(
                check_frozen_polynomial_scale(&valid[..end]),
                Err(PolynomialScaleError::InvalidFrame),
                "truncated at {end}"
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_scale(&trailing),
            Err(PolynomialScaleError::InvalidFrame)
        );
    }

    #[test]
    fn frozen_sum_witness_checks_exact_coefficients_and_cancellation() {
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[2, -3], &[4, 5], &[6, 2])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[2, -3], &[-2, 3], &[0])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[i64::MIN], &[0], &[i64::MIN])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[2, -3], &[4, 5], &[6, 3])),
            Err(PolynomialSumError::CoefficientMismatch {
                power: 1,
                expected: 2,
                actual: 3,
            })
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[1; 16], &[-1; 16], &[0])),
            Ok(())
        );
    }

    #[test]
    fn frozen_sum_witness_rejects_overflow_noncanonical_and_malformed_frames() {
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[i64::MAX], &[1], &[0])),
            Err(PolynomialSumError::CoefficientOutOfRange)
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[i64::MIN], &[-1], &[0])),
            Err(PolynomialSumError::CoefficientOutOfRange)
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[1, 0], &[1], &[2])),
            Err(PolynomialSumError::NonCanonicalOperand)
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[1], &[0], &[1, 0])),
            Err(PolynomialSumError::NonCanonicalSum)
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[1], &[0], &[0])),
            Err(PolynomialSumError::CoefficientMismatch {
                power: 0,
                expected: 1,
                actual: 0,
            })
        );
        assert_eq!(
            check_frozen_polynomial_sum(&product_frame(&[1; 17], &[0], &[1])),
            Err(PolynomialSumError::InvalidFrame)
        );
        let valid = product_frame(&[1, -2], &[2, 1], &[3, -1]);
        for end in 0..valid.len() {
            assert_eq!(
                check_frozen_polynomial_sum(&valid[..end]),
                Err(PolynomialSumError::InvalidFrame),
                "truncated at {end}"
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_sum(&trailing),
            Err(PolynomialSumError::InvalidFrame)
        );
    }

    #[test]
    fn frozen_derivative_witness_checks_exact_formal_derivative() {
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[7, -3, 2], &[-3, 4])),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[i64::MIN], &[0])),
            Ok(())
        );
        let mut highest = vec![0; 16];
        highest[15] = 1;
        let mut expected = vec![0; 15];
        expected[14] = 15;
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&highest, &expected)),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[7, -3, 2], &[-3, 5])),
            Err(PolynomialDerivativeError::CoefficientMismatch {
                power: 1,
                expected: 4,
                actual: 5,
            })
        );
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[0, 0, i64::MAX], &[0])),
            Err(PolynomialDerivativeError::CoefficientOutOfRange)
        );
    }

    #[test]
    fn frozen_derivative_witness_rejects_malformed_and_noncanonical_frames() {
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[1, 0], &[0])),
            Err(PolynomialDerivativeError::NonCanonicalOperand)
        );
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[3], &[0, 0])),
            Err(PolynomialDerivativeError::NonCanonicalDerivative)
        );
        assert_eq!(
            check_frozen_polynomial_derivative(&frame(&[1; 17], &[0])),
            Err(PolynomialDerivativeError::InvalidFrame)
        );
        let valid = frame(&[3, -2, 1], &[-2, 2]);
        for end in 0..valid.len() {
            assert_eq!(
                check_frozen_polynomial_derivative(&valid[..end]),
                Err(PolynomialDerivativeError::InvalidFrame),
                "truncated at {end}"
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_derivative(&trailing),
            Err(PolynomialDerivativeError::InvalidFrame)
        );
    }

    fn evaluation_frame(x: i64, polynomial: &[i64], claimed: i64) -> Vec<u8> {
        let mut out = x.to_be_bytes().to_vec();
        out.push(polynomial.len() as u8);
        for coefficient in polynomial {
            out.extend_from_slice(&coefficient.to_be_bytes());
        }
        out.extend_from_slice(&claimed.to_be_bytes());
        out
    }

    #[test]
    fn frozen_evaluation_witness_checks_exact_horner_without_identity_authority() {
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(4, &[2, -3, 1], 6)),
            Ok(())
        );
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(4, &[2, -3, 1], 7)),
            Err(PolynomialEvaluationError::ValueMismatch {
                expected: 6,
                actual: 7
            })
        );
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(0, &[i64::MIN], i64::MIN)),
            Ok(())
        );
        let mut degree_fifteen = vec![0; 16];
        degree_fifteen[15] = 1;
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(0, &degree_fifteen, 0)),
            Ok(())
        );
        // A finite sample of a nonzero polynomial is not an identity certificate.
        let vanishing = [0, -5040, 13068, -13132, 6769, -1960, 322, -28, 1];
        for x in 0..8 {
            assert_eq!(
                check_frozen_polynomial_evaluation(&evaluation_frame(x, &vanishing, 0)),
                Ok(())
            );
        }
        assert!(check_frozen_polynomial_identity(&frame(&vanishing, &[0])).is_err());
    }

    #[test]
    fn frozen_evaluation_witness_rejects_overflow_and_noncanonical_frames() {
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(1, &[i64::MAX, 1], 0)),
            Err(PolynomialEvaluationError::ValueOutOfRange)
        );
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(i64::MAX, &[1; 5], 0)),
            Err(PolynomialEvaluationError::ArithmeticOverflow)
        );
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(1, &[1, 0], 1)),
            Err(PolynomialEvaluationError::NonCanonicalPolynomial)
        );
        assert_eq!(
            check_frozen_polynomial_evaluation(&evaluation_frame(1, &[1; 17], 1)),
            Err(PolynomialEvaluationError::InvalidFrame)
        );
        let valid = evaluation_frame(-2, &[1, 3], -5);
        for end in 0..valid.len() {
            assert_eq!(
                check_frozen_polynomial_evaluation(&valid[..end]),
                Err(PolynomialEvaluationError::InvalidFrame),
                "truncated at {end}"
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_polynomial_evaluation(&trailing),
            Err(PolynomialEvaluationError::InvalidFrame)
        );
    }
}
