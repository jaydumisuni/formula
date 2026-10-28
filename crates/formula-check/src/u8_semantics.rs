//! Bounded, independent U8 semantics comparison over frozen postfix terms.
//!
//! This is a preparatory P2 checker primitive, not a certificate, trust-root
//! decision, evidence-envelope validator, or promotion operation. Its inputs
//! are immutable bytes rather than producer-owned expression objects.
//!
//! Encoding v0 (local preparation only):
//! 0x00 = input x, 0x01 <byte> = constant;
//! 0x02 = wrapping add, 0x03 = wrapping subtract, 0x04 = wrapping multiply;
//! 0x05 = XOR, 0x06 = AND, 0x07 = OR, 0x08 = bitwise NOT.
//! Binary operators consume left then right in postfix order.

const MAX_TERM_BYTES: usize = 1024;
const MAX_STACK_DEPTH: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum U8EquivalenceError {
    MalformedLeft,
    MalformedRight,
    Counterexample { input: u8, left: u8, right: u8 },
}

/// Interpret a frozen term without callbacks, allocations, ambient I/O, or
/// producer-supplied results. Invalid or unbounded programs fail closed.
fn evaluate(term: &[u8], input: u8) -> Option<u8> {
    if term.is_empty() || term.len() > MAX_TERM_BYTES {
        return None;
    }
    let mut stack = [0_u8; MAX_STACK_DEPTH];
    let mut depth = 0_usize;
    let mut offset = 0_usize;
    while offset < term.len() {
        let opcode = term[offset];
        offset += 1;
        match opcode {
            0x00 | 0x01 => {
                if depth == MAX_STACK_DEPTH {
                    return None;
                }
                let value = if opcode == 0x00 {
                    input
                } else {
                    let value = *term.get(offset)?;
                    offset += 1;
                    value
                };
                stack[depth] = value;
                depth += 1;
            }
            0x08 => {
                if depth == 0 {
                    return None;
                }
                stack[depth - 1] = !stack[depth - 1];
            }
            0x02..=0x07 => {
                if depth < 2 {
                    return None;
                }
                let left = stack[depth - 2];
                let right = stack[depth - 1];
                stack[depth - 2] = match opcode {
                    0x02 => left.wrapping_add(right),
                    0x03 => left.wrapping_sub(right),
                    0x04 => left.wrapping_mul(right),
                    0x05 => left ^ right,
                    0x06 => left & right,
                    0x07 => left | right,
                    _ => unreachable!(),
                };
                depth -= 1;
            }
            _ => return None,
        }
    }
    (depth == 1).then_some(stack[0])
}

/// Exhaustively compare two *frozen* U8 unary terms for all 256 inputs.
///
/// A successful comparison establishes equality for this bounded term grammar
/// only. It does not validate a judgement, certify evidence, or admit authority.
pub fn check_frozen_u8_equivalence(
    left_term: &[u8],
    right_term: &[u8],
) -> Result<(), U8EquivalenceError> {
    for input in u8::MIN..=u8::MAX {
        let left = evaluate(left_term, input).ok_or(U8EquivalenceError::MalformedLeft)?;
        let right = evaluate(right_term, input).ok_or(U8EquivalenceError::MalformedRight)?;
        if left != right {
            return Err(U8EquivalenceError::Counterexample { input, left, right });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_addition_matches_multiplication_for_every_u8() {
        // x + x = 2 * x in U8, including the wrapping boundary.
        assert_eq!(
            check_frozen_u8_equivalence(&[0, 0, 2], &[0, 1, 2, 4]),
            Ok(())
        );
        // x + 1 = x - 255 under U8 wrapping arithmetic.
        assert_eq!(
            check_frozen_u8_equivalence(&[0, 1, 1, 2], &[0, 1, 255, 3]),
            Ok(())
        );
    }

    #[test]
    fn bitwise_semantics_are_independently_enumerated() {
        // (x & 255) | (x ^ x) = x; ~~x = x.
        assert_eq!(
            check_frozen_u8_equivalence(&[0, 1, 255, 6, 0, 0, 5, 7], &[0]),
            Ok(())
        );
        assert_eq!(check_frozen_u8_equivalence(&[0, 8, 8], &[0]), Ok(()));
    }

    #[test]
    fn false_identity_returns_first_exact_counterexample() {
        assert_eq!(
            check_frozen_u8_equivalence(&[0], &[0, 1, 1, 2]),
            Err(U8EquivalenceError::Counterexample {
                input: 0,
                left: 0,
                right: 1,
            })
        );
        // x*x is not x for all U8 values; the first difference is x=2.
        assert_eq!(
            check_frozen_u8_equivalence(&[0, 0, 4], &[0]),
            Err(U8EquivalenceError::Counterexample {
                input: 2,
                left: 4,
                right: 2,
            })
        );
    }

    #[test]
    fn malformed_or_ambiguous_terms_fail_closed() {
        for invalid in [
            vec![],
            vec![1],       // missing constant operand
            vec![2],       // binary stack underflow
            vec![8],       // unary stack underflow
            vec![0, 0],    // extra final stack entry
            vec![0xff],    // unknown opcode
            vec![0; 1025], // oversized byte stream
            vec![0; 129],  // stack depth overflow
        ] {
            assert_eq!(
                check_frozen_u8_equivalence(&invalid, &[0]),
                Err(U8EquivalenceError::MalformedLeft)
            );
            assert_eq!(
                check_frozen_u8_equivalence(&[0], &invalid),
                Err(U8EquivalenceError::MalformedRight)
            );
        }
    }

    #[test]
    fn boundary_sized_valid_program_is_bounded() {
        // 1023 NOT operators plus one input: exactly 1024 bytes.
        let mut term = vec![0];
        term.extend(std::iter::repeat_n(8, 1023));
        // An odd number of NOTs is equivalent to a single NOT.
        assert_eq!(check_frozen_u8_equivalence(&term, &[0, 8]), Ok(()));
    }
}
