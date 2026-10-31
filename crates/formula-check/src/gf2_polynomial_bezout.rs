//! Independent bounded Bézout witness preflight for GF(2)[x] gcd claims.
//! Source-only P2 preparation; this cannot issue certificates or admit authority.

// Wire v0: five big-endian u16 bitsets, in order a, b, claimed gcd,
// Bézout coefficient u, Bézout coefficient v. Bit i is the coefficient of x^i.
// A nonzero common divisor g is greatest iff u*a + v*b = g for some u,v.
const FRAME_BYTES: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialBezoutError {
    InvalidFrame,
    BothZero,
    ZeroGcd,
    NotCommonDivisor,
    BezoutMismatch { expected: u16, actual: u32 },
}

fn remainder(mut dividend: u16, divisor: u16) -> u16 {
    debug_assert_ne!(divisor, 0);
    while dividend != 0 && dividend.leading_zeros() <= divisor.leading_zeros() {
        dividend ^= divisor << (divisor.leading_zeros() - dividend.leading_zeros());
    }
    dividend
}

fn carryless_product(a: u16, b: u16) -> u32 {
    let mut result = 0_u32;
    for bit in 0..16 {
        if b & (1_u16 << bit) != 0 {
            result ^= u32::from(a) << bit;
        }
    }
    result
}

/// Verify a frozen gcd claim by divisibility and an exact Bézout identity.
/// The products use all 31 polynomial coefficient positions, so high-degree
/// terms cannot disappear through u16 truncation before XOR cancellation.
pub fn check_frozen_gf2_polynomial_bezout(body: &[u8]) -> Result<(), Gf2PolynomialBezoutError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialBezoutError::InvalidFrame)?;
    let [a, b, gcd, u, v] =
        core::array::from_fn(|i| u16::from_be_bytes([bytes[2 * i], bytes[2 * i + 1]]));
    if a == 0 && b == 0 {
        return Err(Gf2PolynomialBezoutError::BothZero);
    }
    if gcd == 0 {
        return Err(Gf2PolynomialBezoutError::ZeroGcd);
    }
    if remainder(a, gcd) != 0 || remainder(b, gcd) != 0 {
        return Err(Gf2PolynomialBezoutError::NotCommonDivisor);
    }
    let actual = carryless_product(a, u) ^ carryless_product(b, v);
    if actual != u32::from(gcd) {
        return Err(Gf2PolynomialBezoutError::BezoutMismatch {
            expected: gcd,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(a: u16, b: u16, gcd: u16, u: u16, v: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        for (i, value) in [a, b, gcd, u, v].into_iter().enumerate() {
            bytes[2 * i..2 * i + 2].copy_from_slice(&value.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn exact_bezout_and_high_degree_cancellation_are_admitted() {
        // x*(x^2+1) + (x^3+1) = x+1.
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b101, 0b1001, 0b11, 0b10, 1)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b10, 0b11, 1, 1, 1)),
            Ok(())
        );
        // The two degree-30 products cancel exactly, not by truncation.
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0x8000, 0x8000, 0x8000, 0x8001, 0x8000,)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0, 0b11, 0b11, 0, 1)),
            Ok(())
        );
    }

    #[test]
    fn forged_claims_and_frame_boundaries_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0, 0, 0, 0, 0)),
            Err(Gf2PolynomialBezoutError::BothZero)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b11, 0b11, 0, 0, 0)),
            Err(Gf2PolynomialBezoutError::ZeroGcd)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b11, 0b11, 0b10, 0, 0)),
            Err(Gf2PolynomialBezoutError::NotCommonDivisor)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0b11, 0b11, 1, 1, 0)),
            Err(Gf2PolynomialBezoutError::BezoutMismatch {
                expected: 1,
                actual: 0b11,
            })
        );
        // A product above degree 15 must not be silently truncated.
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(0x8000, 0x8000, 0x8000, 0x8000, 0)),
            Err(Gf2PolynomialBezoutError::BezoutMismatch {
                expected: 0x8000,
                actual: 1_u32 << 30,
            })
        );
        let valid = frame(0b101, 0b1001, 0b11, 0b10, 1);
        for n in 0..valid.len() {
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&valid[..n]),
                Err(Gf2PolynomialBezoutError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&padded),
            Err(Gf2PolynomialBezoutError::InvalidFrame)
        );
    }

    #[test]
    fn exhaustive_small_pairs_have_independent_bezout_witnesses() {
        // Enumerate common divisors and coefficients independently of the
        // checker's divisibility path; every nonzero pair has a witness.
        for a in 0_u16..16 {
            for b in 0_u16..16 {
                if a == 0 && b == 0 {
                    continue;
                }
                let mut gcd = 0_u16;
                for candidate in 1_u16..16 {
                    if oracle_divides(a, candidate)
                        && oracle_divides(b, candidate)
                        && candidate.leading_zeros() < gcd.leading_zeros()
                    {
                        gcd = candidate;
                    }
                }
                let mut witness = None;
                'search: for u in 0_u16..16 {
                    for v in 0_u16..16 {
                        if (oracle_product(a, u) ^ oracle_product(b, v)) == u32::from(gcd) {
                            witness = Some((u, v));
                            break 'search;
                        }
                    }
                }
                let (u, v) = witness.expect("small GF(2)[x] gcd has bounded Bézout witness");
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(a, b, gcd, u, v)),
                    Ok(()),
                    "a={a:#b} b={b:#b}"
                );
            }
        }
    }

    fn oracle_product(a: u16, b: u16) -> u32 {
        // Coefficient-by-coefficient convolution, not the checker shift path.
        let mut result = 0_u32;
        for i in 0..16 {
            for j in 0..16 {
                if (a & (1_u16 << i)) != 0 && (b & (1_u16 << j)) != 0 {
                    result ^= 1_u32 << (i + j);
                }
            }
        }
        result
    }

    fn oracle_divides(mut dividend: u16, divisor: u16) -> bool {
        // Independent bit-position cancellation, without leading_zeros.
        for bit in (0..16).rev() {
            if dividend & (1_u16 << bit) == 0 {
                continue;
            }
            let degree = 15 - divisor.leading_zeros();
            if bit < degree {
                return false;
            }
            dividend ^= divisor << (bit - degree);
        }
        dividend == 0
    }
}
