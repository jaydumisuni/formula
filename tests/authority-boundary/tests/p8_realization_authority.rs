use std::{
    fs,
    path::{Path, PathBuf},
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn visit_rs_files(path: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
        .map(|entry| entry.expect("directory entry must be readable").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            visit_rs_files(&entry, out);
        } else if entry.extension().and_then(|value| value.to_str()) == Some("rs") {
            out.push(entry);
        }
    }
}

#[test]
fn realization_generator_cannot_reach_checker_or_store_authority() {
    let root = root();
    let manifest = fs::read_to_string(root.join("crates/formula-realize/Cargo.toml")).unwrap();
    for forbidden in ["formula-check", "formula-store"] {
        assert!(
            !manifest.contains(forbidden),
            "formula-realize production manifest crosses P8 authority boundary via {forbidden}"
        );
    }

    let mut files = Vec::new();
    visit_rs_files(&root.join("crates/formula-realize/src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        for forbidden in [
            "formula_check",
            "formula_store",
            "RealizationAuthorization",
            "AuthorityStore",
            "admit_realization",
        ] {
            assert!(
                !text.contains(forbidden),
                "realization source {} crosses P8 authority boundary via {forbidden}",
                file.display()
            );
        }
    }
}

#[test]
fn realization_authorization_has_no_public_constructor_or_mutable_fields() {
    let source =
        fs::read_to_string(root().join("crates/formula-check/src/realization.rs")).unwrap();
    let start = source
        .find("pub struct RealizationAuthorization")
        .expect("RealizationAuthorization definition exists");
    let rest = &source[start..];
    let end = rest
        .find("\n}\n\nimpl RealizationAuthorization")
        .expect("RealizationAuthorization struct closes before impl");
    let definition = &rest[..end];

    for field in [
        "realization_manifest",
        "semantic_target",
        "universe_generation",
        "world",
        "authority_contract",
        "observer",
        "specialization_digest",
        "source_digest",
        "toolchain_digest",
        "binary_digest",
    ] {
        assert!(
            !definition.contains(&format!("pub {field}:")),
            "RealizationAuthorization exposes mutable authority-bearing field {field}"
        );
    }

    let impl_start = rest.find("impl RealizationAuthorization").unwrap();
    let authorize_start = rest
        .find("pub fn authorize_native_u8_realization_v1")
        .expect("checker authorization function exists");
    let authorization_impl = &rest[impl_start..authorize_start];
    assert!(
        !authorization_impl.contains("pub fn new("),
        "RealizationAuthorization must not expose a public constructor"
    );
}

#[test]
fn realization_store_consumes_only_checker_issued_authorization() {
    let source = fs::read_to_string(
        root().join("crates/formula-store/src/authority_store/realization_store.rs"),
    )
    .unwrap();

    assert!(source.contains("use formula_check::realization::RealizationAuthorization;"));
    assert!(source.contains("pub fn admit_realization("));
    assert!(source.contains("authorization: &RealizationAuthorization"));
    for forbidden in [
        "NativeRealizationManifest",
        "RealizationCheckManifest",
        "formula_check::u8::BoolExpr",
    ] {
        assert!(
            !source.contains(forbidden),
            "realization store is deciding semantic correctness via {forbidden}"
        );
    }
}
