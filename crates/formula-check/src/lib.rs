//! Non-authoritative P2 evidence-metadata preflight.
//!
//! This is an isolated preparation slice, not a certificate checker, promotion
//! decision, or completion of P1/P2. It only binds a recorded metadata envelope
//! to the caller's frozen judgement, exact payload bytes, and pinned checker.
//! Proof semantics, trust-root admission, and full authority-contract policy
//! remain independent unimplemented gates.

pub mod gf2_normal_form;
pub mod gf2_semantics;
pub mod polynomial_identity;
pub mod u8_semantics;

use formula_core::{
    ArtifactDigest, AuthorityContract, EvidenceEnvelopeMetadata, Judgement, RealizationMetadata,
};

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

/// Structural-only P2 realization evidence custody. A match is NOT a
/// realization-equivalence proof or an authenticated observer admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealizationPreflightError {
    BindingMismatch,
}

/// Compare recorded realization metadata with four independently frozen
/// identities, including the semantic target and observer. The caller must
/// authenticate those pins and bind the implementation/evidence digests to
/// actual frozen artifacts outside this helper. Producer-controlled pins are
/// insufficient. No certificate, semantic equivalence or promotion follows.
pub fn preflight_frozen_realization_metadata(
    recorded: &RealizationMetadata,
    frozen_semantic_target: ArtifactDigest,
    independently_pinned_observer: ArtifactDigest,
    frozen_implementation: ArtifactDigest,
    frozen_realization_evidence: ArtifactDigest,
) -> Result<(), RealizationPreflightError> {
    let expected = RealizationMetadata::new(
        frozen_semantic_target,
        independently_pinned_observer,
        frozen_implementation,
        frozen_realization_evidence,
    );
    if recorded.canonical_bytes() != expected.canonical_bytes() {
        return Err(RealizationPreflightError::BindingMismatch);
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

/// A separately supplied checker identity is a necessary P2 input, not a
/// trust root. The caller must authenticate the pin outside this preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedEvidenceBundlePreflightError {
    CheckerDigestMismatch,
    Bundle(EvidenceBundlePreflightError),
}

/// Reject a producer's self-consistent checker-version substitution before
/// evaluating the frozen policy and exact evidence metadata. This additional
/// pin must come from an independent authority; a producer-controlled pin
/// would provide no security. No certificate or promotion is issued here.
pub fn preflight_pinned_frozen_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedEvidenceBundlePreflightError> {
    if frozen.checker_digest != independently_pinned_checker_digest {
        return Err(PinnedEvidenceBundlePreflightError::CheckerDigestMismatch);
    }
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedEvidenceBundlePreflightError::Bundle)
}

/// Non-authoritative frozen-policy, metadata, and U8-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum U8BundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(u8_semantics::U8EvidenceBodyError),
}

/// First bind the exact frozen bytes and policy; then check the bounded U8
/// pair in those same bytes. This does NOT establish that the Judgement names
/// those terms, issue a certificate, admit a trust root, or promote authority.
pub fn preflight_frozen_u8_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), U8BundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(U8BundlePreflightError::Bundle)?;
    // This bounded checker understands only the finite-exhaustive U8 grammar.
    // A different admitted evidence class must not borrow its semantic result.
    if frozen.evidence_family != "finite-exhaustive" {
        return Err(U8BundlePreflightError::WrongEvidenceFamily);
    }
    u8_semantics::check_frozen_u8_evidence_body(frozen.payload_bytes)
        .map_err(U8BundlePreflightError::Body)
}

/// Non-authoritative checker-pin, frozen-policy, metadata and U8-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedU8BundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(u8_semantics::U8EvidenceBodyError),
}

/// Bind the caller's independently authenticated checker identity before
/// evaluating all 256 inputs of the exact frozen U8 evidence body. A producer
/// cannot substitute a self-consistent checker version or rehash false equality.
/// This does not establish judgement semantics, a trust root or a certificate.
pub fn preflight_pinned_frozen_u8_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedU8BundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedU8BundlePreflightError::Pinned)?;
    if frozen.evidence_family != "finite-exhaustive" {
        return Err(PinnedU8BundlePreflightError::WrongEvidenceFamily);
    }
    u8_semantics::check_frozen_u8_evidence_body(frozen.payload_bytes)
        .map_err(PinnedU8BundlePreflightError::Body)
}

/// Non-authoritative frozen-policy, metadata, and Boolean/GF(2) witness preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gf2BundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(gf2_semantics::Gf2WitnessError),
    Algebraic(gf2_normal_form::NormalFormError),
}

/// Require exact frozen policy and evidence bytes before checking a bounded
/// two-variable Boolean-to-GF(2) translation witness. This does NOT establish
/// that the Judgement names the two terms, issue a certificate, admit a trust
/// root, or grant promotion authority.
pub fn preflight_frozen_gf2_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), Gf2BundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(Gf2BundlePreflightError::Bundle)?;
    check_frozen_gf2_body(frozen)
}

/// Shared frozen-body checker: both entry points use identical semantics and
/// normal-form corroboration after their respective authority-neutral preflights.
fn check_frozen_gf2_body(frozen: &FrozenEvidence<'_>) -> Result<(), Gf2BundlePreflightError> {
    if frozen.evidence_family != "boolean-gf2-translation" {
        return Err(Gf2BundlePreflightError::WrongEvidenceFamily);
    }
    gf2_semantics::check_frozen_boolean_gf2_witness(frozen.payload_bytes)
        .map_err(Gf2BundlePreflightError::Body)?;
    // Independently interpret the same bounded terms as GF(2) normal forms.
    // Agreement is preparatory corroboration, never certificate authority.
    let first = frozen
        .payload_bytes
        .get(..2)
        .ok_or(Gf2BundlePreflightError::Body(
            gf2_semantics::Gf2WitnessError::InvalidFrame,
        ))?;
    let boolean_len = usize::from(u16::from_be_bytes([first[0], first[1]]));
    let boolean = &frozen.payload_bytes[2..2 + boolean_len];
    let gf2 = &frozen.payload_bytes[4 + boolean_len..];
    gf2_normal_form::check_frozen_boolean_gf2_normal_form(boolean, gf2)
        .map_err(Gf2BundlePreflightError::Algebraic)
}

/// Non-authoritative checker-pin, frozen-policy, metadata and GF(2) preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedGf2BundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    Gf2(Gf2BundlePreflightError),
}

/// Bind a separately authenticated checker digest before inspecting the exact
/// frozen Boolean/GF(2) witness. This is a necessary preparation check only:
/// neither this function nor the supplied digest authenticates a trust root,
/// certifies a Judgement or grants promotion authority.
pub fn preflight_pinned_frozen_gf2_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedGf2BundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedGf2BundlePreflightError::Pinned)?;
    check_frozen_gf2_body(frozen).map_err(PinnedGf2BundlePreflightError::Gf2)
}

/// Non-authoritative frozen-policy, metadata, and exact polynomial-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialBundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialIdentityError),
}

/// Require frozen metadata and policy membership before checking canonical
/// integer-polynomial coefficient identity. This is NOT certificate authority:
/// judgement semantics, checker trust root and promotion remain separate gates.
pub fn preflight_frozen_polynomial_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PolynomialBundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PolynomialBundlePreflightError::Bundle)?;
    if frozen.evidence_family != "polynomial-identity" {
        return Err(PolynomialBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_identity(frozen.payload_bytes)
        .map_err(PolynomialBundlePreflightError::Body)
}

/// A caller-supplied, separately authenticated checker pin must be checked
/// alongside the frozen policy, metadata, and exact polynomial identity body.
/// This still cannot establish judgement semantics or certificate authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedPolynomialBundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialIdentityError),
}

/// Compose the independent checker identity pin and exact Z[n] identity
/// preflight on the same frozen bytes, so neither step can be omitted by a
/// caller of this helper. The caller must authenticate the pin independently;
/// no promotion or certificate is issued by this non-authoritative check.
pub fn preflight_pinned_frozen_polynomial_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedPolynomialBundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedPolynomialBundlePreflightError::Pinned)?;
    if frozen.evidence_family != "polynomial-identity" {
        return Err(PinnedPolynomialBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_identity(frozen.payload_bytes)
        .map_err(PinnedPolynomialBundlePreflightError::Body)
}

/// Non-authoritative frozen-policy, metadata, and exact product-body preflight.
/// The evidence-family identity is distinct from polynomial-identity and sum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialProductBundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialProductError),
}

/// Independently check the *same frozen bytes* after exact contract and
/// metadata preflight. This never establishes theorem judgement semantics,
/// checker trust-root authority, certificate validity, or promotion.
pub fn preflight_frozen_polynomial_product_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PolynomialProductBundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PolynomialProductBundlePreflightError::Bundle)?;
    if frozen.evidence_family != "polynomial-product" {
        return Err(PolynomialProductBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_product(frozen.payload_bytes)
        .map_err(PolynomialProductBundlePreflightError::Body)
}

/// Non-authoritative checker-pin, frozen-policy, and exact product-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedPolynomialProductBundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialProductError),
}

/// Bind a caller-authenticated checker identity before independently checking
/// exact bounded Z[n] multiplication on the same frozen evidence bytes.
/// The pin must originate outside the producer; this does not certify the
/// Judgement, authenticate a trust root, or authorize promotion.
pub fn preflight_pinned_frozen_polynomial_product_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedPolynomialProductBundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedPolynomialProductBundlePreflightError::Pinned)?;
    if frozen.evidence_family != "polynomial-product" {
        return Err(PinnedPolynomialProductBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_product(frozen.payload_bytes)
        .map_err(PinnedPolynomialProductBundlePreflightError::Body)
}

/// Non-authoritative frozen-policy, metadata, and exact sum-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialSumBundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialSumError),
}

/// Independently check bounded polynomial addition after binding the exact
/// frozen policy and bytes. This is preparation, never certificate authority.
pub fn preflight_frozen_polynomial_sum_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PolynomialSumBundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PolynomialSumBundlePreflightError::Bundle)?;
    if frozen.evidence_family != "polynomial-sum" {
        return Err(PolynomialSumBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_sum(frozen.payload_bytes)
        .map_err(PolynomialSumBundlePreflightError::Body)
}

/// Non-authoritative composition of an independently authenticated checker pin,
/// exact frozen policy/metadata, and the bounded polynomial-sum body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedPolynomialSumBundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialSumError),
}

/// The checker identity is supplied by an independent caller, not accepted from
/// producer-controlled evidence. This preflight does not certify a Judgement,
/// authenticate the pin, or authorize promotion.
pub fn preflight_pinned_frozen_polynomial_sum_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedPolynomialSumBundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedPolynomialSumBundlePreflightError::Pinned)?;
    if frozen.evidence_family != "polynomial-sum" {
        return Err(PinnedPolynomialSumBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_sum(frozen.payload_bytes)
        .map_err(PinnedPolynomialSumBundlePreflightError::Body)
}

/// Non-authoritative frozen-policy, metadata, and exact scalar-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialScaleBundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialScaleError),
}

/// Bind exact policy and payload bytes before checking a bounded Z[n] scalar
/// product. This cannot certify the enclosing Judgement or admit authority.
pub fn preflight_frozen_polynomial_scale_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PolynomialScaleBundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PolynomialScaleBundlePreflightError::Bundle)?;
    if frozen.evidence_family != "polynomial-scale" {
        return Err(PolynomialScaleBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_scale(frozen.payload_bytes)
        .map_err(PolynomialScaleBundlePreflightError::Body)
}

/// Non-authoritative composition of a caller-authenticated checker pin,
/// exact frozen policy/metadata, and bounded integer-polynomial scaling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedPolynomialScaleBundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialScaleError),
}

/// Reject producer-side checker substitution and rehashed false scalar products
/// on the same frozen evidence bytes. The caller must authenticate the pin
/// independently; this does not grant certificate or promotion authority.
pub fn preflight_pinned_frozen_polynomial_scale_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedPolynomialScaleBundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedPolynomialScaleBundlePreflightError::Pinned)?;
    if frozen.evidence_family != "polynomial-scale" {
        return Err(PinnedPolynomialScaleBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_scale(frozen.payload_bytes)
        .map_err(PinnedPolynomialScaleBundlePreflightError::Body)
}

/// Non-authoritative frozen-policy, metadata, and exact difference-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialDifferenceBundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialDifferenceError),
}

/// Check bounded integer-polynomial subtraction only after exact frozen policy
/// and payload-byte binding. The caller must separately establish judgement
/// semantics, checker trust and certification before granting authority.
pub fn preflight_frozen_polynomial_difference_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PolynomialDifferenceBundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PolynomialDifferenceBundlePreflightError::Bundle)?;
    if frozen.evidence_family != "polynomial-difference" {
        return Err(PolynomialDifferenceBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_difference(frozen.payload_bytes)
        .map_err(PolynomialDifferenceBundlePreflightError::Body)
}

/// Non-authoritative checker-pin, frozen-policy, and exact difference-body preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedPolynomialDifferenceBundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialDifferenceError),
}

/// Require a caller-authenticated checker pin before independently checking
/// exact bounded Z[n] subtraction on the same frozen payload. A producer's
/// self-consistent checker swap or rehashed false result cannot satisfy this
/// preflight. The caller still owns trust-root, judgement, and certificate gates.
pub fn preflight_pinned_frozen_polynomial_difference_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedPolynomialDifferenceBundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedPolynomialDifferenceBundlePreflightError::Pinned)?;
    if frozen.evidence_family != "polynomial-difference" {
        return Err(PinnedPolynomialDifferenceBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_difference(frozen.payload_bytes)
        .map_err(PinnedPolynomialDifferenceBundlePreflightError::Body)
}

/// Non-authoritative frozen-policy, metadata, and exact formal-derivative preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialDerivativeBundlePreflightError {
    Bundle(EvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialDerivativeError),
}

/// Bind the frozen contract and payload identity before independently checking
/// the bounded Z[n] derivative. This does not certify the Judgement, establish
/// checker trust, or authorize mathematical promotion.
pub fn preflight_frozen_polynomial_derivative_evidence_bundle(
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PolynomialDerivativeBundlePreflightError> {
    preflight_frozen_evidence_bundle(
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PolynomialDerivativeBundlePreflightError::Bundle)?;
    if frozen.evidence_family != "polynomial-derivative" {
        return Err(PolynomialDerivativeBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_derivative(frozen.payload_bytes)
        .map_err(PolynomialDerivativeBundlePreflightError::Body)
}

/// Non-authoritative composition of an independently authenticated checker pin,
/// exact frozen policy/metadata, and the bounded formal derivative body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinnedPolynomialDerivativeBundlePreflightError {
    Pinned(PinnedEvidenceBundlePreflightError),
    WrongEvidenceFamily,
    Body(polynomial_identity::PolynomialDerivativeError),
}

/// Reject a self-consistent producer-side checker substitution even when its
/// derivative arithmetic is valid. The pin must be authenticated by the caller;
/// this is neither certificate issuance nor judgement/promotion authority.
pub fn preflight_pinned_frozen_polynomial_derivative_evidence_bundle(
    independently_pinned_checker_digest: ArtifactDigest,
    recorded_contract_digest: ArtifactDigest,
    frozen_contract: &AuthorityContract,
    recorded_metadata: &EvidenceEnvelopeMetadata,
    frozen: &FrozenEvidence<'_>,
) -> Result<(), PinnedPolynomialDerivativeBundlePreflightError> {
    preflight_pinned_frozen_evidence_bundle(
        independently_pinned_checker_digest,
        recorded_contract_digest,
        frozen_contract,
        recorded_metadata,
        frozen,
    )
    .map_err(PinnedPolynomialDerivativeBundlePreflightError::Pinned)?;
    if frozen.evidence_family != "polynomial-derivative" {
        return Err(PinnedPolynomialDerivativeBundlePreflightError::WrongEvidenceFamily);
    }
    polynomial_identity::check_frozen_polynomial_derivative(frozen.payload_bytes)
        .map_err(PinnedPolynomialDerivativeBundlePreflightError::Body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// A producer-controlled, digest-rebound subtraction witness cannot replace
    /// the independently pinned checker or the exact frozen arithmetic body.
    fn pinned_polynomial_difference_preflight_rejects_substitution() {
        fn frame(vectors: &[&[i64]]) -> Vec<u8> {
            let mut bytes = Vec::new();
            for coefficients in vectors {
                bytes.push(coefficients.len() as u8);
                for coefficient in *coefficients {
                    bytes.extend_from_slice(&coefficient.to_be_bytes());
                }
            }
            bytes
        }
        let j = judgement(digest(b"pinned-difference-world"), vec![]);
        let pinned = digest(b"independent-difference-checker");
        let producer = digest(b"producer-checker");
        let policy = AuthorityContract::new(vec!["polynomial-difference".to_owned()]);
        let valid = frame(&[&[5, 3], &[2, 1], &[3, 2]]);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "polynomial-difference",
            payload_bytes: &valid,
            checker_digest: pinned,
        };
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&valid),
            pinned,
        );
        let preflight = preflight_pinned_frozen_polynomial_difference_evidence_bundle;
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        // Matching producer metadata cannot substitute its own checker identity.
        let forged = FrozenEvidence {
            checker_digest: producer,
            ..frozen
        };
        let forged_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            forged.evidence_family,
            digest(&valid),
            producer,
        );
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &forged_metadata,
                &forged
            ),
            Err(PinnedPolynomialDifferenceBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        // Rehashing a false subtraction result still fails the exact body check.
        let wrong = frame(&[&[5, 3], &[2, 1], &[4, 2]]);
        let wrong_frozen = FrozenEvidence {
            payload_bytes: &wrong,
            ..frozen
        };
        let wrong_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&wrong),
            pinned,
        );
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &wrong_metadata,
                &wrong_frozen
            ),
            Err(PinnedPolynomialDifferenceBundlePreflightError::Body(
                polynomial_identity::PolynomialDifferenceError::CoefficientMismatch {
                    power: 0,
                    expected: 3,
                    actual: 4
                }
            ))
        );

        // Even an admitted, pinned polynomial-sum family cannot borrow subtraction.
        let other_policy = AuthorityContract::new(vec!["polynomial-sum".to_owned()]);
        let other_frozen = FrozenEvidence {
            evidence_family: "polynomial-sum",
            ..frozen
        };
        let other_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_frozen.evidence_family,
            digest(&valid),
            pinned,
        );
        assert_eq!(
            preflight(
                pinned,
                other_policy.structural_digest(),
                &other_policy,
                &other_metadata,
                &other_frozen
            ),
            Err(PinnedPolynomialDifferenceBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn frozen_polynomial_preflight_binds_family_policy_metadata_and_exact_coefficients() {
        fn body(left: &[i64], right: &[i64]) -> Vec<u8> {
            let mut bytes = Vec::new();
            for coefficients in [left, right] {
                bytes.push(coefficients.len() as u8);
                for coefficient in coefficients {
                    bytes.extend_from_slice(&coefficient.to_be_bytes());
                }
            }
            bytes
        }
        let j = judgement(digest(b"poly-world"), vec![digest(b"poly-dependency")]);
        let checker = digest(b"checker-polynomial-v0");
        let family = "polynomial-identity";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let exact_body = body(&[2, -3, 5], &[2, -3, 5]);
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&exact_body),
            checker,
        );
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &exact_body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_polynomial_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );

        let false_body = body(&[2, -3, 5], &[2, -3, 5, 1]);
        let false_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&false_body),
            checker,
        );
        let false_frozen = FrozenEvidence {
            payload_bytes: &false_body,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &false_recorded,
                &false_frozen,
            ),
            Err(PolynomialBundlePreflightError::Body(
                polynomial_identity::PolynomialIdentityError::CoefficientMismatch {
                    power: 3,
                    left: 0,
                    right: 1,
                }
            ))
        );
        assert_eq!(
            preflight_frozen_polynomial_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &false_frozen,
            ),
            Err(PolynomialBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let wrong_family = "finite-exhaustive";
        let wrong_policy = AuthorityContract::new(vec![wrong_family.to_owned()]);
        let wrong_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            wrong_family,
            digest(&exact_body),
            checker,
        );
        let wrong_frozen = FrozenEvidence {
            evidence_family: wrong_family,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_evidence_bundle(
                wrong_policy.structural_digest(),
                &wrong_policy,
                &wrong_recorded,
                &wrong_frozen,
            ),
            Err(PolynomialBundlePreflightError::WrongEvidenceFamily)
        );
    }

    fn polynomial_triple_frame(a: &[i64], b: &[i64], result: &[i64]) -> Vec<u8> {
        let mut bytes = Vec::new();
        for coefficients in [a, b, result] {
            bytes.push(coefficients.len() as u8);
            for coefficient in coefficients {
                bytes.extend_from_slice(&coefficient.to_be_bytes());
            }
        }
        bytes
    }

    #[test]
    fn frozen_polynomial_derivative_bundle_requires_bound_policy_and_exact_arithmetic() {
        fn pair(operand: &[i64], derivative: &[i64]) -> Vec<u8> {
            let mut body = Vec::new();
            for coefficients in [operand, derivative] {
                body.push(coefficients.len() as u8);
                for coefficient in coefficients {
                    body.extend_from_slice(&coefficient.to_be_bytes());
                }
            }
            body
        }
        let j = judgement(
            digest(b"derivative-world"),
            vec![digest(b"derivative-dependency")],
        );
        let checker = digest(b"checker-polynomial-derivative-v0");
        let family = "polynomial-derivative";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = pair(&[7, -3, 2], &[-3, 4]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_polynomial_derivative_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );

        let forged = pair(&[7, -3, 2], &[-3, 5]);
        let forged_frozen = FrozenEvidence {
            payload_bytes: &forged,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_derivative_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &forged_frozen,
            ),
            Err(PolynomialDerivativeBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let forged_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&forged), checker);
        assert_eq!(
            preflight_frozen_polynomial_derivative_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged_frozen,
            ),
            Err(PolynomialDerivativeBundlePreflightError::Body(
                polynomial_identity::PolynomialDerivativeError::CoefficientMismatch {
                    power: 1,
                    expected: 4,
                    actual: 5,
                }
            ))
        );

        let wrong_family = "polynomial-identity";
        let wrong_policy = AuthorityContract::new(vec![wrong_family.to_owned()]);
        let wrong_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            wrong_family,
            digest(&body),
            checker,
        );
        let wrong_frozen = FrozenEvidence {
            evidence_family: wrong_family,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_derivative_evidence_bundle(
                wrong_policy.structural_digest(),
                &wrong_policy,
                &wrong_recorded,
                &wrong_frozen,
            ),
            Err(PolynomialDerivativeBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn pinned_derivative_preflight_rejects_checker_substitution_before_body_admission() {
        fn frame(operand: &[i64], derivative: &[i64]) -> Vec<u8> {
            let mut bytes = Vec::new();
            for coefficients in [operand, derivative] {
                bytes.push(coefficients.len() as u8);
                for coefficient in coefficients {
                    bytes.extend_from_slice(&coefficient.to_be_bytes());
                }
            }
            bytes
        }
        let judgement = judgement(digest(b"derivative-world"), vec![digest(b"dependency")]);
        let checker = digest(b"independently-pinned-derivative-checker");
        let replacement_checker = digest(b"producer-replacement-checker");
        let family = "polynomial-derivative";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = frame(&[7, -3, 2], &[-3, 4]);
        let recorded = EvidenceEnvelopeMetadata::new(
            judgement.structural_digest(),
            family,
            digest(&body),
            checker,
        );
        let frozen = FrozenEvidence {
            judgement: &judgement,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_derivative_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );

        let replacement_recorded = EvidenceEnvelopeMetadata::new(
            judgement.structural_digest(),
            family,
            digest(&body),
            replacement_checker,
        );
        let replacement = FrozenEvidence {
            checker_digest: replacement_checker,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_derivative_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &replacement_recorded,
                &replacement,
            ),
            Err(PinnedPolynomialDerivativeBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        let forged_body = frame(&[7, -3, 2], &[-3, 5]);
        let forged_recorded = EvidenceEnvelopeMetadata::new(
            judgement.structural_digest(),
            family,
            digest(&forged_body),
            checker,
        );
        let forged = FrozenEvidence {
            payload_bytes: &forged_body,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_derivative_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged,
            ),
            Err(PinnedPolynomialDerivativeBundlePreflightError::Body(
                polynomial_identity::PolynomialDerivativeError::CoefficientMismatch {
                    power: 1,
                    expected: 4,
                    actual: 5,
                }
            ))
        );

        let wrong_policy = AuthorityContract::new(vec!["polynomial-identity".to_owned()]);
        assert_eq!(
            preflight_pinned_frozen_polynomial_derivative_evidence_bundle(
                checker,
                wrong_policy.structural_digest(),
                &wrong_policy,
                &recorded,
                &frozen,
            ),
            Err(PinnedPolynomialDerivativeBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::Bundle(EvidenceBundlePreflightError::Contract(
                    ContractPreflightError::FamilyNotAdmitted
                ))
            ))
        );
    }

    #[test]
    fn pinned_product_preflight_rejects_checker_substitution_and_forged_arithmetic() {
        fn frame(a: &[i64], b: &[i64], product: &[i64]) -> Vec<u8> {
            let mut bytes = Vec::new();
            for coefficients in [a, b, product] {
                bytes.push(coefficients.len() as u8);
                for coefficient in coefficients {
                    bytes.extend_from_slice(&coefficient.to_be_bytes());
                }
            }
            bytes
        }
        let j = judgement(
            digest(b"product-world"),
            vec![digest(b"product-dependency")],
        );
        let checker = digest(b"independently-pinned-product-checker");
        let substituted_checker = digest(b"producer-controlled-product-checker");
        let family = "polynomial-product";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = frame(&[2, -3], &[4, 5], &[8, -2, -15]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_product_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        let replaced_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&body),
            substituted_checker,
        );
        let replaced = FrozenEvidence {
            checker_digest: substituted_checker,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_product_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &replaced_recorded,
                &replaced
            ),
            Err(PinnedPolynomialProductBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        let forged_body = frame(&[2, -3], &[4, 5], &[8, -2, -14]);
        let forged_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&forged_body),
            checker,
        );
        let forged = FrozenEvidence {
            payload_bytes: &forged_body,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_product_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged
            ),
            Err(PinnedPolynomialProductBundlePreflightError::Body(
                polynomial_identity::PolynomialProductError::CoefficientMismatch {
                    power: 2,
                    expected: -15,
                    actual: -14,
                }
            ))
        );

        let wrong_policy = AuthorityContract::new(vec!["polynomial-identity".to_owned()]);
        assert_eq!(
            preflight_pinned_frozen_polynomial_product_evidence_bundle(
                checker,
                wrong_policy.structural_digest(),
                &wrong_policy,
                &recorded,
                &frozen
            ),
            Err(PinnedPolynomialProductBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::Bundle(EvidenceBundlePreflightError::Contract(
                    ContractPreflightError::FamilyNotAdmitted
                ))
            ))
        );
    }

    #[test]
    fn frozen_polynomial_product_bundle_requires_bound_metadata_and_exact_arithmetic() {
        let j = judgement(
            digest(b"product-world"),
            vec![digest(b"product-dependency")],
        );
        let checker = digest(b"checker-polynomial-product-v0");
        let family = "polynomial-product";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = polynomial_triple_frame(&[2, -3], &[4, 5], &[8, -2, -15]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_polynomial_product_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );

        let forged = polynomial_triple_frame(&[2, -3], &[4, 5], &[8, -2, -14]);
        let forged_frozen = FrozenEvidence {
            payload_bytes: &forged,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_product_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &forged_frozen,
            ),
            Err(PolynomialProductBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let forged_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&forged), checker);
        assert_eq!(
            preflight_frozen_polynomial_product_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged_frozen,
            ),
            Err(PolynomialProductBundlePreflightError::Body(
                polynomial_identity::PolynomialProductError::CoefficientMismatch {
                    power: 2,
                    expected: -15,
                    actual: -14,
                }
            ))
        );
        let other_family = "polynomial-identity";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&forged),
            checker,
        );
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..forged_frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_product_evidence_bundle(
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen,
            ),
            Err(PolynomialProductBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn frozen_polynomial_sum_bundle_requires_bound_metadata_and_exact_arithmetic() {
        let j = judgement(digest(b"sum-world"), vec![digest(b"sum-dependency")]);
        let checker = digest(b"checker-polynomial-sum-v0");
        let family = "polynomial-sum";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = polynomial_triple_frame(&[2, -3], &[4, 5], &[6, 2]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_polynomial_sum_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );
        let forged = polynomial_triple_frame(&[2, -3], &[4, 5], &[6, 3]);
        let forged_frozen = FrozenEvidence {
            payload_bytes: &forged,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_sum_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &forged_frozen,
            ),
            Err(PolynomialSumBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let forged_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&forged), checker);
        assert_eq!(
            preflight_frozen_polynomial_sum_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged_frozen,
            ),
            Err(PolynomialSumBundlePreflightError::Body(
                polynomial_identity::PolynomialSumError::CoefficientMismatch {
                    power: 1,
                    expected: 2,
                    actual: 3,
                }
            ))
        );
        let other_family = "polynomial-product";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&forged),
            checker,
        );
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..forged_frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_sum_evidence_bundle(
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen,
            ),
            Err(PolynomialSumBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn pinned_polynomial_sum_preflight_rejects_checker_substitution_and_forged_sum() {
        let j = judgement(
            digest(b"pinned-sum-world"),
            vec![digest(b"pinned-sum-dependency")],
        );
        let checker = digest(b"independently-approved-sum-checker");
        let substituted_checker = digest(b"producer-replaced-sum-checker");
        let family = "polynomial-sum";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = polynomial_triple_frame(&[2, -3], &[4, 5], &[6, 2]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_sum_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        // A producer may rewrite both the envelope and its claimed checker,
        // but cannot rewrite the independently supplied checker identity.
        let replaced_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&body),
            substituted_checker,
        );
        let replaced = FrozenEvidence {
            checker_digest: substituted_checker,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_sum_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &replaced_recorded,
                &replaced
            ),
            Err(PinnedPolynomialSumBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        // Matching the forged payload digest does not make false arithmetic true.
        let forged_body = polynomial_triple_frame(&[2, -3], &[4, 5], &[6, 3]);
        let forged_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&forged_body),
            checker,
        );
        let forged = FrozenEvidence {
            payload_bytes: &forged_body,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_sum_evidence_bundle(
                checker,
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged
            ),
            Err(PinnedPolynomialSumBundlePreflightError::Body(
                polynomial_identity::PolynomialSumError::CoefficientMismatch {
                    power: 1,
                    expected: 2,
                    actual: 3,
                }
            ))
        );

        let wrong_policy = AuthorityContract::new(vec!["polynomial-product".to_owned()]);
        assert_eq!(
            preflight_pinned_frozen_polynomial_sum_evidence_bundle(
                checker,
                wrong_policy.structural_digest(),
                &wrong_policy,
                &recorded,
                &frozen
            ),
            Err(PinnedPolynomialSumBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::Bundle(EvidenceBundlePreflightError::Contract(
                    ContractPreflightError::FamilyNotAdmitted
                ))
            ))
        );
        let other_family = "polynomial-product";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&body),
            checker,
        );
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..frozen
        };
        assert_eq!(
            preflight_pinned_frozen_polynomial_sum_evidence_bundle(
                checker,
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen
            ),
            Err(PinnedPolynomialSumBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn frozen_polynomial_difference_bundle_binds_policy_bytes_and_arithmetic() {
        let j = judgement(
            digest(b"difference-world"),
            vec![digest(b"difference-dependency")],
        );
        let checker = digest(b"checker-polynomial-difference-v0");
        let family = "polynomial-difference";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let body = polynomial_triple_frame(&[5, -3], &[2, -5], &[3, 2]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_polynomial_difference_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        let forged = polynomial_triple_frame(&[5, -3], &[2, -5], &[3, 3]);
        let forged_frozen = FrozenEvidence {
            payload_bytes: &forged,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_difference_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &forged_frozen
            ),
            Err(PolynomialDifferenceBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let forged_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&forged), checker);
        assert_eq!(
            preflight_frozen_polynomial_difference_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged_frozen
            ),
            Err(PolynomialDifferenceBundlePreflightError::Body(
                polynomial_identity::PolynomialDifferenceError::CoefficientMismatch {
                    power: 1,
                    expected: 2,
                    actual: 3
                }
            ))
        );
        let other_family = "polynomial-sum";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&body),
            checker,
        );
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_difference_evidence_bundle(
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen
            ),
            Err(PolynomialDifferenceBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn frozen_polynomial_scale_bundle_binds_policy_bytes_and_exact_scalar() {
        let j = judgement(digest(b"scale-world"), vec![digest(b"scale-dependency")]);
        let checker = digest(b"checker-polynomial-scale-v0");
        let family = "polynomial-scale";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let mut body = (-2_i64).to_be_bytes().to_vec();
        for coefficients in [&[2_i64, -3][..], &[-4_i64, 6][..]] {
            body.push(coefficients.len() as u8);
            for coefficient in coefficients {
                body.extend_from_slice(&coefficient.to_be_bytes());
            }
        }
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_polynomial_scale_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );
        let mut forged = body.clone();
        *forged.last_mut().unwrap() = 5;
        let forged_frozen = FrozenEvidence {
            payload_bytes: &forged,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_scale_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &forged_frozen
            ),
            Err(PolynomialScaleBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let forged_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&forged), checker);
        assert_eq!(
            preflight_frozen_polynomial_scale_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &forged_recorded,
                &forged_frozen
            ),
            Err(PolynomialScaleBundlePreflightError::Body(
                polynomial_identity::PolynomialScaleError::CoefficientMismatch {
                    power: 1,
                    expected: 6,
                    actual: 5
                }
            ))
        );
        let other_family = "polynomial-sum";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&body),
            checker,
        );
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_polynomial_scale_evidence_bundle(
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen
            ),
            Err(PolynomialScaleBundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn pinned_polynomial_scale_preflight_rejects_checker_swap_and_rehashed_false_product() {
        let j = judgement(digest(b"pinned-scale-world"), vec![]);
        let pinned = digest(b"independent-scale-checker");
        let producer = digest(b"producer-scale-checker");
        let family = "polynomial-scale";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let mut body = (-2_i64).to_be_bytes().to_vec();
        for coefficients in [&[2_i64, -3][..], &[-4_i64, 6][..]] {
            body.push(coefficients.len() as u8);
            for coefficient in coefficients {
                body.extend_from_slice(&coefficient.to_be_bytes());
            }
        }
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: pinned,
        };
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), pinned);
        let preflight = preflight_pinned_frozen_polynomial_scale_evidence_bundle;
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        let swapped = FrozenEvidence {
            checker_digest: producer,
            ..frozen
        };
        let swapped_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), producer);
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &swapped_recorded,
                &swapped
            ),
            Err(PinnedPolynomialScaleBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        let mut wrong = body.clone();
        *wrong.last_mut().expect("nonempty scale frame") = 5;
        let wrong_frozen = FrozenEvidence {
            payload_bytes: &wrong,
            ..frozen
        };
        let wrong_recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&wrong), pinned);
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &wrong_recorded,
                &wrong_frozen
            ),
            Err(PinnedPolynomialScaleBundlePreflightError::Body(
                polynomial_identity::PolynomialScaleError::CoefficientMismatch {
                    power: 1,
                    expected: 6,
                    actual: 5
                }
            ))
        );

        let other_family = "polynomial-product";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..frozen
        };
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&body),
            pinned,
        );
        assert_eq!(
            preflight(
                pinned,
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen
            ),
            Err(PinnedPolynomialScaleBundlePreflightError::WrongEvidenceFamily)
        );
    }

    /// Derive a deterministic fixture digest without reusing a production trust root.
    fn digest(tag: &[u8]) -> ArtifactDigest {
        ArtifactDigest::sha256(tag)
    }

    /// Construct a fixture judgement with explicit world and dependency identity.
    fn judgement(world: ArtifactDigest, dependencies: Vec<ArtifactDigest>) -> Judgement {
        Judgement::new(world, "finite-exact", b"x == x".to_vec(), dependencies)
    }

    #[test]
    fn frozen_gf2_witness_requires_policy_metadata_and_semantic_agreement() {
        let j = judgement(digest(b"gf2-world"), vec![digest(b"gf2-dependency")]);
        let checker = digest(b"checker-gf2-v0");
        let family = "boolean-gf2-translation";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        // Boolean x OR y, GF(2) x + y + xy.
        let body = vec![0, 3, 0, 1, 6, 0, 7, 0, 1, 4, 0, 1, 5, 4];
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_gf2_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen,
            ),
            Ok(())
        );
        let forged = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(b"not-the-frozen-body"),
            checker,
        );
        assert_eq!(
            preflight_frozen_gf2_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &forged,
                &frozen,
            ),
            Err(Gf2BundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let false_body = vec![0, 3, 0, 1, 6, 0, 3, 0, 1, 4];
        let false_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&false_body),
            checker,
        );
        let false_frozen = FrozenEvidence {
            payload_bytes: &false_body,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_gf2_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &false_recorded,
                &false_frozen,
            ),
            Err(Gf2BundlePreflightError::Body(
                gf2_semantics::Gf2WitnessError::Counterexample {
                    x: true,
                    y: true,
                    boolean: true,
                    gf2: false,
                }
            ))
        );
        let other_family = "finite-exhaustive";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&body),
            checker,
        );
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_gf2_evidence_bundle(
                other_policy.structural_digest(),
                &other_policy,
                &other_recorded,
                &other_frozen,
            ),
            Err(Gf2BundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn pinned_gf2_preflight_binds_checker_policy_and_exact_semantics() {
        let j = judgement(digest(b"pinned-gf2-world"), vec![digest(b"gf2-dependency")]);
        let pinned = digest(b"independently-authenticated-gf2-checker");
        let producer = digest(b"producer-controlled-gf2-checker");
        let family = "boolean-gf2-translation";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        // x OR y == x + y + xy over GF(2).
        let body = vec![0, 3, 0, 1, 6, 0, 7, 0, 1, 4, 0, 1, 5, 4];
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: pinned,
        };
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), pinned);
        let check = preflight_pinned_frozen_gf2_evidence_bundle;
        assert_eq!(
            check(
                pinned,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        // Rehashing a producer-selected checker and matching envelope is not
        // independent checker authentication.
        let replaced = FrozenEvidence {
            checker_digest: producer,
            ..frozen
        };
        let replaced_metadata =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), producer);
        assert_eq!(
            check(
                pinned,
                policy.structural_digest(),
                &policy,
                &replaced_metadata,
                &replaced
            ),
            Err(PinnedGf2BundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        // Even a matching frozen envelope cannot authorize a false GF(2) witness.
        let false_body = vec![0, 3, 0, 1, 6, 0, 3, 0, 1, 4];
        let false_frozen = FrozenEvidence {
            payload_bytes: &false_body,
            ..frozen
        };
        let false_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            family,
            digest(&false_body),
            pinned,
        );
        assert_eq!(
            check(
                pinned,
                policy.structural_digest(),
                &policy,
                &false_metadata,
                &false_frozen
            ),
            Err(PinnedGf2BundlePreflightError::Gf2(
                Gf2BundlePreflightError::Body(gf2_semantics::Gf2WitnessError::Counterexample {
                    x: true,
                    y: true,
                    boolean: true,
                    gf2: false,
                })
            ))
        );

        let other_family = "finite-exhaustive";
        let other_policy = AuthorityContract::new(vec![other_family.to_owned()]);
        let other_frozen = FrozenEvidence {
            evidence_family: other_family,
            ..frozen
        };
        let other_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_family,
            digest(&body),
            pinned,
        );
        assert_eq!(
            check(
                pinned,
                other_policy.structural_digest(),
                &other_policy,
                &other_metadata,
                &other_frozen
            ),
            Err(PinnedGf2BundlePreflightError::Gf2(
                Gf2BundlePreflightError::WrongEvidenceFamily
            ))
        );
    }

    #[test]
    /// P2 U8 preflight must bind the independently supplied checker identity
    /// before policy, exact metadata and exhaustive frozen-body verification.
    /// This is preparation only; the caller still owns checker trust and certificates.
    fn pinned_u8_preflight_rejects_checker_substitution_and_false_evidence() {
        let j = judgement(digest(b"pinned-u8-world"), vec![digest(b"dependency")]);
        let pinned = digest(b"independent-u8-checker");
        let producer = digest(b"producer-u8-checker");
        let policy = AuthorityContract::new(vec!["finite-exhaustive".to_owned()]);
        let body = vec![0, 1, 0, 0, 1, 0]; // x == x
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "finite-exhaustive",
            payload_bytes: &body,
            checker_digest: pinned,
        };
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&body),
            pinned,
        );
        let preflight = preflight_pinned_frozen_u8_evidence_bundle;
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        let substituted = FrozenEvidence {
            checker_digest: producer,
            ..frozen
        };
        let forged_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&body),
            producer,
        );
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &forged_metadata,
                &substituted
            ),
            Err(PinnedU8BundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        // Even rehashed producer evidence cannot turn a false equality into proof.
        let false_body = vec![0, 1, 0, 0, 4, 0, 1, 1, 2]; // x != x + 1
        let false_frozen = FrozenEvidence {
            payload_bytes: &false_body,
            ..frozen
        };
        let false_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&false_body),
            pinned,
        );
        assert_eq!(
            preflight(
                pinned,
                policy.structural_digest(),
                &policy,
                &false_metadata,
                &false_frozen
            ),
            Err(PinnedU8BundlePreflightError::Body(
                u8_semantics::U8EvidenceBodyError::Semantics(
                    u8_semantics::U8EquivalenceError::Counterexample {
                        input: 0,
                        left: 0,
                        right: 1
                    }
                )
            ))
        );

        let other_policy = AuthorityContract::new(vec!["polynomial-identity".to_owned()]);
        let other_frozen = FrozenEvidence {
            evidence_family: "polynomial-identity",
            ..frozen
        };
        let other_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            other_frozen.evidence_family,
            digest(&body),
            pinned,
        );
        assert_eq!(
            preflight(
                pinned,
                other_policy.structural_digest(),
                &other_policy,
                &other_metadata,
                &other_frozen
            ),
            Err(PinnedU8BundlePreflightError::WrongEvidenceFamily)
        );
        assert_eq!(
            preflight(
                pinned,
                other_policy.structural_digest(),
                &other_policy,
                &recorded,
                &frozen
            ),
            Err(PinnedU8BundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::Bundle(EvidenceBundlePreflightError::Contract(
                    ContractPreflightError::FamilyNotAdmitted
                ))
            ))
        );
    }

    #[test]
    fn frozen_u8_preflight_rejects_unrelated_admitted_evidence_family() {
        // An unrelated family can be admitted by its own contract and carry a
        // digest-matched U8 body; it must not acquire the U8 checker result.
        let j = judgement(digest(b"world"), vec![]);
        let checker = digest(b"checker-u8-v0");
        let body = vec![0, 1, 0, 0, 1, 0]; // x == x
        let family = "polynomial-identity";
        let policy = AuthorityContract::new(vec![family.to_owned()]);
        let recorded =
            EvidenceEnvelopeMetadata::new(j.structural_digest(), family, digest(&body), checker);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: family,
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_u8_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Err(U8BundlePreflightError::WrongEvidenceFamily)
        );
    }

    #[test]
    fn frozen_u8_body_preflight_requires_exact_policy_metadata_and_semantics() {
        let j = judgement(digest(b"world"), vec![digest(b"dependency")]);
        let checker = digest(b"checker-u8-v0");
        let policy = AuthorityContract::new(vec!["finite-exhaustive".to_owned()]);
        let body = vec![0, 3, 0, 0, 2, 0, 4, 0, 1, 2, 4];
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            "finite-exhaustive",
            digest(&body),
            checker,
        );
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "finite-exhaustive",
            payload_bytes: &body,
            checker_digest: checker,
        };
        assert_eq!(
            preflight_frozen_u8_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );
        let wrong = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            "finite-exhaustive",
            digest(b"other"),
            checker,
        );
        assert_eq!(
            preflight_frozen_u8_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &wrong,
                &frozen
            ),
            Err(U8BundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let excluded = AuthorityContract::new(vec!["polynomial-identity".to_owned()]);
        assert_eq!(
            preflight_frozen_u8_evidence_bundle(
                excluded.structural_digest(),
                &excluded,
                &recorded,
                &frozen
            ),
            Err(U8BundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Contract(ContractPreflightError::FamilyNotAdmitted)
            ))
        );

        // Rebinding the digest to a false equality cannot turn it into evidence.
        let false_body = vec![0, 1, 0, 0, 4, 0, 1, 1, 2];
        let false_recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            "finite-exhaustive",
            digest(&false_body),
            checker,
        );
        let false_frozen = FrozenEvidence {
            payload_bytes: &false_body,
            ..frozen
        };
        assert_eq!(
            preflight_frozen_u8_evidence_bundle(
                policy.structural_digest(),
                &policy,
                &false_recorded,
                &false_frozen
            ),
            Err(U8BundlePreflightError::Body(
                u8_semantics::U8EvidenceBodyError::Semantics(
                    u8_semantics::U8EquivalenceError::Counterexample {
                        input: 0,
                        left: 0,
                        right: 1
                    }
                )
            ))
        );
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
    fn pinned_polynomial_identity_preflight_binds_pin_policy_metadata_and_exact_body() {
        fn pair(left: i64, right: i64) -> Vec<u8> {
            let mut bytes = vec![1];
            bytes.extend_from_slice(&left.to_be_bytes());
            bytes.push(1);
            bytes.extend_from_slice(&right.to_be_bytes());
            bytes
        }
        let j = judgement(digest(b"pinned-poly-world"), vec![digest(b"dependency")]);
        let pinned = digest(b"independently-authorized-checker");
        let unreviewed = digest(b"producer-supplied-checker");
        let policy = AuthorityContract::new(vec!["polynomial-identity".to_owned()]);
        let policy_digest = policy.structural_digest();
        let valid_body = pair(3, 3);
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "polynomial-identity",
            payload_bytes: &valid_body,
            checker_digest: pinned,
        };
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&valid_body),
            pinned,
        );
        assert_eq!(
            preflight_pinned_frozen_polynomial_evidence_bundle(
                pinned,
                policy_digest,
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        // A self-consistent producer-controlled checker version is not the pin.
        let forged = FrozenEvidence {
            checker_digest: unreviewed,
            ..frozen
        };
        let forged_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&valid_body),
            unreviewed,
        );
        assert_eq!(
            preflight_pinned_frozen_polynomial_evidence_bundle(
                pinned,
                policy_digest,
                &policy,
                &forged_metadata,
                &forged
            ),
            Err(PinnedPolynomialBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::CheckerDigestMismatch
            ))
        );

        // A digest-matched, policy-admitted payload still needs exact body checking.
        let invalid_body = pair(3, 4);
        let invalid = FrozenEvidence {
            payload_bytes: &invalid_body,
            ..frozen
        };
        let invalid_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(&invalid_body),
            pinned,
        );
        assert_eq!(
            preflight_pinned_frozen_polynomial_evidence_bundle(
                pinned,
                policy_digest,
                &policy,
                &invalid_metadata,
                &invalid
            ),
            Err(PinnedPolynomialBundlePreflightError::Body(
                polynomial_identity::PolynomialIdentityError::CoefficientMismatch {
                    power: 0,
                    left: 3,
                    right: 4
                }
            ))
        );

        // A policy-admitted non-identity family cannot select this body checker.
        let other_policy = AuthorityContract::new(vec!["polynomial-sum".to_owned()]);
        let wrong_family = FrozenEvidence {
            evidence_family: "polynomial-sum",
            ..frozen
        };
        let wrong_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            wrong_family.evidence_family,
            digest(&valid_body),
            pinned,
        );
        assert_eq!(
            preflight_pinned_frozen_polynomial_evidence_bundle(
                pinned,
                other_policy.structural_digest(),
                &other_policy,
                &wrong_metadata,
                &wrong_family
            ),
            Err(PinnedPolynomialBundlePreflightError::WrongEvidenceFamily)
        );

        let excluded = AuthorityContract::new(vec!["finite-exhaustive".to_owned()]);
        assert_eq!(
            preflight_pinned_frozen_polynomial_evidence_bundle(
                pinned,
                excluded.structural_digest(),
                &excluded,
                &recorded,
                &frozen
            ),
            Err(PinnedPolynomialBundlePreflightError::Pinned(
                PinnedEvidenceBundlePreflightError::Bundle(EvidenceBundlePreflightError::Contract(
                    ContractPreflightError::FamilyNotAdmitted
                ))
            ))
        );
    }

    #[test]
    /// A producer may rehash a forged checker version and matching envelope.
    /// A separately supplied checker pin must reject that self-consistent swap.
    fn independently_pinned_checker_digest_rejects_producer_version_swap() {
        let j = judgement(digest(b"world"), vec![digest(b"dependency")]);
        let pinned = digest(b"independent-checker-v1");
        let producer = digest(b"unreviewed-checker-v2");
        let policy = AuthorityContract::new(vec!["polynomial-identity".to_owned()]);
        let body = b"frozen-body";
        let frozen = FrozenEvidence {
            judgement: &j,
            evidence_family: "polynomial-identity",
            payload_bytes: body,
            checker_digest: pinned,
        };
        let recorded = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(body),
            pinned,
        );
        let policy_digest = policy.structural_digest();
        assert_eq!(
            preflight_pinned_frozen_evidence_bundle(
                pinned,
                policy_digest,
                &policy,
                &recorded,
                &frozen
            ),
            Ok(())
        );

        let substituted = FrozenEvidence {
            checker_digest: producer,
            ..frozen
        };
        let forged_metadata = EvidenceEnvelopeMetadata::new(
            j.structural_digest(),
            frozen.evidence_family,
            digest(body),
            producer,
        );
        // Existing metadata/policy custody alone cannot distinguish the swap.
        assert_eq!(
            preflight_frozen_evidence_bundle(
                policy_digest,
                &policy,
                &forged_metadata,
                &substituted
            ),
            Ok(())
        );
        assert_eq!(
            preflight_pinned_frozen_evidence_bundle(
                pinned,
                policy_digest,
                &policy,
                &forged_metadata,
                &substituted
            ),
            Err(PinnedEvidenceBundlePreflightError::CheckerDigestMismatch)
        );
        assert_eq!(
            preflight_pinned_frozen_evidence_bundle(
                producer,
                policy_digest,
                &policy,
                &recorded,
                &frozen
            ),
            Err(PinnedEvidenceBundlePreflightError::CheckerDigestMismatch)
        );
        assert_eq!(
            preflight_pinned_frozen_evidence_bundle(
                pinned,
                policy_digest,
                &policy,
                &forged_metadata,
                &frozen
            ),
            Err(PinnedEvidenceBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Metadata(EvidencePreflightError::BindingMismatch)
            ))
        );
        let unrelated = AuthorityContract::new(vec!["finite-exhaustive".to_owned()]);
        assert_eq!(
            preflight_pinned_frozen_evidence_bundle(
                pinned,
                unrelated.structural_digest(),
                &unrelated,
                &recorded,
                &frozen
            ),
            Err(PinnedEvidenceBundlePreflightError::Bundle(
                EvidenceBundlePreflightError::Contract(ContractPreflightError::FamilyNotAdmitted)
            ))
        );
    }

    #[test]
    /// P2 realization preparation must bind all four frozen identities. A
    /// self-consistent producer rehash is not independent observer authority.
    fn frozen_realization_metadata_rejects_target_observer_and_artifact_swaps() {
        use formula_core::RealizationMetadata;
        let target = digest(b"independent-semantic-target");
        let observer = digest(b"independent-observer");
        let implementation = digest(b"frozen-implementation");
        let evidence = digest(b"frozen-realization-evidence");
        let recorded = RealizationMetadata::new(target, observer, implementation, evidence);
        assert_eq!(
            preflight_frozen_realization_metadata(
                &recorded,
                target,
                observer,
                implementation,
                evidence
            ),
            Ok(())
        );
        let substitutions = [
            RealizationMetadata::new(digest(b"other-target"), observer, implementation, evidence),
            RealizationMetadata::new(target, digest(b"other-observer"), implementation, evidence),
            RealizationMetadata::new(target, observer, digest(b"other-implementation"), evidence),
            RealizationMetadata::new(target, observer, implementation, digest(b"other-evidence")),
        ];
        for forged in &substitutions {
            assert_eq!(
                preflight_frozen_realization_metadata(
                    forged,
                    target,
                    observer,
                    implementation,
                    evidence
                ),
                Err(RealizationPreflightError::BindingMismatch)
            );
        }
        // Even a matching producer-rehashed record cannot replace the
        // independently frozen target and observer supplied to this check.
        let forged = RealizationMetadata::new(
            digest(b"other-target"),
            digest(b"other-observer"),
            implementation,
            evidence,
        );
        assert_eq!(
            preflight_frozen_realization_metadata(
                &forged,
                target,
                observer,
                implementation,
                evidence
            ),
            Err(RealizationPreflightError::BindingMismatch)
        );
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
