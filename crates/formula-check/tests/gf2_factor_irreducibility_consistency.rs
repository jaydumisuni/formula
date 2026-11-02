//! P2 source-only cross-family evidence: independent factor enumeration must
//! agree with both frozen irreducibility and frozen factor-witness preflights.
//! No certificate, Judgement, or promotion authority is granted.

use formula_check::gf2_polynomial_factor_witness::{
    Gf2PolynomialFactorWitnessError, check_frozen_gf2_polynomial_factor_witness,
};
use formula_check::gf2_polynomial_irreducibility::{
    Gf2PolynomialIrreducibilityError, check_frozen_gf2_polynomial_irreducibility,
};

fn factor_frame(polynomial: u16, factor: u16, cofactor: u16) -> [u8; 6] {
    let mut bytes = [0_u8; 6];
    for (i, value) in [polynomial, factor, cofactor].into_iter().enumerate() {
        bytes[i * 2..i * 2 + 2].copy_from_slice(&value.to_be_bytes());
    }
    bytes
}

// A coefficient-pair parity oracle, deliberately not the shift/XOR
// accumulation in the production factor-witness checker.
fn independent_product(lhs: u16, rhs: u16) -> u32 {
    let mut result = 0_u32;
    for degree in 0..=30 {
        let mut parity = 0_u32;
        for lhs_degree in 0..=15 {
            if degree >= lhs_degree && degree - lhs_degree <= 15 {
                parity ^= u32::from((lhs >> lhs_degree) & 1)
                    & u32::from((rhs >> (degree - lhs_degree)) & 1);
            }
        }
        result |= parity << degree;
    }
    result
}

#[test]
fn exhaustive_small_polynomials_bind_factor_and_irreducibility_claims() {
    // All factors of a degree <= 7 polynomial have degree <= 6.
    // Build the independent factor relation before consulting either checker.
    let mut witnesses = vec![Vec::<(u16, u16)>::new(); 256];
    for factor in 2_u16..=127 {
        for cofactor in 2_u16..=127 {
            let product = independent_product(factor, cofactor);
            if product <= 255 {
                witnesses[product as usize].push((factor, cofactor));
            }
        }
    }

    let mut irreducible_count = 0;
    let mut reducible_count = 0;
    let mut accepted_witnesses = 0;
    let mut rejected_forgeries = 0;
    for polynomial in 2_u16..=255 {
        let factor_pairs = &witnesses[polynomial as usize];
        let irreducible = factor_pairs.is_empty();
        let [hi, lo] = polynomial.to_be_bytes();
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[hi, lo, u8::from(irreducible)]),
            Ok(()),
            "independent factor classification polynomial={polynomial:#x}"
        );
        assert_eq!(
            check_frozen_gf2_polynomial_irreducibility(&[hi, lo, u8::from(!irreducible)]),
            Err(Gf2PolynomialIrreducibilityError::IrreducibilityMismatch {
                expected: irreducible,
                actual: !irreducible,
            })
        );
        if irreducible {
            irreducible_count += 1;
            // x*x is a forged factorization for every irreducible input.
            assert_eq!(
                check_frozen_gf2_polynomial_factor_witness(&factor_frame(polynomial, 2, 2)),
                Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
                    expected: polynomial,
                    actual: 4,
                })
            );
            rejected_forgeries += 1;
        } else {
            reducible_count += 1;
            for &(factor, cofactor) in factor_pairs {
                assert_eq!(
                    check_frozen_gf2_polynomial_factor_witness(&factor_frame(
                        polynomial, factor, cofactor
                    )),
                    Ok(())
                );
                accepted_witnesses += 1;
                // The same valid factors cannot justify a rehashed polynomial.
                assert_eq!(
                    check_frozen_gf2_polynomial_factor_witness(&factor_frame(
                        polynomial ^ 1,
                        factor,
                        cofactor
                    )),
                    Err(Gf2PolynomialFactorWitnessError::ProductMismatch {
                        expected: polynomial ^ 1,
                        actual: u32::from(polynomial),
                    })
                );
                rejected_forgeries += 1;
            }
        }
    }
    assert_eq!(irreducible_count, 41);
    assert_eq!(reducible_count, 213);
    assert_eq!(accepted_witnesses, 1_284);
    assert_eq!(rejected_forgeries, accepted_witnesses + irreducible_count);
}
