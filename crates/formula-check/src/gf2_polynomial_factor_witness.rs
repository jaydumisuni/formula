//! Independent bounded GF(2)[x] nontrivial-factor witness preflight.
//! This is P2 source preparation only, not certificate/Judgement authority.
//!
//! Frozen v0 frame: three big-endian u16 coefficient bitsets: polynomial,
//! nonconstant factor, nonconstant cofactor. Bit i represents x^i.
//! A successful check witnesses reducibility, never irreducibility.

const FRAME_BYTES: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialFactorWitnessError {
    InvalidFrame,
    ConstantPolynomial,
    TrivialFactor,
    ProductMismatch { expected: u16, actual: u32 },
}

/// Recompute a nontrivial factorization from frozen bytes, without accepting
/// a producer's derived polynomial or truncating intermediate coefficients.
/// The product of two u16 polynomials occupies at most 31 bits, so u32
/// carryless multiplication is exact. Both factors must have degree >= 1.
pub fn check_frozen_gf2_polynomial_factor_witness(
    body: &[u8],
) -> Result<(), Gf2PolynomialFactorWitnessError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialFactorWitnessError::InvalidFrame)?;
    let polynomial = u16::from_be_bytes([bytes[0], bytes[1]]);
    let factor = u16::from_be_bytes([bytes[2], bytes[3]]);
    let cofactor = u16::from_be_bytes([bytes[4], bytes[5]]);
    if polynomial < 2 {
        return Err(Gf2PolynomialFactorWitnessError::ConstantPolynomial);
    }
    if factor < 2 || cofactor < 2 {
        return Err(Gf2PolynomialFactorWitnessError::TrivialFactor);
    }

    let mut actual = 0_u32;
    for bit in 0..16 {
        if cofactor & (1_u16 << bit) != 0 {
            actual ^= u32::from(factor) << bit;
        }
    }
    if actual != u32::from(polynomial) {
        return Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
            expected: polynomial,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(polynomial: u16, factor: u16, cofactor: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0; FRAME_BYTES];
        bytes[0..2].copy_from_slice(&polynomial.to_be_bytes());
        bytes[2..4].copy_from_slice(&factor.to_be_bytes());
        bytes[4..6].copy_from_slice(&cofactor.to_be_bytes());
        bytes
    }

    // Independent coefficient-convolution oracle; deliberately avoids
    // the checker's shift-and-XOR accumulation algorithm.
    fn convolution(lhs: u16, rhs: u16) -> u32 {
        let mut product = 0_u32;
        for degree in 0..31 {
            let mut parity = 0_u32;
            for i in 0..16 {
                if degree >= i && degree - i < 16 {
                    parity ^= u32::from((lhs >> i) & 1) & u32::from((rhs >> (degree - i)) & 1);
                }
            }
            product |= parity << degree;
        }
        product
    }

    #[test]
    fn malformed_and_trivial_claims_fail_closed() {
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_factor_witness(&frame(6, 2, 3)[..length]),
                Err(Gf2PolynomialFactorWitnessError::InvalidFrame)
            );
        }
        assert_eq!(
            check_frozen_gf2_polynomial_factor_witness(&[0, 6, 0, 2, 0, 3, 0]),
            Err(Gf2PolynomialFactorWitnessError::InvalidFrame)
        );
        for polynomial in [0, 1] {
            assert_eq!(
                check_frozen_gf2_polynomial_factor_witness(&frame(polynomial, 2, 3)),
                Err(Gf2PolynomialFactorWitnessError::ConstantPolynomial)
            );
        }
        for (factor, cofactor) in [(0, 3), (1, 3), (3, 0), (3, 1)] {
            assert_eq!(
                check_frozen_gf2_polynomial_factor_witness(&frame(6, factor, cofactor)),
                Err(Gf2PolynomialFactorWitnessError::TrivialFactor)
            );
        }
    }

    #[test]
    fn exhaustive_small_factor_pairs_and_forged_product() {
        for factor in 2_u16..=63 {
            for cofactor in 2_u16..=63 {
                let product = convolution(factor, cofactor);
                assert!(product <= u32::from(u16::MAX));
                let polynomial = product as u16;
                assert_eq!(
                    check_frozen_gf2_polynomial_factor_witness(&frame(
                        polynomial, factor, cofactor
                    )),
                    Ok(())
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_factor_witness(&frame(
                        polynomial ^ 1,
                        factor,
                        cofactor
                    )),
                    Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
                        expected: polynomial ^ 1,
                        actual: product,
                    })
                );
            }
        }
    }

    #[test]
    fn high_degree_overflow_and_irreducible_forgery_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_factor_witness(&frame(0x8000, 0x80, 0x100)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_factor_witness(&frame(0x8000, 0x8000, 0x8000)),
            Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
                expected: 0x8000,
                actual: 0x4000_0000,
            })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_factor_witness(&frame(0x11b, 2, 0x8d)),
            Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
                expected: 0x11b,
                actual: 0x11a,
            })
        );
    }
}
