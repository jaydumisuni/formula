//! P2 source-only cross-family proof: every reducible u16 GF(2)[x]
//! polynomial has an independently recovered nontrivial factor witness.
//! This does not issue a certificate, Judgement, or promotion authority.

use formula_check::gf2_polynomial_factor_witness::{
    Gf2PolynomialFactorWitnessError, check_frozen_gf2_polynomial_factor_witness,
};
use formula_check::gf2_polynomial_irreducibility::{
    Gf2PolynomialIrreducibilityError, check_frozen_gf2_polynomial_irreducibility,
};

fn factor_frame(polynomial: u16, factor: u16, cofactor: u16) -> [u8; 6] {
    let [p_hi, p_lo] = polynomial.to_be_bytes();
    let [f_hi, f_lo] = factor.to_be_bytes();
    let [c_hi, c_lo] = cofactor.to_be_bytes();
    [p_hi, p_lo, f_hi, f_lo, c_hi, c_lo]
}

// Independent polynomial long division recovers the quotient as well as the
// remainder; the production factor checker instead recomputes a carryless
// product. All coefficients are bounded to the u16 polynomial domain.
fn quotient_if_divides(polynomial: u16, factor: u16) -> Option<u16> {
    let factor_degree = 15 - factor.leading_zeros();
    let mut remainder = u32::from(polynomial);
    let mut quotient = 0_u32;
    while remainder != 0 {
        let remainder_degree = 31 - remainder.leading_zeros();
        if remainder_degree < factor_degree {
            break;
        }
        let shift = remainder_degree - factor_degree;
        quotient ^= 1_u32 << shift;
        remainder ^= u32::from(factor) << shift;
    }
    (remainder == 0).then_some(quotient as u16)
}

fn first_nontrivial_factor(polynomial: u16) -> Option<(u16, u16)> {
    let degree = 15 - polynomial.leading_zeros();
    for factor_degree in 1..=degree / 2 {
        let first_monic = 1_u16 << factor_degree;
        let past_last_monic = 1_u16 << (factor_degree + 1);
        for factor in first_monic..past_last_monic {
            if let Some(cofactor) = quotient_if_divides(polynomial, factor) {
                assert!(
                    cofactor >= 2,
                    "nontrivial cofactor polynomial={polynomial:#x}"
                );
                return Some((factor, cofactor));
            }
        }
    }
    None
}

#[test]
fn full_u16_reducibility_claims_have_independent_factor_witnesses() {
    let mut irreducibles = 0_usize;
    let mut accepted_witnesses = 0_usize;
    let mut rejected_rehashed_witnesses = 0_usize;
    for polynomial in 2_u16..=u16::MAX {
        let witness = first_nontrivial_factor(polynomial);
        let irreducible = witness.is_none();
        let [high, low] = polynomial.to_be_bytes();
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[high, low, u8::from(irreducible)]),
            Ok(()),
            "factor-derived classification polynomial={polynomial:#x}"
        );
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[high, low, u8::from(!irreducible)]),
            Err(Gf2PolynomialIrreducibilityError::IrreducibilityMismatch {
                expected: irreducible,
                actual: !irreducible,
            }),
            "forged classification polynomial={polynomial:#x}"
        );
        if let Some((factor, cofactor)) = witness {
            assert_eq!(
                check_frozen_gf2_polynomial_factor_witness(&factor_frame(
                    polynomial, factor, cofactor,
                )),
                Ok(()),
                "valid factor witness polynomial={polynomial:#x}"
            );
            accepted_witnesses += 1;
            assert_eq!(
                check_frozen_gf2_polynomial_factor_witness(&factor_frame(
                    polynomial ^ 1,
                    factor,
                    cofactor,
                )),
                Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
                    expected: polynomial ^ 1,
                    actual: u32::from(polynomial),
                }),
                "rehashed factor forgery polynomial={polynomial:#x}"
            );
            rejected_rehashed_witnesses += 1;
        } else {
            irreducibles += 1;
        }
    }
    assert_eq!(irreducibles, 4_720);
    assert_eq!(accepted_witnesses, 60_814);
    assert_eq!(rejected_rehashed_witnesses, accepted_witnesses);
}
