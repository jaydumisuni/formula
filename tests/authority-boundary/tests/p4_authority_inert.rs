use std::{
    fs,
    path::{Path, PathBuf},
};

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
fn p4_engine_sources_are_authority_inert() {
    let engine_src = root().join("crates/formula-engine/src");
    let mut files = Vec::new();
    visit_files(&engine_src, &mut files);

    let forbidden = [
        "publish_generation",
        "rollback_generation",
        "update_authority",
        "authority_transaction",
        "PromotionCandidate",
        "admit_promotion_candidate",
        "formula_check",
        "formula-check",
    ];

    for file in files {
        let Ok(text) = fs::read_to_string(&file) else {
            continue;
        };
        for token in forbidden {
            assert!(
                !text.contains(token),
                "P4 engine source {} crosses authority boundary via forbidden token {token}",
                file.display()
            );
        }
    }
}
