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
fn search_production_cannot_reach_promotion_authority_or_store() {
    let root = root();
    let engine_manifest =
        fs::read_to_string(root.join("crates/formula-engine/Cargo.toml")).unwrap();
    for forbidden in ["formula-store", "formula-check", "formula-first-light"] {
        assert!(
            !engine_manifest.contains(forbidden),
            "formula-engine production manifest crosses P7 authority boundary via {forbidden}"
        );
    }

    let mut files = Vec::new();
    visit_rs_files(&root.join("crates/formula-engine/src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        for forbidden in [
            "formula_store",
            "formula_check",
            "PromotionAuthorization",
            "AuthorityStore",
            "publish_generation",
            ".promote(",
        ] {
            assert!(
                !text.contains(forbidden),
                "search source {} crosses P7 authority boundary via {forbidden}",
                file.display()
            );
        }
    }
}

#[test]
fn first_light_production_dependencies_remain_sealed_harness_only() {
    let manifest =
        fs::read_to_string(root().join("crates/formula-first-light/Cargo.toml")).unwrap();
    let production = manifest
        .split("[dev-dependencies]")
        .next()
        .expect("manifest has production section");

    assert!(production.contains("formula-core"));
    assert!(production.contains("formula-engine"));
    for forbidden in ["formula-store", "formula-check"] {
        assert!(
            !production.contains(forbidden),
            "First-Light production dependency graph crosses promotion authority boundary via {forbidden}"
        );
    }
}

#[test]
fn raw_generation_publication_is_not_a_public_api() {
    let source =
        fs::read_to_string(root().join("crates/formula-store/src/authority_store.rs")).unwrap();
    assert!(
        !source.contains("pub fn publish_generation("),
        "raw generation publication is publicly callable"
    );
    assert!(source.contains("pub(crate) fn publish_generation_inner("));
}

#[test]
fn promotion_store_accepts_only_checker_issued_authorization() {
    let source =
        fs::read_to_string(root().join("crates/formula-store/src/promotion_store.rs")).unwrap();
    assert!(source.contains("use formula_check::promotion::PromotionAuthorization;"));
    assert!(source.contains("pub fn promote("));
    assert!(source.contains("authorization: &PromotionAuthorization"));
    assert!(!source.contains("FrozenCandidate"));
    assert!(!source.contains("PromotionManifest"));
}

#[test]
fn promotion_authorization_has_no_public_constructor_or_mutable_fields() {
    let source = fs::read_to_string(root().join("crates/formula-check/src/promotion.rs")).unwrap();
    let start = source
        .find("pub struct PromotionAuthorization")
        .expect("PromotionAuthorization definition exists");
    let rest = &source[start..];
    let end = rest
        .find("\n}\n\nimpl PromotionAuthorization")
        .expect("PromotionAuthorization struct closes before impl");
    let definition = &rest[..end];

    assert!(!definition.contains("pub parent_generation:"));
    assert!(!definition.contains("pub frozen_candidate:"));
    assert!(!definition.contains("pub promotion_candidate:"));
    assert!(!definition.contains("pub proposed_admissions:"));
    assert!(!definition.contains("pub authority_bindings:"));
    assert!(!definition.contains("pub policy_digest:"));
    assert!(!definition.contains("pub supersedes:"));

    let impl_start = rest.find("impl PromotionAuthorization").unwrap();
    let decision_start = rest.find("pub enum PromotionDecision").unwrap();
    let authorization_impl = &rest[impl_start..decision_start];
    assert!(
        !authorization_impl.contains("pub fn new("),
        "PromotionAuthorization must not expose a public constructor"
    );
}
