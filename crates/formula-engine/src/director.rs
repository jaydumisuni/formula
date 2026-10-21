//! Formula-owned, authority-inert query-to-work orchestration.
//! The director delegates mathematical compilation to the existing CompilerV1.
//! It cannot certify, promote, or declare externally executed work proven.

use crate::{
    compiler::{CompiledCampaign, CompilerError, CompilerInputs, CompilerV1},
    query::{QueryIR, SideEffectPolicy},
    region::CompilerAuthoritySnapshot,
    reuse::{CompiledReuseCampaign, ReuseRequest},
};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::{BTreeMap, BTreeSet};

const DIRECTOR_SCHEMA: &str = "formula-director-manifest-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectorError {
    Compiler(CompilerError),
    InvalidCampaign,
    NoWorkCells,
    UnboundObligation,
    AuthorityMismatch,
    UnauthorizedSideEffects,
    DuplicateWorkCell,
    NoncanonicalWorkCell,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectedWorkCell {
    plan_digest: ArtifactDigest,
    obligation_digest: ArtifactDigest,
    required_authority: ArtifactDigest,
    canonical_bytes: Vec<u8>,
}

impl DirectedWorkCell {
    pub fn plan_digest(&self) -> ArtifactDigest {
        self.plan_digest
    }
    pub fn obligation_digest(&self) -> ArtifactDigest {
        self.obligation_digest
    }
    pub fn required_authority(&self) -> ArtifactDigest {
        self.required_authority
    }
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
    pub fn verify_bytes(&self) -> bool {
        ArtifactDigest::of_bytes(&self.canonical_bytes) == self.plan_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectedCampaign {
    pub compiled: CompiledCampaign,
    query_digest: ArtifactDigest,
    manifest_digest: ArtifactDigest,
    manifest_bytes: Vec<u8>,
    work: Vec<DirectedWorkCell>,
}

impl DirectedCampaign {
    pub fn query_digest(&self) -> ArtifactDigest {
        self.query_digest
    }
    pub fn manifest_digest(&self) -> ArtifactDigest {
        self.manifest_digest
    }
    pub fn manifest_bytes(&self) -> &[u8] {
        &self.manifest_bytes
    }
    pub fn work(&self) -> &[DirectedWorkCell] {
        &self.work
    }
}

pub struct FormulaMathematicalDirector;

impl FormulaMathematicalDirector {
    /// Reuses an already-admitted mathematical capability through the native
    /// compiler. No discovery work cells are scheduled for this route.
    pub fn plan_reuse(
        query: &QueryIR,
        snapshot: &CompilerAuthoritySnapshot,
        inputs: CompilerInputs,
        request: &ReuseRequest,
    ) -> Result<CompiledReuseCampaign, DirectorError> {
        CompilerV1::compile_reuse(query, snapshot, inputs, request).map_err(DirectorError::Compiler)
    }

    pub fn plan(
        query: &QueryIR,
        snapshot: &CompilerAuthoritySnapshot,
        inputs: CompilerInputs,
    ) -> Result<DirectedCampaign, DirectorError> {
        if query.side_effect_policy() != SideEffectPolicy::deny_all() {
            return Err(DirectorError::UnauthorizedSideEffects);
        }
        let compiled =
            CompilerV1::compile(query, snapshot, inputs).map_err(DirectorError::Compiler)?;
        compiled
            .campaign
            .validate()
            .map_err(|_| DirectorError::InvalidCampaign)?;
        if compiled.work_cells.is_empty() {
            return Err(DirectorError::NoWorkCells);
        }

        let obligations: BTreeSet<_> = compiled.obligations.iter().map(|o| o.digest()).collect();
        let mut work = Vec::with_capacity(compiled.work_cells.len());
        let mut seen = BTreeSet::new();
        for cell in &compiled.work_cells {
            if !obligations.contains(&cell.obligation_digest()) {
                return Err(DirectorError::UnboundObligation);
            }
            if cell.resource_budget() != query.resource_contract() {
                return Err(DirectorError::InvalidCampaign);
            }
            if cell.required_authority() != query.authority_contract() {
                return Err(DirectorError::AuthorityMismatch);
            }
            if cell.side_effect_limits() != SideEffectPolicy::deny_all()
                || cell.side_effect_limits().allows_authority_write()
            {
                return Err(DirectorError::UnauthorizedSideEffects);
            }
            let bytes = cell.canonical_bytes();
            let digest = cell.digest();
            if ArtifactDigest::of_bytes(&bytes) != digest {
                return Err(DirectorError::NoncanonicalWorkCell);
            }
            if !seen.insert(digest) {
                return Err(DirectorError::DuplicateWorkCell);
            }
            work.push(DirectedWorkCell {
                plan_digest: digest,
                obligation_digest: cell.obligation_digest(),
                required_authority: cell.required_authority(),
                canonical_bytes: bytes,
            });
        }
        work.sort_by_key(DirectedWorkCell::plan_digest);
        let manifest = CanonicalValue::Object(BTreeMap::from([
            (
                "campaign".into(),
                CanonicalValue::Digest(compiled.campaign.digest()),
            ),
            ("query".into(), CanonicalValue::Digest(query.digest())),
            (
                "replay_manifest".into(),
                CanonicalValue::Digest(compiled.replay_manifest.digest()),
            ),
            (
                "schema".into(),
                CanonicalValue::String(DIRECTOR_SCHEMA.into()),
            ),
            (
                "work_cells".into(),
                CanonicalValue::Array(
                    work.iter()
                        .map(|item| CanonicalValue::Digest(item.plan_digest()))
                        .collect(),
                ),
            ),
        ]));
        let manifest_bytes = manifest.to_canonical_bytes();
        let manifest_digest = ArtifactDigest::of_bytes(&manifest_bytes);
        Ok(DirectedCampaign {
            compiled,
            query_digest: query.digest(),
            manifest_digest,
            manifest_bytes,
            work,
        })
    }
}
