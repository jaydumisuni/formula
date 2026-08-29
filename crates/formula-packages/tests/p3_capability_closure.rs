use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    theory::{CapabilityContract, ClosureContext, StructureWitness, TheoryPackageManifest},
};
use formula_packages::closure::derive_capabilities;

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn package(goal: ArtifactDigest, capability: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "test.package".into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        vec![],
    )
}

#[test]
fn certified_witness_unlocks_only_the_matching_world_and_active_package() {
    let goal = d("goal:ring");
    let capability = d("cap:divide");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let context = ClosureContext::new(
        d("generation-1"),
        d("world-1"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence"));

    let without = derive_capabilities(&context, &[], std::slice::from_ref(&package));
    let with = derive_capabilities(&context, &[witness], std::slice::from_ref(&package));

    assert!(!without.contains(capability));
    assert!(with.contains(capability));

    let wrong_world = StructureWitness::new(d("world-2"), goal, d("evidence"));
    let leaked = derive_capabilities(&context, &[wrong_world], &[package]);
    assert!(!leaked.contains(capability));
}

#[test]
fn closure_is_deterministic_and_context_identity_is_generation_scoped() {
    let goal = d("goal:ring");
    let capability = d("cap:divide");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(d("world-1"), goal, d("evidence"));
    let context_a = ClosureContext::new(
        d("generation-1"), d("world-1"), vec![package_digest], d("rules"), d("policy"),
    );
    let context_b = ClosureContext::new(
        d("generation-2"), d("world-1"), vec![package_digest], d("rules"), d("policy"),
    );

    let a1 = derive_capabilities(&context_a, std::slice::from_ref(&witness), std::slice::from_ref(&package));
    let a2 = derive_capabilities(&context_a, std::slice::from_ref(&witness), std::slice::from_ref(&package));
    let b = derive_capabilities(&context_b, &[witness], &[package]);

    assert_eq!(a1, a2);
    assert_ne!(a1.context_digest(), b.context_digest());
    assert!(a1.contains(capability));
    assert!(b.contains(capability));
}
