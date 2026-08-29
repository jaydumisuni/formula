use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{
        CanonicalMorphism, CapabilityContract, ClosureContext, CompositionClaim, CompositionClass,
        StructureWitness, TheoryPackageManifest,
    },
};
use formula_packages::{
    activation::{ActivationError, validate_activation},
    closure::{AdmittedStructureWitness, ClosureError, derive_capabilities},
    morphisms::{MorphismRegistry, MorphismRegistryError},
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn package(goal: ArtifactDigest, capability: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "authority.test".into(),
        d("foundation"),
        vec![],
        vec![CapabilityContract::new(capability, vec![goal])],
        vec![],
        vec!["shared:+".into()],
    )
}

#[test]
fn activation_requires_packages_and_composition_claims_admitted_by_generation() {
    let a = package(d("goal-a"), d("cap-a"));
    let b = TheoryPackageManifest::new(
        "authority.test.b".into(),
        d("foundation"),
        vec![],
        vec![],
        vec![],
        vec!["shared:+".into()],
    );
    let requested = vec![a.structural_digest(), b.structural_digest()];
    let claim = CompositionClaim::new(
        a.structural_digest(),
        b.structural_digest(),
        CompositionClass::CertifiedCombination,
        d("composition-evidence"),
    );

    let no_packages = UniverseGeneration::new(1, None, vec![], vec![]);
    assert_eq!(
        validate_activation(
            &no_packages,
            &[a.clone(), b.clone()],
            &[claim.clone()],
            &requested
        ),
        Err(ActivationError::PackageNotAdmitted)
    );

    let no_claim = UniverseGeneration::new(
        1,
        None,
        vec![a.structural_digest(), b.structural_digest()],
        vec![claim.evidence()],
    );
    assert_eq!(
        validate_activation(
            &no_claim,
            &[a.clone(), b.clone()],
            &[claim.clone()],
            &requested
        ),
        Err(ActivationError::CompositionClaimNotAdmitted)
    );

    let no_evidence_binding = UniverseGeneration::new(
        1,
        None,
        vec![
            a.structural_digest(),
            b.structural_digest(),
            claim.structural_digest(),
        ],
        vec![],
    );
    assert_eq!(
        validate_activation(
            &no_evidence_binding,
            &[a.clone(), b.clone()],
            &[claim.clone()],
            &requested,
        ),
        Err(ActivationError::CompositionEvidenceNotAuthorityBound)
    );

    let generation = UniverseGeneration::new(
        1,
        None,
        vec![
            a.structural_digest(),
            b.structural_digest(),
            claim.structural_digest(),
        ],
        vec![claim.evidence()],
    );
    let activated =
        validate_activation(&generation, &[a, b], &[claim.clone()], &requested).unwrap();
    assert_eq!(activated.generation(), generation.digest());
    assert_eq!(activated.composition_claims(), &[claim.structural_digest()]);
}

#[test]
fn morphism_registry_accepts_only_generation_admitted_morphisms() {
    let morphism = CanonicalMorphism::new(d("source"), d("target"), d("map"), vec![], true, true);
    let empty = UniverseGeneration::new(1, None, vec![], vec![]);
    assert_eq!(
        MorphismRegistry::new(&empty, vec![morphism.clone()]),
        Err(MorphismRegistryError::MorphismNotAdmitted)
    );

    let admitted = UniverseGeneration::new(1, None, vec![morphism.structural_digest()], vec![]);
    assert!(MorphismRegistry::new(&admitted, vec![morphism]).is_ok());
}

#[test]
fn closure_rejects_an_activated_package_set_from_another_generation() {
    let goal = d("goal");
    let capability = d("capability");
    let package = package(goal, capability);
    let package_digest = package.structural_digest();
    let witness = StructureWitness::new(d("world"), goal, d("witness-evidence"));
    let generation_1 = UniverseGeneration::new(
        1,
        None,
        vec![package_digest, witness.structural_digest()],
        vec![witness.evidence()],
    );
    let generation_2 =
        UniverseGeneration::new(2, Some(generation_1.digest()), vec![package_digest], vec![]);
    let activated_1 = validate_activation(
        &generation_1,
        std::slice::from_ref(&package),
        &[],
        &[package_digest],
    )
    .unwrap();
    let admitted_witness = AdmittedStructureWitness::new(&generation_1, witness).unwrap();
    let context_2 = ClosureContext::new(
        generation_2.digest(),
        d("world"),
        vec![package_digest],
        d("rules"),
        d("policy"),
    );

    assert_eq!(
        derive_capabilities(&context_2, &activated_1, &[admitted_witness], &[package],),
        Err(ClosureError::GenerationMismatch)
    );
}
