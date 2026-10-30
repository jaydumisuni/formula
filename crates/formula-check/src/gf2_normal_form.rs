//! Independent algebraic-normal-form cross-check for frozen two-input terms.
//!
//! This is a second, non-authoritative P2 preparation path. Unlike the
//! truth-table evaluator, it interprets syntax in the Boolean quotient ring
//! GF(2)[x,y]/(x*x=x, y*y=y). It cannot issue certificates or admit trust roots.
//! Monomial bits are ordered 1, x, y, xy.
const MAX_TERM_BYTES: usize = 1024;
const MAX_STACK_DEPTH: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalFormError {
    MalformedBoolean,
    MalformedGf2,
    CoefficientMismatch {
        monomial: u8,
        boolean: bool,
        gf2: bool,
    },
}

#[derive(Clone, Copy)]
enum Grammar {
    Boolean,
    Gf2,
}

/// Multiply two multilinear polynomials modulo x*x=x and y*y=y.
/// XOR accumulation is essential: duplicate monomials cancel in GF(2).
fn multiply(a: u8, b: u8) -> u8 {
    let mut product = 0;
    for i in 0..4u8 {
        if a & (1 << i) == 0 {
            continue;
        }
        for j in 0..4u8 {
            if b & (1 << j) != 0 {
                product ^= 1 << (i | j);
            }
        }
    }
    product
}

fn polynomial(term: &[u8], grammar: Grammar) -> Option<u8> {
    if term.is_empty() || term.len() > MAX_TERM_BYTES {
        return None;
    }
    let mut stack = [0u8; MAX_STACK_DEPTH];
    let mut depth = 0usize;
    for &opcode in term {
        match opcode {
            0..=3 => {
                if depth == MAX_STACK_DEPTH {
                    return None;
                }
                stack[depth] = match opcode {
                    0 => 0b0010, // x
                    1 => 0b0100, // y
                    2 => 0,      // false / zero
                    _ => 0b0001, // true / one
                };
                depth += 1;
            }
            4 if matches!(grammar, Grammar::Boolean) => {
                if depth == 0 {
                    return None;
                }
                stack[depth - 1] ^= 1; // NOT p = 1 + p
            }
            4..=7 => {
                if depth < 2 {
                    return None;
                }
                let left = stack[depth - 2];
                let right = stack[depth - 1];
                stack[depth - 2] = match (grammar, opcode) {
                    (Grammar::Boolean, 5) | (Grammar::Gf2, 5) => multiply(left, right),
                    (Grammar::Boolean, 6) => left ^ right ^ multiply(left, right),
                    (Grammar::Boolean, 7) | (Grammar::Gf2, 4) => left ^ right,
                    _ => return None,
                };
                depth -= 1;
            }
            _ => return None,
        }
    }
    (depth == 1).then_some(stack[0])
}

/// Compare exact algebraic-normal-form coefficients without sampling inputs.
/// This is independent corroboration, NOT a certificate or authority decision.
pub fn check_frozen_boolean_gf2_normal_form(
    boolean_term: &[u8],
    gf2_term: &[u8],
) -> Result<(), NormalFormError> {
    let boolean =
        polynomial(boolean_term, Grammar::Boolean).ok_or(NormalFormError::MalformedBoolean)?;
    let gf2 = polynomial(gf2_term, Grammar::Gf2).ok_or(NormalFormError::MalformedGf2)?;
    for monomial in 0..4u8 {
        let mask = 1 << monomial;
        if (boolean & mask) != (gf2 & mask) {
            return Err(NormalFormError::CoefficientMismatch {
                monomial,
                boolean: boolean & mask != 0,
                gf2: gf2 & mask != 0,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_idempotence_and_cancellation_are_exact() {
        assert_eq!(multiply(0b0010, 0b0010), 0b0010); // x*x=x
        assert_eq!(multiply(0b0100, 0b0100), 0b0100); // y*y=y
        assert_eq!(multiply(0b1000, 0b1000), 0b1000); // xy*xy=xy
        assert_eq!(multiply(0b0011, 0b0011), 0b0011); // (1+x)^2=1+x
        assert_eq!(multiply(0b1111, 0b1111), 0b1111);
        assert_eq!(multiply(0b0010, 0b0100), 0b1000);
        assert_eq!(multiply(0b0011, 0b0011) ^ 0b0011, 0);
    }

    #[test]
    fn boolean_or_and_not_have_independent_normal_forms() {
        assert_eq!(
            check_frozen_boolean_gf2_normal_form(&[0, 1, 6], &[0, 1, 4, 0, 1, 5, 4]),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_normal_form(&[0, 4], &[3, 0, 4]),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_normal_form(&[0, 1, 5], &[0, 1, 5]),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_normal_form(&[0, 1, 7], &[0, 1, 4]),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_normal_form(&[0, 1, 6], &[0, 1, 4]),
            Err(NormalFormError::CoefficientMismatch {
                monomial: 3,
                boolean: true,
                gf2: false,
            })
        );
    }

    #[test]
    fn all_256_truth_table_pairs_agree_with_independent_enumeration() {
        fn boolean_dnf(mask: u8) -> Vec<u8> {
            let mut term = Vec::new();
            let mut count = 0;
            for x in 0..2 {
                for y in 0..2 {
                    if mask & (1 << (2 * x + y)) == 0 {
                        continue;
                    }
                    term.push(0);
                    if x == 0 {
                        term.push(4);
                    }
                    term.push(1);
                    if y == 0 {
                        term.push(4);
                    }
                    term.push(5);
                    if count > 0 {
                        term.push(6);
                    }
                    count += 1;
                }
            }
            if count == 0 { vec![2] } else { term }
        }

        fn gf2_anf(mask: u8) -> Vec<u8> {
            let f00 = mask & 1 != 0;
            let f01 = mask & 2 != 0;
            let f10 = mask & 4 != 0;
            let f11 = mask & 8 != 0;
            let monomials: [(bool, &[u8]); 4] = [
                (f00, &[3]),
                (f10 ^ f00, &[0]),
                (f01 ^ f00, &[1]),
                (f11 ^ f10 ^ f01 ^ f00, &[0, 1, 5]),
            ];
            let mut term = Vec::new();
            let mut count = 0;
            for (present, monomial) in monomials {
                if present {
                    term.extend_from_slice(monomial);
                    if count > 0 {
                        term.push(4);
                    }
                    count += 1;
                }
            }
            if count == 0 { vec![2] } else { term }
        }

        for boolean_mask in 0..16u8 {
            for gf2_mask in 0..16u8 {
                let boolean = boolean_dnf(boolean_mask);
                let gf2 = gf2_anf(gf2_mask);
                let algebraic = check_frozen_boolean_gf2_normal_form(&boolean, &gf2);
                let enumerated =
                    crate::gf2_semantics::check_frozen_boolean_gf2_equivalence(&boolean, &gf2);
                assert_eq!(
                    algebraic.is_ok(),
                    enumerated.is_ok(),
                    "normal form and enumeration disagree for {boolean_mask:04b}/{gf2_mask:04b}"
                );
                assert_eq!(algebraic.is_ok(), boolean_mask == gf2_mask);
            }
        }
    }

    #[test]
    fn invalid_grammar_and_resource_limits_fail_closed() {
        for invalid in [
            vec![],
            vec![4],
            vec![0, 1],
            vec![0xff],
            vec![0; 129],
            vec![0; 1025],
        ] {
            assert_eq!(
                check_frozen_boolean_gf2_normal_form(&invalid, &[0]),
                Err(NormalFormError::MalformedBoolean)
            );
            assert_eq!(
                check_frozen_boolean_gf2_normal_form(&[0], &invalid),
                Err(NormalFormError::MalformedGf2)
            );
        }
        assert_eq!(
            check_frozen_boolean_gf2_normal_form(&[0], &[0, 1, 6]),
            Err(NormalFormError::MalformedGf2)
        );
    }
}
