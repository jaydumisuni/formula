//! Multi-WorkCell candidate collection. Ptah transports and runs exact
//! Formula cells; only Formula's independent checker can establish math truth.
//!
//! Checkpoint replay is caller-controlled: reconstruct from durable Ptah
//! candidate bytes/receipts, never from an acknowledgement alone. This module
//! does not claim that E03 transport or remote retries are implemented here.

use crate::director::DirectedCampaign;
use crate::ptah_handoff::{
    PtahCandidateReceipt, PtahContractError, PtahHandoff, UntrustedCandidate,
};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PtahCampaignError {
    EmptyCampaign,
    InvalidHandoff(PtahContractError),
    UnexpectedCell,
    DuplicateConflictingResult,
    IncompleteResults,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PtahCampaignBatch {
    manifest_digest: ArtifactDigest,
    cells: BTreeMap<ArtifactDigest, PtahHandoff>,
    /// A real candidate receipt, never a mere E02/A05 dispatch acknowledgement.
    candidates: BTreeMap<ArtifactDigest, PtahCandidateReceipt>,
}

impl PtahCampaignBatch {
    pub fn from_director(plan: &DirectedCampaign) -> Result<Self, PtahCampaignError> {
        Self::from_directors(std::slice::from_ref(plan))
    }

    /// Several independently compiled, fully grounded Formula campaigns can
    /// contribute one bounded candidate collection. The compiler, not this
    /// adapter, owns decomposition within each campaign.
    pub fn from_directors(plans: &[DirectedCampaign]) -> Result<Self, PtahCampaignError> {
        if plans.is_empty() {
            return Err(PtahCampaignError::EmptyCampaign);
        }
        let mut cells = BTreeMap::new();
        let mut manifests = BTreeSet::new();
        for plan in plans {
            if plan.work().is_empty() {
                return Err(PtahCampaignError::EmptyCampaign);
            }
            if !manifests.insert(plan.manifest_digest()) {
                return Err(PtahCampaignError::DuplicateConflictingResult);
            }
            for work in plan.work() {
                let packet = PtahHandoff::from_director(plan, work.plan_digest())
                    .map_err(PtahCampaignError::InvalidHandoff)?;
                if cells.insert(work.plan_digest(), packet).is_some() {
                    return Err(PtahCampaignError::DuplicateConflictingResult);
                }
            }
        }
        let manifest_digest =
            CanonicalValue::Array(manifests.into_iter().map(CanonicalValue::Digest).collect())
                .digest();
        Ok(Self {
            manifest_digest,
            cells,
            candidates: BTreeMap::new(),
        })
    }

    pub fn manifest_for_cell(&self, cell: ArtifactDigest) -> Option<ArtifactDigest> {
        self.cells.get(&cell).map(PtahHandoff::manifest_digest)
    }

    pub fn manifest_digest(&self) -> ArtifactDigest {
        self.manifest_digest
    }

    /// All cells are ordered by content identity, not remote scheduling order.
    pub fn outgoing_packets(&self) -> Vec<(ArtifactDigest, Vec<u8>)> {
        self.cells
            .iter()
            .map(|(&digest, h)| (digest, h.wire_bytes()))
            .collect()
    }

    /// Replayed evidence must contain actual candidate bytes, exact hashes and
    /// matching campaign/cell identity. An Ack or success exit is insufficient.
    /// Identical readbacks are idempotent; a conflicting retry fails closed.
    pub fn observe(&mut self, receipt: PtahCandidateReceipt) -> Result<(), PtahCampaignError> {
        let work_cell = receipt.work_cell_digest;
        let packet = self
            .cells
            .get(&work_cell)
            .ok_or(PtahCampaignError::UnexpectedCell)?;
        packet
            .inspect_candidate(receipt.clone())
            .map_err(PtahCampaignError::InvalidHandoff)?;
        if let Some(existing) = self.candidates.get(&work_cell) {
            if existing != &receipt {
                return Err(PtahCampaignError::DuplicateConflictingResult);
            }
            return Ok(());
        }
        self.candidates.insert(work_cell, receipt);
        Ok(())
    }

    pub fn pending_cells(&self) -> BTreeSet<ArtifactDigest> {
        self.cells
            .keys()
            .filter(|k| !self.candidates.contains_key(k))
            .copied()
            .collect()
    }

    /// Canonical, digest-bound summary for caller-owned durable checkpoints.
    /// No mathematical PASS flag or authority status is present.
    pub fn observation_bytes(&self) -> Vec<u8> {
        let completed = self
            .candidates
            .iter()
            .map(|(&cell, receipt)| {
                CanonicalValue::Object(BTreeMap::from([
                    ("cell".into(), CanonicalValue::Digest(cell)),
                    (
                        "candidate_digest".into(),
                        CanonicalValue::Digest(receipt.claimed_digest),
                    ),
                ]))
            })
            .collect();
        let pending = self
            .pending_cells()
            .into_iter()
            .map(CanonicalValue::Digest)
            .collect();
        CanonicalValue::Object(BTreeMap::from([
            (
                "schema".into(),
                CanonicalValue::String("formula-ptah-candidate-collection-v1".into()),
            ),
            (
                "manifest".into(),
                CanonicalValue::Digest(self.manifest_digest),
            ),
            ("completed".into(), CanonicalValue::Array(completed)),
            ("pending".into(), CanonicalValue::Array(pending)),
        ]))
        .to_canonical_bytes()
    }

    pub fn candidates(
        &self,
    ) -> Result<Vec<(ArtifactDigest, UntrustedCandidate)>, PtahCampaignError> {
        if !self.pending_cells().is_empty() {
            return Err(PtahCampaignError::IncompleteResults);
        }
        let mut out = Vec::with_capacity(self.cells.len());
        for (&cell, packet) in &self.cells {
            let receipt = self
                .candidates
                .get(&cell)
                .ok_or(PtahCampaignError::IncompleteResults)?;
            let untrusted = packet
                .inspect_candidate(receipt.clone())
                .map_err(PtahCampaignError::InvalidHandoff)?;
            out.push((cell, untrusted));
        }
        Ok(out)
    }
}
