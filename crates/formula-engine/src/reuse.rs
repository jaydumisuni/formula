use crate::query::{QueryIR, RequestedResultClass};
use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::BTreeMap;

const REUSE_REQUEST_SCHEMA_V1: &str = "formula-reuse-request-v1";
const RESOLVED_CAPABILITY_SCHEMA_V1: &str = "formula-resolved-capability-v1";
const REUSE_EXECUTION_SCHEMA_V1: &str = "formula-reuse-execution-plan-v1";
const REUSE_METRICS_SCHEMA_V1: &str = "formula-reuse-metrics-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseRequest {
    query_digest: ArtifactDigest,
    required_semantic_capability: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    result_class: RequestedResultClass,
}

impl ReuseRequest {
    pub fn new(query: &QueryIR, required_semantic_capability: ArtifactDigest) -> Self {
        Self {
            query_digest: query.digest(),
            required_semantic_capability,
            universe_generation: query.universe_generation(),
            world: query.world(),
            authority_contract: query.authority_contract(),
            observer: query.observer(),
            result_class: query.requested_result_class(),
        }
    }

    pub fn query_digest(&self) -> ArtifactDigest { self.query_digest }
    pub fn required_semantic_capability(&self) -> ArtifactDigest { self.required_semantic_capability }
    pub fn universe_generation(&self) -> ArtifactDigest { self.universe_generation }
    pub fn world(&self) -> ArtifactDigest { self.world }
    pub fn authority_contract(&self) -> ArtifactDigest { self.authority_contract }
    pub fn observer(&self) -> ArtifactDigest { self.observer }
    pub fn result_class(&self) -> RequestedResultClass { self.result_class }

    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            ("authority_contract".into(), CanonicalValue::Digest(self.authority_contract)),
            ("observer".into(), CanonicalValue::Digest(self.observer)),
            ("query_digest".into(), CanonicalValue::Digest(self.query_digest)),
            ("required_semantic_capability".into(), CanonicalValue::Digest(self.required_semantic_capability)),
            ("result_class".into(), CanonicalValue::String(self.result_class.as_str().into())),
            ("schema".into(), CanonicalValue::String(REUSE_REQUEST_SCHEMA_V1.into())),
            ("universe_generation".into(), CanonicalValue::Digest(self.universe_generation)),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ])).digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCapability {
    reuse_request: ArtifactDigest,
    primitive: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

impl ResolvedCapability {
    pub(crate) fn new(request: &ReuseRequest) -> Self {
        Self {
            reuse_request: request.digest(),
            primitive: request.required_semantic_capability(),
            universe_generation: request.universe_generation(),
            world: request.world(),
            authority_contract: request.authority_contract(),
            observer: request.observer(),
        }
    }

    pub fn primitive(&self) -> ArtifactDigest { self.primitive }
    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            ("authority_contract".into(), CanonicalValue::Digest(self.authority_contract)),
            ("observer".into(), CanonicalValue::Digest(self.observer)),
            ("primitive".into(), CanonicalValue::Digest(self.primitive)),
            ("reuse_request".into(), CanonicalValue::Digest(self.reuse_request)),
            ("schema".into(), CanonicalValue::String(RESOLVED_CAPABILITY_SCHEMA_V1.into())),
            ("universe_generation".into(), CanonicalValue::Digest(self.universe_generation)),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ])).digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseExecutionPlan {
    query: ArtifactDigest,
    resolved_capability: ArtifactDigest,
    primitive: ArtifactDigest,
    result_class: RequestedResultClass,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

impl ReuseExecutionPlan {
    pub(crate) fn new(query: &QueryIR, resolved: &ResolvedCapability) -> Self {
        Self {
            query: query.digest(),
            resolved_capability: resolved.digest(),
            primitive: resolved.primitive(),
            result_class: query.requested_result_class(),
            universe_generation: query.universe_generation(),
            world: query.world(),
            authority_contract: query.authority_contract(),
            observer: query.observer(),
        }
    }

    pub fn primitive(&self) -> ArtifactDigest { self.primitive }
    pub fn result_class(&self) -> RequestedResultClass { self.result_class }
    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            ("authority_contract".into(), CanonicalValue::Digest(self.authority_contract)),
            ("observer".into(), CanonicalValue::Digest(self.observer)),
            ("primitive".into(), CanonicalValue::Digest(self.primitive)),
            ("query".into(), CanonicalValue::Digest(self.query)),
            ("resolved_capability".into(), CanonicalValue::Digest(self.resolved_capability)),
            ("result_class".into(), CanonicalValue::String(self.result_class.as_str().into())),
            ("schema".into(), CanonicalValue::String(REUSE_EXECUTION_SCHEMA_V1.into())),
            ("universe_generation".into(), CanonicalValue::Digest(self.universe_generation)),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ])).digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseMetrics {
    primitive_discovery_candidate_spaces: u64,
    primitive_discovery_work_cells: u64,
    resolved_capability_count: u64,
    execution_work_items: u64,
}

impl ReuseMetrics {
    pub(crate) fn canonical_single_reuse() -> Self {
        Self {
            primitive_discovery_candidate_spaces: 0,
            primitive_discovery_work_cells: 0,
            resolved_capability_count: 1,
            execution_work_items: 1,
        }
    }

    pub fn primitive_discovery_candidate_spaces(&self) -> u64 { self.primitive_discovery_candidate_spaces }
    pub fn primitive_discovery_work_cells(&self) -> u64 { self.primitive_discovery_work_cells }
    pub fn resolved_capability_count(&self) -> u64 { self.resolved_capability_count }
    pub fn execution_work_items(&self) -> u64 { self.execution_work_items }
    pub fn digest(&self) -> ArtifactDigest {
        CanonicalValue::Object(BTreeMap::from([
            ("execution_work_items".into(), CanonicalValue::String(self.execution_work_items.to_string())),
            ("primitive_discovery_candidate_spaces".into(), CanonicalValue::String(self.primitive_discovery_candidate_spaces.to_string())),
            ("primitive_discovery_work_cells".into(), CanonicalValue::String(self.primitive_discovery_work_cells.to_string())),
            ("resolved_capability_count".into(), CanonicalValue::String(self.resolved_capability_count.to_string())),
            ("schema".into(), CanonicalValue::String(REUSE_METRICS_SCHEMA_V1.into())),
        ])).digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledReuseCampaign {
    pub(crate) campaign: crate::campaign::CampaignIR,
    pub(crate) resolved_capability: ResolvedCapability,
    pub(crate) execution_plans: Vec<ReuseExecutionPlan>,
    pub(crate) metrics: ReuseMetrics,
}

impl CompiledReuseCampaign {
    pub fn campaign(&self) -> &crate::campaign::CampaignIR { &self.campaign }
    pub fn resolved_capability(&self) -> &ResolvedCapability { &self.resolved_capability }
    pub fn execution_plans(&self) -> &[ReuseExecutionPlan] { &self.execution_plans }
    pub fn metrics(&self) -> &ReuseMetrics { &self.metrics }
    pub fn work_cells(&self) -> &[crate::work_cell::WorkCellPlan] { &[] }
}
