//! P2 independent cross-family zero-operand boundary regression.
//! A valid Bezout identity with one zero operand does not manufacture an
//! inverse of zero in the quotient. This is not certificate or trust admission.

use formula_check::gf2_polynomial_bezout::{
    Gf2PolynomialBezoutError, check_frozen_gf2_polynomial_bezout,
};
use formula_check::gf2_polynomial_modular_inverse::{
    Gf2PolynomialModularInverseError, check_frozen_gf2_polynomial_modular_inverse,
};

fn frame<const N: usize>(values: [u16; N]) -> Vec<u8> {
    values.into_iter().flat_map(u16::to_be_bytes).collect()
}

#[test]
fn zero_operand_bezout_identity_cannot_be_reinterpreted_as_a_modular_unit() {
    let mut valid_bezout_orientations = 0_u32;
    let mut rejected_zero_inverses = 0_u32;

    // Includes reducible, irreducible, and full-width frozen moduli.
    // Expectations follow directly from 0*s + m*1 = m over GF(2)[x];
    // they do not use either production checker as an oracle.
    for modulus in [0b10_u16, 0b11, 0b100, 0b1011, 0x8000, 0x8001, 0xffff] {
        for free_coefficient in [0_u16, 1, 0x8000, 0xffff] {
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame([
                    0,
                    modulus,
                    modulus,
                    free_coefficient,
                    1,
                ])),
                Ok(())
            );
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame([
                    modulus,
                    0,
                    modulus,
                    1,
                    free_coefficient,
                ])),
                Ok(())
            );
            // No choice of the coefficient multiplying zero changes the GCD.
            for forged in [
                [0, modulus, 1, free_coefficient, 1],
                [modulus, 0, 1, 1, free_coefficient],
            ] {
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(forged)),
                    Err(Gf2PolynomialBezoutError::GcdMismatch {
                        expected: modulus,
                        actual: 1,
                    })
                );
            }
            valid_bezout_orientations += 2;
        }

        // The zero residue times any reduced candidate remains zero.
        for candidate in [0_u16, 1, modulus >> 1] {
            assert_eq!(
                check_frozen_gf2_polynomial_modular_inverse(&frame([modulus, 0, candidate,])),
                Err(Gf2PolynomialModularInverseError::NotInverse { remainder: 0 })
            );
            rejected_zero_inverses += 1;
        }
    }

    assert_eq!(valid_bezout_orientations, 56);
    assert_eq!(rejected_zero_inverses, 21);
    assert_eq!(
        check_frozen_gf2_polynomial_bezout(&frame([0, 0, 0, 0, 0])),
        Err(Gf2PolynomialBezoutError::BothZero)
    );
    assert_eq!(
        check_frozen_gf2_polynomial_modular_inverse(&frame([1, 0, 0])),
        Err(Gf2PolynomialModularInverseError::ConstantModulus)
    );
}
