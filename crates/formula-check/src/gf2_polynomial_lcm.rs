//! Independent bounded GF(2)[x] LCM witness preflight on frozen bytes.
//! This is non-authoritative P2 preparation, not a certificate or promotion.

// Wire v0: two big-endian u16 polynomial bitsets and one big-endian u32
// claimed least common multiple. Bit i is the coefficient of x^i.
// The all-zero operand pair is excluded to avoid an undefined GCD dependency.
const FRAME_BYTES: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialLcmError {
    InvalidFrame,
    BothZero,
    NonExactDivision,
    LcmMismatch { expected: u32, actual: u32 },
}

/// Recompute the exact LCM without accepting a producer-provided GCD.
///
/// Euclid operates on bounded u16 polynomials. The carryless product and
/// subsequent exact division use u32, preserving all degree-30 coefficients.
/// No wrapping, search state, or certificate authority is involved.
pub fn check_frozen_gf2_polynomial_lcm(body: &[u8]) -> Result<(), Gf2PolynomialLcmError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialLcmError::InvalidFrame)?;
    let lhs = u16::from_be_bytes([bytes[0], bytes[1]]);
    let rhs = u16::from_be_bytes([bytes[2], bytes[3]]);
    let claimed = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

    if lhs == 0 && rhs == 0 {
        return Err(Gf2PolynomialLcmError::BothZero);
    }
    let expected = if lhs == 0 || rhs == 0 {
        0
    } else {
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
        let gcd = u32::from(a);

        let mut product = 0_u32;
        for bit in 0..16 {
            if rhs & (1_u16 << bit) != 0 {
                product ^= u32::from(lhs) << bit;
            }
        }

        let mut remainder = product;
        let mut quotient = 0_u32;
        while remainder != 0 && remainder.leading_zeros() <= gcd.leading_zeros() {
            let shift = gcd.leading_zeros() - remainder.leading_zeros();
            quotient ^= 1_u32 << shift;
            remainder ^= gcd << shift;
        }
        if remainder != 0 {
            return Err(Gf2PolynomialLcmError::NonExactDivision);
        }
        quotient
    };

    if claimed != expected {
        return Err(Gf2PolynomialLcmError::LcmMismatch {
            expected,
            actual: claimed,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(lhs: u16, rhs: u16, lcm: u32) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        bytes[0..2].copy_from_slice(&lhs.to_be_bytes());
        bytes[2..4].copy_from_slice(&rhs.to_be_bytes());
        bytes[4..8].copy_from_slice(&lcm.to_be_bytes());
        bytes
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

    // Independent small-domain oracle: enumerate multiples rather than
    // using Euclid, a supplied GCD, or the production LCM implementation.
    fn independent_small_lcm(lhs: u16, rhs: u16) -> u32 {
        if lhs == 0 || rhs == 0 {
            return 0;
        }
        let mut lhs_multiples = [false; 1024];
        for quotient in 1_u16..32 {
            lhs_multiples[carryless_product(lhs, quotient) as usize] = true;
        }
        let mut best = None;
        for quotient in 1_u16..32 {
            let candidate = carryless_product(rhs, quotient);
            if lhs_multiples[candidate as usize]
                && best
                    .is_none_or(|current: u32| candidate.leading_zeros() > current.leading_zeros())
            {
                best = Some(candidate);
            }
        }
        best.expect("nonzero polynomials have a common multiple in the bounded small domain")
    }

    #[test]
    fn malformed_and_undefined_frames_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_lcm(&frame(0, 0, 0)),
            Err(Gf2PolynomialLcmError::BothZero)
        );
        let valid = frame(0x8000, 0x8001, 0x4000_8000);
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_lcm(&valid[..length]),
                Err(Gf2PolynomialLcmError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_lcm(&padded),
            Err(Gf2PolynomialLcmError::InvalidFrame)
        );
    }

    #[test]
    fn exhaustive_small_domain_matches_independent_multiple_enumeration() {
        for lhs in 0_u16..32 {
            for rhs in 0_u16..32 {
                if lhs == 0 && rhs == 0 {
                    continue;
                }
                let expected = independent_small_lcm(lhs, rhs);
                assert_eq!(
                    check_frozen_gf2_polynomial_lcm(&frame(lhs, rhs, expected)),
                    Ok(()),
                    "lhs={lhs:#b} rhs={rhs:#b}"
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_lcm(&frame(lhs, rhs, expected ^ 1)),
                    Err(Gf2PolynomialLcmError::LcmMismatch {
                        expected,
                        actual: expected ^ 1,
                    })
                );
            }
        }
    }

    #[test]
    fn high_degree_zero_and_coprime_boundaries_are_exact() {
        for (lhs, rhs, expected) in [
            (0, 0x8000, 0),
            (0x8000, 0, 0),
            (0x8000, 0x4000, 0x8000),
            (0x8000, 0x8001, 0x4000_8000),
            (0xffff, 0xffff, 0xffff),
            (1, 0xffff, 0xffff),
            (0x8000, 0x8000, 0x8000),
            (0x8000, 0x4001, 0x2000_8000),
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_lcm(&frame(lhs, rhs, expected)),
                Ok(()),
                "lhs={lhs:#x} rhs={rhs:#x}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_lcm(&frame(lhs, rhs, expected ^ 1)),
                Err(Gf2PolynomialLcmError::LcmMismatch {
                    expected,
                    actual: expected ^ 1,
                })
            );
        }
    }
}
