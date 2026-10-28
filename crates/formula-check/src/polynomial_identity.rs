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
}
