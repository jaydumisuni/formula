use formula_core::digest::ArtifactDigest;
use formula_engine::{
    compiler::{
        CompilerError, CompilerInputs, CompilerV1, ImplicitMorphism, ParentResolution,
        RepresentationRoute,
    },
    decomposition::{AggregationSemantics, ChildObligation, Decomposition},
    query::{
        ActivatedPackageBinding, KnownBinding, Metavariable, QueryIR, RequestedResultClass,
        ResourceContract, SideEffectPolicy, TargetRequest,
    },
    reduction::ReductionEdge,
    region::CompilerAuthoritySnapshot,
    representation::{
        ExactnessClass, InformationLoss, PreservationMetadata, RepresentationEdge,
        RepresentationNode,
    },
    theory_profile::{OperationalEstimate, ProfileFact},
};

fn d(byte: u8) -> ArtifactDigest {
    ArtifactDigest::of_bytes(&[byte; 32])
}

fn packages() -> ActivatedPackageBinding {
    ActivatedPackageBinding::new(d(1), vec![d(30), d(31)], vec![d(32)])
}

fn query() -> QueryIR {
    QueryIR::new(
        d(1),
        d(2),
        vec![KnownBinding::new("a", d(3))],
        vec![Metavariable::new("x", "value")],
        vec![TargetRequest::new(d(4), RequestedResultClass::Witness)],
        d(5),
        d(6),
        ResourceContract::new(100, 1024, 50),
        SideEffectPolicy::deny_all(),
        packages(),
    )
}

fn snapshot() -> CompilerAuthoritySnapshot {
    CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        packages(),
        vec![d(3), d(4)],
        vec![d(40)],
        vec![d(41)],
    )
}

fn inputs() -> CompilerInputs {
    CompilerInputs::new(d(5), d(6), d(50), d(51))
        .with_exact_properties(vec![ProfileFact::new("finite", d(52))])
        .with_operational_estimates(vec![OperationalEstimate::new("route-cost", 10)])
        .with_parent_resolution(ParentResolution::Unique(d(53)))
        .with_compiler_policy_version("compiler-v1")
        .with_scheduler_policy_version("scheduler-v1")
}

#[test]
fn identical_exact_inputs_compile_to_identical_campaign_and_replay() {
    let left = CompilerV1::compile(&query(), &snapshot(), inputs()).unwrap();
    let right = CompilerV1::compile(&query(), &snapshot(), inputs()).unwrap();

    assert_eq!(left.campaign.digest(), right.campaign.digest());
    assert_eq!(left.replay_manifest.digest(), right.replay_manifest.digest());
    assert_eq!(left.obligations, right.obligations);
    assert_eq!(left.work_cells, right.work_cells);
}

#[test]
fn exact_context_mismatches_fail_closed() {
    let q = query();
    let bad_generation = CompilerAuthoritySnapshot::new(
        d(99), d(2), packages(), vec![], vec![], vec![],
    );
    assert_eq!(
        CompilerV1::compile(&q, &bad_generation, inputs()).unwrap_err(),
        CompilerError::GenerationMismatch
    );

    let bad_world = CompilerAuthoritySnapshot::new(
        d(1), d(99), packages(), vec![], vec![], vec![],
    );
    assert_eq!(
        CompilerV1::compile(&q, &bad_world, inputs()).unwrap_err(),
        CompilerError::WorldMismatch
    );

    let bad_packages = CompilerAuthoritySnapshot::new(
        d(1),
        d(2),
        ActivatedPackageBinding::new(d(1), vec![d(88)], vec![]),
        vec![], vec![], vec![],
    );
    assert_eq!(
        CompilerV1::compile(&q, &bad_packages, inputs()).unwrap_err(),
        CompilerError::PackageContextMismatch
    );

    assert_eq!(
        CompilerV1::compile(&q, &snapshot(), inputs().with_expected_observer(d(98))).unwrap_err(),
        CompilerError::ObserverMismatch
    );
    assert_eq!(
        CompilerV1::compile(&q, &snapshot(), inputs().with_expected_authority(d(97))).unwrap_err(),
        CompilerError::AuthorityMismatch
    );
}

#[test]
fn lossy_implicit_morphism_and_ambiguous_parent_fail_closed() {
    let lossy = inputs().with_implicit_morphisms(vec![ImplicitMorphism::new(
        d(60),
        InformationLoss::Declared,
        false,
    )]);
    assert_eq!(
        CompilerV1::compile(&query(), &snapshot(), lossy).unwrap_err(),
        CompilerError::ImplicitLossyMorphism
    );

    let ambiguous = inputs().with_parent_resolution(ParentResolution::Ambiguous(vec![d(61), d(62)]));
    assert_eq!(
        CompilerV1::compile(&query(), &snapshot(), ambiguous).unwrap_err(),
        CompilerError::AmbiguousParent
    );
}

#[test]
fn invalid_representation_reduction_and_decomposition_fail_closed() {
    let source = RepresentationNode::new(d(4), d(70), d(2), ExactnessClass::Exact, d(5));
    let target = RepresentationNode::new(d(4), d(71), d(2), ExactnessClass::Exact, d(5));
    let invalid_representation = RepresentationRoute::new(
        source.clone(),
        target.clone(),
        RepresentationEdge::new(
            source.digest(),
            target.digest(),
            d(72),
            None,
            InformationLoss::None,
            None,
            Some(d(73)),
            vec![],
        ),
    );
    assert_eq!(
        CompilerV1::compile(
            &query(),
            &snapshot(),
            inputs().with_representation_routes(vec![invalid_representation]),
        )
        .unwrap_err(),
        CompilerError::InvalidRepresentation
    );

    let decision_only = ReductionEdge::new(
        "source",
        "target",
        vec![RequestedResultClass::Decision],
        d(74),
        None,
        vec![],
        Some(d(75)),
    );
    assert_eq!(
        CompilerV1::compile(
            &query(),
            &snapshot(),
            inputs().with_reduction_paths(vec![vec![decision_only]]),
        )
        .unwrap_err(),
        CompilerError::ReductionResultClassLoss
    );

    let invalid_decomposition = Decomposition::new(
        d(4),
        d(2),
        vec![ChildObligation::new(d(76), d(2))],
        None,
        Some(AggregationSemantics::And),
        None,
        Some(d(77)),
    );
    assert_eq!(
        CompilerV1::compile(
            &query(),
            &snapshot(),
            inputs().with_decompositions(vec![invalid_decomposition]),
        )
        .unwrap_err(),
        CompilerError::InvalidDecomposition
    );
}

#[test]
fn valid_explicit_routes_compile_without_search() {
    let source = RepresentationNode::new(d(4), d(80), d(2), ExactnessClass::Exact, d(5));
    let target = RepresentationNode::new(d(4), d(81), d(2), ExactnessClass::Exact, d(5));
    let representation = RepresentationRoute::new(
        source.clone(),
        target.clone(),
        RepresentationEdge::new(
            source.digest(),
            target.digest(),
            d(82),
            Some(PreservationMetadata::new(
                ExactnessClass::Exact,
                vec![RequestedResultClass::Witness],
            )),
            InformationLoss::None,
            Some(d(83)),
            Some(d(84)),
            vec![],
        ),
    );
    let reduction = ReductionEdge::new(
        "source",
        "target",
        vec![RequestedResultClass::Witness],
        d(85),
        Some(d(86)),
        vec![],
        Some(d(87)),
    );
    let decomposition = Decomposition::new(
        d(4),
        d(2),
        vec![ChildObligation::new(d(88), d(2))],
        None,
        Some(AggregationSemantics::And),
        Some(d(89)),
        Some(d(90)),
    );

    let compiled = CompilerV1::compile(
        &query(),
        &snapshot(),
        inputs()
            .with_representation_routes(vec![representation])
            .with_reduction_paths(vec![vec![reduction]])
            .with_decompositions(vec![decomposition]),
    )
    .unwrap();

    assert!(!compiled.obligations.is_empty());
    assert!(!compiled.work_cells.is_empty());
}
