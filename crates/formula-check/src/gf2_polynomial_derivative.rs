//! Independent bounded GF(2)[x] formal-derivative witness preflight.
//! Source-only P2 preparation: no certificate, Judgement, or promotion.
//!
//! Frozen v0 frame: big-endian u16 polynomial followed by big-endian u16
//! claimed derivative. Bit i denotes the coefficient of x^i. The zero
//! polynomial and constants have the canonical zero derivative.

const FRAME_BYTES: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialDerivativeError {
    InvalidFrame,
    DerivativeMismatch { expected: u16, actual: u16 },
}

/// Independently recompute the formal derivative of frozen GF(2)[x] bytes.
///
/// Characteristic two eliminates even-exponent terms and maps each odd
/// exponent x^(2k+1) to x^(2k). This is an exact bounded coefficient check,
/// not proof that any external Judgement denotes the supplied polynomial.
pub fn check_frozen_gf2_polynomial_derivative(
    body: &[u8],
) -> Result<(), Gf2PolynomialDerivativeError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialDerivativeError::InvalidFrame)?;
    let polynomial = u16::from_be_bytes([bytes[0], bytes[1]]);
    let claimed = u16::from_be_bytes([bytes[2], bytes[3]]);

    // Odd powers occupy alternating coefficient bits. No intermediate
    // multiplication or narrowing can silently discard a high-degree term.
    let expected = (polynomial & 0xaaaa) >> 1;
    if claimed != expected {
        return Err(Gf2PolynomialDerivativeError::DerivativeMismatch {
            expected,
            actual: claimed,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(polynomial: u16, derivative: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        bytes[..2].copy_from_slice(&polynomial.to_be_bytes());
        bytes[2..].copy_from_slice(&derivative.to_be_bytes());
        bytes
    }

    // Independent coefficient-rule oracle: do not reuse the production
    // checker's alternating-bit mask or shift expression.
    fn coefficient_derivative(polynomial: u16) -> u16 {
        let mut derivative = 0_u16;
        for exponent in (1..16).step_by(2) {
            if polynomial & (1_u16 << exponent) != 0 {
                derivative |= 1_u16 << (exponent - 1);
            }
        }
        derivative
    }

    #[test]
    fn malformed_frames_fail_closed_and_constants_have_zero_derivative() {
        let valid = frame(0x8003, 0x4001);
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_derivative(&valid[..length]),
                Err(Gf2PolynomialDerivativeError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_derivative(&padded),
            Err(Gf2PolynomialDerivativeError::InvalidFrame)
        );
        for polynomial in [0_u16, 1] {
            assert_eq!(check_frozen_gf2_polynomial_derivative(&frame(polynomial, 0)), Ok(()));
            assert_eq!(
                check_frozen_gf2_polynomial_derivative(&frame(polynomial, 1)),
                Err(Gf2PolynomialDerivativeError::DerivativeMismatch {
                    expected: 0,
                    actual: 1,
                })
            );
        }
    }

    #[test]
    fn exhaustive_u16_domain_matches_independent_coefficient_oracle() {
        for polynomial in 0_u16..=u16::MAX {
            let expected = coefficient_derivative(polynomial);
            assert_eq!(
                check_frozen_gf2_polynomial_derivative(&frame(polynomial, expected)),
                Ok(()),
                "polynomial={polynomial:#06x}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_derivative(&frame(polynomial, expected ^ 1)),
                Err(Gf2PolynomialDerivativeError::DerivativeMismatch {
                    expected,
                    actual: expected ^ 1,
                }),
                "forged polynomial={polynomial:#06x}"
            );
        }
    }

    #[test]
    fn high_degree_and_even_power_boundaries() {
        for (polynomial, derivative) in [
            (0x8000, 0x4000), // x^15 -> x^14
            (0x4000, 0x0000), // x^14 -> 0
            (0x8002, 0x4001), // x^15 + x -> x^14 + 1
            (0x8003, 0x4001), // x^15 + x + 1 -> x^14 + 1
            (0xffff, 0x5555), // all odd powers survive with exponent - 1
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_derivative(&frame(polynomial, derivative)),
                Ok(())
            );
        }
    }
}
