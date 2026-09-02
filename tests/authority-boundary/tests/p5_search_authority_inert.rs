use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn p5_search_sources_cannot_cross_checker_sealed_or_authority_boundary() {
    let source_root = root().join("crates/formula-engine/src");
    let p5_sources = [
        "candidate_space.rs",
        "affine_polynomial.rs",
        "route_space.rs",
        "observational.rs",
        "discovery.rs",
        "search_policy.rs",
    ];
    let forbidden = [
        "formula_check",
        "formula-check",
        "formula_first_light",
        "formula-first-light",
        "tests/first-light/sealed",
        "P0_SEALED_SENTINEL",
        "publish_generation",
        "rollback_generation",
        "update_authority",
        "authority_transaction",
        "admit_promotion_candidate",
        "PromotionCandidate",
    ];

    for source in p5_sources {
        let path = source_root.join(source);
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
        for token in forbidden {
            assert!(
                !text.contains(token),
                "P5 search source {} crosses forbidden boundary via token {token}",
                path.display()
            );
        }
    }
}
