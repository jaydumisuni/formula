//! Non-authoritative P2 evidence-metadata preflight.
//!
//! This is an isolated preparation slice, not a certificate checker, promotion
//! decision, or completion of P1/P2. It only binds a recorded metadata envelope
//! to the caller's frozen judgement, exact payload bytes, and pinned checker.
//! Proof semantics, trust-root admission, and authority-contract policy remain
//! independent unimplemented gates.

use formula_core::{ArtifactDigest, EvidenceEnvelopeMetadata, Judgement};

/// Inputs pinned by the caller before any independent evidence-body checking.
pub struct FrozenEvidence<'a> {
    pub judgement: &'a Judgement,
    pub evidence_family: &'a str,
    pub payload_bytes: &'a [u8],
    pub checker_digest: ArtifactDigest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidencePreflightError {
    InvalidFamily,
    BindingMismatch,
}

/// Reject metadata substitution without claiming that evidence is sound.
///
/// The judgement's structural digest commits to its world, proposition, and
/// dependencies. The payload digest commits to exact supplied bytes. Matching
/// these fields is necessary but never sufficient for certification.
pub fn preflight_evidence_metadata(
    recorded: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), EvidencePreflightError> {
    if frozen.evidence_family.is_empty() || frozen.evidence_family.trim() != frozen.evidence_family
    {
        return Err(EvidencePreflightError::InvalidFamily);
    }
    let expected = EvidenceEnvelopeMetadata::new(
        frozen.judgement.structural_digest(),
        frozen.evidence_family,
        ArtifactDigest::sha256(frozen.payload_bytes),
        frozen.checker_digest,
    );
    if recorded.canonical_bytes() != expected.canonical_bytes() {
        return Err(EvidencePreflightError::BindingMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(tag: &[u8]) -> ArtifactDigest {
        ArtifactDigest::sha256(tag)
    }

    fn judgement(world: ArtifactDigest, dependencies: Vec<ArtifactDigest>) -> Judgement {
        Judgement::new(world, "finite-exact", b"x == x".to_vec(), dependencies)
    }

    #[test]
    fn exact_frozen_binding_is_only_preflighted() {
        let j = judgement(digest(b"world"), vec![digest(b"dependency")]);
        let checker = digest(b"checker-v1");
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "finite-exhaustive",
            payload_bytes: b"proof-body-v1",
            checker_digest: checker,
        };
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(frozen.payload_bytes),
            checker,
        );
        assert_eq!(preflight_evidence_metadata(&recorded, &frozen), Ok(()));
    }

    #[test]
    fn substituted_target_world_and_dependencies_fail_closed() {
        let j = judgement(digest(b"world"), vec![digest(b"dependency")]);
        let altered_world = judgement(digest(b"other-world"), vec![digest(b"dependency")]);
        let altered_dependencies = judgement(digest(b"world"), vec![digest(b"other-dependency")]);
        let checker = digest(b"checker-v1");
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            "finite-exhaustive",
            digest(b"proof"),
            checker,
        );
        for wrong in [&altered_world, &altered_dependencies] {
            let frozen = FrozenEvidence {
                judgement: wrong,
                evidence_family: "finite-exhaustive",
                payload_bytes: b"proof",
                checker_digest: checker,
            };
            assert_eq!(
                preflight_evidence_metadata(&recorded, &frozen),
                Err(EvidencePreflightError::BindingMismatch)
            );
        }
        let forged = EvidenceEnvelopeMetadata::new(
            digest(b"unrelated-target"),
            "finite-exhaustive",
            digest(b"proof"),
            checker,
        );
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "finite-exhaustive",
            payload_bytes: b"proof",
            checker_digest: checker,
        };
        assert_eq!(
            preflight_evidence_metadata(&forged, &frozen),
            Err(EvidencePreflightError::BindingMismatch)
        );
    }

    #[test]
    fn changed_payload_family_or_checker_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker-v1");
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            "finite-exhaustive",
            digest(b"proof"),
            checker,
        );
        for (family, body, version) in [
            ("finite-exhaustive", b"altered".as_slice(), checker),
            ("untrusted-family", b"proof".as_slice(), checker),
            (
                "finite-exhaustive",
                b"proof".as_slice(),
                digest(b"checker-v2"),
            ),
        ] {
            let frozen = FrozenEvidence {
                judgement: &j,
                evidence_family: family,
                payload_bytes: body,
                checker_digest: version,
            };
            assert_eq!(
                preflight_evidence_metadata(&recorded, &frozen),
                Err(EvidencePreflightError::BindingMismatch)
            );
        }
    }

    #[test]
    fn empty_or_whitespace_padded_family_is_rejected() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            "finite-exhaustive",
            digest(b"proof"),
            checker,
        );
        for family in ["", " finite-exhaustive", "finite-exhaustive "] {
            let frozen = FrozenEvidence {
                judgement: &j,
                evidence_family: family,
                payload_bytes: b"proof",
                checker_digest: checker,
            };
            assert_eq!(
                preflight_evidence_metadata(&recorded, &frozen),
                Err(EvidencePreflightError::InvalidFamily)
            );
        }
    }
}
