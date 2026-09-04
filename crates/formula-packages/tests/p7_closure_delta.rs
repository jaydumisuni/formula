use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{CapabilityContract, ClosureContext, StructureWitness, TheoryPackageManifest},
};
use formula_packages::{
    activation::validate_activation,
    closure::{
        AdmittedStructureWitness, CapabilityClosureDelta, WitnessAdmissionError,
        derive_capabilities,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn package(goal: ArtifactDigest, capability: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "p7.delta.package".into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        vec![],
    )
}

#[test]
fn admitted_authority_bound_witness_derives_added_capability_in_u1() {
    let world = d("world");
    let goal = d("goal:field");
    let capability = d("cap:divide");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(world, goal, d("evidence:field"));

    let u0 = UniverseGeneration::new(0, None, vec![package_digest], vec![]);
    let u1 = UniverseGeneration::new(
        1,
        Some(u0.digest()),
        vec![package_digest, witness.structural_digest()],
        vec![witness.evidence()],
    );
    let active0 =
        validate_activation(&u0, std::slice::from_ref(&package), &[], &[package_digest]).unwrap();
    let active1 =
        validate_activation(&u1, std::slice::from_ref(&package), &[], &[package_digest]).unwrap();
    let admitted = AdmittedStructureWitness::new(&u1, witness).unwrap();
    let context0 = ClosureContext::new(
        u0.digest(),
        world,
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let context1 = ClosureContext::new(
        u1.digest(),
        world,
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    let before =
        derive_capabilities(&context0, &active0, &[], std::slice::from_ref(&package)).unwrap();
    let after = derive_capabilities(
        &context1,
        &active1,
        &[admitted],
        std::slice::from_ref(&package),
    )
    .unwrap();
    let delta = CapabilityClosureDelta::between(&before, &after);

    assert_eq!(delta.before_context_digest(), before.context_digest());
    assert_eq!(delta.after_context_digest(), after.context_digest());
    assert_eq!(delta.added().collect::<Vec<_>>(), vec![capability]);
    assert!(delta.removed().next().is_none());
}

#[test]
fn unadmitted_or_unbound_witness_cannot_manufacture_closure_delta() {
    let world = d("world");
    let goal = d("goal:field");
    let capability = d("cap:divide");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(world, goal, d("evidence:field"));

    let missing_admission =
        UniverseGeneration::new(1, None, vec![package_digest], vec![witness.evidence()]);
    assert_eq!(
        AdmittedStructureWitness::new(&missing_admission, witness.clone()).unwrap_err(),
        WitnessAdmissionError::WitnessNotAdmitted
    );

    let missing_binding = UniverseGeneration::new(
        1,
        None,
        vec![package_digest, witness.structural_digest()],
        vec![],
    );
    assert_eq!(
        AdmittedStructureWitness::new(&missing_binding, witness).unwrap_err(),
        WitnessAdmissionError::EvidenceNotAuthorityBound
    );

    let active = validate_activation(
        &missing_binding,
        std::slice::from_ref(&package),
        &[],
        &[package_digest],
    )
    .unwrap();
    let context = ClosureContext::new(
        missing_binding.digest(),
        world,
        vec![package_digest],
        d("rules"),
        d("policy"),
    );
    let unchanged =
        derive_capabilities(&context, &active, &[], std::slice::from_ref(&package)).unwrap();
    let delta = CapabilityClosureDelta::between(&unchanged, &unchanged);
    assert!(!unchanged.contains(capability));
    assert!(delta.added().next().is_none());
    assert!(delta.removed().next().is_none());
}
