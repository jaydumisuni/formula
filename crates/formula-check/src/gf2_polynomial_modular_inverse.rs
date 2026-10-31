//! Independent bounded GF(2)[x] modular-inverse witness preflight.
//! Source-only P2 preparation: no certificate, Judgement, or promotion.
//!
//! Frozen v0 frame: three big-endian u16 bitsets (modulus, element, claimed
//! inverse). The modulus has positive degree; both operands must be reduced.
//! Invertibility is checked by recomputing element * inverse mod modulus = 1.

const FRAME_BYTES: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialModularInverseError {
    InvalidFrame,
    ConstantModulus,
    UnreducedOperand,
    NotInverse { remainder: u16 },
}

/// Independently check a claimed inverse in the quotient GF(2)[x]/(modulus).
///
/// The modulus need not be irreducible: units can exist in a reducible
/// quotient too. This only checks one frozen algebraic relation, never the
/// trust or provenance of a candidate mathematical certificate.
pub fn check_frozen_gf2_polynomial_modular_inverse(
    body: &[u8],
) -> Result<(), Gf2PolynomialModularInverseError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialModularInverseError::InvalidFrame)?;
    let modulus = u16::from_be_bytes([bytes[0], bytes[1]]);
    let element = u16::from_be_bytes([bytes[2], bytes[3]]);
    let claimed_inverse = u16::from_be_bytes([bytes[4], bytes[5]]);
    if modulus < 2 {
        return Err(Gf2PolynomialModularInverseError::ConstantModulus);
    }
    if (element != 0 && element.leading_zeros() <= modulus.leading_zeros())
        || (claimed_inverse != 0 && claimed_inverse.leading_zeros() <= modulus.leading_zeros())
    {
        return Err(Gf2PolynomialModularInverseError::UnreducedOperand);
    }

    // A pair of degree-at-most-14 polynomials has product degree <= 28.
    // Use a u32 for the entire carryless product: never truncate to u16.
    let mut product = 0_u32;
    for bit in 0..16 {
        if claimed_inverse & (1_u16 << bit) != 0 {
            product ^= u32::from(element) << bit;
        }
    }
    let divisor = u32::from(modulus);
    while product != 0 && product.leading_zeros() <= divisor.leading_zeros() {
        let shift = divisor.leading_zeros() - product.leading_zeros();
        product ^= divisor << shift;
    }
    // Reduced remainder degree < modulus degree <= 15, hence fits u16.
    let remainder = product as u16;
    if remainder != 1 {
        return Err(Gf2PolynomialModularInverseError::NotInverse { remainder });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(modulus: u16, element: u16, inverse: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        for (index, value) in [modulus, element, inverse].into_iter().enumerate() {
            bytes[2 * index..2 * index + 2].copy_from_slice(&value.to_be_bytes());
        }
        bytes
    }

    // Independent coefficient convolution, rather than the production
    // shift/XOR accumulation, for the small-domain brute-force oracle.
    fn convolution(lhs: u16, rhs: u16) -> u32 {
        let mut result = 0_u32;
        for degree in 0..31 {
            let mut parity = 0_u32;
            for i in 0..16 {
                if degree >= i && degree - i < 16 {
                    parity ^= u32::from((lhs >> i) & 1) & u32::from((rhs >> (degree - i)) & 1);
                }
            }
            result |= parity << degree;
        }
        result
    }

    fn independent_inverse(modulus: u16, element: u16, candidate: u16) -> bool {
        let product = convolution(element, candidate);
        // For modulus <= degree 4 and reduced operands, the quotient
        // degree is <= 3; 0..=255 is a deliberately generous bound.
        (0_u16..=255).any(|quotient| product ^ convolution(modulus, quotient) == 1)
    }

    #[test]
    fn exhaustive_small_quotients_match_independent_quotient_enumeration() {
        for modulus in 2_u16..=31 {
            let limit = 1_u16 << (15 - modulus.leading_zeros());
            for element in 0..limit {
                for candidate in 0..limit {
                    let actual = check_frozen_gf2_polynomial_modular_inverse(&frame(
                        modulus, element, candidate,
                    ));
                    let expected = independent_inverse(modulus, element, candidate);
                    assert_eq!(
                        actual.is_ok(),
                        expected,
                        "modulus={modulus:#b} element={element:#b} candidate={candidate:#b}"
                    );
                }
            }
        }
    }

    #[test]
    fn exact_frames_and_noncanonical_operands_fail_closed() {
        let valid = frame(0b1011, 0b10, 0b101);
        assert_eq!(check_frozen_gf2_polynomial_modular_inverse(&valid), Ok(()));
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_inverse(&valid[..length]),
                Err(Gf2PolynomialModularInverseError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_modular_inverse(&padded),
            Err(Gf2PolynomialModularInverseError::InvalidFrame)
        );
        for modulus in [0, 1] {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_inverse(&frame(modulus, 1, 1)),
                Err(Gf2PolynomialModularInverseError::ConstantModulus)
            );
        }
        for (element, inverse) in [(0b1000, 1), (1, 0b1000), (0b1011, 1)] {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_inverse(&frame(0b1011, element, inverse,)),
                Err(Gf2PolynomialModularInverseError::UnreducedOperand)
            );
        }
    }

    #[test]
    fn zero_nonunits_and_high_degree_witnesses() {
        assert_eq!(
            check_frozen_gf2_polynomial_modular_inverse(&frame(0b100, 0b10, 1)),
            Err(Gf2PolynomialModularInverseError::NotInverse { remainder: 2 })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_inverse(&frame(0b1011, 0, 0)),
            Err(Gf2PolynomialModularInverseError::NotInverse { remainder: 0 })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_inverse(&frame(0x8000, 1, 1)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_inverse(&frame(0x8000, 2, 0x4000)),
            Err(Gf2PolynomialModularInverseError::NotInverse { remainder: 0 })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_inverse(&frame(0b1011, 0b10, 0b11)),
            Err(Gf2PolynomialModularInverseError::NotInverse { remainder: 6 })
        );
    }
}
