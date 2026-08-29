use std::path::Path;

const CRATES: &[&str] = &[
    "formula-core",
    "formula-store",
    "formula-check",
    "formula-engine",
    "formula-packages",
    "formula-realize",
    "formula-first-light",
    "formula-cli",
];

#[test]
fn canonical_p0_crate_boundaries_exist() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for name in CRATES {
        let dir = root.join("crates").join(name);
        assert!(
            dir.join("Cargo.toml").is_file(),
            "missing {name}/Cargo.toml"
        );
        assert!(dir.join("src").is_dir(), "missing {name}/src");
    }
}
