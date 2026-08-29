use formula_core::{
    artifacts::StructuralIdentity,
    canonical::CanonicalValue,
    digest::ArtifactDigest,
    theory::{
        CapabilityContract, CanonicalMorphism, ClosureContext, CompositionClaim, CompositionClass,
        FactPolarity, FederationAdapterManifest, SharedFact, StructureWitness, TheoryPackageManifest,
    },
};
use formula_packages::{
    activation::{validate_activation, ActivationError},
    closure::derive_capabilities,
    federation::{
        validate_federation_adapter, FederationError, FederationMode, FederationRequest,
    },
    morphisms::{resolve_common_parent, CommonParentResolution, MorphismRegistry},
    shared_facts::{fact_satisfies, FactRequirement, FactUseDecision},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn package(
    name: &str,
    goal: ArtifactDigest,
    capability: ArtifactDigest,
    interference_surface: Vec<String>,
) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        name.into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        interference_surface,
    )
}

#[test]
fn world_generation_and_activation_boundaries_do_not_leak_capability() {
    let goal = d("goal");
    let capability = d("capability");
    let package = package("pkg", goal, capability, vec![]);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence"));

    let active = ClosureContext::new(
        d("generation-1"),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let deactivated = ClosureContext::new(
        d("generation-1"),
        d("world-1"),
        vec![],
        d("rules"),
        d("policy"),
    );
    let another_generation = ClosureContext::new(
        d("generation-2"),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let another_world = ClosureContext::new(
        d("generation-1"),
        d("world-2"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    let active_closure = derive_capabilities(
        &active,
        std::slice::from_ref(&witness),
        std::slice::from_ref(&package),
    );
    assert!(active_closure.contains(capability));

    let deactivated_closure = derive_capabilities(
        &deactivated,
        std::slice::from_ref(&witness),
        std::slice::from_ref(&package),
    );
    assert!(!deactivated_closure.contains(capability));

    let wrong_world = derive_capabilities(
        &another_world,
        std::slice::from_ref(&witness),
        std::slice::from_ref(&package),
    );
    assert!(!wrong_world.contains(capability));

    let another_generation_closure =
        derive_capabilities(&another_generation, &[witness], &[package]);
    assert_ne!(
        active_closure.context_digest(),
        another_generation_closure.context_digest()
    );
}

#[test]
fn package_interference_and_common_parent_ambiguity_fail_closed() {
    let a = package("a", d("ga"), d("ca"), vec!["shared".into()]);
    let b = package("b", d("gb"), d("cb"), vec!["shared".into()]);
    let requested = vec![a.structural_digest(), b.structural_digest()];

    assert_eq!(
        validate_activation(&[a.clone(), b.clone()], &[], &requested),
        Err(ActivationError::InterferenceUnproven)
    );
    let unsupported = CompositionClaim::new(
        a.structural_digest(),
        b.structural_digest(),
        CompositionClass::Unsupported,
        d("evidence"),
    );
    assert_eq!(
        validate_activation(&[a, b], &[unsupported], &requested),
        Err(ActivationError::CompositionNotAdmissible)
    );

    let registry = MorphismRegistry::new(vec![
        CanonicalMorphism::new(d("left"), d("p1"), d("l-p1"), vec![], true, true),
        CanonicalMorphism::new(d("right"), d("p1"), d("r-p1"), vec![], true, true),
        CanonicalMorphism::new(d("left"), d("p2"), d("l-p2"), vec![], true, true),
        CanonicalMorphism::new(d("right"), d("p2"), d("r-p2"), vec![], true, true),
    ]);
    assert_eq!(
        resolve_common_parent(&registry, d("left"), d("right")),
        CommonParentResolution::Ambiguous
    );
}

#[test]
fn weak_shared_fact_and_candidate_only_federation_cannot_manufacture_authority() {
    let over = SharedFact::new(
        d("world"),
        d("subject"),
        CanonicalValue::String("possible region".into()),
        FactPolarity::OverApproximation,
        d("evidence"),
    );
    assert_eq!(
        fact_satisfies(&over, FactRequirement::Exact),
        FactUseDecision::Rejected
    );
    assert_eq!(
        fact_satisfies(&over, FactRequirement::ExistenceWitness),
        FactUseDecision::Rejected
    );

    let adapter = FederationAdapterManifest::new(
        "adapter".into(),
        d("package"),
        vec![d("input")],
        vec![d("output")],
        vec![d("translation")],
        vec![d("checker")],
        vec![],
        vec!["EXACT_WITNESS".into()],
        true,
    );
    let request = FederationRequest::new(
        "EXACT_WITNESS".into(),
        true,
        Some(d("checker")),
        Some(d("translation")),
        false,
    );
    assert_eq!(
        validate_federation_adapter(&adapter, FederationMode::CandidateOnly, &request),
        Err(FederationError::CandidateOnlyCannotAuthorize)
    );
}
