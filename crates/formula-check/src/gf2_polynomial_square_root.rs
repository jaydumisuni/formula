//! Independent bounded GF(2)[x] square-root witness preflight.
//! Source-only P2 preparation: no Judgement, certificate, or promotion.
//!
//! Frozen v0 frame: big-endian u16 polynomial followed by big-endian u16
//! claimed root. A polynomial is a square in GF(2)[x] exactly when every
//! odd-exponent coefficient vanishes. Zero and one have unique roots.

const FRAME_BYTES: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialSquareRootError {
    InvalidFrame,
    NotPerfectSquare,
    RootMismatch { expected: u16, actual: u16 },
}

/// Independently check the exact square root of a frozen GF(2) polynomial.
///
/// Frobenius squaring doubles exponents without cross terms in characteristic
/// two. The root is unique; a degree-15 input has a root of degree at most 7.
/// This is a bounded evidence-body check, never mathematical admission.
pub fn check_frozen_gf2_polynomial_square_root(
    body: &[u8],
) -> Result<(), Gf2PolynomialSquareRootError> {
    let bytes: &[u8; FRAME_BYTES] = body
        .try_into()
        .map_err(|_| Gf2PolynomialSquareRootError::InvalidFrame)?;
    let polynomial = u16::from_be_bytes([bytes[0], bytes[1]]);
    let claimed = u16::from_be_bytes([bytes[2], bytes[3]]);

    if polynomial & 0xaaaa != 0 {
        return Err(Gf2PolynomialSquareRootError::NotPerfectSquare);
    }

    let mut expected = 0_u16;
    for exponent in 0..8 {
        if polynomial & (1_u16 << (2 * exponent)) != 0 {
            expected |= 1_u16 << exponent;
        }
    }
    if claimed != expected {
        return Err(Gf2PolynomialSquareRootError::RootMismatch {
            expected,
            actual: claimed,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(polynomial: u16, root: u16) -> [u8; FRAME_BYTES] {
        let mut bytes = [0_u8; FRAME_BYTES];
        bytes[..2].copy_from_slice(&polynomial.to_be_bytes());
        bytes[2..].copy_from_slice(&root.to_be_bytes());
        bytes
    }

    // Independent polynomial multiplication, not the production parity
    // mask or even-coefficient extraction algorithm.
    fn carryless_square(root: u16) -> u32 {
        let mut product = 0_u32;
        for bit in 0..8 {
            if root & (1_u16 << bit) != 0 {
                product ^= u32::from(root) << bit;
            }
        }
        product
    }

    #[test]
    fn exact_frame_and_high_root_bits_fail_closed() {
        let valid = frame(0x4001, 0x0081);
        for length in 0..FRAME_BYTES {
            assert_eq!(
                check_frozen_gf2_polynomial_square_root(&valid[..length]),
                Err(Gf2PolynomialSquareRootError::InvalidFrame)
            );
        }
        let mut padded = valid.to_vec();
        padded.push(0);
        assert_eq!(
            check_frozen_gf2_polynomial_square_root(&padded),
            Err(Gf2PolynomialSquareRootError::InvalidFrame)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_square_root(&frame(0x4001, 0x0181)),
            Err(Gf2PolynomialSquareRootError::RootMismatch {
                expected: 0x0081,
                actual: 0x0181,
            })
        );
    }

    #[test]
    fn exhaustive_u16_domain_matches_independent_carryless_squares() {
        let mut roots = vec![None; 1 << 16];
        for root in 0_u16..=255 {
            let polynomial = carryless_square(root) as usize;
            assert!(roots[polynomial].replace(root).is_none());
        }

        for polynomial in 0_u16..=u16::MAX {
            match roots[usize::from(polynomial)] {
                Some(root) => {
                    assert_eq!(
                        check_frozen_gf2_polynomial_square_root(&frame(polynomial, root)),
                        Ok(()),
                        "polynomial={polynomial:#06x}"
                    );
                    assert_eq!(
                        check_frozen_gf2_polynomial_square_root(&frame(polynomial, root ^ 1)),
                        Err(Gf2PolynomialSquareRootError::RootMismatch {
                            expected: root,
                            actual: root ^ 1,
                        }),
                        "forged root for polynomial={polynomial:#06x}"
                    );
                }
                None => {
                    assert_eq!(
                        check_frozen_gf2_polynomial_square_root(&frame(polynomial, 0)),
                        Err(Gf2PolynomialSquareRootError::NotPerfectSquare),
                        "nonsquare polynomial={polynomial:#06x}"
                    );
                }
            }
        }
    }

    #[test]
    fn zero_unit_high_degree_and_odd_exponents() {
        for (polynomial, root) in [
            (0x0000, 0x0000),
            (0x0001, 0x0001),
            (0x0004, 0x0002),
            (0x4000, 0x0080),
            (0x5555, 0x00ff),
            (0x4001, 0x0081),
        ] {
            assert_eq!(
                check_frozen_gf2_polynomial_square_root(&frame(polynomial, root)),
                Ok(())
            );
        }
        for polynomial in [0x0002, 0x0003, 0x8000, 0xffff] {
            assert_eq!(
                check_frozen_gf2_polynomial_square_root(&frame(polynomial, 0)),
                Err(Gf2PolynomialSquareRootError::NotPerfectSquare)
            );
        }
    }
}
