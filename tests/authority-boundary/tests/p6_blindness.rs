use std::{
    fs,
    path::{Path, PathBuf},
};

const SEALED_PATH: &str = "tests/first-light/sealed";
const FIRST_LIGHT_CRATE_HYPHEN: &str = "formula-first-light";
const FIRST_LIGHT_CRATE_UNDERSCORE: &str = "formula_first_light";
const FL_A_TARGET_SCHEMA: &str = "formula-p6-fl-a-sealed-target-v1";
const FL_C_TARGET_SCHEMA: &str = "formula-p6-fl-c-sealed-u8-target-v1";

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

fn compact(text: &str) -> String {
    text.chars().filter(|character| !character.is_whitespace()).collect()
}

#[test]
fn discovery_production_sources_cannot_depend_on_sealed_first_light_harness() {
    let root = root();
    for crate_name in ["formula-engine", "formula-packages"] {
        let crate_root = root.join("crates").join(crate_name);
        let manifest = fs::read_to_string(crate_root.join("Cargo.toml")).unwrap();
        assert!(
            !manifest.contains(FIRST_LIGHT_CRATE_HYPHEN),
            "discovery crate {crate_name} depends on sealed First-Light harness"
        );

        let mut files = Vec::new();
        visit_rs_files(&crate_root.join("src"), &mut files);
        for file in files {
            let text = fs::read_to_string(&file).unwrap();
            assert!(
                !text.contains(FIRST_LIGHT_CRATE_UNDERSCORE),
                "discovery source references sealed harness via {}",
                file.display()
            );
            assert!(
                !text.contains(SEALED_PATH),
                "discovery source references sealed fixture path via {}",
                file.display()
            );
            assert!(
                !text.contains(FL_A_TARGET_SCHEMA) && !text.contains(FL_C_TARGET_SCHEMA),
                "discovery source embeds sealed target identity via {}",
                file.display()
            );
        }
    }
}

#[test]
fn discovery_production_sources_do_not_embed_hidden_first_light_answers() {
    let root = root();
    let forbidden_fl_c = compact(
        "BoolExpr::and(\
         BoolExpr::neq_zero(ByteExpr::x()),\
         BoolExpr::eq_zero(ByteExpr::bit_and(\
         ByteExpr::x(),\
         ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one())\
         )))",
    );
    let forbidden_fl_a_ascending = "1,7,21,35,35,21,7";
    let forbidden_fl_a_descending = "7,21,35,35,21,7,1";

    for crate_name in ["formula-engine", "formula-packages"] {
        let src_root = root.join("crates").join(crate_name).join("src");
        let mut files = Vec::new();
        visit_rs_files(&src_root, &mut files);
        for file in files {
            let text = compact(&fs::read_to_string(&file).unwrap());
            assert!(
                !text.contains(&forbidden_fl_c),
                "discovery source embeds final FL-C answer via {}",
                file.display()
            );
            assert!(
                !text.contains(forbidden_fl_a_ascending)
                    && !text.contains(forbidden_fl_a_descending),
                "discovery source embeds expanded FL-A coefficients via {}",
                file.display()
            );
        }
    }
}

#[test]
fn sealed_target_semantics_are_owned_by_first_light_crate() {
    let root = root();
    let fl_a = fs::read_to_string(root.join("crates/formula-first-light/src/fl_a.rs")).unwrap();
    let fl_c = fs::read_to_string(root.join("crates/formula-first-light/src/fl_c.rs")).unwrap();
    assert!(fl_a.contains(FL_A_TARGET_SCHEMA));
    assert!(fl_c.contains(FL_C_TARGET_SCHEMA));
}
