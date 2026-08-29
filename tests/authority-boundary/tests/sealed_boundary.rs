use std::{fs, path::{Path, PathBuf}};

const PUBLIC_TOKEN: &str = "formula-p0-public-fixture-v1";
const SEALED_TOKEN: &str = "formula-p0-sealed-fixture-v1";
const SEALED_PATH: &str = "tests/first-light/sealed";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn visit_files(path: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
        .map(|entry| entry.expect("directory entry must be readable").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            visit_files(&entry, out);
        } else if entry.is_file() {
            out.push(entry);
        }
    }
}

#[test]
fn sealed_first_light_fixture_is_owned_outside_discovery_crates() {
    let root = root();
    let public = root.join("tests/first-light/public/P0_PUBLIC_SENTINEL.txt");
    let sealed = root.join("tests/first-light/sealed/P0_SEALED_SENTINEL.txt");

    assert!(public.is_file(), "missing public First-Light P0 fixture");
    assert!(sealed.is_file(), "missing sealed First-Light P0 fixture");
    assert_eq!(fs::read_to_string(&public).unwrap(), format!("{PUBLIC_TOKEN}\n"));
    assert_eq!(fs::read_to_string(&sealed).unwrap(), format!("{SEALED_TOKEN}\n"));

    for crate_name in ["formula-engine", "formula-packages"] {
        let crate_root = root.join("crates").join(crate_name);
        let mut files = Vec::new();
        visit_files(&crate_root, &mut files);
        for file in files {
            let Ok(text) = fs::read_to_string(&file) else { continue };
            assert!(
                !text.contains(SEALED_TOKEN),
                "discovery crate {crate_name} leaks sealed fixture token via {}",
                file.display()
            );
            assert!(
                !text.contains("P0_SEALED_SENTINEL.txt"),
                "discovery crate {crate_name} references sealed fixture filename via {}",
                file.display()
            );
            assert!(
                !text.contains(SEALED_PATH),
                "discovery crate {crate_name} references sealed fixture path via {}",
                file.display()
            );
        }
    }
}
