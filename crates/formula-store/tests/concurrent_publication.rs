//! P1 D2-P02: two independent OS processes must not rebind a generation.
//! Construction regression only; no Formula authority is admitted.

#[cfg(target_os = "linux")]
#[test]
fn cross_process_publish_child() {
    use formula_store::{BlobStore, GenerationIndex};
    use std::fs;
    use std::path::PathBuf;
    use std::thread;
    use std::time::{Duration, Instant};

    let Some(root) = std::env::var_os("FORMULA_P1_CROSS_PROCESS_ROOT") else {
        return; // This helper is inert in an ordinary test invocation.
    };
    let root = PathBuf::from(root);
    let role = std::env::var("FORMULA_P1_CROSS_PROCESS_ROLE").unwrap();
    let bytes: &[u8] = match role.as_str() {
        "first" => b"first independent process",
        "second" => b"second independent process",
        _ => panic!("unknown process role"),
    };
    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    let digest = store.put(bytes).unwrap();
    // Publish the child-produced digest before releasing the process barrier.
    fs::write(root.join(format!("ready-{role}")), digest.to_hex()).unwrap();

    let deadline = Instant::now() + Duration::from_secs(15);
    while !root.join("start").exists() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(10));
    }
    assert!(root.join("start").exists(), "publication barrier timed out");

    let disposition = match index.publish(&store, 42, digest) {
        Ok(()) => "ok",
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => "already_exists",
        Err(error) => panic!("unexpected publication error: {error}"),
    };
    fs::write(root.join(format!("result-{role}")), disposition).unwrap();
}

#[cfg(target_os = "linux")]
#[test]
fn competing_processes_preserve_one_exact_history_binding() {
    use formula_core::ArtifactDigest;
    use formula_store::{BlobStore, GenerationIndex};
    use std::fs;
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "formula-p1-cross-process-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&root).expect("create isolated fixture");

    let binary = std::env::current_exe().expect("current test binary");
    let children: Vec<_> = ["first", "second"]
        .into_iter()
        .map(|role| {
            Command::new(&binary)
                .args(["--exact", "cross_process_publish_child", "--nocapture"])
                .env("FORMULA_P1_CROSS_PROCESS_ROOT", &root)
                .env("FORMULA_P1_CROSS_PROCESS_ROLE", role)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("spawn independent publisher")
        })
        .collect();

    let deadline = Instant::now() + Duration::from_secs(15);
    while !(root.join("ready-first").exists() && root.join("ready-second").exists())
        && Instant::now() < deadline
    {
        thread::sleep(Duration::from_millis(10));
    }
    assert!(
        root.join("ready-first").exists() && root.join("ready-second").exists(),
        "both publishers must reach the process barrier"
    );
    fs::write(root.join("start"), b"go").unwrap();

    for child in children {
        let output = child.wait_with_output().expect("wait for publisher");
        assert!(
            output.status.success(),
            "publisher failed: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let first = fs::read_to_string(root.join("result-first")).unwrap();
    let second = fs::read_to_string(root.join("result-second")).unwrap();
    assert!(
        (first == "ok" && second == "already_exists")
            || (second == "ok" && first == "already_exists"),
        "exactly one process may publish a distinct manifest: {first}, {second}"
    );

    let store = BlobStore::new(root.join("store"));
    let index = GenerationIndex::new(root.join("index"));
    // Read immutable child evidence; a parent put() would silently repair a missing
    // blob and could make a broken publication look valid.
    let first_digest =
        ArtifactDigest::from_hex(&fs::read_to_string(root.join("ready-first")).unwrap()).unwrap();
    let second_digest =
        ArtifactDigest::from_hex(&fs::read_to_string(root.join("ready-second")).unwrap()).unwrap();
    let winner = if first == "ok" {
        first_digest
    } else {
        second_digest
    };
    assert_eq!(index.historical_manifest(42).unwrap(), winner);
    assert_eq!(
        index.verified_active_generation(&store).unwrap(),
        (42, winner)
    );
    fs::remove_dir_all(root).expect("remove isolated fixture");
}
