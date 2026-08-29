use formula_core::{artifacts::StructuralIdentity, digest::ArtifactDigest};
use formula_packages::builtin::{
    boolean_package, gf2_matrix_package, gf2_package, gf2_vector_package, integer_package,
    polynomial_integer_package, rational_package, u8_package,
};

fn d(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

#[test]
fn builtin_package_manifests_are_deterministic_and_dependency_bound() {
    let foundation = d("foundation");
    let integer = integer_package(foundation);
    let rational = rational_package(foundation);
    let boolean = boolean_package(foundation);
    let u8_pkg = u8_package(foundation);
    let gf2 = gf2_package(foundation);
    let poly = polynomial_integer_package(foundation, 3);
    let vector = gf2_vector_package(foundation, 8);
    let matrix = gf2_matrix_package(foundation, 4, 8);

    assert_eq!(integer.package_id(), "formula.integer.v1");
    assert_eq!(rational.package_id(), "formula.rational.v1");
    assert_eq!(boolean.package_id(), "formula.boolean.v1");
    assert_eq!(u8_pkg.package_id(), "formula.u8.v1");
    assert_eq!(gf2.package_id(), "formula.gf2.v1");
    assert_eq!(poly.package_id(), "formula.polynomial.integer.v1.n3");
    assert_eq!(vector.package_id(), "formula.gf2.vector.v1.n8");
    assert_eq!(matrix.package_id(), "formula.gf2.matrix.v1.r4c8");

    assert!(integer.dependencies().is_empty());
    assert!(rational.dependencies().contains(&integer.structural_digest()));
    assert!(poly.dependencies().contains(&integer.structural_digest()));
    assert!(vector.dependencies().contains(&gf2.structural_digest()));
    assert!(matrix.dependencies().contains(&gf2.structural_digest()));

    assert!(!integer.semantic_exports().is_empty());
    assert!(!integer.capabilities().is_empty());
    assert_eq!(integer.structural_digest(), integer_package(foundation).structural_digest());
}
