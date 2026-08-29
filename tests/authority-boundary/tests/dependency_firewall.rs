use std::{path::PathBuf, process::Command};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cargo_tree(package: &str) -> String {
    let out = Command::new("cargo")
        .args([
            "tree", "--prefix", "none", "--edges", "normal", "-p", package,
        ])
        .current_dir(root())
        .output()
        .expect("cargo tree must execute");
    assert!(
        out.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("cargo tree output must be UTF-8")
}

#[test]
fn checker_depends_on_core_but_not_search_or_realization() {
    let tree = cargo_tree("formula-check");
    assert!(
        tree.contains("formula-core"),
        "checker must share immutable schema through formula-core"
    );
    for forbidden in ["formula-engine", "formula-realize", "formula-first-light"] {
        assert!(
            !tree.contains(forbidden),
            "checker must not depend on {forbidden}"
        );
    }
}

#[test]
fn engine_cannot_link_checker_implementation() {
    let tree = cargo_tree("formula-engine");
    assert!(
        !tree.contains("formula-check"),
        "engine may submit frozen artifacts to an independent checker process but may not link checker implementation"
    );
}
