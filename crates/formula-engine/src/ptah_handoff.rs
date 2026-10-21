//! Authority-inert Formula-to-Ptah packet and untrusted result inspection.
//!
//! A transport may copy these exact bytes, but cannot certify, promote,
//! or generate a mathematical verdict. Formula's independent checker must
//! evaluate all returned candidate evidence before a promotion is possible.

use crate::director::{DirectedCampaign, DirectedWorkCell};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PtahContractError {
    WorkCellNotFound,
    CorruptManifest,
    CorruptWorkCell,
    MismatchedManifest,
    MismatchedWorkCell,
    MismatchedCandidateDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PtahHandoff {
    manifest_digest: ArtifactDigest,
    manifest_bytes: Vec<u8>,
    work_cell_digest: ArtifactDigest,
    work_cell_bytes: Vec<u8>,
    obligation_digest: ArtifactDigest,
    required_authority: ArtifactDigest,
}

impl PtahHandoff {
    /// Form only from a product-owned Director plan, never from a Ptah verdict.
    pub fn from_director(
        plan: &DirectedCampaign,
        work_cell_digest: ArtifactDigest,
    ) -> Result<Self, PtahContractError> {
        let work = plan
            .work()
            .iter()
            .find(|item: &&DirectedWorkCell| item.plan_digest() == work_cell_digest)
            .ok_or(PtahContractError::WorkCellNotFound)?;
        let handoff = Self {
            manifest_digest: plan.manifest_digest(),
            manifest_bytes: plan.manifest_bytes().to_vec(),
            work_cell_digest,
            work_cell_bytes: work.canonical_bytes().to_vec(),
            obligation_digest: work.obligation_digest(),
            required_authority: work.required_authority(),
        };
        handoff.validate()?;
        Ok(handoff)
    }

    pub fn manifest_digest(&self) -> ArtifactDigest {
        self.manifest_digest
    }
    pub fn manifest_bytes(&self) -> &[u8] {
        &self.manifest_bytes
    }
    pub fn work_cell_digest(&self) -> ArtifactDigest {
        self.work_cell_digest
    }
    pub fn work_cell_bytes(&self) -> &[u8] {
        &self.work_cell_bytes
    }
    pub fn obligation_digest(&self) -> ArtifactDigest {
        self.obligation_digest
    }
    pub fn required_authority(&self) -> ArtifactDigest {
        self.required_authority
    }

    /// Portable canonical JSON envelope. Carries the exact byte-bound
    /// Formula work plan without treating a Ptah receipt as a certificate.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let manifest_text = String::from_utf8(self.manifest_bytes.clone())
            .expect("canonical Formula manifest must be UTF-8 JSON");
        let work_cell_text = String::from_utf8(self.work_cell_bytes.clone())
            .expect("canonical Formula work cell must be UTF-8 JSON");
        CanonicalValue::Object(BTreeMap::from([
            (
                "manifest_digest".into(),
                CanonicalValue::Digest(self.manifest_digest),
            ),
            (
                "manifest_json".into(),
                CanonicalValue::String(manifest_text),
            ),
            (
                "obligation_digest".into(),
                CanonicalValue::Digest(self.obligation_digest),
            ),
            (
                "required_authority".into(),
                CanonicalValue::Digest(self.required_authority),
            ),
            (
                "result_policy".into(),
                CanonicalValue::String("CANDIDATE_ONLY".into()),
            ),
            (
                "schema".into(),
                CanonicalValue::String("formula-ptah-handoff-v1".into()),
            ),
            (
                "work_cell_digest".into(),
                CanonicalValue::Digest(self.work_cell_digest),
            ),
            (
                "work_cell_json".into(),
                CanonicalValue::String(work_cell_text),
            ),
        ]))
        .to_canonical_bytes()
    }

    pub fn validate(&self) -> Result<(), PtahContractError> {
        if ArtifactDigest::of_bytes(&self.manifest_bytes) != self.manifest_digest {
            return Err(PtahContractError::CorruptManifest);
        }
        if ArtifactDigest::of_bytes(&self.work_cell_bytes) != self.work_cell_digest {
            return Err(PtahContractError::CorruptWorkCell);
        }
        Ok(())
    }

    /// Verify transport integrity ONLY. The result remains untrusted candidate
    /// data until Formula checks the mathematical evidence independently.
    pub fn inspect_candidate(
        &self,
        receipt: PtahCandidateReceipt,
    ) -> Result<UntrustedCandidate, PtahContractError> {
        self.validate()?;
        if receipt.manifest_digest != self.manifest_digest {
            return Err(PtahContractError::MismatchedManifest);
        }
        if receipt.work_cell_digest != self.work_cell_digest {
            return Err(PtahContractError::MismatchedWorkCell);
        }
        if ArtifactDigest::of_bytes(&receipt.bytes) != receipt.claimed_digest {
            return Err(PtahContractError::MismatchedCandidateDigest);
        }
        Ok(UntrustedCandidate {
            digest: receipt.claimed_digest,
            bytes: receipt.bytes,
        })
    }
}

/// Unverified bytes and exact requested identities returned by a Ptah executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PtahCandidateReceipt {
    pub manifest_digest: ArtifactDigest,
    pub work_cell_digest: ArtifactDigest,
    pub claimed_digest: ArtifactDigest,
    pub bytes: Vec<u8>,
}

/// This type deliberately has no promote/approve/certify operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UntrustedCandidate {
    digest: ArtifactDigest,
    bytes: Vec<u8>,
}

impl UntrustedCandidate {
    pub fn digest(&self) -> ArtifactDigest {
        self.digest
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
