//! Independent bounded GF(2)[x] square-free witness preflight on frozen bytes.
//! Non-authoritative P2 preparation: no Judgement, certificate, or promotion.

// Wire v0: big-endian u16 polynomial, followed by exactly one boolean byte.
// Bit i is the coefficient of x^i. Zero is excluded because it is divisible
// by every nonzero polynomial and has no square-free factorization.
const FRAME_BYTES: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialSquareFreeError {
    InvalidFrame,
    ZeroPolynomial,
    InvalidClaim,
    SquareFreeMismatch { expected: bool, actual: bool },
}

/// Independently recompute whether a frozen GF(2) polynomial is square-free.
///
/// In characteristic two, the formal derivative retains exactly the odd
/// exponents. A nonzero polynomial is square-free iff gcd(f, f') = 1.
/// Euclidean remainders strictly decrease degree; all values fit in u16.
pub fn check_frozen_gf2_polynomial_squarefree(
    body: &[u8],
) -> Result<(), Gf2PolynomialSquareFreeError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialSquareFreeError::InvalidFrame)?;
    let polynomial = u16::from_be_bytes([bytes[0], bytes[1]]);
    if polynomial == 0 {
        return Err(Gf2PolynomialSquareFreeError::ZeroPolynomial);
    }
    let claimed = match bytes[2] {
        0 => false,
        1 => true,
        _ => return Err(Gf2PolynomialSquareFreeError::InvalidClaim),
    };

    let mut derivative = 0_u16;
    for exponent in (1..16).step_by(2) {
        if polynomial & (1_u16 << exponent) != 0 {
            derivative |= 1_u16 << (exponent - 1);
        }
    }

    let mut a = polynomial;
    let mut b = derivative;
    while b != 0 {
        let mut remainder = a;
        while remainder != 0 && remainder.leading_zeros() <= b.leading_zeros() {
            let shift = b.leading_zeros() - remainder.leading_zeros();
            remainder ^= b << shift;
        }
        a = b;
        b = remainder;
    }
    let expected = a == 1;
    if claimed != expected {
        return Err(Gf2PolynomialSquareFreeError::SquareFreeMismatch {
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

    // Independent small-domain oracle: enumerate all possible nonconstant
    // square divisors and their quotients, never using a derivative or GCD.
    // For degree <= 7, any repeated factor has degree <= 3.
    fn carryless_product(lhs: u16, rhs: u16) -> u32 {
        let mut result = 0_u32;
        for bit in 0..16 {
            if rhs & (1_u16 << bit) != 0 {
                result ^= u32::from(lhs) << bit;
            }
        }
        result
    }

    fn independent_squarefree(polynomial: u16) -> bool {
        for divisor in 2_u16..16 {
            let square = carryless_product(divisor, divisor) as u16;
            for quotient in 0_u16..256 {
                if carryless_product(square, quotient) == u32::from(polynomial) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn malformed_zero_and_noncanonical_claims_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_squarefree(&frame(0, 0)),
            Err(Gf2PolynomialSquareFreeError::ZeroPolynomial)
        );
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_squarefree(&frame(3, 1)[..length]),
                Err(Gf2PolynomialSquareFreeError::InvalidFrame)
            );
        }
        assert_eq!(
            check_frozen_gf2_polynomial_squarefree(&[0, 3, 1, 0]),
            Err(Gf2PolynomialSquareFreeError::InvalidFrame)
        );
        for claim in [2, 127, 255] {
            assert_eq!(
                check_frozen_gf2_polynomial_squarefree(&frame(3, claim)),
                Err(Gf2PolynomialSquareFreeError::InvalidClaim)
            );
        }
    }

    #[test]
    fn exhaustive_small_domain_matches_independent_square_divisor_enumeration() {
        for polynomial in 1_u16..=255 {
            let expected = independent_squarefree(polynomial);
            assert_eq!(
                check_frozen_gf2_polynomial_squarefree(&frame(polynomial, u8::from(expected))),
                Ok(()),
                "polynomial={polynomial:#b}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_squarefree(&frame(polynomial, u8::from(!expected))),
                Err(Gf2PolynomialSquareFreeError::SquareFreeMismatch {
                    expected,
                    actual: !expected,
                }),
                "forged polynomial={polynomial:#b}"
            );
        }
    }

    #[test]
    fn high_degree_derivative_and_constant_boundaries() {
        for (polynomial, expected) in [
            (0x0001, true),  // unit polynomial
            (0x0002, true),  // x
            (0x0003, true),  // x + 1
            (0x0004, false), // x^2
            (0x0005, false), // (x + 1)^2
            (0x4001, false), // x^14 + 1, zero formal derivative
            (0x8000, false), // x^15, repeated x factor
            (0x8001, true),  // x^15 + 1, coprime to x^14
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_squarefree(&frame(polynomial, u8::from(expected))),
                Ok(()),
                "polynomial={polynomial:#x}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_squarefree(&frame(polynomial, u8::from(!expected))),
                Err(Gf2PolynomialSquareFreeError::SquareFreeMismatch {
                    expected,
                    actual: !expected,
                })
            );
        }
    }
}
