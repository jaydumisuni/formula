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
    // A single terminal question mark is a permitted spelling variant.
    // Repeated or interior question marks must not alias an approved QueryIR.
    let trimmed = text.trim();
    let reduced = trimmed.strip_suffix('?').unwrap_or(trimmed);
    if reduced.contains('?') {
        return Err(GroundingError::InvalidText);
    }
    let reduced = reduced.to_ascii_lowercase();
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
        // A phrase's result-class prefix must describe EVERY target, not
        // merely the first sorted target in QueryIR. Empty or heterogeneous
        // target sets require explicit QueryIR, not one misleading utterance.
        let class = query
            .grounded_result_class()
            .ok_or(GroundingError::ClassMismatch)?;
        let prefix = match class {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{ActivatedPackageBinding, ResourceContract, TargetRequest};

    fn d(n: u8) -> ArtifactDigest {
        ArtifactDigest::of_bytes(&[n; 32])
    }

    fn query(targets: Vec<TargetRequest>) -> QueryIR {
        QueryIR::new(
            d(1),
            d(2),
            vec![],
            vec![],
            targets,
            d(4),
            d(5),
            ResourceContract::new(100, 1024, 50),
            SideEffectPolicy::deny_all(),
            ActivatedPackageBinding::new(d(1), vec![d(30)], vec![]),
        )
    }

    #[test]
    fn reject_empty_and_mixed_result_classes_before_text_grounding() {
        assert_eq!(
            GroundedPhrase::new("is the result valid", query(vec![]), Some(d(9))),
            Err(GroundingError::ClassMismatch)
        );
        let mixed = query(vec![
            TargetRequest::new(d(3), RequestedResultClass::Count),
            TargetRequest::new(d(6), RequestedResultClass::Decision),
        ]);
        assert_eq!(
            GroundedPhrase::new("count approved targets", mixed, Some(d(9))),
            Err(GroundingError::ClassMismatch)
        );
    }

    #[test]
    fn unapproved_question_mark_aliases_fail_closed() {
        let q = query(vec![TargetRequest::new(d(3), RequestedResultClass::Count)]);
        let approved =
            GroundedPhrase::new("count approved targets", q.clone(), Some(d(9))).unwrap();
        for instruction in [
            "count approved targets??",
            "count approved? targets",
            "count approved targets? ?",
        ] {
            assert_eq!(
                resolve(instruction, std::slice::from_ref(&approved)),
                Err(GroundingError::InvalidText)
            );
        }
        assert_eq!(
            GroundedPhrase::new("count approved targets??", q.clone(), Some(d(9))),
            Err(GroundingError::InvalidText)
        );
        assert_eq!(resolve("count approved targets?", &[approved]).unwrap(), q);
    }

    #[test]
    fn homogeneous_multi_target_grounding_remains_possible() {
        let homogeneous = query(vec![
            TargetRequest::new(d(3), RequestedResultClass::Count),
            TargetRequest::new(d(6), RequestedResultClass::Count),
        ]);
        let phrase =
            GroundedPhrase::new("count approved targets", homogeneous.clone(), Some(d(9))).unwrap();
        assert_eq!(
            resolve("count approved targets?", &[phrase]).unwrap(),
            homogeneous
        );
    }
}
