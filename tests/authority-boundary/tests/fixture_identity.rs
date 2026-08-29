use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const FIXTURES: &[(&str, &[u8], &str)] = &[
    (
        "tests/first-light/public/P0_PUBLIC_SENTINEL.txt",
        b"formula-p0-public-fixture-v1\n",
        "aa80868b5e7eba07725cc68c22a5e31116e44648",
    ),
    (
        "tests/first-light/sealed/P0_SEALED_SENTINEL.txt",
        b"formula-p0-sealed-fixture-v1\n",
        "e879c8f2756306a1a7cf29084de267ce65fa3a37",
    ),
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn git_blob_oid(root: &Path, relative: &str) -> String {
    let out = Command::new("git")
        .args(["hash-object", relative])
        .current_dir(root)
        .output()
        .expect("git hash-object must execute");
    assert!(
        out.status.success(),
        "git hash-object failed for {relative}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("git hash-object output must be UTF-8")
        .trim()
        .to_owned()
}

#[test]
fn p0_fixture_bytes_and_source_identities_are_frozen() {
    let root = root();
    for (relative, expected_bytes, expected_oid) in FIXTURES {
        let path = root.join(relative);
        let bytes =
            fs::read(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        assert_eq!(
            bytes.as_slice(),
            *expected_bytes,
            "fixture byte drift: {relative}"
        );
        assert_eq!(
            git_blob_oid(&root, relative),
            *expected_oid,
            "fixture Git blob identity drift: {relative}"
        );
    }
}
