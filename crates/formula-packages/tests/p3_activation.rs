use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    generation::UniverseGeneration,
    theory::{CompositionClaim, CompositionClass, TheoryPackageManifest},
};
use formula_packages::activation::{ActivationError, validate_activation};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn pkg(name: &str, surface: &str) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        name.into(),
        d("foundation"),
        vec![],
        vec![],
        vec![],
        vec![surface.into()],
    )
}

#[test]
fn overlapping_packages_require_an_admissible_exact_composition_claim() {
    let a = pkg("a", "shared:+");
    let b = pkg("b", "shared:+");
    let requested = vec![a.structural_digest(), b.structural_digest()];
    let packages_only = UniverseGeneration::new(1, None, requested.clone(), vec![]);

    assert_eq!(
        validate_activation(&packages_only, &[a.clone(), b.clone()], &[], &requested),
        Err(ActivationError::InterferenceUnproven)
    );

    let unsupported = CompositionClaim::new(
        a.structural_digest(),
        b.structural_digest(),
        CompositionClass::Unsupported,
        d("unsupported-evidence"),
    );
    let unsupported_generation = UniverseGeneration::new(
        1,
        None,
        vec![
            a.structural_digest(),
            b.structural_digest(),
            unsupported.structural_digest(),
        ],
        vec![unsupported.evidence()],
    );
    assert_eq!(
        validate_activation(
            &unsupported_generation,
            &[a.clone(), b.clone()],
            std::slice::from_ref(&unsupported),
            &requested,
        ),
        Err(ActivationError::CompositionNotAdmissible)
    );

    let certified = CompositionClaim::new(
        a.structural_digest(),
        b.structural_digest(),
        CompositionClass::CertifiedCombination,
        d("certified-evidence"),
    );
    let certified_generation = UniverseGeneration::new(
        1,
        None,
        vec![
            a.structural_digest(),
            b.structural_digest(),
            certified.structural_digest(),
        ],
        vec![certified.evidence()],
    );
    let activated = validate_activation(
        &certified_generation,
        &[a, b],
        std::slice::from_ref(&certified),
        &requested,
    )
    .unwrap();
    assert_eq!(activated.generation(), certified_generation.digest());
    assert_eq!(activated.digests().len(), 2);
    assert_eq!(activated.composition_claims(), &[certified.structural_digest()]);
}

#[test]
fn non_exact_composition_classes_do_not_enable_exact_activation() {
    for class in [
        CompositionClass::SoundCooperation,
        CompositionClass::HeuristicOnly,
        CompositionClass::Unsupported,
        CompositionClass::Quarantined,
    ] {
        let a = pkg("a", "shared:+");
        let b = pkg("b", "shared:+");
        let requested = vec![a.structural_digest(), b.structural_digest()];
        let claim = CompositionClaim::new(
            a.structural_digest(),
            b.structural_digest(),
            class,
            d(class.as_str()),
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
        assert_eq!(
            validate_activation(&generation, &[a, b], &[claim], &requested),
            Err(ActivationError::CompositionNotAdmissible)
        );
    }
}
