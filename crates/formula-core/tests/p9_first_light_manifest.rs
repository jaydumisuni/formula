use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    first_light::{
        FirstLightManifestError, FirstLightNativeEvidence, FirstLightProofManifest,
        FirstLightReuseEvidence, FirstLightTargetEvidence, NegativeControlEvidence,
        NegativeControlId, NegativeControlManifest,
    },
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn complete_controls(reverse: bool) -> NegativeControlManifest {
    let mut controls: Vec<_> = NegativeControlId::ALL
        .iter()
        .copied()
        .map(|id| NegativeControlEvidence::new(id, d(id.as_str())))
        .collect();
    if reverse {
        controls.reverse();
    }
    NegativeControlManifest::complete(controls).unwrap()
}

fn target(prefix: &str, auxiliary: Vec<ArtifactDigest>) -> FirstLightTargetEvidence {
    FirstLightTargetEvidence::new(
        d(&format!("{prefix}-query")),
        d(&format!("{prefix}-campaign")),
        d(&format!("{prefix}-candidate")),
        d(&format!("{prefix}-certification")),
        auxiliary,
    )
}

fn native() -> FirstLightNativeEvidence {
    FirstLightNativeEvidence::new(
        d("native-source"),
        d("native-toolchain"),
        d("native-binary"),
        d("native-realization"),
    )
}

fn reuse() -> FirstLightReuseEvidence {
    FirstLightReuseEvidence::new(
        d("reuse-query"),
        d("reuse-campaign"),
        d("reuse-capability"),
        d("reuse-execution-plan"),
        d("reuse-result"),
        d("reuse-metrics"),
        d("native-realization"),
    )
}

fn manifest(controls: NegativeControlManifest) -> FirstLightProofManifest {
    FirstLightProofManifest::new(
        "source-commit-123".into(),
        d("u0"),
        d("u1"),
        d("world"),
        d("activated-packages"),
        target("fl-a", vec![d("a2"), d("a1")]),
        target("fl-b", vec![d("b1")]),
        target("fl-c", vec![d("c1")]),
        d("promotion"),
        d("closure-before"),
        d("closure-after"),
        d("closure-delta"),
        native(),
        reuse(),
        controls,
        d("verifier-v1"),
        d("checker-v1"),
    )
}

#[test]
fn negative_control_manifest_requires_exact_nc01_through_nc12_once_each() {
    let left = complete_controls(false);
    let right = complete_controls(true);
    assert_eq!(left.structural_digest(), right.structural_digest());
    assert_eq!(left.controls().len(), 12);
    assert!(left.is_complete());

    let mut missing: Vec<_> = NegativeControlId::ALL[..11]
        .iter()
        .copied()
        .map(|id| NegativeControlEvidence::new(id, d(id.as_str())))
        .collect();
    assert_eq!(
        NegativeControlManifest::complete(missing.clone()).unwrap_err(),
        FirstLightManifestError::MissingNegativeControl(NegativeControlId::PromotionParentRace)
    );

    missing.push(NegativeControlEvidence::new(
        NegativeControlId::ModifiedSealedTarget,
        d("duplicate"),
    ));
    assert_eq!(
        NegativeControlManifest::complete(missing).unwrap_err(),
        FirstLightManifestError::DuplicateNegativeControl(NegativeControlId::ModifiedSealedTarget)
    );
}

#[test]
fn target_and_negative_control_set_like_inputs_have_deterministic_identity() {
    let a = target("fl-a", vec![d("a2"), d("a1"), d("a1")]);
    let b = target("fl-a", vec![d("a1"), d("a2")]);
    assert_eq!(a.structural_digest(), b.structural_digest());
    assert_eq!(a.auxiliary(), &[d("a1"), d("a2")]);
}

#[test]
fn every_authority_bearing_section_perturbs_first_light_manifest_identity() {
    let base = manifest(complete_controls(false));
    let changed = FirstLightProofManifest::new(
        base.source_commit().into(),
        base.u0_digest(),
        base.u1_digest(),
        base.world(),
        base.activated_package_set(),
        base.fl_a().clone(),
        base.fl_b().clone(),
        base.fl_c().clone(),
        base.promotion_digest(),
        base.closure_before(),
        d("different-closure-after"),
        base.closure_delta(),
        base.native().clone(),
        base.reuse().clone(),
        complete_controls(false),
        base.verifier_identity(),
        base.checker_identity(),
    );
    assert_ne!(base.structural_digest(), changed.structural_digest());

    let changed_control = NegativeControlManifest::complete(
        NegativeControlId::ALL
            .iter()
            .copied()
            .map(|id| {
                let evidence = if id == NegativeControlId::ForgedEvidence {
                    d("changed-forged-evidence-control")
                } else {
                    d(id.as_str())
                };
                NegativeControlEvidence::new(id, evidence)
            })
            .collect(),
    )
    .unwrap();
    assert_ne!(
        base.structural_digest(),
        manifest(changed_control).structural_digest()
    );
}

#[test]
fn first_light_manifest_contains_no_runtime_metadata_surface() {
    let value = manifest(complete_controls(false)).canonical_value();
    let rendered = String::from_utf8(value.to_canonical_bytes()).unwrap();
    for forbidden in ["timestamp", "runner", "workspace", "hostname", "duration"] {
        assert!(!rendered.contains(forbidden));
    }
}
