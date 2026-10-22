//! Grounded text selection of an already-authorized Formula QueryIR.
//! An untrusted language model cannot mint targets or universe generations.

use crate::query::{QueryIR, RequestedResultClass, SideEffectPolicy};
use formula_core::digest::ArtifactDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroundingError {
    InvalidText,
    MissingEvidence,
    ClassMismatch,
    UnknownMeaning,
    AmbiguousMeaning,
}

fn normalize(text: &str) -> Result<String, GroundingError> {
    if text.is_empty() || text.len() > 2048 || !text.is_ascii() {
        return Err(GroundingError::InvalidText);
    }
    let reduced = text.trim().trim_end_matches('?').to_ascii_lowercase();
    if reduced.bytes().any(|b| !b.is_ascii_graphic() && b != b' ') {
        return Err(GroundingError::InvalidText);
    }
    let normalized = reduced
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if normalized.is_empty() || normalized.len() > 512 {
        return Err(GroundingError::InvalidText);
    }
    Ok(normalized)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroundedPhrase {
    text: String,
    query: QueryIR,
    grounding_evidence: ArtifactDigest,
}

impl GroundedPhrase {
    pub fn new(
        text: &str,
        query: QueryIR,
        evidence: Option<ArtifactDigest>,
    ) -> Result<Self, GroundingError> {
        let text = normalize(text)?;
        let prefix = match query.requested_result_class() {
            RequestedResultClass::Count => &["count ", "how many "][..],
            RequestedResultClass::Decision => &["is ", "does ", "decide "][..],
            RequestedResultClass::Witness => &["find ", "show "][..],
            RequestedResultClass::Optimum => &["optimize ", "minimize ", "maximize "][..],
            RequestedResultClass::Bound => &["bound ", "estimate "][..],
        };
        if !prefix.iter().any(|start| text.starts_with(start))
            || query.side_effect_policy() != SideEffectPolicy::deny_all()
        {
            return Err(GroundingError::ClassMismatch);
        }
        Ok(Self {
            text,
            query,
            grounding_evidence: evidence.ok_or(GroundingError::MissingEvidence)?,
        })
    }
    pub fn evidence(&self) -> ArtifactDigest {
        self.grounding_evidence
    }
}

pub fn resolve(
    instruction: &str,
    preapproved: &[GroundedPhrase],
) -> Result<QueryIR, GroundingError> {
    let text = normalize(instruction)?;
    let mut matches = preapproved.iter().filter(|item| item.text == text);
    let first = matches.next().ok_or(GroundingError::UnknownMeaning)?;
    if matches.next().is_some() {
        return Err(GroundingError::AmbiguousMeaning);
    }
    Ok(first.query.clone())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroundedPlanningError {
    Grounding(GroundingError),
    Compiler(crate::director::DirectorError),
}
