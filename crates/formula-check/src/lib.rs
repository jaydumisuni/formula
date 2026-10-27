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

/// Invisible Unicode format, combining, and variation-selector marks are not
/// visible evidence-family identity. Preserve meaningful visible Unicode names.
fn is_hidden_family_mark(c: char) -> bool {
    matches!(
        c,
        '\u{34f}'
            | '\u{ad}'
            | '\u{180b}'..='\u{180d}'
            | '\u{180f}'
            | '\u{fe00}'..='\u{fe0f}'
            | '\u{e0100}'..='\u{e01ef}'
            | '\u{600}'..='\u{605}'
            | '\u{6dd}'
            | '\u{70f}'
            | '\u{890}'..='\u{891}'
            | '\u{8e2}'
            | '\u{61c}'
            | '\u{180e}'
            | '\u{200b}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{206f}'
            | '\u{feff}'
            | '\u{fff9}'..='\u{fffb}'
            | '\u{110bd}'
            | '\u{110cd}'
            | '\u{13430}'..='\u{1343f}'
            | '\u{1bca0}'..='\u{1bca3}'
            | '\u{1d173}'..='\u{1d17a}'
            | '\u{e0001}'
            | '\u{e0020}'..='\u{e007f}'
    )
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
    if frozen.evidence_family.is_empty()
        || frozen.evidence_family.trim() != frozen.evidence_family
        || frozen
            .evidence_family
            .chars()
            .any(|c| c.is_control() || is_hidden_family_mark(c))
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

    /// Derive a deterministic fixture digest without reusing a production trust root.
    fn digest(tag: &[u8]) -> ArtifactDigest {
        ArtifactDigest::sha256(tag)
    }

    /// Construct a fixture judgement with explicit world and dependency identity.
    fn judgement(world: ArtifactDigest, dependencies: Vec<ArtifactDigest>) -> Judgement {
        Judgement::new(world, "finite-exact", b"x == x".to_vec(), dependencies)
    }

    #[test]
    /// Accept matching metadata without claiming that the evidence body is valid.
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
    /// Reject target, world, and dependency substitutions against a frozen judgement.
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
    /// Reject changes to payload bytes, evidence family, or checker identity.
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
    /// Reject control-bearing family identifiers even when both sides match exactly.
    fn control_characters_in_family_fail_even_with_matching_metadata() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for family in [
            "finite\nexhaustive",
            "finite\0exhaustive",
            "finite\u{7f}exhaustive",
            "finite\u{85}exhaustive",
        ] {
            let recorded = EvidenceEnvelopeMetadata::new(
                j.structural_digest(),
                family,
                digest(b"proof"),
                checker,
            );
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

    #[test]
    /// Reject invisible formatting marks even when both sides bind the same bytes.
    fn invisible_format_marks_in_family_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for family in [
            "finite\u{ad}exhaustive",
            "finite\u{61c}exhaustive",
            "finite\u{180e}exhaustive",
            "finite\u{200b}exhaustive",
            "finite\u{202e}exhaustive",
            "finite\u{2066}exhaustive",
            "finite\u{feff}exhaustive",
            "finite\u{e0001}exhaustive",
            "finite\u{e0020}exhaustive",
            "finite\u{e0061}exhaustive",
            "finite\u{e007f}exhaustive",
        ] {
            let recorded = EvidenceEnvelopeMetadata::new(
                j.structural_digest(),
                family,
                digest(b"proof"),
                checker,
            );
            let frozen = FrozenEvidence {
                judgement: &j,
                evidence_family: family,
                payload_bytes: b"proof",
                checker_digest: checker,
            };
            assert_eq!(
                preflight_evidence_metadata(&recorded, &frozen),
                Err(EvidencePreflightError::InvalidFamily),
                "hidden format mark in {family:?} must not be admitted",
            );
        }
    }

    #[test]
    /// Reject every remaining Unicode 16 Cf family mark, including script-specific controls.
    fn other_unicode_format_controls_in_family_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for mark in [
            '\u{600}',
            '\u{605}',
            '\u{6dd}',
            '\u{70f}',
            '\u{890}',
            '\u{891}',
            '\u{8e2}',
            '\u{fff9}',
            '\u{fffb}',
            '\u{110bd}',
            '\u{110cd}',
            '\u{13430}',
            '\u{1343f}',
            '\u{1bca0}',
            '\u{1bca3}',
            '\u{1d173}',
            '\u{1d17a}',
        ] {
            let family = format!("finite{mark}exhaustive");
            let recorded = EvidenceEnvelopeMetadata::new(
                j.structural_digest(),
                &family,
                digest(b"proof"),
                checker,
            );
            let frozen = FrozenEvidence {
                judgement: &j,
                evidence_family: &family,
                payload_bytes: b"proof",
                checker_digest: checker,
            };
            assert_eq!(
                preflight_evidence_metadata(&recorded, &frozen),
                Err(EvidencePreflightError::InvalidFamily),
                "Unicode format control U+{:04X} must not be admitted",
                mark as u32,
            );
        }
    }

    #[test]
    /// Invisible combining controls and variation selectors must not alias family identity.
    fn default_ignorable_non_format_marks_in_family_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for mark in [
            '\u{34f}',   // combining grapheme joiner
            '\u{180b}',  // Mongolian free variation selector one
            '\u{180f}',  // Mongolian free variation selector four
            '\u{fe0f}',  // variation selector 16
            '\u{e0100}', // supplementary variation selector 17
            '\u{e01ef}', // supplementary variation selector 256
        ] {
            let family = format!("finite{mark}exhaustive");
            let recorded = EvidenceEnvelopeMetadata::new(
                j.structural_digest(),
                &family,
                digest(b"proof"),
                checker,
            );
            let frozen = FrozenEvidence {
                judgement: &j,
                evidence_family: &family,
                payload_bytes: b"proof",
                checker_digest: checker,
            };
            assert_eq!(
                preflight_evidence_metadata(&recorded, &frozen),
                Err(EvidencePreflightError::InvalidFamily),
                "default-ignorable mark U+{:04X} must not be admitted",
                mark as u32,
            );
        }
    }

    #[test]
    /// Preserve meaningful Unicode family names; the guard targets hidden marks.
    fn visible_unicode_family_still_preflights() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        let family = "finite-épreuve";
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: b"proof",
            checker_digest: checker,
        };
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(b"proof"), checker);
        assert_eq!(preflight_evidence_metadata(&recorded, &frozen), Ok(()));
    }

    #[test]
    /// Reject empty and whitespace-padded family identifiers before digest matching.
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
