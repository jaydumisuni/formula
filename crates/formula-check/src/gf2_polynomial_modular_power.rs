//! Independent bounded GF(2)[x] modular-power witness preflight.
//! Source-only P2 preparation: no certificate, Judgement, or promotion.
//!
//! Frozen v0 frame: four big-endian u16 bitsets (modulus, base, exponent,
//! claimed residue). The modulus must have positive degree; the base and
//! claimed residue must be reduced modulo it. Exponent zero is valid.

const FRAME_BYTES: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialModularPowerError {
    InvalidFrame,
    ConstantModulus,
    UnreducedOperand,
    NotPower { computed: u16 },
}

/// Independently check a bounded modular-power relation over GF(2)[x].
///
/// The modulus need not be irreducible. The exponent is a frozen u16, and
/// repeated squaring is bounded to at most sixteen iterations. This checks
/// only an exact algebraic relation, not certificate provenance or authority.
pub fn check_frozen_gf2_polynomial_modular_power(
    body: &[u8],
) -> Result<(), Gf2PolynomialModularPowerError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialModularPowerError::InvalidFrame)?;
    let modulus = u16::from_be_bytes([bytes[0], bytes[1]]);
    let base = u16::from_be_bytes([bytes[2], bytes[3]]);
    let mut exponent = u16::from_be_bytes([bytes[4], bytes[5]]);
    let claimed = u16::from_be_bytes([bytes[6], bytes[7]]);

    if modulus < 2 {
        return Err(Gf2PolynomialModularPowerError::ConstantModulus);
    }
    if (base != 0 && base.leading_zeros() <= modulus.leading_zeros())
        || (claimed != 0 && claimed.leading_zeros() <= modulus.leading_zeros())
    {
        return Err(Gf2PolynomialModularPowerError::UnreducedOperand);
    }

    // The unit polynomial is reduced for every positive-degree modulus.
    let mut result = 1_u16;
    let mut power = base;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = multiply_mod(result, power, modulus);
        }
        exponent >>= 1;
        if exponent != 0 {
            power = multiply_mod(power, power, modulus);
        }
    }
    if claimed != result {
        return Err(Gf2PolynomialModularPowerError::NotPower { computed: result });
    }
    Ok(())
}

fn multiply_mod(lhs: u16, rhs: u16, modulus: u16) -> u16 {
    // Two reduced u16 polynomials have product degree at most 28.
    // Carryless multiplication must not truncate intermediate coefficients.
    let mut product = 0_u32;
    for bit in 0..16 {
        if rhs & (1_u16 << bit) != 0 {
            product ^= u32::from(lhs) << bit;
        }
    }
    let divisor = u32::from(modulus);
    while product != 0 && product.leading_zeros() <= divisor.leading_zeros() {
        let shift = divisor.leading_zeros() - product.leading_zeros();
        product ^= divisor << shift;
    }
    product as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(modulus: u16, base: u16, exponent: u16, claimed: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        for (index, value) in [modulus, base, exponent, claimed].into_iter().enumerate() {
            bytes[2 * index..2 * index + 2].copy_from_slice(&value.to_be_bytes());
        }
        bytes
    }

    // Independent bit-coefficient convolution and descending coefficient
    // elimination, not the production shift/XOR multiplication routine.
    fn reference_multiply(lhs: u16, rhs: u16, modulus: u16) -> u16 {
        let mut coefficients = [false; 31];
        for i in 0..16 {
            for j in 0..16 {
                if lhs & (1_u16 << i) != 0 && rhs & (1_u16 << j) != 0 {
                    coefficients[i + j] = !coefficients[i + j];
                }
            }
        }
        let degree = (15 - modulus.leading_zeros()) as usize;
        for position in (degree..=30).rev() {
            if coefficients[position] {
                for offset in 0..=degree {
                    if modulus & (1_u16 << offset) != 0 {
                        let index = position - degree + offset;
                        coefficients[index] = !coefficients[index];
                    }
                }
            }
        }
        let mut reduced = 0_u16;
        for (degree, coefficient) in coefficients.iter().enumerate().take(16) {
            if *coefficient {
                reduced |= 1_u16 << degree;
            }
        }
        reduced
    }

    fn reference_power(base: u16, exponent: u16, modulus: u16) -> u16 {
        let mut result = 1_u16;
        for _ in 0..exponent {
            result = reference_multiply(result, base, modulus);
        }
        result
    }

    #[test]
    fn small_quotients_match_independent_repeated_multiplication() {
        for modulus in 2_u16..=31 {
            let limit = 1_u16 << (15 - modulus.leading_zeros());
            for base in 0..limit {
                for exponent in 0..=20 {
                    let expected = reference_power(base, exponent, modulus);
                    assert_eq!(
                        check_frozen_gf2_polynomial_modular_power(&frame(
                            modulus, base, exponent, expected
                        )),
                        Ok(()),
                        "modulus={modulus:#b} base={base:#b} exponent={exponent}"
                    );
                    assert_eq!(
                        check_frozen_gf2_polynomial_modular_power(&frame(
                            modulus,
                            base,
                            exponent,
                            expected ^ 1
                        )),
                        Err(Gf2PolynomialModularPowerError::NotPower { computed: expected }),
                        "forged modulus={modulus:#b} base={base:#b} exponent={exponent}"
                    );
                }
            }
        }
    }

    #[test]
    fn exact_frame_and_reduced_operand_boundaries() {
        let valid = frame(0b1011, 0b10, 3, 0b11);
        assert_eq!(check_frozen_gf2_polynomial_modular_power(&valid), Ok(()));
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_power(&valid[..length]),
                Err(Gf2PolynomialModularPowerError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_modular_power(&padded),
            Err(Gf2PolynomialModularPowerError::InvalidFrame)
        );
        for modulus in [0, 1] {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_power(&frame(modulus, 1, 2, 1)),
                Err(Gf2PolynomialModularPowerError::ConstantModulus)
            );
        }
        for (base, claimed) in [(0b1000, 1), (1, 0b1000), (0b1011, 1)] {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_power(&frame(0b1011, base, 1, claimed)),
                Err(Gf2PolynomialModularPowerError::UnreducedOperand)
            );
        }
    }

    #[test]
    fn exponent_zero_and_high_degree_cases() {
        assert_eq!(
            check_frozen_gf2_polynomial_modular_power(&frame(0x8000, 0, 0, 1)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_power(&frame(0x8000, 1, u16::MAX, 1)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_power(&frame(0x8000, 0x7ffe, u16::MAX, 0)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_power(&frame(0b100, 0b10, 2, 0)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_modular_power(&frame(0b1011, 0b10, 3, 0b10)),
            Err(Gf2PolynomialModularPowerError::NotPower { computed: 0b11 })
        );
    }
}
