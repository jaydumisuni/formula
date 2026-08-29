use formula_core::{digest::ArtifactDigest, theory::CanonicalMorphism};
use formula_packages::morphisms::{
    resolve_common_parent, CommonParentResolution, MorphismRegistry,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn unique_lossless_canonical_common_parent_resolves() {
    let parent = d("parent");
    let registry = MorphismRegistry::new(vec![
        CanonicalMorphism::new(d("left"), parent, d("left->parent"), vec![], true, true),
        CanonicalMorphism::new(d("right"), parent, d("right->parent"), vec![], true, true),
    ]);

    assert_eq!(
        resolve_common_parent(&registry, d("left"), d("right")),
        CommonParentResolution::ProvenUnique { parent }
    );
}

#[test]
fn multiple_common_parents_are_ambiguous_and_noncanonical_paths_are_ignored() {
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

    let unsafe_registry = MorphismRegistry::new(vec![
        CanonicalMorphism::new(d("left"), d("parent"), d("lossy"), vec![], true, false),
        CanonicalMorphism::new(d("right"), d("parent"), d("noncanonical"), vec![], false, true),
    ]);
    assert_eq!(
        resolve_common_parent(&unsafe_registry, d("left"), d("right")),
        CommonParentResolution::Unknown
    );
}

#[test]
fn missing_common_parent_is_unknown() {
    let registry = MorphismRegistry::new(vec![]);
    assert_eq!(
        resolve_common_parent(&registry, d("left"), d("right")),
        CommonParentResolution::Unknown
    );
}
