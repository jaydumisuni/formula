use formula_core::{canonical::CanonicalValue, digest::ArtifactDigest};
use std::collections::{BTreeMap, BTreeSet};

const CAMPAIGN_SCHEMA_V1: &str = "formula-campaign-ir-v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CampaignNodeKind {
    Goal,
    Route,
    Obligation,
    WorldRef,
    ArtifactRef,
    FactRef,
    WorkCellPlanRef,
    ResultRef,
}

impl CampaignNodeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Goal => "GOAL",
            Self::Route => "ROUTE",
            Self::Obligation => "OBLIGATION",
            Self::WorldRef => "WORLD_REF",
            Self::ArtifactRef => "ARTIFACT_REF",
            Self::FactRef => "FACT_REF",
            Self::WorkCellPlanRef => "WORK_CELL_PLAN_REF",
            Self::ResultRef => "RESULT_REF",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CampaignAggregation {
    And,
    Or,
}

impl CampaignAggregation {
    fn as_str(self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CampaignNode {
    id: ArtifactDigest,
    kind: CampaignNodeKind,
    generation: ArtifactDigest,
    world: ArtifactDigest,
    aggregation: Option<CampaignAggregation>,
}

impl CampaignNode {
    pub fn new(
        id: ArtifactDigest,
        kind: CampaignNodeKind,
        generation: ArtifactDigest,
        world: ArtifactDigest,
        aggregation: Option<CampaignAggregation>,
    ) -> Self {
        Self {
            id,
            kind,
            generation,
            world,
            aggregation,
        }
    }

    pub fn id(&self) -> ArtifactDigest {
        self.id
    }

    pub fn kind(&self) -> CampaignNodeKind {
        self.kind
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "aggregation".into(),
                self.aggregation
                    .map(|value| CanonicalValue::String(value.as_str().into()))
                    .unwrap_or(CanonicalValue::Null),
            ),
            ("generation".into(), CanonicalValue::Digest(self.generation)),
            ("id".into(), CanonicalValue::Digest(self.id)),
            (
                "kind".into(),
                CanonicalValue::String(self.kind.as_str().into()),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CampaignEdgeKind {
    Requires,
    Produces,
    Satisfies,
    Refutes,
    AlternativeTo,
    DecomposesInto,
    ReducesTo,
    TransportsTo,
    Unlocks,
}

impl CampaignEdgeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Requires => "REQUIRES",
            Self::Produces => "PRODUCES",
            Self::Satisfies => "SATISFIES",
            Self::Refutes => "REFUTES",
            Self::AlternativeTo => "ALTERNATIVE_TO",
            Self::DecomposesInto => "DECOMPOSES_INTO",
            Self::ReducesTo => "REDUCES_TO",
            Self::TransportsTo => "TRANSPORTS_TO",
            Self::Unlocks => "UNLOCKS",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CampaignEdge {
    source: ArtifactDigest,
    target: ArtifactDigest,
    kind: CampaignEdgeKind,
}

impl CampaignEdge {
    pub fn new(source: ArtifactDigest, target: ArtifactDigest, kind: CampaignEdgeKind) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }

    fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "kind".into(),
                CanonicalValue::String(self.kind.as_str().into()),
            ),
            ("source".into(), CanonicalValue::Digest(self.source)),
            ("target".into(), CanonicalValue::Digest(self.target)),
        ]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CampaignError {
    DanglingReference,
    GenerationMismatch,
    WorldMismatch,
    IllegalAggregation,
    RouteWithoutObligation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignIR {
    generation: ArtifactDigest,
    world: ArtifactDigest,
    nodes: Vec<CampaignNode>,
    edges: Vec<CampaignEdge>,
}

impl CampaignIR {
    pub fn new(
        generation: ArtifactDigest,
        world: ArtifactDigest,
        mut nodes: Vec<CampaignNode>,
        mut edges: Vec<CampaignEdge>,
    ) -> Self {
        nodes.sort_unstable();
        nodes.dedup();
        edges.sort_unstable();
        edges.dedup();
        Self {
            generation,
            world,
            nodes,
            edges,
        }
    }

    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.nodes.iter().any(|node| node.generation != self.generation) {
            return Err(CampaignError::GenerationMismatch);
        }
        if self.nodes.iter().any(|node| node.world != self.world) {
            return Err(CampaignError::WorldMismatch);
        }
        if self.nodes.iter().any(|node| {
            node.aggregation.is_some()
                && !matches!(node.kind, CampaignNodeKind::Goal | CampaignNodeKind::Route)
        }) {
            return Err(CampaignError::IllegalAggregation);
        }

        let node_ids: BTreeSet<_> = self.nodes.iter().map(CampaignNode::id).collect();
        if self
            .edges
            .iter()
            .any(|edge| !node_ids.contains(&edge.source) || !node_ids.contains(&edge.target))
        {
            return Err(CampaignError::DanglingReference);
        }

        for route in self
            .nodes
            .iter()
            .filter(|node| node.kind() == CampaignNodeKind::Route)
        {
            let has_obligation = self.edges.iter().any(|edge| {
                edge.source == route.id()
                    && edge.kind == CampaignEdgeKind::Requires
                    && self.nodes.iter().any(|node| {
                        node.id() == edge.target && node.kind() == CampaignNodeKind::Obligation
                    })
            });
            if !has_obligation {
                return Err(CampaignError::RouteWithoutObligation);
            }
        }
        Ok(())
    }

    pub fn canonical_value(&self) -> CanonicalValue {
        CanonicalValue::Object(BTreeMap::from([
            (
                "edges".into(),
                CanonicalValue::Array(
                    self.edges
                        .iter()
                        .map(CampaignEdge::canonical_value)
                        .collect(),
                ),
            ),
            ("generation".into(), CanonicalValue::Digest(self.generation)),
            (
                "nodes".into(),
                CanonicalValue::Array(
                    self.nodes
                        .iter()
                        .map(CampaignNode::canonical_value)
                        .collect(),
                ),
            ),
            (
                "schema".into(),
                CanonicalValue::String(CAMPAIGN_SCHEMA_V1.into()),
            ),
            ("world".into(), CanonicalValue::Digest(self.world)),
        ]))
    }

    pub fn digest(&self) -> ArtifactDigest {
        self.canonical_value().digest()
    }
}
