use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    theory::{CompositionClaim, CompositionClass, TheoryPackageManifest},
};
use formula_packages::activation::{validate_activation, ActivationError};

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
        validate_activation(&[a.clone(), b.clone()], &[unsupported], &requested),
        Err(ActivationError::CompositionNotAdmissible)
    );

    let certified = CompositionClaim::new(
        a.structural_digest(),
        b.structural_digest(),
        CompositionClass::CertifiedCombination,
        d("evidence"),
    );
    let activated = validate_activation(&[a, b], &[certified], &requested).unwrap();
    assert_eq!(activated.digests().len(), 2);
}

#[test]
fn non_exact_composition_classes_do_not_enable_exact_activation() {
    let a = pkg("a", "shared:+");
    let b = pkg("b", "shared:+");
    let requested = vec![a.structural_digest(), b.structural_digest()];

    for class in [
        CompositionClass::SoundCooperation,
        CompositionClass::HeuristicOnly,
        CompositionClass::Unsupported,
        CompositionClass::Quarantined,
    ] {
        let claim = CompositionClaim::new(
            a.structural_digest(), b.structural_digest(), class, d(class.as_str()),
        );
        assert_eq!(
            validate_activation(&[a.clone(), b.clone()], &[claim], &requested),
            Err(ActivationError::CompositionNotAdmissible)
        );
    }
}
