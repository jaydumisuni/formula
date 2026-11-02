//! P2 source-only cross-family regression: a valid Bezout unit witness must
//! induce a modular inverse, while a nonunit cannot be admitted as one.
//! This is not certificate issuance, trust-root admission, or promotion.

use formula_check::gf2_polynomial_bezout::check_frozen_gf2_polynomial_bezout;
use formula_check::gf2_polynomial_modular_inverse::{
    Gf2PolynomialModularInverseError, check_frozen_gf2_polynomial_modular_inverse,
};

fn frame<const N: usize>(values: [u16; N]) -> Vec<u8> {
    values.into_iter().flat_map(u16::to_be_bytes).collect()
}

// Independent u64 polynomial arithmetic. This does not call either checker.
fn product(lhs: u64, mut rhs: u64) -> u64 {
    let mut result = 0_u64;
    let mut shifted = lhs;
    while rhs != 0 {
        if rhs & 1 != 0 {
            result ^= shifted;
        }
        rhs >>= 1;
        shifted <<= 1;
    }
    result
}

fn div_rem(mut numerator: u64, denominator: u64) -> (u64, u64) {
    assert_ne!(denominator, 0);
    let mut quotient = 0_u64;
    while numerator != 0 && numerator.leading_zeros() <= denominator.leading_zeros() {
        let shift = denominator.leading_zeros() - numerator.leading_zeros();
        quotient ^= 1_u64 << shift;
        numerator ^= denominator << shift;
    }
    (quotient, numerator)
}

fn extended_gcd(lhs: u16, rhs: u16) -> (u16, u16, u16) {
    let (mut old_r, mut r) = (u64::from(lhs), u64::from(rhs));
    let (mut old_s, mut s) = (1_u64, 0_u64);
    let (mut old_t, mut t) = (0_u64, 1_u64);
    while r != 0 {
        let (q, remainder) = div_rem(old_r, r);
        let next_s = old_s ^ product(q, s);
        let next_t = old_t ^ product(q, t);
        (old_r, r) = (r, remainder);
        (old_s, s) = (s, next_s);
        (old_t, t) = (t, next_t);
    }
    (
        u16::try_from(old_r).expect("bounded GCD"),
        u16::try_from(old_s).expect("bounded s"),
        u16::try_from(old_t).expect("bounded t"),
    )
}

#[test]
fn independent_bezout_units_and_nonunits_agree_with_modular_inverse_preflight() {
    let mut unit_directions = 0;
    let mut nonunit_directions = 0;
    for (lhs, rhs) in [
        (0x8001_u16, 0x8003_u16),
        (0x8000, 0x8001),
        (0xffff, 0xaaaa),
        (0x00f0, 0x005a),
        (0x0013, 0x001d),
        (0x0036, 0x001e),
        (0x002a, 0x003e),
        (0x0003, 0x0005),
        (0x1234, 0x4321),
        (0x7fff, 0x8000),
        (0x0101, 0x0111),
    ] {
        let (gcd, s, t) = extended_gcd(lhs, rhs);
        assert_eq!(
            product(u64::from(lhs), u64::from(s)) ^ product(u64::from(rhs), u64::from(t)),
            u64::from(gcd),
            "independent Bezout relation lhs={lhs:#x} rhs={rhs:#x}"
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame([lhs, rhs, gcd, s, t])),
            Ok(())
        );

        // Each direction is checked separately: a checker must not ignore
        // either Bezout coefficient or mistake a nonunit for a quotient unit.
        for (modulus, element, coefficient) in [(rhs, lhs, s), (lhs, rhs, t)] {
            let reduced_element = div_rem(u64::from(element), u64::from(modulus)).1 as u16;
            let reduced_coefficient = div_rem(u64::from(coefficient), u64::from(modulus)).1 as u16;
            let remainder = div_rem(
                product(u64::from(reduced_element), u64::from(reduced_coefficient)),
                u64::from(modulus),
            )
            .1 as u16;
            let actual = check_frozen_gf2_polynomial_modular_inverse(&frame([
                modulus,
                reduced_element,
                reduced_coefficient,
            ]));
            if gcd == 1 {
                unit_directions += 1;
                assert_eq!(remainder, 1);
                assert_eq!(actual, Ok(()), "unit modulus={modulus:#x}");
                // Flipping one coefficient bit destroys a true inverse.
                let forged = reduced_coefficient ^ 1;
                let forged_remainder = div_rem(
                    product(u64::from(reduced_element), u64::from(forged)),
                    u64::from(modulus),
                )
                .1 as u16;
                assert_ne!(forged_remainder, 1);
                assert_eq!(
                    check_frozen_gf2_polynomial_modular_inverse(&frame([
                        modulus,
                        reduced_element,
                        forged,
                    ])),
                    Err(Gf2PolynomialModularInverseError::NotInverse {
                        remainder: forged_remainder
                    })
                );
            } else {
                nonunit_directions += 1;
                assert_ne!(remainder, 1);
                assert_eq!(
                    actual,
                    Err(Gf2PolynomialModularInverseError::NotInverse { remainder }),
                    "nonunit modulus={modulus:#x}"
                );
            }
        }
    }
    assert!(unit_directions >= 8, "unit coverage: {unit_directions}");
    assert!(
        nonunit_directions >= 4,
        "nonunit coverage: {nonunit_directions}"
    );
}

// A Bezout coefficient pair is not unique: adding (q * rhs, q * lhs)
// preserves the exact identity, while each coefficient's residue class
// modulo the opposite operand stays fixed. Independently exercise that
// equivalence against the modular-inverse checker, including nonunits.
// This is source-only checker preparation, not a certificate or promotion.
#[test]
fn small_domain_syzygy_representatives_preserve_inverse_classes() {
    let mut unit_directions = 0_u32;
    let mut nonunit_directions = 0_u32;
    let mut one_sided_s_rejections = 0_u32;
    let mut one_sided_t_rejections = 0_u32;
    for lhs in 2_u16..=31 {
        for rhs in 2_u16..=31 {
            let (gcd, s, t) = extended_gcd(lhs, rhs);
            for q in [1_u64, 2, 3] {
                let shifted_s = u16::try_from(u64::from(s) ^ product(u64::from(rhs), q))
                    .expect("bounded small-domain syzygy coefficient");
                let shifted_t = u16::try_from(u64::from(t) ^ product(u64::from(lhs), q))
                    .expect("bounded small-domain syzygy coefficient");
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame([
                        lhs, rhs, gcd, shifted_s, shifted_t,
                    ])),
                    Ok(()),
                    "valid syzygy lhs={lhs:#b} rhs={rhs:#b} q={q:#b}"
                );

                // A one-sided change by rhs keeps s's quotient residue
                // unchanged but breaks the full Bezout identity. Inverse
                // equivalence must never be treated as Bezout authority.
                let one_sided_s = shifted_s ^ rhs;
                assert!(
                    check_frozen_gf2_polynomial_bezout(&frame([
                        lhs,
                        rhs,
                        gcd,
                        one_sided_s,
                        shifted_t,
                    ]))
                    .is_err(),
                    "one-sided forged Bezout lhs={lhs:#b} rhs={rhs:#b}"
                );
                one_sided_s_rejections += 1;

                // The symmetric forgery also preserves t's quotient class
                // modulo lhs, but changes the full Bezout relation. Check
                // both coefficients independently before inverse-class tests.
                let one_sided_t = shifted_t ^ lhs;
                assert!(
                    check_frozen_gf2_polynomial_bezout(&frame([
                        lhs,
                        rhs,
                        gcd,
                        shifted_s,
                        one_sided_t,
                    ]))
                    .is_err(),
                    "one-sided forged t lhs={lhs:#b} rhs={rhs:#b}"
                );
                one_sided_t_rejections += 1;

                for (modulus, element, original, shifted) in
                    [(rhs, lhs, s, shifted_s), (lhs, rhs, t, shifted_t)]
                {
                    let reduced_element = div_rem(u64::from(element), u64::from(modulus)).1 as u16;
                    let original_residue =
                        div_rem(u64::from(original), u64::from(modulus)).1 as u16;
                    let shifted_residue = div_rem(u64::from(shifted), u64::from(modulus)).1 as u16;
                    assert_eq!(
                        shifted_residue, original_residue,
                        "inverse class modulus={modulus:#b} q={q:#b}"
                    );
                    let actual = check_frozen_gf2_polynomial_modular_inverse(&frame([
                        modulus,
                        reduced_element,
                        shifted_residue,
                    ]));
                    if gcd == 1 {
                        unit_directions += 1;
                        assert_eq!(actual, Ok(()));
                    } else {
                        nonunit_directions += 1;
                        assert!(
                            matches!(
                                actual,
                                Err(Gf2PolynomialModularInverseError::NotInverse { .. })
                            ),
                            "nonunit modulus={modulus:#b} element={element:#b}"
                        );
                    }
                }
                assert_eq!(
                    div_rem(u64::from(one_sided_s), u64::from(rhs)).1,
                    div_rem(u64::from(shifted_s), u64::from(rhs)).1
                );
                assert_eq!(
                    div_rem(u64::from(one_sided_t), u64::from(lhs)).1,
                    div_rem(u64::from(shifted_t), u64::from(lhs)).1
                );
            }
        }
    }
    assert!(unit_directions > 1000, "unit directions: {unit_directions}");
    assert!(
        nonunit_directions > 1000,
        "nonunit directions: {nonunit_directions}"
    );
    assert_eq!(one_sided_s_rejections, 2700);
    assert_eq!(one_sided_t_rejections, 2700);
}

// An untrusted producer must not turn a genuine nonunit GCD into unit
// authority by choosing alternate bounded Bezout coefficients. Independently
// exhaust the coefficient domain, rather than trusting a checker's rejection
// alone. This is P2 source-only negative evidence, not certification.
#[test]
fn bounded_nonunit_pairs_reject_all_forged_unit_bezout_coefficients() {
    let mut nonunit_pairs = 0_u32;
    let mut rejected_claims = 0_u32;
    for lhs in 2_u16..=15 {
        for rhs in 2_u16..=15 {
            let (gcd, s, t) = extended_gcd(lhs, rhs);
            if gcd == 1 {
                continue;
            }
            nonunit_pairs += 1;
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame([lhs, rhs, gcd, s, t])),
                Ok(()),
                "genuine nonunit witness lhs={lhs:#b} rhs={rhs:#b}"
            );
            for forged_s in 0_u16..=15 {
                for forged_t in 0_u16..=15 {
                    let relation = product(u64::from(lhs), u64::from(forged_s))
                        ^ product(u64::from(rhs), u64::from(forged_t));
                    assert_ne!(
                        relation, 1,
                        "independent nonunit impossibility lhs={lhs:#b} rhs={rhs:#b}"
                    );
                    assert!(
                        check_frozen_gf2_polynomial_bezout(&frame([
                            lhs, rhs, 1, forged_s, forged_t,
                        ]))
                        .is_err(),
                        "forged unit claim lhs={lhs:#b} rhs={rhs:#b} s={forged_s:#b} t={forged_t:#b}"
                    );
                    rejected_claims += 1;
                }
            }
        }
    }
    assert_eq!(nonunit_pairs, 98);
    assert_eq!(rejected_claims, 25_088);
}

// A shared nonmonomial factor can be cancelled from the independent Bezout
// equation, but it cannot be cancelled into an inverse in either quotient.
// Derive the witnesses from coprime base operands rather than trusting the
// production GCD or inverse checker. This remains non-authoritative P2 proof.
#[test]
fn nonmonomial_common_factors_preserve_bezout_but_never_create_modular_units() {
    let mut factor_pairs = 0_u32;
    let mut nonunit_directions = 0_u32;
    for (base_lhs, base_rhs) in [
        (0b101_u16, 0b111_u16),
        (0b1011, 0b1101),
        (0b10011, 0b101),
        (0b100101, 0b100011),
        (0x4001, 0x4000),
    ] {
        let (base_gcd, s, t) = extended_gcd(base_lhs, base_rhs);
        assert_eq!(base_gcd, 1, "independent coprime base");
        let factors: &[u16] = if base_lhs == 0x4001 {
            &[0b11] // Degree-15 products stay inside the frozen u16 frame.
        } else {
            &[0b11, 0b111, 0b1011, 0b1101, 0b10011]
        };
        for &factor in factors {
            let lhs = u16::try_from(product(u64::from(base_lhs), u64::from(factor)))
                .expect("bounded factored lhs");
            let rhs = u16::try_from(product(u64::from(base_rhs), u64::from(factor)))
                .expect("bounded factored rhs");
            let (actual_gcd, _, _) = extended_gcd(lhs, rhs);
            assert_eq!(actual_gcd, factor, "independent common-factor GCD");
            assert_eq!(
                product(u64::from(lhs), u64::from(s)) ^ product(u64::from(rhs), u64::from(t)),
                u64::from(factor),
                "independent factored Bezout identity"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame([lhs, rhs, factor, s, t])),
                Ok(())
            );
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame([rhs, lhs, factor, t, s])),
                Ok(())
            );
            assert!(check_frozen_gf2_polynomial_bezout(&frame([lhs, rhs, 1, s, t])).is_err());
            factor_pairs += 1;

            for (modulus, element, coefficient) in [(rhs, lhs, s), (lhs, rhs, t)] {
                let reduced_element = div_rem(u64::from(element), u64::from(modulus)).1 as u16;
                let reduced_coefficient =
                    div_rem(u64::from(coefficient), u64::from(modulus)).1 as u16;
                let (quotient_gcd, _, _) = extended_gcd(modulus, reduced_element);
                assert_eq!(quotient_gcd, factor, "independent quotient nonunit");
                let remainder = div_rem(
                    product(u64::from(reduced_element), u64::from(reduced_coefficient)),
                    u64::from(modulus),
                )
                .1 as u16;
                assert_ne!(remainder, 1, "common factor forbids an inverse");
                assert_eq!(
                    check_frozen_gf2_polynomial_modular_inverse(&frame([
                        modulus,
                        reduced_element,
                        reduced_coefficient,
                    ])),
                    Err(Gf2PolynomialModularInverseError::NotInverse { remainder }),
                    "modulus={modulus:#x} element={element:#x} factor={factor:#x}"
                );
                nonunit_directions += 1;
            }
        }
    }
    assert_eq!(factor_pairs, 21);
    assert_eq!(nonunit_directions, 42);
}
