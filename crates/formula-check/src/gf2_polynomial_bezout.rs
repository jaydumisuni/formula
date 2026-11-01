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
                        if carryless_product(lhs, s) ^ carryless_product(rhs, t) == u32::from(gcd) {
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
        let verify = |bytes: &[u8], recorded: &EvidenceEnvelopeMetadata, candidate: &FrozenEvidence<'_>| {
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
            verify(substituted_bytes, &substituted_metadata, &substituted_frozen),
            Err(Gf2PolynomialBezoutBundleError::CheckerBytesPinMismatch)
        );
    }

}
