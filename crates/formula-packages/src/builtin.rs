use formula_core::{
    artifacts::StructuralIdentity,
    digest::ArtifactDigest,
    theory::{CapabilityContract, TheoryPackageManifest},
};

fn id(label: &str) -> ArtifactDigest {
    ArtifactDigest::of_bytes(label.as_bytes())
}

fn cap(label: &str, goals: &[&str]) -> CapabilityContract {
    CapabilityContract::new(id(label), goals.iter().map(|goal| id(goal)).collect())
}

pub fn integer_package(foundation: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "formula.integer.v1".into(),
        foundation,
        vec![id("sem:integer"), id("sem:integer:ring")],
        vec![
            cap("cap:integer:exact-arithmetic", &["goal:integer"]),
            cap("cap:integer:ring", &["goal:integer:ring"]),
        ],
        vec![],
        vec!["formula.integer".into()],
    )
}

pub fn rational_package(foundation: ArtifactDigest) -> TheoryPackageManifest {
    let integer = integer_package(foundation);
    TheoryPackageManifest::new(
        "formula.rational.v1".into(),
        foundation,
        vec![id("sem:rational"), id("sem:rational:field")],
        vec![
            cap("cap:rational:exact-arithmetic", &["goal:rational"]),
            cap("cap:rational:field", &["goal:rational:field"]),
        ],
        vec![integer.structural_digest()],
        vec!["formula.rational".into()],
    )
}

pub fn boolean_package(foundation: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "formula.boolean.v1".into(),
        foundation,
        vec![id("sem:boolean"), id("sem:boolean:xor")],
        vec![cap("cap:boolean:exact", &["goal:boolean"])],
        vec![],
        vec!["formula.boolean".into()],
    )
}

pub fn u8_package(foundation: ArtifactDigest) -> TheoryPackageManifest {
    TheoryPackageManifest::new(
        "formula.u8.v1".into(),
        foundation,
        vec![id("sem:u8:wrapping"), id("sem:u8:bitwise")],
        vec![
            cap("cap:u8:wrapping-arithmetic", &["goal:u8:wrapping"]),
            cap("cap:u8:bitwise", &["goal:u8:bitwise"]),
        ],
        vec![],
        vec!["formula.u8".into()],
    )
}

pub fn gf2_package(foundation: ArtifactDigest) -> TheoryPackageManifest {
    let boolean = boolean_package(foundation);
    TheoryPackageManifest::new(
        "formula.gf2.v1".into(),
        foundation,
        vec![id("sem:gf2"), id("sem:gf2:field")],
        vec![cap("cap:gf2:field", &["goal:gf2:field"])],
        vec![boolean.structural_digest()],
        vec!["formula.gf2".into()],
    )
}

pub fn polynomial_integer_package(
    foundation: ArtifactDigest,
    variables: u32,
) -> TheoryPackageManifest {
    let integer = integer_package(foundation);
    TheoryPackageManifest::new(
        format!("formula.polynomial.integer.v1.n{variables}"),
        foundation,
        vec![id(&format!("sem:polynomial:integer:n{variables}"))],
        vec![cap(
            &format!("cap:polynomial:integer:n{variables}:exact"),
            &["goal:integer:ring"],
        )],
        vec![integer.structural_digest()],
        vec![format!("formula.polynomial.integer.n{variables}")],
    )
}

pub fn gf2_vector_package(foundation: ArtifactDigest, length: u32) -> TheoryPackageManifest {
    let gf2 = gf2_package(foundation);
    TheoryPackageManifest::new(
        format!("formula.gf2.vector.v1.n{length}"),
        foundation,
        vec![id(&format!("sem:gf2:vector:n{length}"))],
        vec![cap(
            &format!("cap:gf2:vector:n{length}:exact"),
            &["goal:gf2:field"],
        )],
        vec![gf2.structural_digest()],
        vec![format!("formula.gf2.vector.n{length}")],
    )
}

pub fn gf2_matrix_package(
    foundation: ArtifactDigest,
    rows: u32,
    columns: u32,
) -> TheoryPackageManifest {
    let gf2 = gf2_package(foundation);
    TheoryPackageManifest::new(
        format!("formula.gf2.matrix.v1.r{rows}c{columns}"),
        foundation,
        vec![id(&format!("sem:gf2:matrix:r{rows}c{columns}"))],
        vec![cap(
            &format!("cap:gf2:matrix:r{rows}c{columns}:exact"),
            &["goal:gf2:field"],
        )],
        vec![gf2.structural_digest()],
        vec![format!("formula.gf2.matrix.r{rows}c{columns}")],
    )
}
