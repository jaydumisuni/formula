//! Frozen GF(2)[x] greatest-common-divisor witness preflight.
//! This is source-only P2 preparation, not mathematical authority.

// Wire v0: exactly three big-endian u16 coefficient bitsets in ascending
// powers, a, b and the claimed gcd. Each bit i denotes coefficient of x^i.
// At least one input must be nonzero; gcd(0,0) has no unique monic witness.
const FRAME_BYTES: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialGcdError {
    InvalidFrame,
    BothZero,
    GcdMismatch { expected: u16, actual: u16 },
}

// The remainder is exact: shifts never exceed the degree of the dividend.
// There is no integer multiplication, rounding, modular truncation or
// producer-provided intermediate that can influence the result.
fn remainder(mut a: u16, b: u16) -> u16 {
    debug_assert_ne!(b, 0);
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        a ^= b << (b.leading_zeros() - a.leading_zeros());
    }
    a
}

/// Independently verify the canonical monic gcd of two frozen GF(2)[x]
/// polynomials using bounded Euclidean division. This is a source-only
/// preflight: it does not issue certificates or admit mathematical authority.
pub fn check_frozen_gf2_polynomial_gcd(body: &[u8]) -> Result<(), Gf2PolynomialGcdError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialGcdError::InvalidFrame)?;
    let mut a = u16::from_be_bytes([bytes[0], bytes[1]]);
    let mut b = u16::from_be_bytes([bytes[2], bytes[3]]);
    let actual = u16::from_be_bytes([bytes[4], bytes[5]]);
    if a == 0 && b == 0 {
        return Err(Gf2PolynomialGcdError::BothZero);
    }
    while b != 0 {
        let r = remainder(a, b);
        a = b;
        b = r;
    }
    if actual != a {
        return Err(Gf2PolynomialGcdError::GcdMismatch {
            expected: a,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(a: u16, b: u16, gcd: u16) -> [u8; 6] {
        let mut result = [0u8; 6];
        for (i, value) in [a, b, gcd].into_iter().enumerate() {
            result[2 * i..2 * i + 2].copy_from_slice(&value.to_be_bytes());
        }
        result
    }

    // Independent exhaustive oracle: enumerate monic common divisors rather
    // than using the Euclidean algorithm used by the checker.
    fn divides(mut dividend: u16, divisor: u16) -> bool {
        while dividend != 0 && dividend.leading_zeros() <= divisor.leading_zeros() {
            dividend ^= divisor << (divisor.leading_zeros() - dividend.leading_zeros());
        }
        dividend == 0
    }

    #[test]
    fn exhaustive_small_pairs_match_independent_common_divisor_enumeration() {
        for a in 0..64u16 {
            for b in 0..64u16 {
                if a == 0 && b == 0 {
                    continue;
                }
                let mut expected = 0u16;
                for candidate in 1..64u16 {
                    if divides(a, candidate)
                        && divides(b, candidate)
                        && candidate.leading_zeros() < expected.leading_zeros()
                    {
                        expected = candidate;
                    }
                }
                assert_ne!(expected, 0);
                assert_eq!(
                    check_frozen_gf2_polynomial_gcd(&frame(a, b, expected)),
                    Ok(())
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_gcd(&frame(a, b, expected ^ 1)),
                    Err(Gf2PolynomialGcdError::GcdMismatch {
                        expected,
                        actual: expected ^ 1,
                    })
                );
            }
        }
    }

    #[test]
    fn malformed_ambiguous_and_high_degree_witnesses_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_gcd(&frame(0, 0, 0)),
            Err(Gf2PolynomialGcdError::BothZero)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_gcd(&frame(0b101, 0b1001, 0b11)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_gcd(&frame(0x8000, 0, 0x8000)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_gcd(&frame(0, 0x8000, 0x8000)),
            Ok(())
        );
        let valid = frame(0b101, 0b1001, 0b11);
        for n in 0..valid.len() {
            assert_eq!(
                check_frozen_gf2_polynomial_gcd(&valid[..n]),
                Err(Gf2PolynomialGcdError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_gcd(&padded),
            Err(Gf2PolynomialGcdError::InvalidFrame)
        );
    }
}
