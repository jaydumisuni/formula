//! Independent bounded GF(2)[x] Bezout witness preflight.
//! Source-only P2 preparation: no certificate, Judgement, or promotion.
//!
//! Frozen v0 frame: five big-endian u16 coefficient bitsets:
//! lhs, rhs, claimed GCD, lhs coefficient, rhs coefficient.
//! Both-zero operands are undefined; all other pairs have a monic GCD.

use crate::{
    FrozenEvidence, PinnedEvidenceBundlePreflightError, preflight_pinned_frozen_evidence_bundle,
};
use formula_core::{ArtifactDigest, AuthorityContract, EvidenceEnvelopeMetadata};

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
pub fn check_frozen_gf2_polynomial_bezout(body: &[u8]) -> Result<(), Gf2PolynomialBezoutError> {
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

    let computed =
        carryless_product(lhs, lhs_coefficient) ^ carryless_product(rhs, rhs_coefficient);
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

/// Frozen-policy, independent-pin, and exact GF(2)[x] Bezout preflight.
/// No trust root, certificate, Judgement semantics, or promotion is conferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2PolynomialBezoutBundleError {
    ContractPinMismatch,
    JudgementPinMismatch,
    PayloadPinMismatch,
    CheckerBytesPinMismatch,
    EmptyCheckerArtifact,
    Bundle(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(Gf2PolynomialBezoutError),
}

/// Require caller-authenticated contract, Judgement, checker and payload pins
/// before checking the exact frozen Bezout relation. Producer-rehashed evidence
/// must not substitute for independently pinned bytes. This is preparation
/// only: the caller still must authenticate the pins and the Judgement meaning.
#[allow(clippy::too_many_arguments)]
pub fn preflight_pinned_frozen_gf2_polynomial_bezout(
    independently_pinned_judgement: ArtifactDigest,
    independently_pinned_payload: ArtifactDigest,
    independently_pinned_checker: ArtifactDigest,
    independently_pinned_contract: ArtifactDigest,
    recorded_contract: ArtifactDigest,
    contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), Gf2PolynomialBezoutBundleError> {
    if independently_pinned_contract != recorded_contract
        || contract.structural_digest() != independently_pinned_contract
    {
        return Err(Gf2PolynomialBezoutBundleError::ContractPinMismatch);
    }
    if frozen.judgement.structural_digest() != independently_pinned_judgement {
        return Err(Gf2PolynomialBezoutBundleError::JudgementPinMismatch);
    }
    if ArtifactDigest::sha256(frozen.payload_bytes) != independently_pinned_payload {
        return Err(Gf2PolynomialBezoutBundleError::PayloadPinMismatch);
    }
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker,
        recorded_contract,
        contract,
        recorded_metadata,
        frozen,
    )
    .map_err(Gf2PolynomialBezoutBundleError::Bundle)?;
    if frozen.evidence_family != "gf2-polynomial-bezout" {
        return Err(Gf2PolynomialBezoutBundleError::WrongEvidenceFamily);
    }
    check_frozen_gf2_polynomial_bezout(frozen.payload_bytes)
        .map_err(Gf2PolynomialBezoutBundleError::Body)
}

/// Bind the frozen checker artifact bytes to a separately authenticated
/// checker digest before accepting the evidence bundle. Matching recorded
/// digests alone do not prove the identity of the actual checker bytes.
/// This is structural custody only, not execution, certification or promotion.
#[allow(clippy::too_many_arguments)]
pub fn preflight_pinned_frozen_gf2_polynomial_bezout_with_checker_bytes(
    independently_pinned_judgement: ArtifactDigest,
    independently_pinned_payload: ArtifactDigest,
    independently_pinned_checker: ArtifactDigest,
    independently_pinned_contract: ArtifactDigest,
    recorded_contract: ArtifactDigest,
    contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    checker_bytes: &[u8],
    frozen: &FrozenEvidence<'_>,
) -> Result<(), Gf2PolynomialBezoutBundleError> {
    // A digest can authenticate zero bytes, but zero bytes are not a checker
    // artifact. Reject even when the producer and independent pin agree.
    if checker_bytes.is_empty() {
        return Err(Gf2PolynomialBezoutBundleError::EmptyCheckerArtifact);
    }
    if ArtifactDigest::sha256(checker_bytes) != independently_pinned_checker {
        return Err(Gf2PolynomialBezoutBundleError::CheckerBytesPinMismatch);
    }
    preflight_pinned_frozen_gf2_polynomial_bezout(
        independently_pinned_judgement,
        independently_pinned_payload,
        independently_pinned_checker,
        independently_pinned_contract,
        recorded_contract,
        contract,
        recorded_metadata,
        frozen,
    )
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

    // Independent schoolbook convolution: never reuse the production
    // carryless_product in the oracle that judges its polynomial identities.
    fn reference_product(lhs: u16, rhs: u16) -> u32 {
        let mut product = 0_u32;
        for left_degree in 0..16 {
            for right_degree in 0..16 {
                if lhs & (1_u16 << left_degree) != 0 && rhs & (1_u16 << right_degree) != 0 {
                    product ^= 1_u32 << (left_degree + right_degree);
                }
            }
        }
        product
    }

    // Reference oracle enumerates exact factor/quotient pairs, not Euclid.
    fn divides(divisor: u16, dividend: u16) -> bool {
        (0_u16..16).any(|quotient| reference_product(divisor, quotient) == u32::from(dividend))
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
                        if reference_product(lhs, s) ^ reference_product(rhs, t) == u32::from(gcd) {
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
    fn exhaustive_small_domain_rejects_forged_bezout_coefficients_and_gcds() {
        // All 255 nonzero operand pairs and all 256 bounded coefficient pairs.
        // The expected result comes from the independent convolution oracle,
        // not from the multiplication routine under test.
        for lhs in 0_u16..16 {
            for rhs in 0_u16..16 {
                if lhs == 0 && rhs == 0 {
                    continue;
                }
                let gcd = independent_small_gcd(lhs, rhs);
                for s in 0_u16..16 {
                    for t in 0_u16..16 {
                        let computed = reference_product(lhs, s) ^ reference_product(rhs, t);
                        let expected = if computed == u32::from(gcd) {
                            Ok(())
                        } else {
                            Err(Gf2PolynomialBezoutError::IdentityMismatch {
                                computed,
                                claimed: gcd,
                            })
                        };
                        assert_eq!(
                            check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, t)),
                            expected,
                            "lhs={lhs:#b} rhs={rhs:#b} s={s:#b} t={t:#b}"
                        );
                    }
                }
                let forged_gcd = gcd ^ 1;
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, forged_gcd, 0, 0)),
                    Err(Gf2PolynomialBezoutError::GcdMismatch {
                        expected: gcd,
                        actual: forged_gcd,
                    }),
                    "lhs={lhs:#b} rhs={rhs:#b}"
                );
            }
        }
    }

    #[test]
    fn full_width_zero_operand_witnesses_use_independent_convolution() {
        // A zero operand fixes the monic GCD exactly to the nonzero operand.
        // Exercise both operand positions, including degree-30 products that
        // would be silently truncated by a u16 multiplication oracle.
        for nonzero in [0x8000_u16, 0xffff, 0x8001, 0x7fff, 0x5555, 0xaaaa] {
            for coefficient in [0_u16, 1, 2, 0x4001, 0x8000, 0xffff] {
                let computed = reference_product(nonzero, coefficient);
                let expected = if computed == u32::from(nonzero) {
                    Ok(())
                } else {
                    Err(Gf2PolynomialBezoutError::IdentityMismatch {
                        computed,
                        claimed: nonzero,
                    })
                };
                for body in [
                    frame(nonzero, 0, nonzero, coefficient, 0),
                    frame(0, nonzero, nonzero, 0, coefficient),
                ] {
                    assert_eq!(
                        check_frozen_gf2_polynomial_bezout(&body),
                        expected,
                        "nonzero={nonzero:#b} coefficient={coefficient:#b} body={body:?}"
                    );
                }
            }
        }
    }

    // This oracle uses wide polynomial long division and an independent
    // extended-Euclid coefficient trace. Neither the production GCD loop
    // nor its carryless_product is used to derive expected witnesses.
    fn reference_div_rem(mut numerator: u32, denominator: u32) -> (u32, u32) {
        assert_ne!(denominator, 0);
        let mut quotient = 0_u32;
        while numerator != 0 && numerator.leading_zeros() <= denominator.leading_zeros() {
            let shift = denominator.leading_zeros() - numerator.leading_zeros();
            quotient ^= 1_u32 << shift;
            numerator ^= denominator << shift;
        }
        (quotient, numerator)
    }

    fn reference_wide_product(lhs: u32, rhs: u64) -> u64 {
        let mut product = 0_u64;
        for bit in 0..32 {
            if lhs & (1_u32 << bit) != 0 {
                product ^= rhs << bit;
            }
        }
        product
    }

    fn independent_extended_gcd(lhs: u16, rhs: u16) -> (u32, u64, u64) {
        let (mut old_r, mut r) = (u32::from(lhs), u32::from(rhs));
        let (mut old_s, mut s) = (1_u64, 0_u64);
        let (mut old_t, mut t) = (0_u64, 1_u64);
        while r != 0 {
            let (quotient, remainder) = reference_div_rem(old_r, r);
            let next_s = old_s ^ reference_wide_product(quotient, s);
            let next_t = old_t ^ reference_wide_product(quotient, t);
            (old_r, r) = (r, remainder);
            (old_s, s) = (s, next_s);
            (old_t, t) = (t, next_t);
        }
        (old_r, old_s, old_t)
    }

    #[test]
    fn full_width_nonzero_pairs_have_independently_derived_bezout_witnesses() {
        // PR #64 exercises high-degree multiplication with one zero operand.
        // These nonzero/nonzero pairs also exercise the actual Euclidean
        // reduction path and independent high-degree Bezout coefficients.
        for (lhs, rhs) in [
            (0x8000_u16, 0x8001_u16),
            (0x8001, 0x8003),
            (0xffff, 0xaaaa),
            (0xbeef, 0xdead),
            (0xe00d, 0xf00d),
            (0xffff, 0xffff),
            (0x5555, 0xaaaa),
            (0xf0f0, 0x0f0f),
            (0x8000, 0x0003),
            (0x0001, 0xffff),
            (0x8001, 0x8001),
            (0x7fff, 0x8000),
        ] {
            let (gcd_wide, s_wide, t_wide) = independent_extended_gcd(lhs, rhs);
            assert!(gcd_wide <= u32::from(u16::MAX));
            assert!(s_wide <= u64::from(u16::MAX));
            assert!(t_wide <= u64::from(u16::MAX));
            let (gcd, s, t) = (gcd_wide as u16, s_wide as u16, t_wide as u16);
            assert_eq!(
                reference_product(lhs, s) ^ reference_product(rhs, t),
                u32::from(gcd),
                "independent witness lhs={lhs:#x} rhs={rhs:#x}"
            );
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, t)),
                Ok(()),
                "valid lhs={lhs:#x} rhs={rhs:#x}"
            );
            let forged_gcd = gcd ^ 1;
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, forged_gcd, s, t)),
                Err(Gf2PolynomialBezoutError::GcdMismatch {
                    expected: gcd,
                    actual: forged_gcd,
                })
            );
            let forged_s = s ^ 1;
            let computed = reference_product(lhs, forged_s) ^ reference_product(rhs, t);
            assert_ne!(computed, u32::from(gcd));
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, forged_s, t)),
                Err(Gf2PolynomialBezoutError::IdentityMismatch {
                    computed,
                    claimed: gcd,
                })
            );

            // The independent oracle must exercise the other coefficient as
            // well: a checker that ignored t could still pass the s-only
            // adversarial matrix. Since rhs is nonzero, flipping t's constant
            // term necessarily changes the exact polynomial identity.
            let forged_t = t ^ 1;
            let computed_t = reference_product(lhs, s) ^ reference_product(rhs, forged_t);
            assert_ne!(computed_t, u32::from(gcd));
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, forged_t)),
                Err(Gf2PolynomialBezoutError::IdentityMismatch {
                    computed: computed_t,
                    claimed: gcd,
                }),
                "forged t lhs={lhs:#x} rhs={rhs:#x}"
            );

            // Bezout is symmetric under swapping both operands and their
            // corresponding coefficients; verify this with the same
            // independently derived full-width witness.
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(rhs, lhs, gcd, t, s)),
                Ok(()),
                "swapped lhs={lhs:#x} rhs={rhs:#x}"
            );
        }
    }

    #[test]
    fn full_width_cancellation_rejects_forgeries_hidden_by_u16_truncation() {
        // Both products exceed u16, but their high terms cancel in GF(2)[x].
        // A checker that truncates before XOR would admit the forged pairs.
        let (lhs, rhs, gcd) = (0x8000_u16, 0x8000_u16, 0x8000_u16);
        let (s, t) = (0x8001_u16, 0x8000_u16);
        assert_eq!(reference_product(lhs, s), 0x4000_8000);
        assert_eq!(reference_product(rhs, t), 0x4000_0000);
        assert_eq!(
            reference_product(lhs, s) ^ reference_product(rhs, t),
            u32::from(gcd)
        );
        assert_eq!(
            check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, t)),
            Ok(())
        );

        for (forged_s, forged_t) in [(s, 0_u16), (1_u16, t)] {
            let computed = reference_product(lhs, forged_s) ^ reference_product(rhs, forged_t);
            assert_eq!(computed, 0x4000_8000);
            assert_eq!(computed & 0xffff, u32::from(gcd));
            assert_ne!(computed, u32::from(gcd));
            assert_eq!(
                check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, forged_s, forged_t)),
                Err(Gf2PolynomialBezoutError::IdentityMismatch {
                    computed,
                    claimed: gcd,
                })
            );
        }
    }

    #[test]
    fn full_width_bezout_witnesses_covary_under_monomial_shift() {
        // Multiplying both operands by x^k must multiply the monic GCD by
        // x^k while preserving the original independently derived Bezout
        // coefficients. The reference products remain u32 throughout.
        for (lhs, rhs) in [
            (0x0013_u16, 0x001d_u16),
            (0x001b, 0x0015),
            (0x0021, 0x0031),
            (0x0003, 0x0005),
        ] {
            let (gcd_wide, s_wide, t_wide) = independent_extended_gcd(lhs, rhs);
            assert!(gcd_wide <= u32::from(u16::MAX));
            assert!(s_wide <= u64::from(u16::MAX));
            assert!(t_wide <= u64::from(u16::MAX));
            let (gcd, s, t) = (gcd_wide as u16, s_wide as u16, t_wide as u16);
            assert_eq!(
                reference_product(lhs, s) ^ reference_product(rhs, t),
                u32::from(gcd)
            );

            for shift in 1..16 {
                if (u32::from(lhs) << shift) > u32::from(u16::MAX)
                    || (u32::from(rhs) << shift) > u32::from(u16::MAX)
                {
                    break;
                }
                let (shifted_lhs, shifted_rhs, shifted_gcd) =
                    (lhs << shift, rhs << shift, gcd << shift);
                let expected =
                    reference_product(shifted_lhs, s) ^ reference_product(shifted_rhs, t);
                assert_eq!(expected, u32::from(shifted_gcd));
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        shifted_lhs,
                        shifted_rhs,
                        shifted_gcd,
                        s,
                        t
                    )),
                    Ok(()),
                    "lhs={lhs:#x} rhs={rhs:#x} shift={shift}"
                );

                let forged_gcd = shifted_gcd ^ 1;
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        shifted_lhs,
                        shifted_rhs,
                        forged_gcd,
                        s,
                        t
                    )),
                    Err(Gf2PolynomialBezoutError::GcdMismatch {
                        expected: shifted_gcd,
                        actual: forged_gcd,
                    })
                );

                let forged_s = s ^ 1;
                let forged_product =
                    reference_product(shifted_lhs, forged_s) ^ reference_product(shifted_rhs, t);
                assert_ne!(forged_product, u32::from(shifted_gcd));
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        shifted_lhs,
                        shifted_rhs,
                        shifted_gcd,
                        forged_s,
                        t
                    )),
                    Err(Gf2PolynomialBezoutError::IdentityMismatch {
                        computed: forged_product,
                        claimed: shifted_gcd,
                    })
                );

                // The shifted witness must still depend on the *other*
                // coefficient. Independently compute the forged polynomial
                // instead of trusting the production multiplication path.
                let forged_t = t ^ 1;
                let forged_t_product =
                    reference_product(shifted_lhs, s) ^ reference_product(shifted_rhs, forged_t);
                assert_ne!(forged_t_product, u32::from(shifted_gcd));
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        shifted_lhs,
                        shifted_rhs,
                        shifted_gcd,
                        s,
                        forged_t
                    )),
                    Err(Gf2PolynomialBezoutError::IdentityMismatch {
                        computed: forged_t_product,
                        claimed: shifted_gcd,
                    }),
                    "forged shifted t lhs={lhs:#x} rhs={rhs:#x} shift={shift}"
                );

                // The same independently derived relation must survive
                // operand/coefficient exchange after the monomial shift.
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        shifted_rhs,
                        shifted_lhs,
                        shifted_gcd,
                        t,
                        s
                    )),
                    Ok(()),
                    "swapped shifted lhs={lhs:#x} rhs={rhs:#x} shift={shift}"
                );
            }
        }
    }

    #[test]
    fn full_width_bezout_syzygy_family_preserves_noncanonical_witnesses() {
        // If (s,t) is a Bezout witness, adding (rhs/gcd)*q to s and
        // (lhs/gcd)*q to t preserves the relation in characteristic two.
        // Derive both quotients by independent wide polynomial division.
        // This catches a checker that accepts only one canonical Euclid
        // witness, truncates cancelling high terms, or ignores a coefficient.
        let mut accepted = 0;
        let mut high_term_cancellations = 0;
        for (lhs, rhs) in [
            (0x8001_u16, 0x8003_u16),
            (0x8000, 0x8001),
            (0x00f0, 0x005a),
            (0x0036, 0x001e),
            (0x0013, 0x001d),
            (0x0021, 0x0031),
            (0x00ff, 0x00aa),
            (0xffff, 0xaaaa),
            (0xdead, 0xbeef),
        ] {
            let (gcd_wide, s_wide, t_wide) = independent_extended_gcd(lhs, rhs);
            let gcd = u16::try_from(gcd_wide).expect("bounded GCD");
            assert!(s_wide <= u64::from(u16::MAX));
            assert!(t_wide <= u64::from(u16::MAX));
            let (lhs_quotient, lhs_remainder) = reference_div_rem(u32::from(lhs), gcd_wide);
            let (rhs_quotient, rhs_remainder) = reference_div_rem(u32::from(rhs), gcd_wide);
            assert_eq!((lhs_remainder, rhs_remainder), (0, 0));

            for q in [1_u16, 2, 3, 0x11, 0x80, 0x100] {
                let s_next = s_wide ^ reference_wide_product(rhs_quotient, u64::from(q));
                let t_next = t_wide ^ reference_wide_product(lhs_quotient, u64::from(q));
                if s_next > u64::from(u16::MAX) || t_next > u64::from(u16::MAX) {
                    continue;
                }
                assert_ne!((s_next, t_next), (s_wide, t_wide));
                let (s, t) = (s_next as u16, t_next as u16);
                let left = reference_product(lhs, s);
                let right = reference_product(rhs, t);
                assert_eq!(left ^ right, u32::from(gcd));
                if left > u32::from(u16::MAX) && right > u32::from(u16::MAX) {
                    high_term_cancellations += 1;
                }
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(lhs, rhs, gcd, s, t)),
                    Ok(()),
                    "noncanonical lhs={lhs:#x} rhs={rhs:#x} q={q:#x}"
                );
                for (forged_s, forged_t) in [(s ^ 1, t), (s, t ^ 1)] {
                    let computed =
                        reference_product(lhs, forged_s) ^ reference_product(rhs, forged_t);
                    assert_ne!(computed, u32::from(gcd));
                    assert_eq!(
                        check_frozen_gf2_polynomial_bezout(&frame(
                            lhs, rhs, gcd, forged_s, forged_t
                        )),
                        Err(Gf2PolynomialBezoutError::IdentityMismatch {
                            computed,
                            claimed: gcd,
                        }),
                        "one-sided forgery lhs={lhs:#x} rhs={rhs:#x} q={q:#x}"
                    );
                }
                accepted += 1;
            }
        }
        assert!(
            accepted >= 35,
            "insufficient noncanonical witnesses: {accepted}"
        );
        assert!(
            high_term_cancellations >= 2,
            "high-degree cancellation cases were not exercised"
        );
    }

    #[test]
    fn nonmonomial_common_factors_preserve_exact_bezout_witnesses() {
        // Scaling both operands by a nonmonomial GF(2) polynomial scales
        // their GCD, not their Bezout coefficients. Unlike x^k covariance,
        // this exercises multi-term convolution and cancellation. Derive
        // the unscaled witness through the independent Euclidean oracle.
        let mut checked = 0;
        let mut nontrivial_base_gcds = 0;
        for (lhs, rhs) in [
            (0x0013_u16, 0x001d_u16),
            (0x002b, 0x0035),
            (0x0101, 0x0111),
            (0x000a, 0x000e),
            (0x0036, 0x001e),
            (0x002a, 0x003e),
            (0x0003, 0x0005),
        ] {
            let (gcd_wide, s_wide, t_wide) = independent_extended_gcd(lhs, rhs);
            let (gcd, s, t) = (
                u16::try_from(gcd_wide).expect("bounded GCD"),
                u16::try_from(s_wide).expect("bounded coefficient s"),
                u16::try_from(t_wide).expect("bounded coefficient t"),
            );
            assert_eq!(
                reference_product(lhs, s) ^ reference_product(rhs, t),
                u32::from(gcd)
            );
            for factor in [0b11_u16, 0b101, 0b111, 0b1011, 0b10011, 0b11001, 0b11111] {
                assert!(factor.count_ones() > 1);
                let scaled_lhs =
                    u16::try_from(reference_product(lhs, factor)).expect("bounded lhs");
                let scaled_rhs =
                    u16::try_from(reference_product(rhs, factor)).expect("bounded rhs");
                let scaled_gcd =
                    u16::try_from(reference_product(gcd, factor)).expect("bounded GCD");
                assert_eq!(
                    reference_product(scaled_lhs, s) ^ reference_product(scaled_rhs, t),
                    u32::from(scaled_gcd)
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        scaled_lhs, scaled_rhs, scaled_gcd, s, t
                    )),
                    Ok(()),
                    "lhs={lhs:#x} rhs={rhs:#x} factor={factor:#x}"
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        scaled_rhs, scaled_lhs, scaled_gcd, t, s
                    )),
                    Ok(())
                );
                let forged_gcd = scaled_gcd ^ 1;
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(
                        scaled_lhs, scaled_rhs, forged_gcd, s, t
                    )),
                    Err(Gf2PolynomialBezoutError::GcdMismatch {
                        expected: scaled_gcd,
                        actual: forged_gcd,
                    })
                );
                for (forged_s, forged_t) in [(s ^ 1, t), (s, t ^ 1)] {
                    let computed = reference_product(scaled_lhs, forged_s)
                        ^ reference_product(scaled_rhs, forged_t);
                    assert_ne!(computed, u32::from(scaled_gcd));
                    assert_eq!(
                        check_frozen_gf2_polynomial_bezout(&frame(
                            scaled_lhs, scaled_rhs, scaled_gcd, forged_s, forged_t
                        )),
                        Err(Gf2PolynomialBezoutError::IdentityMismatch {
                            computed,
                            claimed: scaled_gcd,
                        })
                    );
                }
                checked += 1;
                if gcd != 1 {
                    nontrivial_base_gcds += 1;
                }
            }
        }
        assert_eq!(checked, 49);
        assert!(nontrivial_base_gcds >= 14);
    }

    #[test]
    fn zero_operand_accepts_noncanonical_irrelevant_coefficient() {
        // A Bezout coefficient multiplying zero is semantically unconstrained.
        // Exact verification must not impose a false canonical-zero rule.
        for nonzero in [0x8000_u16, 0xffff] {
            for irrelevant in [0_u16, 1, 0x8000, 0xffff] {
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(nonzero, 0, nonzero, 1, irrelevant)),
                    Ok(())
                );
                assert_eq!(
                    check_frozen_gf2_polynomial_bezout(&frame(0, nonzero, nonzero, irrelevant, 1)),
                    Ok(())
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
    #[test]
    fn pinned_bezout_bundle_checks_frozen_identity_and_exact_relation() {
        let judgement = formula_core::Judgement::new(
            ArtifactDigest::sha256(b"bezout-world"),
            "finite-exact",
            b"bezout-relation".to_vec(),
            vec![ArtifactDigest::sha256(b"bezout-dependency")],
        );
        let target_pin = judgement.structural_digest();
        let checker_pin = ArtifactDigest::sha256(b"independent-bezout-checker");
        let contract = AuthorityContract::new(vec!["gf2-polynomial-bezout".to_owned()]);
        let contract_pin = contract.structural_digest();
        let valid = frame(0b1011, 0b1101, 1, 2, 3);
        let frozen = FrozenEvidence {
            judgement: &judgement,
            evidence_family: "gf2-polynomial-bezout",
            payload_bytes: &valid,
            checker_digest: checker_pin,
        };
        let metadata = EvidenceEnvelopeMetadata::new(
            target_pin,
            frozen.evidence_family,
            ArtifactDigest::sha256(&valid),
            checker_pin,
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                target_pin,
                ArtifactDigest::sha256(&valid),
                checker_pin,
                contract_pin,
                contract_pin,
                &contract,
                &metadata,
                &frozen,
            ),
            Ok(())
        );

        // A false relation remains false even when its producer rehashes
        // the payload and supplies a self-consistent metadata envelope.
        let forged = frame(0b1011, 0b1101, 1, 0, 0);
        let forged_frozen = FrozenEvidence {
            payload_bytes: &forged,
            ..frozen
        };
        let forged_metadata = EvidenceEnvelopeMetadata::new(
            target_pin,
            forged_frozen.evidence_family,
            ArtifactDigest::sha256(&forged),
            checker_pin,
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                target_pin,
                ArtifactDigest::sha256(&forged),
                checker_pin,
                contract_pin,
                contract_pin,
                &contract,
                &forged_metadata,
                &forged_frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::Body(
                Gf2PolynomialBezoutError::IdentityMismatch {
                    computed: 0,
                    claimed: 1,
                }
            ))
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                target_pin,
                ArtifactDigest::sha256(&valid),
                checker_pin,
                contract_pin,
                contract_pin,
                &contract,
                &forged_metadata,
                &forged_frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::PayloadPinMismatch)
        );
    }

    #[test]
    fn pinned_bezout_bundle_rejects_contract_target_and_checker_substitution() {
        let judgement = formula_core::Judgement::new(
            ArtifactDigest::sha256(b"frozen-world"),
            "finite-exact",
            b"exact-bezout".to_vec(),
            vec![],
        );
        let target_pin = judgement.structural_digest();
        let checker_pin = ArtifactDigest::sha256(b"pinned-checker");
        let contract = AuthorityContract::new(vec!["gf2-polynomial-bezout".to_owned()]);
        let contract_pin = contract.structural_digest();
        let body = frame(0b1011, 0b1101, 1, 2, 3);
        let payload_pin = ArtifactDigest::sha256(&body);
        let frozen = FrozenEvidence {
            judgement: &judgement,
            evidence_family: "gf2-polynomial-bezout",
            payload_bytes: &body,
            checker_digest: checker_pin,
        };
        let metadata = EvidenceEnvelopeMetadata::new(
            target_pin,
            frozen.evidence_family,
            payload_pin,
            checker_pin,
        );
        let alien = ArtifactDigest::sha256(b"untrusted-substitution");
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                target_pin,
                payload_pin,
                checker_pin,
                alien,
                contract_pin,
                &contract,
                &metadata,
                &frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::ContractPinMismatch)
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                alien,
                payload_pin,
                checker_pin,
                contract_pin,
                contract_pin,
                &contract,
                &metadata,
                &frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::JudgementPinMismatch)
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                target_pin,
                payload_pin,
                alien,
                contract_pin,
                contract_pin,
                &contract,
                &metadata,
                &frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::Bundle(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );
    }

    #[test]
    fn pinned_bezout_bundle_rejects_other_admitted_family() {
        let judgement = formula_core::Judgement::new(
            ArtifactDigest::sha256(b"family-world"),
            "finite-exact",
            b"family-identity".to_vec(),
            vec![],
        );
        let checker_pin = ArtifactDigest::sha256(b"checker");
        let contract = AuthorityContract::new(vec!["unrelated-family".to_owned()]);
        let contract_pin = contract.structural_digest();
        let body = frame(0b1011, 0b1101, 1, 2, 3);
        let frozen = FrozenEvidence {
            judgement: &judgement,
            evidence_family: "unrelated-family",
            payload_bytes: &body,
            checker_digest: checker_pin,
        };
        let target_pin = judgement.structural_digest();
        let payload_pin = ArtifactDigest::sha256(&body);
        let metadata = EvidenceEnvelopeMetadata::new(
            target_pin,
            frozen.evidence_family,
            payload_pin,
            checker_pin,
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout(
                target_pin,
                payload_pin,
                checker_pin,
                contract_pin,
                contract_pin,
                &contract,
                &metadata,
                &frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::WrongEvidenceFamily)
        );
    }
    #[test]
    fn frozen_checker_artifact_bytes_require_independent_pin() {
        let judgement = formula_core::Judgement::new(
            ArtifactDigest::sha256(b"checker-world"),
            "finite-exact",
            b"bezout-custody".to_vec(),
            vec![],
        );
        let judgement_pin = judgement.structural_digest();
        let checker_bytes = b"frozen-bezout-checker";
        let checker_pin = ArtifactDigest::sha256(checker_bytes);
        let contract = AuthorityContract::new(vec!["gf2-polynomial-bezout".to_owned()]);
        let contract_pin = contract.structural_digest();
        let valid = frame(0b1011, 0b1101, 1, 2, 3);
        let payload_pin = ArtifactDigest::sha256(&valid);
        let frozen = FrozenEvidence {
            judgement: &judgement,
            evidence_family: "gf2-polynomial-bezout",
            payload_bytes: &valid,
            checker_digest: checker_pin,
        };
        let metadata = EvidenceEnvelopeMetadata::new(
            judgement_pin,
            frozen.evidence_family,
            payload_pin,
            checker_pin,
        );
        let verify =
            |bytes: &[u8], recorded: &EvidenceEnvelopeMetadata, candidate: &FrozenEvidence<'_>| {
                preflight_pinned_frozen_gf2_polynomial_bezout_with_checker_bytes(
                    judgement_pin,
                    payload_pin,
                    checker_pin,
                    contract_pin,
                    contract_pin,
                    &contract,
                    recorded,
                    bytes,
                    candidate,
                )
            };
        assert_eq!(verify(checker_bytes, &metadata, &frozen), Ok(()));
        // A self-consistent SHA-256(empty) envelope must not stand in for
        // actual frozen checker artifact bytes.
        let empty_checker_pin = ArtifactDigest::sha256(b"");
        let empty_frozen = FrozenEvidence {
            checker_digest: empty_checker_pin,
            ..frozen
        };
        let empty_metadata = EvidenceEnvelopeMetadata::new(
            judgement_pin,
            empty_frozen.evidence_family,
            payload_pin,
            empty_checker_pin,
        );
        assert_eq!(
            preflight_pinned_frozen_gf2_polynomial_bezout_with_checker_bytes(
                judgement_pin,
                payload_pin,
                empty_checker_pin,
                contract_pin,
                contract_pin,
                &contract,
                &empty_metadata,
                b"",
                &empty_frozen,
            ),
            Err(Gf2PolynomialBezoutBundleError::EmptyCheckerArtifact)
        );
        assert_eq!(
            verify(b"mutated-checker", &metadata, &frozen),
            Err(Gf2PolynomialBezoutBundleError::CheckerBytesPinMismatch)
        );
        let substituted_bytes = b"producer-replaced-checker";
        let substituted_pin = ArtifactDigest::sha256(substituted_bytes);
        let substituted_frozen = FrozenEvidence {
            checker_digest: substituted_pin,
            ..frozen
        };
        let substituted_metadata = EvidenceEnvelopeMetadata::new(
            judgement_pin,
            substituted_frozen.evidence_family,
            payload_pin,
            substituted_pin,
        );
        assert_eq!(
            verify(
                substituted_bytes,
                &substituted_metadata,
                &substituted_frozen
            ),
            Err(Gf2PolynomialBezoutBundleError::CheckerBytesPinMismatch)
        );
    }
}
