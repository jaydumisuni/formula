//! Non-authoritative P2 evidence-metadata preflight.
//!
//! This is an isolated preparation slice, not a certificate checker, promotion
//! decision, or completion of P1/P2. It only binds a recorded metadata envelope
//! to the caller's frozen judgement, exact payload bytes, and pinned checker.
//! Proof semantics, trust-root admission, and full authority-contract policy
//! remain independent unimplemented gates.

pub mod u8_semantics;

use formula_core::{ArtifactDigest, AuthorityContract, EvidenceEnvelopeMetadata, Judgement};

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

// Evidence families are short identity labels, not arbitrary unbounded payloads.
// Bound serialized UTF-8 bytes before constructing the expected envelope.
const MAX_EVIDENCE_FAMILY_BYTES: usize = 256;

/// Hidden Unicode marks, including reserved Default_Ignorable_Code_Point
/// ranges in Unicode 16, cannot serve as stable visible evidence-family identity.
/// Preserve meaningful visible Unicode names.
fn is_hidden_family_mark(c: char) -> bool {
    matches!(
        c,
        '\u{34f}'
            | '\u{115f}'..='\u{1160}'
            | '\u{17b4}'..='\u{17b5}'
            | '\u{3164}'
            | '\u{2800}'
            | '\u{ffa0}'
            | '\u{ad}'
            | '\u{180b}'..='\u{180d}'
            | '\u{180f}'
            | '\u{fe00}'..='\u{fe0f}'
            | '\u{600}'..='\u{605}'
            | '\u{6dd}'
            | '\u{70f}'
            | '\u{890}'..='\u{891}'
            | '\u{8e2}'
            | '\u{61c}'
            | '\u{180e}'
            | '\u{200b}'..='\u{200f}'
            | '\u{2028}'..='\u{2029}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{206f}'
            | '\u{feff}'
            | '\u{fff9}'..='\u{fffb}'
            | '\u{110bd}'
            | '\u{110cd}'
            | '\u{13430}'..='\u{1343f}'
            | '\u{1bca0}'..='\u{1bca3}'
            | '\u{1d173}'..='\u{1d17a}'
            | '\u{fff0}'..='\u{fff8}'
            | '\u{e0000}'..='\u{e0fff}'
    )
}

/// Unicode noncharacters are permanently reserved code points, not stable
/// interchange identities. This excludes U+FDD0..FDEF and each plane's
/// terminal U+FFFE/U+FFFF without rejecting visible Unicode script names.
fn is_unicode_noncharacter(c: char) -> bool {
    let scalar = c as u32;
    (0xfdd0..=0xfdef).contains(&scalar) || (scalar & 0xfffe) == 0xfffe
}

/// Private-use scalars have no public, stable glyph/semantic identity.
/// Exclude them from durable evidence-family labels across all three PUA planes.
fn is_private_use_family_scalar(c: char) -> bool {
    matches!(c as u32, 0xe000..=0xf8ff | 0xf0000..=0xffffd | 0x100000..=0x10fffd)
}

/// A shared identity check keeps policy membership and metadata preflight
/// consistent; neither is an independent proof or trust-root admission.
fn valid_evidence_family(family: &str) -> bool {
    !family.is_empty()
        && family.len() <= MAX_EVIDENCE_FAMILY_BYTES
        && family.trim() == family
        && !family.chars().any(|c| {
            c.is_whitespace()
                || c.is_control()
                || is_hidden_family_mark(c)
                || is_unicode_noncharacter(c)
                || is_private_use_family_scalar(c)
        })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractPreflightError {
    InvalidFamily,
    InvalidContract,
    ContractMismatch,
    FamilyNotAdmitted,
}

/// Check exact frozen policy identity and membership, without certifying evidence.
///
/// A matching digest is not proof that the caller's frozen contract is trusted.
/// This check must be paired with evidence metadata preflight and a future
/// independent certificate checker; it never grants promotion authority.
pub fn preflight_contract_membership(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    evidence_family: &str,
) -> Result<(), ContractPreflightError> {
    if !valid_evidence_family(evidence_family) {
        return Err(ContractPreflightError::InvalidFamily);
    }
    let classes = frozen_contract.admissible_evidence_classes();
    // A policy is a set of exact evidence identities. The frozen encoding
    // preserves class order, so require strictly increasing byte-exact names:
    // reordered or duplicate sets cannot masquerade as distinct valid policies.
    // This is a fail-closed preflight rule, not a change to frozen core encoding.
    if classes.is_empty()
        || classes.iter().any(|class| !valid_evidence_family(class))
        || classes.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(ContractPreflightError::InvalidContract);
    }
    if frozen_contract.structural_digest() != recorded_contract_digest {
        return Err(ContractPreflightError::ContractMismatch);
    }
    if !classes.iter().any(|class| class == evidence_family) {
        return Err(ContractPreflightError::FamilyNotAdmitted);
    }
    Ok(())
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
    if !valid_evidence_family(frozen.evidence_family) {
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

/// Non-authoritative paired preflight result. Neither branch certifies a proof,
/// establishes the contract trust root, or grants promotion authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceBundlePreflightError {
    Contract(ContractPreflightError),
    Metadata(EvidencePreflightError),
}

/// Require both frozen policy membership and exact evidence-metadata binding.
///
/// A metadata envelope can match its payload, checker, and judgement while
/// naming an evidence class not admitted by the frozen authority contract.
/// Callers preparing P2 evidence must not mistake that metadata-only match for
/// policy admission. This helper composes the two necessary checks, but is
/// deliberately NOT an independent evidence-body checker or certification.
pub fn preflight_frozen_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), EvidenceBundlePreflightError> {
    preflight_contract_membership(
        recorded_contract_digest,
        frozen_contract,
        frozen.evidence_family,
    )
    .map_err(EvidenceBundlePreflightError::Contract)?;
    preflight_evidence_metadata(recorded_metadata, frozen)
        .map_err(EvidenceBundlePreflightError::Metadata)
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
            '\u{115f}',  // Hangul choseong filler
            '\u{1160}',  // Hangul jungseong filler
            '\u{17b4}',  // Khmer inherent vowel sign aq
            '\u{17b5}',  // Khmer inherent vowel sign aa
            '\u{3164}',  // Hangul filler
            '\u{ffa0}',  // halfwidth Hangul filler
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
    /// Unicode line/paragraph separators must not create multi-line family aliases.
    fn line_separator_family_aliases_fail_even_with_matching_metadata() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for mark in ['\u{2028}', '\u{2029}'] {
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
                "line separator U+{:04X} must not be admitted",
                mark as u32,
            );
        }
    }

    #[test]
    /// Internal whitespace can disguise distinct evidence-family identities even
    /// when the recorded and frozen envelopes carry identical bytes.
    fn internal_whitespace_family_aliases_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for family in [
            "finite exhaustive",
            "finite\u{a0}exhaustive",   // no-break space
            "finite\u{1680}exhaustive", // ogham space mark
            "finite\u{2007}exhaustive", // figure space
            "finite\u{202f}exhaustive", // narrow no-break space
            "finite\u{3000}exhaustive", // ideographic space
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
                "internal whitespace in {family:?} must not be admitted",
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

    #[test]
    /// U+2800 is a blank Braille cell, not Unicode whitespace. Matching
    /// recorded metadata must not make its visually empty alias admissible.
    fn braille_blank_family_aliases_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for family in [
            "finite\u{2800}exhaustive",
            "\u{2800}finite",
            "finite\u{2800}",
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
                "blank Braille cell must not alias a family identity",
            );
        }

        // A raised Braille dot is visible and remains a valid family character.
        let visible = "finite\u{2801}exhaustive";
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            visible,
            digest(b"proof"),
            checker,
        );
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: visible,
            payload_bytes: b"proof",
            checker_digest: checker,
        };
        assert_eq!(preflight_evidence_metadata(&recorded, &frozen), Ok(()));
    }

    #[test]
    /// Unicode noncharacters are reserved for internal use, not stable
    /// interchange identities, even when both metadata copies match.
    fn unicode_noncharacters_in_family_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for mark in [
            '\u{fdd0}',
            '\u{fdef}',
            '\u{fffe}',
            '\u{ffff}',
            '\u{1fffe}',
            '\u{1ffff}',
            '\u{10fffe}',
            '\u{10ffff}',
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
                "noncharacter U+{:04X} must not be an evidence-family identity",
                mark as u32,
            );
        }
    }

    #[test]
    /// Unicode 16 Default_Ignorable_Code_Point also reserves unassigned
    /// scalars. Exact matching metadata must not make those aliases valid.
    fn reserved_default_ignorable_family_codepoints_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for scalar in [
            0xfff0, 0xfff8, 0xe0000, 0xe001f, 0xe0080, 0xe00ff, 0xe01f0, 0xe0fff,
        ] {
            let family = format!("finite-{}-exhaustive", char::from_u32(scalar).unwrap());
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
                "reserved default-ignorable U+{scalar:04X} must be rejected",
            );
        }
    }

    #[test]
    /// A matching metadata envelope must not turn an unbounded label into
    /// accepted evidence identity. Bound UTF-8 bytes, not Unicode scalars.
    fn bounded_family_identity_bytes_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for family in ["a".repeat(257), "é".repeat(129)] {
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
                "oversized UTF-8 family must be rejected",
            );
        }
        for family in ["a".repeat(256), "é".repeat(128)] {
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
            assert_eq!(preflight_evidence_metadata(&recorded, &frozen), Ok(()));
        }
    }

    #[test]
    /// Exact contract identity and exact family membership are necessary, never
    /// sufficient, for independent certification.
    fn frozen_contract_membership_is_exact_and_non_authoritative() {
        use formula_core::AuthorityContract;

        let frozen = AuthorityContract::new(vec![
            "finite-exhaustive".to_string(),
            "polynomial-identity".to_string(),
        ]);
        let recorded = frozen.structural_digest();
        assert_eq!(
            preflight_contract_membership(recorded, &frozen, "finite-exhaustive"),
            Ok(())
        );
        assert_eq!(
            preflight_contract_membership(recorded, &frozen, "finite-exhaustive-v2"),
            Err(ContractPreflightError::FamilyNotAdmitted)
        );
        let downgraded = AuthorityContract::new(vec!["finite-exhaustive".to_string()]);
        assert_eq!(
            preflight_contract_membership(
                downgraded.structural_digest(),
                &frozen,
                "finite-exhaustive"
            ),
            Err(ContractPreflightError::ContractMismatch)
        );
        let unrelated = AuthorityContract::new(vec!["polynomial-identity".to_string()]);
        assert_eq!(
            preflight_contract_membership(recorded, &unrelated, "polynomial-identity"),
            Err(ContractPreflightError::ContractMismatch)
        );
        for bad in ["", "finite\u{200b}exhaustive", " finite-exhaustive"] {
            let permissive = AuthorityContract::new(vec![bad.to_string()]);
            assert_eq!(
                preflight_contract_membership(permissive.structural_digest(), &permissive, bad),
                Err(ContractPreflightError::InvalidFamily)
            );
        }
        let empty = AuthorityContract::new(vec![]);
        assert_eq!(
            preflight_contract_membership(empty.structural_digest(), &empty, "finite-exhaustive"),
            Err(ContractPreflightError::InvalidContract)
        );
        let contaminated = AuthorityContract::new(vec![
            "finite-exhaustive".to_string(),
            "hidden\u{200b}family".to_string(),
        ]);
        assert_eq!(
            preflight_contract_membership(
                contaminated.structural_digest(),
                &contaminated,
                "finite-exhaustive"
            ),
            Err(ContractPreflightError::InvalidContract)
        );
        let visible = AuthorityContract::new(vec!["有限証明".to_string()]);
        assert_eq!(
            preflight_contract_membership(visible.structural_digest(), &visible, "有限証明"),
            Ok(())
        );
    }

    #[test]
    /// A policy is a set of exact identities: two orderings of the same
    /// classes must not both be accepted as distinct frozen contract encodings.
    fn noncanonical_frozen_contract_order_fails_closed() {
        let canonical = AuthorityContract::new(vec![
            "finite-exhaustive".to_owned(),
            "polynomial-identity".to_owned(),
        ]);
        let reordered = AuthorityContract::new(vec![
            "polynomial-identity".to_owned(),
            "finite-exhaustive".to_owned(),
        ]);
        assert_ne!(canonical.structural_digest(), reordered.structural_digest());
        assert_eq!(
            preflight_contract_membership(
                reordered.structural_digest(),
                &reordered,
                "finite-exhaustive",
            ),
            Err(ContractPreflightError::InvalidContract),
        );
        assert_eq!(
            preflight_contract_membership(
                canonical.structural_digest(),
                &canonical,
                "finite-exhaustive",
            ),
            Ok(()),
        );
    }

    #[test]
    /// Duplicate evidence-class entries make a frozen authority policy
    /// ambiguous even when the recorded digest matches exactly.
    fn duplicate_frozen_contract_classes_fail_closed() {
        for classes in [
            vec!["finite-exhaustive", "finite-exhaustive"],
            vec![
                "finite-exhaustive",
                "polynomial-identity",
                "finite-exhaustive",
            ],
        ] {
            let frozen = AuthorityContract::new(classes.iter().map(|s| (*s).to_owned()).collect());
            assert_eq!(
                preflight_contract_membership(
                    frozen.structural_digest(),
                    &frozen,
                    "finite-exhaustive"
                ),
                Err(ContractPreflightError::InvalidContract),
                "duplicate evidence classes must not be admitted"
            );
        }
        let valid = AuthorityContract::new(vec![
            "finite-exhaustive".to_owned(),
            "polynomial-identity".to_owned(),
        ]);
        assert_eq!(
            preflight_contract_membership(valid.structural_digest(), &valid, "finite-exhaustive"),
            Ok(())
        );
    }

    #[test]
    /// Private-use Unicode scalars have no standardized interchange meaning.
    /// Matching metadata must not turn font-local glyphs into durable family IDs.
    fn private_use_family_scalars_fail_closed() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        for scalar in [0xe000, 0xf8ff, 0xf0000, 0xffffd, 0x100000, 0x10fffd] {
            let family = format!("finite-{}-exhaustive", char::from_u32(scalar).unwrap());
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
                "private-use U+{scalar:04X} must not be an evidence-family identity",
            );
        }

        // The adjacent standardized visible scalars remain accepted.
        for family in ["évidence", "有限証明"] {
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
            assert_eq!(preflight_evidence_metadata(&recorded, &frozen), Ok(()));
        }
    }

    #[test]
    /// A metadata-only match does not establish frozen policy membership.
    /// The paired preflight must reject nonmember, downgraded, and invalid
    /// contracts before any independent certificate checker is considered.
    fn paired_frozen_evidence_preflight_requires_both_bindings() {
        let j = judgement(digest(b"world"), vec![digest(b"dependency")]);
        let checker = digest(b"checker");
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "polynomial-identity",
            payload_bytes: b"frozen-evidence",
            checker_digest: checker,
        };
        let metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(frozen.payload_bytes),
            checker,
        );
        let admitted = AuthorityContract::new(vec![
            "finite-exhaustive".to_owned(),
            "polynomial-identity".to_owned(),
        ]);
        let admitted_digest = admitted.structural_digest();
        assert_eq!(preflight_evidence_metadata(&metadata, &frozen), Ok(()));
        assert_eq!(
            preflight_frozen_evidence_bundle(admitted_digest, &admitted, &metadata, &frozen),
            Ok(())
        );

        let nonmember = AuthorityContract::new(vec!["finite-exhaustive".to_owned()]);
        assert_eq!(
            preflight_frozen_evidence_bundle(
                nonmember.structural_digest(),
                &nonmember,
                &metadata,
                &frozen
            ),
            Err(EvidenceBundlePreflightError::Contract(
                ContractPreflightError::FamilyNotAdmitted
            ))
        );
        assert_eq!(
            preflight_frozen_evidence_bundle(
                nonmember.structural_digest(),
                &admitted,
                &metadata,
                &frozen
            ),
            Err(EvidenceBundlePreflightError::Contract(
                ContractPreflightError::ContractMismatch
            ))
        );

        let duplicate = AuthorityContract::new(vec![
            "polynomial-identity".to_owned(),
            "polynomial-identity".to_owned(),
        ]);
        assert_eq!(
            preflight_frozen_evidence_bundle(
                duplicate.structural_digest(),
                &duplicate,
                &metadata,
                &frozen
            ),
            Err(EvidenceBundlePreflightError::Contract(
                ContractPreflightError::InvalidContract
            ))
        );

        let wrong_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(b"substituted-body"),
            checker,
        );
        assert_eq!(
            preflight_frozen_evidence_bundle(admitted_digest, &admitted, &wrong_metadata, &frozen),
            Err(EvidenceBundlePreflightError::Metadata(
                EvidencePreflightError::BindingMismatch
            ))
        );
    }

    #[test]
    /// Visually equivalent Unicode spellings remain distinct exact frozen
    /// identities. Neither contract membership nor metadata binding may
    /// silently normalize a producer's evidence-family label.
    fn unicode_canonical_equivalents_remain_distinct_frozen_identities() {
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker");
        let composed = "évidence";
        let decomposed = "e\u{301}vidence";
        assert_ne!(composed.as_bytes(), decomposed.as_bytes());

        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: decomposed,
            payload_bytes: b"proof",
            checker_digest: checker,
        };
        let recorded_composed = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            composed,
            digest(frozen.payload_bytes),
            checker,
        );
        let decomposed_policy = AuthorityContract::new(vec![decomposed.to_owned()]);
        assert_eq!(
            preflight_contract_membership(
                decomposed_policy.structural_digest(),
                &decomposed_policy,
                decomposed,
            ),
            Ok(())
        );
        assert_eq!(
            preflight_frozen_evidence_bundle(
                decomposed_policy.structural_digest(),
                &decomposed_policy,
                &recorded_composed,
                &frozen,
            ),
            Err(EvidenceBundlePreflightError::Metadata(
                EvidencePreflightError::BindingMismatch
            ))
        );

        let composed_policy = AuthorityContract::new(vec![composed.to_owned()]);
        let recorded_decomposed = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            decomposed,
            digest(frozen.payload_bytes),
            checker,
        );
        assert_eq!(
            preflight_frozen_evidence_bundle(
                composed_policy.structural_digest(),
                &composed_policy,
                &recorded_decomposed,
                &frozen,
            ),
            Err(EvidenceBundlePreflightError::Contract(
                ContractPreflightError::FamilyNotAdmitted
            ))
        );
        assert_eq!(
            preflight_frozen_evidence_bundle(
                decomposed_policy.structural_digest(),
                &decomposed_policy,
                &recorded_decomposed,
                &frozen,
            ),
            Ok(())
        );
    }

    #[test]
    /// P2 negative producer cases must fail at the *paired* entry point, even
    /// when the original recorded metadata and frozen policy are self-consistent.
    /// This is structural custody only, never semantic certificate acceptance.
    fn paired_frozen_evidence_preflight_rejects_post_freeze_substitutions() {
        let world = digest(b"world");
        let dependency = digest(b"dependency");
        let original = judgement(world, vec![dependency]);
        let changed_world = judgement(digest(b"other-world"), vec![dependency]);
        let changed_dependency = judgement(world, vec![digest(b"other-dependency")]);
        let changed_proposition =
            Judgement::new(world, "finite-exact", b"x != x".to_vec(), vec![dependency]);
        let checker = digest(b"checker-v1");
        let payload = b"certificate-body";
        let policy = AuthorityContract::new(vec!["finite-exhaustive".to_owned()]);
        let recorded = EvidenceEnvelopeMetadata::new(
            original.structural_digest(),
            "finite-exhaustive",
            digest(payload),
            checker,
        );
        let frozen = FrozenEvidence {
            judgement: &original,
            evidence_family: "finite-exhaustive",
            payload_bytes: payload,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );

        let substituted = [
            FrozenEvidence {
                judgement: &changed_world,
                ..frozen
            },
            FrozenEvidence {
                judgement: &changed_dependency,
                ..frozen
            },
            FrozenEvidence {
                judgement: &changed_proposition,
                ..frozen
            },
            FrozenEvidence {
                payload_bytes: b"certificate-body-after-freeze",
                ..frozen
            },
            FrozenEvidence {
                checker_digest: digest(b"checker-v2"),
                ..frozen
            },
        ];
        for changed in substituted {
            assert_eq!(
                preflight_frozen_evidence_bundle(
                    policy.structural_digest(),
                    &policy,
                    &recorded,
                    &changed,
                ),
                Err(EvidenceBundlePreflightError::Metadata(
                    EvidencePreflightError::BindingMismatch
                )),
                "a post-freeze world, dependency, proposition, body, or checker change must fail",
            );
        }
    }
}
