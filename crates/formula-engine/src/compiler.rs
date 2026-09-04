use formula_core::digest::ArtifactDigest;

use crate::{
    campaign::{
        CampaignAggregation, CampaignEdge, CampaignEdgeKind, CampaignIR, CampaignNode,
        CampaignNodeKind,
    },
    decomposition::Decomposition,
    obligation::{ObligationIR, TerminalState},
    query::{QueryIR, SideEffectPolicy},
    reduction::{ReductionEdge, ReductionError, compose_reduction_path},
    region::{CompilerAuthoritySnapshot, RegionError, RelevantRegion},
    replay::ReplayManifest,
    representation::{InformationLoss, RepresentationEdge, RepresentationNode},
    reuse::{
        CompiledReuseCampaign, ResolvedCapability, ReuseExecutionPlan, ReuseMetrics, ReuseRequest,
    },
    theory_profile::{OperationalEstimate, ProfileFact, TheoryProfile},
    work_cell::{CheckpointPolicy, StopCondition, WorkCellPlan},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParentResolution {
    NoneRequired,
    Unique(ArtifactDigest),
    Ambiguous(Vec<ArtifactDigest>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplicitMorphism {
    morphism: ArtifactDigest,
    information_loss: InformationLoss,
    explicit: bool,
}

impl ImplicitMorphism {
    pub fn new(
        morphism: ArtifactDigest,
        information_loss: InformationLoss,
        explicit: bool,
    ) -> Self {
        Self {
            morphism,
            information_loss,
            explicit,
        }
    }

    fn semantic_digest(&self) -> ArtifactDigest {
        ArtifactDigest::of_bytes(
            format!(
                "formula-implicit-morphism-v1|{}|{:?}|{}",
                self.morphism.as_str(),
                self.information_loss,
                self.explicit
            )
            .as_bytes(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentationRoute {
    source: RepresentationNode,
    target: RepresentationNode,
    edge: RepresentationEdge,
}

impl RepresentationRoute {
    pub fn new(
        source: RepresentationNode,
        target: RepresentationNode,
        edge: RepresentationEdge,
    ) -> Self {
        Self {
            source,
            target,
            edge,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerInputs {
    expected_observer: ArtifactDigest,
    expected_authority: ArtifactDigest,
    evidence_requirement: ArtifactDigest,
    random_key: ArtifactDigest,
    exact_properties: Vec<ProfileFact>,
    operational_estimates: Vec<OperationalEstimate>,
    parent_resolution: ParentResolution,
    implicit_morphisms: Vec<ImplicitMorphism>,
    representation_routes: Vec<RepresentationRoute>,
    reduction_paths: Vec<Vec<ReductionEdge>>,
    decompositions: Vec<Decomposition>,
    compiler_policy_version: String,
    scheduler_policy_version: String,
}

impl CompilerInputs {
    pub fn new(
        expected_observer: ArtifactDigest,
        expected_authority: ArtifactDigest,
        evidence_requirement: ArtifactDigest,
        random_key: ArtifactDigest,
    ) -> Self {
        Self {
            expected_observer,
            expected_authority,
            evidence_requirement,
            random_key,
            exact_properties: Vec::new(),
            operational_estimates: Vec::new(),
            parent_resolution: ParentResolution::NoneRequired,
            implicit_morphisms: Vec::new(),
            representation_routes: Vec::new(),
            reduction_paths: Vec::new(),
            decompositions: Vec::new(),
            compiler_policy_version: "compiler-v1".into(),
            scheduler_policy_version: "scheduler-v1".into(),
        }
    }

    pub fn with_expected_observer(mut self, value: ArtifactDigest) -> Self {
        self.expected_observer = value;
        self
    }

    pub fn with_expected_authority(mut self, value: ArtifactDigest) -> Self {
        self.expected_authority = value;
        self
    }

    pub fn with_exact_properties(mut self, values: Vec<ProfileFact>) -> Self {
        self.exact_properties = values;
        self
    }

    pub fn with_operational_estimates(mut self, values: Vec<OperationalEstimate>) -> Self {
        self.operational_estimates = values;
        self
    }

    pub fn with_parent_resolution(mut self, value: ParentResolution) -> Self {
        self.parent_resolution = value;
        self
    }

    pub fn with_implicit_morphisms(mut self, values: Vec<ImplicitMorphism>) -> Self {
        self.implicit_morphisms = values;
        self
    }

    pub fn with_representation_routes(mut self, values: Vec<RepresentationRoute>) -> Self {
        self.representation_routes = values;
        self
    }

    pub fn with_reduction_paths(mut self, values: Vec<Vec<ReductionEdge>>) -> Self {
        self.reduction_paths = values;
        self
    }

    pub fn with_decompositions(mut self, values: Vec<Decomposition>) -> Self {
        self.decompositions = values;
        self
    }

    pub fn with_compiler_policy_version(mut self, value: impl Into<String>) -> Self {
        self.compiler_policy_version = value.into();
        self
    }

    pub fn with_scheduler_policy_version(mut self, value: impl Into<String>) -> Self {
        self.scheduler_policy_version = value.into();
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompilerError {
    GenerationMismatch,
    WorldMismatch,
    PackageContextMismatch,
    ObserverMismatch,
    AuthorityMismatch,
    ReuseRequestMismatch,
    RequiredCapabilityUnavailable,
    ImplicitLossyMorphism,
    AmbiguousParent,
    InvalidRepresentation,
    ReductionResultClassLoss,
    InvalidReduction,
    InvalidDecomposition,
    InvalidCampaign,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledCampaign {
    pub region: RelevantRegion,
    pub theory_profile: TheoryProfile,
    pub campaign: CampaignIR,
    pub obligations: Vec<ObligationIR>,
    pub work_cells: Vec<WorkCellPlan>,
    pub replay_manifest: ReplayManifest,
}

pub struct CompilerV1;

impl CompilerV1 {
    pub fn compile(
        query: &QueryIR,
        snapshot: &CompilerAuthoritySnapshot,
        inputs: CompilerInputs,
    ) -> Result<CompiledCampaign, CompilerError> {
        if query.observer() != inputs.expected_observer {
            return Err(CompilerError::ObserverMismatch);
        }
        if query.authority_contract() != inputs.expected_authority {
            return Err(CompilerError::AuthorityMismatch);
        }

        let region = RelevantRegion::from_snapshot(query, snapshot).map_err(map_region_error)?;

        match &inputs.parent_resolution {
            ParentResolution::Ambiguous(_) => return Err(CompilerError::AmbiguousParent),
            ParentResolution::NoneRequired | ParentResolution::Unique(_) => {}
        }

        for morphism in &inputs.implicit_morphisms {
            if morphism.information_loss == InformationLoss::Declared && !morphism.explicit {
                return Err(CompilerError::ImplicitLossyMorphism);
            }
        }

        let requested = query.requested_result_class();
        let mut route_digests = Vec::new();

        if let ParentResolution::Unique(parent) = inputs.parent_resolution {
            route_digests.push(ArtifactDigest::of_bytes(
                format!("formula-parent-resolution-v1|{}", parent.as_str()).as_bytes(),
            ));
        }

        for morphism in &inputs.implicit_morphisms {
            route_digests.push(morphism.semantic_digest());
        }

        for route in &inputs.representation_routes {
            route
                .edge
                .validate(&route.source, &route.target, requested)
                .map_err(|_| CompilerError::InvalidRepresentation)?;
            route_digests.push(route.edge.digest());
        }

        for path in &inputs.reduction_paths {
            let composed =
                compose_reduction_path(path, requested).map_err(|error| match error {
                    ReductionError::RequestedResultNotPreserved => {
                        CompilerError::ReductionResultClassLoss
                    }
                    _ => CompilerError::InvalidReduction,
                })?;
            route_digests.push(composed.digest());
        }

        for decomposition in &inputs.decompositions {
            decomposition
                .validate()
                .map_err(|_| CompilerError::InvalidDecomposition)?;
            route_digests.push(decomposition.digest());
        }

        route_digests.sort_unstable();
        route_digests.dedup();

        let theory_profile = TheoryProfile::compile(
            &region,
            &inputs.exact_properties,
            &inputs.operational_estimates,
        );

        let obligation = ObligationIR::new(
            query.universe_generation(),
            query.world(),
            route_digests.clone(),
            query.requested_result_class().as_str(),
            query.observer(),
            query.authority_contract(),
            region.admitted_capabilities().to_vec(),
            route_digests.clone(),
            query.resource_contract(),
            vec![
                TerminalState::Satisfied,
                TerminalState::Refuted,
                TerminalState::CertifiedBound,
                TerminalState::SemanticUnknown,
                TerminalState::ResourceBoundedUnknown,
                TerminalState::UndecidableGeneralClass,
                TerminalState::Superseded,
                TerminalState::BlockedByAuthority,
            ],
        );

        let mut work_inputs = vec![query.digest(), region.digest(), theory_profile.digest()];
        work_inputs.extend(route_digests.iter().copied());
        let replay_key = ArtifactDigest::of_bytes(
            format!(
                "formula-work-cell-replay-v1|{}|{}",
                inputs.random_key.as_str(),
                obligation.digest().as_str()
            )
            .as_bytes(),
        );
        let work_cell = WorkCellPlan::new(
            obligation.digest(),
            work_inputs,
            query.activated_packages().package_digests().to_vec(),
            region.admitted_capabilities().to_vec(),
            inputs.evidence_requirement,
            query.authority_contract(),
            query.resource_contract(),
            replay_key,
            CheckpointPolicy::AtStopBoundary,
            SideEffectPolicy::deny_all(),
            vec![
                StopCondition::Satisfied,
                StopCondition::Refuted,
                StopCondition::CertifiedBound,
                StopCondition::SemanticUnknown,
                StopCondition::ResourceBoundedUnknown,
                StopCondition::BlockedByAuthority,
            ],
        );

        let mut route_node_ids = route_digests;
        route_node_ids.push(theory_profile.digest());
        route_node_ids.sort_unstable();
        route_node_ids.dedup();

        let mut nodes = vec![
            CampaignNode::new(
                query.digest(),
                CampaignNodeKind::Goal,
                query.universe_generation(),
                query.world(),
                Some(CampaignAggregation::Or),
            ),
            CampaignNode::new(
                obligation.digest(),
                CampaignNodeKind::Obligation,
                query.universe_generation(),
                query.world(),
                None,
            ),
            CampaignNode::new(
                work_cell.digest(),
                CampaignNodeKind::WorkCellPlanRef,
                query.universe_generation(),
                query.world(),
                None,
            ),
        ];
        let mut edges = vec![CampaignEdge::new(
            obligation.digest(),
            work_cell.digest(),
            CampaignEdgeKind::Unlocks,
        )];

        for route_id in route_node_ids {
            nodes.push(CampaignNode::new(
                route_id,
                CampaignNodeKind::Route,
                query.universe_generation(),
                query.world(),
                Some(CampaignAggregation::And),
            ));
            edges.push(CampaignEdge::new(
                query.digest(),
                route_id,
                CampaignEdgeKind::AlternativeTo,
            ));
            edges.push(CampaignEdge::new(
                route_id,
                obligation.digest(),
                CampaignEdgeKind::Requires,
            ));
        }

        let campaign = CampaignIR::new(query.universe_generation(), query.world(), nodes, edges);
        campaign
            .validate()
            .map_err(|_| CompilerError::InvalidCampaign)?;

        let activated_package_set =
            ArtifactDigest::of_bytes(format!("{:?}", query.activated_packages()).as_bytes());
        let replay_manifest = ReplayManifest::new(
            query.universe_generation(),
            query.world(),
            query.digest(),
            activated_package_set,
            region.digest(),
            theory_profile.digest(),
            inputs.compiler_policy_version,
            inputs.scheduler_policy_version,
            query.resource_contract(),
            inputs.random_key,
            campaign.digest(),
        );

        Ok(CompiledCampaign {
            region,
            theory_profile,
            campaign,
            obligations: vec![obligation],
            work_cells: vec![work_cell],
            replay_manifest,
        })
    }

    pub fn compile_reuse(
        query: &QueryIR,
        snapshot: &CompilerAuthoritySnapshot,
        inputs: CompilerInputs,
        request: &ReuseRequest,
    ) -> Result<CompiledReuseCampaign, CompilerError> {
        if request.query_digest() != query.digest()
            || request.universe_generation() != query.universe_generation()
            || request.world() != query.world()
            || request.authority_contract() != query.authority_contract()
            || request.observer() != query.observer()
            || request.result_class() != query.requested_result_class()
        {
            return Err(CompilerError::ReuseRequestMismatch);
        }
        if query.observer() != inputs.expected_observer {
            return Err(CompilerError::ObserverMismatch);
        }
        if query.authority_contract() != inputs.expected_authority {
            return Err(CompilerError::AuthorityMismatch);
        }

        let region = RelevantRegion::from_snapshot(query, snapshot).map_err(map_region_error)?;
        if !region
            .admitted_capabilities()
            .contains(&request.required_semantic_capability())
        {
            return Err(CompilerError::RequiredCapabilityUnavailable);
        }

        let theory_profile = TheoryProfile::compile(
            &region,
            &inputs.exact_properties,
            &inputs.operational_estimates,
        );
        let resolved = ResolvedCapability::new(request);
        let execution = ReuseExecutionPlan::new(query, &resolved);
        let metrics = ReuseMetrics::canonical_single_reuse();

        let obligation = ObligationIR::new(
            query.universe_generation(),
            query.world(),
            vec![resolved.primitive()],
            query.requested_result_class().as_str(),
            query.observer(),
            query.authority_contract(),
            vec![resolved.primitive()],
            vec![],
            query.resource_contract(),
            vec![TerminalState::Satisfied, TerminalState::BlockedByAuthority],
        );

        let nodes = vec![
            CampaignNode::new(
                query.digest(),
                CampaignNodeKind::Goal,
                query.universe_generation(),
                query.world(),
                Some(CampaignAggregation::Or),
            ),
            CampaignNode::new(
                resolved.primitive(),
                CampaignNodeKind::ArtifactRef,
                query.universe_generation(),
                query.world(),
                None,
            ),
            CampaignNode::new(
                obligation.digest(),
                CampaignNodeKind::Obligation,
                query.universe_generation(),
                query.world(),
                None,
            ),
            CampaignNode::new(
                execution.digest(),
                CampaignNodeKind::ExecutionPlanRef,
                query.universe_generation(),
                query.world(),
                None,
            ),
        ];
        let edges = vec![
            CampaignEdge::new(
                query.digest(),
                resolved.primitive(),
                CampaignEdgeKind::Requires,
            ),
            CampaignEdge::new(
                resolved.primitive(),
                obligation.digest(),
                CampaignEdgeKind::Unlocks,
            ),
            CampaignEdge::new(
                obligation.digest(),
                execution.digest(),
                CampaignEdgeKind::Unlocks,
            ),
        ];
        let campaign = CampaignIR::new(query.universe_generation(), query.world(), nodes, edges);
        campaign
            .validate()
            .map_err(|_| CompilerError::InvalidCampaign)?;

        Ok(CompiledReuseCampaign {
            campaign,
            resolved_capability: resolved,
            execution_plans: vec![execution],
            metrics,
        })
    }
}

fn map_region_error(error: RegionError) -> CompilerError {
    match error {
        RegionError::GenerationMismatch => CompilerError::GenerationMismatch,
        RegionError::WorldMismatch => CompilerError::WorldMismatch,
        RegionError::PackageContextMismatch => CompilerError::PackageContextMismatch,
    }
}
