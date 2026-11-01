//! Independent bounded GF(2)[x] Bezout witness preflight.
//! Source-only P2 preparation: no certificate, Judgement, or promotion.
//!
//! Frozen v0 frame: five big-endian u16 coefficient bitsets:
//! lhs, rhs, claimed GCD, lhs coefficient, rhs coefficient.
//! Both-zero operands are undefined; all other pairs have a monic GCD.

const FRAME_BYTES: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialBezoutError {
    InvalidFrame,
    BothZero,
    GcdMismatch { expected: u16, actual: u16 },
    IdentityMismatch { computed: u32, claimed: u16 },
}

/// Check both the true monic GCD and an exact polynomial Bezout identity.
///
/// The claimed coefficients are untrusted and need not be minimal. Carryless
/// products use u32 so degree-30 terms cannot disappear through truncation.
/// This does not authenticate the witness or grant mathematical authority.
pub fn check_frozen_gf2_polynomial_bezout(
    body: &[u8],
) -> Result<(), Gf2PolynomialBezoutError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialBezoutError::InvalidFrame)?;
    let [lhs, rhs, claimed, lhs_coefficient, rhs_coefficient] =
        core::array::from_fn(|i| u16::from_be_bytes([bytes[2 * i], bytes[2 * i + 1]]));

    if lhs == 0 && rhs == 0 {
        return Err(Gf2PolynomialBezoutError::BothZero);
    }

    let mut a = lhs;
    let mut b = rhs;
    while b != 0 {
        let mut remainder = a;
        while remainder != 0 && remainder.leading_zeros() <= b.leading_zeros() {
            let shift = b.leading_zeros() - remainder.leading_zeros();
            remainder ^= b << shift;
        }
        a = b;
        b = remainder;
    }
    if claimed != a {
        return Err(Gf2PolynomialBezoutError::GcdMismatch {
            expected: a,
            actual: claimed,
        });
    }

    let computed = carryless_product(lhs, lhs_coefficient)
        ^ carryless_product(rhs, rhs_coefficient);
    if computed != u32::from(claimed) {
        return Err(Gf2PolynomialBezoutError::IdentityMismatch { computed, claimed });
    }
    Ok(())
}

fn carryless_product(lhs: u16, rhs: u16) -> u32 {
    let mut result = 0_u32;
    for bit in 0..16 {
        if rhs & (1_u16 << bit) != 0 {
            result ^= u32::from(lhs) << bit;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(lhs: u16, rhs: u16, gcd: u16, s: u16, t: u16) -> [u8; FRAME_BYTES] {
        let mut body = [0_u8; FRAME_BYTES];
        for (index, value) in [lhs, rhs, gcd, s, t].into_iter().enumerate() {
            body[2 * index..2 * index + 2].copy_from_slice(&value.to_be_bytes());
        }
        body
    }

    // Reference oracle enumerates exact factor/quotient pairs, not Euclid.
    fn divides(divisor: u16, dividend: u16) -> bool {
        (0_u16..16).any(|quotient| carryless_product(divisor, quotient) == u32::from(dividend))
    }

    fn independent_small_gcd(lhs: u16, rhs: u16) -> u16 {
        let mut best = 0_u16;
        for candidate in 1_u16..16 {
            if candidate.leading_zeros() < best.leading_zeros()
                && divides(candidate, lhs)
                && divides(candidate, rhs)
            {
                best = candidate;
            }
        }
        best
    }

    #[test]
    fn exhaustive_small_domain_has_exact_bezout_witnesses() {
        for lhs in 0_u16..16 {
            for rhs in 0_u16..16 {
                if lhs == 0 && rhs == 0 {
                    continue;
                }
                let gcd = independent_small_gcd(lhs, rhs);
                assert_ne!(gcd, 0);
                let mut witness = None;
                'search: for s in 0_u16..16 {
                    for t in 0_u16..16 {
                        if carryless_product(lhs, s) ^ carryless_product(rhs, t)
                            == u32::from(gcd)
                        {
                            witness = Some((s, t));
                            break 'search;
                        }
                    }
                }
                let (s, t) = witness.expect("bounded small-domain Bezout witness");
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, t)),
                    Ok(()),
                    "lhs={lhs:#b} rhs={rhs:#b}"
                );
            }
        }
    }

    #[test]
    fn malformed_frames_and_undefined_pair_fail_closed() {
        let valid = frame(0b1011, 0b1101, 1, 2, 3);
        assert_eq!(check_frozen_gf2_polynomial_bezout(&valid), Ok(()));
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&valid[..length]),
                Err(Gf2PolynomialBezoutError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&padded),
            Err(Gf2PolynomialBezoutError::InvalidFrame)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0, 0, 0, 0, 0)),
            Err(Gf2PolynomialBezoutError::BothZero)
        );
    }

    #[test]
    fn forged_gcd_and_identity_fail_independently() {
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b1011, 0b1101, 3, 2, 3)),
            Err(Gf2PolynomialBezoutError::GcdMismatch {
                expected: 1,
                actual: 3,
            })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b1011, 0b1101, 1, 0, 0)),
            Err(Gf2PolynomialBezoutError::IdentityMismatch {
                computed: 0,
                claimed: 1,
            })
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0x8000, 0, 0x8000, 0x8000, 0)),
            Err(Gf2PolynomialBezoutError::IdentityMismatch {
                computed: 0x4000_0000,
                claimed: 0x8000,
            })
        );
    }

    #[test]
    fn high_degree_and_zero_operand_witnesses() {
        for (lhs, rhs, gcd, s, t) in [
            (0x8000, 0, 0x8000, 1, 0),
            (0, 0x8000, 0x8000, 0, 1),
            (0xffff, 0xffff, 0xffff, 0, 1),
            (0x8000, 0x8001, 1, 1, 1),
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, t)),
                Ok(())
            );
        }
    }
}
