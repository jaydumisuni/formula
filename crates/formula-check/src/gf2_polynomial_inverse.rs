//! Independent bounded GF(2)[x] modular-inverse witness preflight.
//! P2 source-only preparation: this grants no certificate or trust-root authority.

// Wire v0: three big-endian u16 coefficient bitsets (value, modulus, inverse).
// Bit i is the coefficient of x^i. The quotient ring must be nontrivial.
const FRAME_BYTES: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialInverseError {
    InvalidFrame,
    TrivialModulus,
    ZeroOperand,
    NotInverse { remainder: u32 },
}

/// Verify a frozen multiplicative inverse in GF(2)[x]/(modulus).
/// The exact 31-bit carryless product is reduced before comparing with one.
/// This checks a witness, not the provenance of the modulus or any certificate.
pub fn check_frozen_gf2_polynomial_inverse(body: &[u8]) -> Result<(), Gf2PolynomialInverseError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialInverseError::InvalidFrame)?;
    let [value, modulus, inverse] =
        core::array::from_fn(|i| u16::from_be_bytes([bytes[2 * i], bytes[2 * i + 1]]));
    if modulus <= 1 {
        return Err(Gf2PolynomialInverseError::TrivialModulus);
    }
    if value == 0 || inverse == 0 {
        return Err(Gf2PolynomialInverseError::ZeroOperand);
    }

    let mut product = 0_u32;
    for bit in 0..16 {
        if inverse & (1_u16 << bit) != 0 {
            product ^= u32::from(value) << bit;
        }
    }
    let divisor = u32::from(modulus);
    while product != 0 && product.leading_zeros() <= divisor.leading_zeros() {
        product ^= divisor << (divisor.leading_zeros() - product.leading_zeros());
    }
    if product != 1 {
        return Err(Gf2PolynomialInverseError::NotInverse { remainder: product });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(value: u16, modulus: u16, inverse: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        for (i, value) in [value, modulus, inverse].into_iter().enumerate() {
            bytes[2 * i..2 * i + 2].copy_from_slice(&value.to_be_bytes());
        }
        bytes
    }

    fn oracle_product(a: u16, b: u16) -> u32 {
        // Coefficient convolution, independent of the checker's shift product.
        let mut product = 0_u32;
        for i in 0..16 {
            for j in 0..16 {
                if a & (1_u16 << i) != 0 && b & (1_u16 << j) != 0 {
                    product ^= 1_u32 << (i + j);
                }
            }
        }
        product
    }

    fn oracle_remainder(mut product: u32, modulus: u16) -> u32 {
        let degree = (0..16)
            .rev()
            .find(|&i| modulus & (1_u16 << i) != 0)
            .expect("nonzero modulus");
        for bit in (degree..=30).rev() {
            if product & (1_u32 << bit) != 0 {
                product ^= u32::from(modulus) << (bit - degree);
            }
        }
        product
    }

    #[test]
    fn exact_inverse_and_high_degree_reduction() {
        // x*(x+1) = x^2+x = 1 mod (x^2+x+1).
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&frame(0b10, 0b111, 0b11)),
            Ok(())
        );
        // x*(x^14+1) = x^15+x = 1 mod (x^15+x+1).
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&frame(0b10, 0x8003, 0x4001)),
            Ok(())
        );
        // x^15 is congruent to x+1 modulo x^15+x+1. Its inverse is
        // x^14+...+x, yielding a degree-29 product before exact reduction.
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&frame(0x8000, 0x8003, 0x7ffe)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&frame(0x8000, 0x8003, 0x7fff)),
            Err(Gf2PolynomialInverseError::NotInverse { remainder: 0b10 })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&frame(1, 0b10, 1)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&frame(0b11, 0b111, 0b11)),
            Err(Gf2PolynomialInverseError::NotInverse { remainder: 0b10 })
        );
    }

    #[test]
    fn zero_ring_nonunits_and_frame_boundaries_fail_closed() {
        for modulus in [0, 1] {
            assert_eq!(
                check_frozen_gf2_polynomial_inverse(&frame(1, modulus, 1)),
                Err(Gf2PolynomialInverseError::TrivialModulus)
            );
        }
        for (value, inverse) in [(0, 1), (1, 0), (0, 0)] {
            assert_eq!(
                check_frozen_gf2_polynomial_inverse(&frame(value, 0b111, inverse)),
                Err(Gf2PolynomialInverseError::ZeroOperand)
            );
        }
        // x is a zero divisor modulo x^2; no inverse exists.
        assert!(matches!(
            check_frozen_gf2_polynomial_inverse(&frame(0b10, 0b100, 0b11)),
            Err(Gf2PolynomialInverseError::NotInverse { .. })
        ));
        let valid = frame(0b10, 0b111, 0b11);
        for n in 0..valid.len() {
            assert_eq!(
                check_frozen_gf2_polynomial_inverse(&valid[..n]),
                Err(Gf2PolynomialInverseError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_inverse(&padded),
            Err(Gf2PolynomialInverseError::InvalidFrame)
        );
    }

    #[test]
    fn exhaustive_small_witnesses_match_independent_polynomial_oracle() {
        for modulus in 2_u16..32 {
            for value in 0_u16..32 {
                for inverse in 0_u16..32 {
                    let expected = value != 0
                        && inverse != 0
                        && oracle_remainder(oracle_product(value, inverse), modulus) == 1;
                    let actual =
                        check_frozen_gf2_polynomial_inverse(&frame(value, modulus, inverse));
                    assert_eq!(
                        actual.is_ok(),
                        expected,
                        "modulus={modulus:#b} value={value:#b} inverse={inverse:#b}"
                    );
                }
            }
        }
    }
}
