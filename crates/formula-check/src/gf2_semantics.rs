//! Independent, bounded Boolean-to-GF(2) witness checking for P2 preparation.
//!
//! This checker consumes frozen bytes and enumerates all four assignments to
//! two Boolean inputs. It is not a certificate, trusted translation, proof of
//! the enclosing Judgement, or authority/promotion decision.
//!
//! Boolean postfix v0: 0=x, 1=y, 2=false, 3=true, 4=NOT,
//! 5=AND, 6=OR, 7=XOR.
//! GF(2) postfix v0: 0=x, 1=y, 2=zero, 3=one, 4=addition/XOR,
//! 5=multiplication/AND. No implicit coercion or extra opcodes.

const MAX_TERM_BYTES: usize = 1024;
const MAX_STACK_DEPTH: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2WitnessError {
    InvalidFrame,
    MalformedBoolean,
    MalformedGf2,
    Counterexample {
        x: bool,
        y: bool,
        boolean: bool,
        gf2: bool,
    },
}

#[derive(Clone, Copy)]
enum Grammar {
    Boolean,
    Gf2,
}

fn evaluate(term: &[u8], x: bool, y: bool, grammar: Grammar) -> Option<bool> {
    if term.is_empty() || term.len() > MAX_TERM_BYTES {
        return None;
    }
    let mut stack = [false; MAX_STACK_DEPTH];
    let mut depth = 0usize;
    for &op in term {
        match op {
            0..=3 => {
                if depth == MAX_STACK_DEPTH {
                    return None;
                }
                stack[depth] = match op {
                    0 => x,
                    1 => y,
                    2 => false,
                    _ => true,
                };
                depth += 1;
            }
            4 if matches!(grammar, Grammar::Boolean) => {
                if depth == 0 {
                    return None;
                }
                stack[depth - 1] = !stack[depth - 1];
            }
            4..=7 => {
                if depth < 2 {
                    return None;
                }
                let a = stack[depth - 2];
                let b = stack[depth - 1];
                stack[depth - 2] = match (grammar, op) {
                    (Grammar::Boolean, 5) | (Grammar::Gf2, 5) => a & b,
                    (Grammar::Boolean, 6) => a | b,
                    (Grammar::Boolean, 7) | (Grammar::Gf2, 4) => a ^ b,
                    _ => return None,
                };
                depth -= 1;
            }
            _ => return None,
        }
    }
    (depth == 1).then_some(stack[0])
}

/// Exhaustively compare a frozen Boolean expression and a frozen GF(2)
/// expression for every pair of inputs. This is only a bounded semantic witness.
pub fn check_frozen_boolean_gf2_equivalence(
    boolean_term: &[u8],
    gf2_term: &[u8],
) -> Result<(), Gf2WitnessError> {
    for x in [false, true] {
        for y in [false, true] {
            let boolean = evaluate(boolean_term, x, y, Grammar::Boolean)
                .ok_or(Gf2WitnessError::MalformedBoolean)?;
            let gf2 =
                evaluate(gf2_term, x, y, Grammar::Gf2).ok_or(Gf2WitnessError::MalformedGf2)?;
            if boolean != gf2 {
                return Err(Gf2WitnessError::Counterexample { x, y, boolean, gf2 });
            }
        }
    }
    Ok(())
}

/// Strict framing: big-endian u16 Boolean length + Boolean term, followed by
/// big-endian u16 GF(2) length + GF(2) term. No trailing bytes are permitted.
pub fn check_frozen_boolean_gf2_witness(body: &[u8]) -> Result<(), Gf2WitnessError> {
    if body.len() > 4 + 2 * MAX_TERM_BYTES {
        return Err(Gf2WitnessError::InvalidFrame);
    }
    let first = body.get(..2).ok_or(Gf2WitnessError::InvalidFrame)?;
    let bool_len = usize::from(u16::from_be_bytes([first[0], first[1]]));
    if !(1..=MAX_TERM_BYTES).contains(&bool_len) {
        return Err(Gf2WitnessError::InvalidFrame);
    }
    let boolean_term = body
        .get(2..2 + bool_len)
        .ok_or(Gf2WitnessError::InvalidFrame)?;
    let second = body
        .get(2 + bool_len..4 + bool_len)
        .ok_or(Gf2WitnessError::InvalidFrame)?;
    let gf2_len = usize::from(u16::from_be_bytes([second[0], second[1]]));
    if !(1..=MAX_TERM_BYTES).contains(&gf2_len) || body.len() != 4 + bool_len + gf2_len {
        return Err(Gf2WitnessError::InvalidFrame);
    }
    check_frozen_boolean_gf2_equivalence(boolean_term, &body[4 + bool_len..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(boolean: &[u8], gf2: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&(boolean.len() as u16).to_be_bytes());
        body.extend_from_slice(boolean);
        body.extend_from_slice(&(gf2.len() as u16).to_be_bytes());
        body.extend_from_slice(gf2);
        body
    }

    #[test]
    fn boolean_or_requires_gf2_cross_term() {
        // x OR y = x + y + xy over GF(2), not x + y.
        assert_eq!(
            check_frozen_boolean_gf2_witness(&frame(&[0, 1, 6], &[0, 1, 4, 0, 1, 5, 4])),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_witness(&frame(&[0, 1, 6], &[0, 1, 4])),
            Err(Gf2WitnessError::Counterexample {
                x: true,
                y: true,
                boolean: true,
                gf2: false,
            })
        );
    }

    #[test]
    fn not_and_xor_have_independent_gf2_witnesses() {
        assert_eq!(
            check_frozen_boolean_gf2_equivalence(&[0, 4], &[0, 3, 4]),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_equivalence(&[0, 1, 5], &[0, 1, 5]),
            Ok(())
        );
        assert_eq!(
            check_frozen_boolean_gf2_equivalence(&[0, 1, 7], &[0, 1, 4]),
            Ok(())
        );
    }

    #[test]
    fn false_translation_returns_exact_first_assignment() {
        assert_eq!(
            check_frozen_boolean_gf2_equivalence(&[0, 4], &[0]),
            Err(Gf2WitnessError::Counterexample {
                x: false,
                y: false,
                boolean: true,
                gf2: false,
            })
        );
    }

    #[test]
    fn malformed_grammars_are_rejected_separately() {
        for invalid in [
            vec![],
            vec![4],
            vec![5],
            vec![0, 1],
            vec![0xff],
            vec![0; 129],
            vec![0; 1025],
        ] {
            assert_eq!(
                check_frozen_boolean_gf2_equivalence(&invalid, &[0]),
                Err(Gf2WitnessError::MalformedBoolean)
            );
            assert_eq!(
                check_frozen_boolean_gf2_equivalence(&[0], &invalid),
                Err(Gf2WitnessError::MalformedGf2)
            );
        }
        assert_eq!(
            check_frozen_boolean_gf2_equivalence(&[0, 1, 6], &[0, 1, 6]),
            Err(Gf2WitnessError::MalformedGf2)
        );
        assert_eq!(
            check_frozen_boolean_gf2_equivalence(&[0, 1, 7], &[0, 1, 4]),
            Ok(())
        );
    }

    #[test]
    fn framing_rejects_truncation_padding_and_oversize() {
        let valid = frame(&[0], &[0]);
        assert_eq!(check_frozen_boolean_gf2_witness(&valid), Ok(()));
        for invalid in [
            vec![],
            vec![0],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0, 2, 0],
            vec![0; 2053],
        ] {
            assert_eq!(
                check_frozen_boolean_gf2_witness(&invalid),
                Err(Gf2WitnessError::InvalidFrame)
            );
        }
        let mut trailing = valid;
        trailing.push(0);
        assert_eq!(
            check_frozen_boolean_gf2_witness(&trailing),
            Err(Gf2WitnessError::InvalidFrame)
        );
    }

    #[test]
    fn maximum_valid_term_size_is_bounded() {
        let mut boolean = vec![0];
        boolean.extend(std::iter::repeat_n(4, 1023));
        let mut gf2 = vec![0];
        for _ in 0..511 {
            gf2.extend([3, 4]);
        }
        assert_eq!(boolean.len(), 1024);
        assert_eq!(gf2.len(), 1023);
        // 1023 Boolean NOTs and 511 GF(2) additions of one both negate x.
        assert_eq!(
            check_frozen_boolean_gf2_witness(&frame(&boolean, &gf2)),
            Ok(())
        );
        gf2.push(3);
        assert_eq!(
            check_frozen_boolean_gf2_witness(&frame(&boolean, &gf2)),
            Err(Gf2WitnessError::MalformedGf2)
        );
        gf2.push(4);
        assert_eq!(
            check_frozen_boolean_gf2_witness(&frame(&boolean, &gf2)),
            Err(Gf2WitnessError::InvalidFrame)
        );
    }
}
