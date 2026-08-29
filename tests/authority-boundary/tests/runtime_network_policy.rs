use std::{collections::BTreeSet, fs, path::PathBuf, process::Command};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cargo_tree_packages(package: &str) -> BTreeSet<String> {
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
    String::from_utf8(out.stdout)
        .expect("cargo tree output must be UTF-8")
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

fn allowlist() -> BTreeSet<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("runtime-allowlist.txt");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("missing runtime allowlist {}: {e}", path.display()));
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

#[test]
fn canonical_first_light_runtime_closure_is_explicitly_allowlisted() {
    let allowed = allowlist();
    for root_package in ["formula-first-light", "formula-cli"] {
        let packages = cargo_tree_packages(root_package);
        for package in packages {
            assert!(
                allowed.contains(&package),
                "{root_package} runtime closure contains unapproved package {package}"
            );
        }
    }
}
