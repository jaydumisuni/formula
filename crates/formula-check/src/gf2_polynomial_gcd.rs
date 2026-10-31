//! Independent bounded GF(2)[x] GCD witness preflight on frozen bytes.
//! This is a non-authoritative source-only P2 preparation slice.
//! It does not authenticate a Judgement, issue a certificate, or promote mathematics.

// Wire v0: three big-endian u16 polynomial bitsets (lhs, rhs, claimed gcd).
// Bit i is the coefficient of x^i. The all-zero input pair has no
// uniquely defined greatest common divisor and must fail closed.
const FRAME_BYTES: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialGcdError {
    InvalidFrame,
    BothZero,
    GcdMismatch { expected: u16, actual: u16 },
}

/// Recompute the exact monic GCD over GF(2)[x] from two frozen operands.
///
/// This checker does not trust the producer's claimed GCD or a supplied
/// quotient. Euclidean remainder steps strictly decrease polynomial degree;
/// all intermediates fit in u16 because each XOR eliminates the leading term.
pub fn check_frozen_gf2_polynomial_gcd(body: &[u8]) -> Result<(), Gf2PolynomialGcdError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialGcdError::InvalidFrame)?;
    let mut lhs = u16::from_be_bytes([bytes[0], bytes[1]]);
    let mut rhs = u16::from_be_bytes([bytes[2], bytes[3]]);
    let claimed = u16::from_be_bytes([bytes[4], bytes[5]]);

    if lhs == 0 && rhs == 0 {
        return Err(Gf2PolynomialGcdError::BothZero);
    }

    while rhs != 0 {
        let mut remainder = lhs;
        while remainder != 0 && remainder.leading_zeros() <= rhs.leading_zeros() {
            let shift = rhs.leading_zeros() - remainder.leading_zeros();
            remainder ^= rhs << shift;
        }
        lhs = rhs;
        rhs = remainder;
    }

    if claimed != lhs {
        return Err(Gf2PolynomialGcdError::GcdMismatch {
            expected: lhs,
            actual: claimed,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(lhs: u16, rhs: u16, gcd: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        for (i, polynomial) in [lhs, rhs, gcd].into_iter().enumerate() {
            bytes[2 * i..2 * i + 2].copy_from_slice(&polynomial.to_be_bytes());
        }
        bytes
    }

    // An independent, exhaustive small-domain oracle: enumerate candidate
    // divisors and quotient products instead of reusing Euclidean division.
    fn carryless_product(lhs: u16, rhs: u16) -> u32 {
        let mut product = 0_u32;
        for bit in 0..16 {
            if rhs & (1_u16 << bit) != 0 {
                product ^= u32::from(lhs) << bit;
            }
        }
        product
    }

    fn divides(candidate: u16, dividend: u16) -> bool {
        (0_u16..32).any(|quotient| carryless_product(candidate, quotient) == u32::from(dividend))
    }

    fn independent_small_gcd(lhs: u16, rhs: u16) -> u16 {
        let mut best = 0_u16;
        for candidate in 1_u16..32 {
            if candidate.leading_zeros() < best.leading_zeros()
                && divides(candidate, lhs)
                && divides(candidate, rhs)
            {
                best = candidate;
            }
        }
        best
    }

    #[test]
    fn malformed_and_undefined_frames_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_gcd(&frame(0, 0, 0)),
            Err(Gf2PolynomialGcdError::BothZero)
        );
        let valid = frame(0x8000, 0x4000, 0x4000);
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_gcd(&valid[..length]),
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

    #[test]
    fn exhaustive_small_domain_matches_independent_divisor_enumeration() {
        for lhs in 0_u16..32 {
            for rhs in 0_u16..32 {
                if lhs == 0 && rhs == 0 {
                    continue;
                }
                let expected = independent_small_gcd(lhs, rhs);
                assert_ne!(expected, 0);
                assert_eq!(
                    check_frozen_gf2_polynomial_gcd(&frame(lhs, rhs, expected)),
                    Ok(()),
                    "lhs={lhs:#b} rhs={rhs:#b}"
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_gcd(&frame(lhs, rhs, expected ^ 1)),
                    Err(Gf2PolynomialGcdError::GcdMismatch {
                        expected,
                        actual: expected ^ 1,
                    })
                );
            }
        }
    }

    #[test]
    fn high_degree_zero_and_coprime_boundaries_are_exact() {
        for (lhs, rhs, expected) in [
            (0, 0x8000, 0x8000),
            (0x8000, 0, 0x8000),
            (0x8000, 0x4000, 0x4000),
            (0x8000, 0x8001, 1),
            (0xffff, 0xffff, 0xffff),
            (1, 0xffff, 1),
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_gcd(&frame(lhs, rhs, expected)),
                Ok(()),
                "lhs={lhs:#x} rhs={rhs:#x}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_gcd(&frame(lhs, rhs, expected ^ 1)),
                Err(Gf2PolynomialGcdError::GcdMismatch {
                    expected,
                    actual: expected ^ 1,
                })
            );
        }
    }
}
