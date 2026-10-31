//! Bounded exact GF(2)[x] quotient/remainder preflight on frozen bytes.
//! No certificate, Judgement, trust-root, or promotion authority is granted.

// Wire v0 is exactly four big-endian u16 polynomial bitsets in ascending
// powers: dividend, nonzero divisor, quotient, remainder. Bit i is x^i.
// Every u16 has a unique canonical encoding, including zero.
const FRAME_BYTES: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialDivisionError {
    InvalidFrame,
    ZeroDivisor,
    RemainderDegreeNotReduced,
    DegreeOutOfBounds,
    CoefficientMismatch { expected: u16, actual: u16 },
}

/// Independently check dividend = divisor * quotient + remainder in GF(2)[x].
/// All coefficients and intermediate carryless products are exact; no samples,
/// wrapping or producer-provided algebraic assertions can grant authority.
pub fn check_frozen_gf2_polynomial_division(body: &[u8]) -> Result<(), Gf2PolynomialDivisionError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialDivisionError::InvalidFrame)?;
    let [dividend, divisor, quotient, remainder] = [
        u16::from_be_bytes([bytes[0], bytes[1]]),
        u16::from_be_bytes([bytes[2], bytes[3]]),
        u16::from_be_bytes([bytes[4], bytes[5]]),
        u16::from_be_bytes([bytes[6], bytes[7]]),
    ];
    if divisor == 0 {
        return Err(Gf2PolynomialDivisionError::ZeroDivisor);
    }
    if remainder != 0 && remainder.leading_zeros() <= divisor.leading_zeros() {
        return Err(Gf2PolynomialDivisionError::RemainderDegreeNotReduced);
    }

    // A nonzero product over a field has degree equal to the sum of degrees.
    // Reject out-of-representation witnesses instead of silently truncating.
    let mut product = 0_u32;
    for bit in 0..16 {
        if quotient & (1_u16 << bit) != 0 {
            product ^= u32::from(divisor) << bit;
        }
    }
    let expected = product ^ u32::from(remainder);
    let expected =
        u16::try_from(expected).map_err(|_| Gf2PolynomialDivisionError::DegreeOutOfBounds)?;
    if expected != dividend {
        return Err(Gf2PolynomialDivisionError::CoefficientMismatch {
            expected,
            actual: dividend,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(dividend: u16, divisor: u16, quotient: u16, remainder: u16) -> [u8; 8] {
        let mut body = [0_u8; 8];
        for (i, value) in [dividend, divisor, quotient, remainder]
            .into_iter()
            .enumerate()
        {
            body[2 * i..2 * i + 2].copy_from_slice(&value.to_be_bytes());
        }
        body
    }

    #[test]
    fn zero_divisor_remainder_degree_and_frame_boundaries_fail_closed() {
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0, 0, 0, 0)),
            Err(Gf2PolynomialDivisionError::ZeroDivisor)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0, 0b11, 0, 0b10)),
            Err(Gf2PolynomialDivisionError::RemainderDegreeNotReduced)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0, 1, 0, 1)),
            Err(Gf2PolynomialDivisionError::RemainderDegreeNotReduced)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0, 0x8000, 2, 0)),
            Err(Gf2PolynomialDivisionError::DegreeOutOfBounds)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0x8000, 1, 0x8000, 0)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0, 0b11, 0, 0)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(1, 0b11, 0, 1)),
            Ok(())
        );
        let valid = frame(0b1000, 0b11, 0b111, 1);
        for length in 0..valid.len() {
            assert_eq!(
                check_frozen_gf2_polynomial_division(&valid[..length]),
                Err(Gf2PolynomialDivisionError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_division(&padded),
            Err(Gf2PolynomialDivisionError::InvalidFrame)
        );
    }

    #[test]
    fn exhaustive_small_gf2_divisions_match_independent_long_division() {
        // Independent bitwise long division constructs the quotient/remainder;
        // the checker must validate the frozen relation for every input pair.
        for dividend in 0_u16..64 {
            for divisor in 1_u16..32 {
                let mut remainder = dividend;
                let mut quotient = 0_u16;
                while remainder != 0 && remainder.leading_zeros() <= divisor.leading_zeros() {
                    let shift = divisor.leading_zeros() - remainder.leading_zeros();
                    quotient ^= 1_u16 << shift;
                    remainder ^= divisor << shift;
                }
                let body = frame(dividend, divisor, quotient, remainder);
                assert_eq!(
                    check_frozen_gf2_polynomial_division(&body),
                    Ok(()),
                    "dividend={dividend:#b} divisor={divisor:#b}"
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_division(&frame(
                        dividend ^ 1,
                        divisor,
                        quotient,
                        remainder,
                    )),
                    Err(Gf2PolynomialDivisionError::CoefficientMismatch {
                        expected: dividend,
                        actual: dividend ^ 1,
                    })
                );
            }
        }
    }

    #[test]
    fn exact_gf2_quotient_remainder_and_forged_claim() {
        // (x + 1)(x^2 + x + 1) + 1 = x^3 over GF(2).
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0b1000, 0b11, 0b111, 1)),
            Ok(())
        );
        assert_eq!(
            check_frozen_gf2_polynomial_division(&frame(0b1001, 0b11, 0b111, 1)),
            Err(Gf2PolynomialDivisionError::CoefficientMismatch {
                expected: 0b1000,
                actual: 0b1001,
            })
        );
    }
}
