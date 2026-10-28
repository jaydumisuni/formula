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
}
