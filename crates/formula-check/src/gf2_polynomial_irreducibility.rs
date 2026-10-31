//! Independent bounded GF(2)[x] irreducibility witness preflight.
//! This is P2 preparation, not a certificate, Judgement, or promotion.
//!
//! Wire v0: a big-endian u16 coefficient bitset followed by a canonical
//! boolean claim byte (0 or 1). Bit i is the coefficient of x^i.
//! Zero and unit polynomials are not irreducible and are rejected as inputs.

const FRAME_BYTES: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialIrreducibilityError {
    InvalidFrame,
    ConstantPolynomial,
    InvalidClaim,
    IrreducibilityMismatch { expected: bool, actual: bool },
}

/// Independently recompute a bounded irreducibility claim over GF(2)[x].
///
/// A nonconstant polynomial is reducible iff it has a monic nonconstant
/// divisor of degree at most half its own degree. Enumerate those divisors
/// and perform exact XOR polynomial remainder, without trusting a supplied
/// factorization or a producer's claim. All intermediates fit in u16.
pub fn check_frozen_gf2_polynomial_irreducibility(
    body: &[u8],
) -> Result<(), Gf2PolynomialIrreducibilityError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialIrreducibilityError::InvalidFrame)?;
    let polynomial = u16::from_be_bytes([bytes[0], bytes[1]]);
    if polynomial < 2 {
        return Err(Gf2PolynomialIrreducibilityError::ConstantPolynomial);
    }
    let claimed = match bytes[2] {
        0 => false,
        1 => true,
        _ => return Err(Gf2PolynomialIrreducibilityError::InvalidClaim),
    };

    let degree = 15 - polynomial.leading_zeros();
    let mut reducible = false;
    'factors: for factor_degree in 1..=degree / 2 {
        let first_monic = 1_u16 << factor_degree;
        let past_last_monic = 1_u16 << (factor_degree + 1);
        for factor in first_monic..past_last_monic {
            let mut remainder = polynomial;
            while remainder != 0 && remainder.leading_zeros() <= factor.leading_zeros() {
                let shift = factor.leading_zeros() - remainder.leading_zeros();
                remainder ^= factor << shift;
            }
            if remainder == 0 {
                reducible = true;
                break 'factors;
            }
        }
    }

    let expected = !reducible;
    if claimed != expected {
        return Err(Gf2PolynomialIrreducibilityError::IrreducibilityMismatch {
            expected,
            actual: claimed,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(polynomial: u16, claim: u8) -> [u8; FRAME_BYTES] {
        let [high, low] = polynomial.to_be_bytes();
        [high, low, claim]
    }

    // Independent small-domain oracle: enumerate nonconstant factor pairs
    // and compare their carryless product, without polynomial division.
    // Degree <= 7 guarantees at least one factor has degree <= 3.
    fn carryless_product(lhs: u16, rhs: u16) -> u32 {
        let mut product = 0_u32;
        for bit in 0..16 {
            if rhs & (1_u16 << bit) != 0 {
                product ^= u32::from(lhs) << bit;
            }
        }
        product
    }

    fn independent_irreducible(polynomial: u16) -> bool {
        for factor in 2_u16..16 {
            for quotient in 2_u16..=255 {
                if carryless_product(factor, quotient) == u32::from(polynomial) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn malformed_constant_and_noncanonical_claims_fail_closed() {
        for polynomial in [0, 1] {
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(polynomial, 0)),
                Err(Gf2PolynomialIrreducibilityError::ConstantPolynomial)
            );
        }
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(3, 1)[..length]),
                Err(Gf2PolynomialIrreducibilityError::InvalidFrame)
            );
        }
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[0, 3, 1, 0]),
            Err(Gf2PolynomialIrreducibilityError::InvalidFrame)
        );
        for claim in [2, 127, 255] {
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(3, claim)),
                Err(Gf2PolynomialIrreducibilityError::InvalidClaim)
            );
        }
    }

    #[test]
    fn exhaustive_small_domain_matches_independent_factor_pair_oracle() {
        for polynomial in 2_u16..=255 {
            let expected = independent_irreducible(polynomial);
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(polynomial, u8::from(expected))),
                Ok(()),
                "polynomial={polynomial:#b}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(polynomial, u8::from(!expected))),
                Err(Gf2PolynomialIrreducibilityError::IrreducibilityMismatch {
                    expected,
                    actual: !expected,
                }),
                "forged polynomial={polynomial:#b}"
            );
        }
    }

    #[test]
    fn degree_one_and_high_degree_boundaries() {
        for (polynomial, expected) in [
            (0x0002, true),  // x
            (0x0003, true),  // x + 1
            (0x0004, false), // x^2
            (0x0005, false), // (x + 1)^2
            (0x011b, true),  // degree-eight irreducible
            (0x011d, true),  // another degree-eight irreducible
            (0x8000, false), // x^15
            (0x8001, false), // x^15 + 1
            (0x8003, true),  // x^15 + x + 1
            (0x8005, false), // x^15 + x^2 + 1
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(polynomial, u8::from(expected))),
                Ok(()),
                "polynomial={polynomial:#x}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_irreducibility(&frame(polynomial, u8::from(!expected))),
                Err(Gf2PolynomialIrreducibilityError::IrreducibilityMismatch {
                    expected,
                    actual: !expected,
                })
            );
        }
    }
}
