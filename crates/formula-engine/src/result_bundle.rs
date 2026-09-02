use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

use crate::obligation::TerminalState;

const RESULT_BUNDLE_SCHEMA_V1: &str = "formula-result-bundle-v1";

fn sorted_digests(mut values: Vec<ArtifactDigest>) -> Vec<ArtifactDigest> {
    values.sort_unstable();
    values.dedup();
    values
}

fn digest_array(values: &[ArtifactDigest]) -> CanonicalValue {
    CanonicalValue::Array(values.iter().copied().map(CanonicalValue::Digest).collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResultBundle {
    query_digest: ArtifactDigest,
    campaign_digest: ArtifactDigest,
    terminal_state: TerminalState,
    observer_result_refs: Vec<ArtifactDigest>,
    evidence_refs: Vec<ArtifactDigest>,
    certified_bounds: Vec<ArtifactDigest>,
    counterexample_refs: Vec<ArtifactDigest>,
    unresolved_obligations: Vec<ArtifactDigest>,
    promotion_candidate_refs: Vec<ArtifactDigest>,
    provenance: Vec<ArtifactDigest>,
}

impl ResultBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        query_digest: ArtifactDigest,
        campaign_digest: ArtifactDigest,
        terminal_state: TerminalState,
        observer_result_refs: Vec<ArtifactDigest>,
        evidence_refs: Vec<ArtifactDigest>,
        certified_bounds: Vec<ArtifactDigest>,
        counterexample_refs: Vec<ArtifactDigest>,
        unresolved_obligations: Vec<ArtifactDigest>,
        promotion_candidate_refs: Vec<ArtifactDigest>,
        provenance: Vec<ArtifactDigest>,
    ) -> Self {
        Self {
            query_digest,
            campaign_digest,
            terminal_state,
            observer_result_refs: sorted_digests(observer_result_refs),
            evidence_refs: sorted_digests(evidence_refs),
            certified_bounds: sorted_digests(certified_bounds),
            counterexample_refs: sorted_digests(counterexample_refs),
            unresolved_obligations: sorted_digests(unresolved_obligations),
            promotion_candidate_refs: sorted_digests(promotion_candidate_refs),
            provenance: sorted_digests(provenance),
        }
    }

    pub fn terminal_state(&self) -> TerminalState {
        self.terminal_state
    }

    pub fn evidence_refs(&self) -> &[ArtifactDigest] {
        &self.evidence_refs
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "campaign_digest".into(),
                CanonicalValue::Digest(self.campaign_digest),
            ),
            (
                "certified_bounds".into(),
                digest_array(&self.certified_bounds),
            ),
            (
                "counterexample_refs".into(),
                digest_array(&self.counterexample_refs),
            ),
            ("evidence_refs".into(), digest_array(&self.evidence_refs)),
            (
                "observer_result_refs".into(),
                digest_array(&self.observer_result_refs),
            ),
            (
                "promotion_candidate_refs".into(),
                digest_array(&self.promotion_candidate_refs),
            ),
            ("provenance".into(), digest_array(&self.provenance)),
            (
                "query_digest".into(),
                CanonicalValue::Digest(self.query_digest),
            ),
            (
                "schema".into(),
                CanonicalValue::String(RESULT_BUNDLE_SCHEMA_V1.into()),
            ),
            (
                "terminal_state".into(),
                CanonicalValue::String(self.terminal_state.as_str().into()),
            ),
            (
                "unresolved_obligations".into(),
                digest_array(&self.unresolved_obligations),
            ),
        ]))
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_value().to_canonical_bytes()
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
