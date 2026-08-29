# P0 Repository + Reproducible Build Skeleton Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement canonical roadmap phase P0: the smallest reproducible Rust workspace that proves checker/search/sealed-target separation, pins source/toolchain/dependency identity, and has no network dependency in the canonical First-Light runtime path.

**Architecture:** Build the full canonical crate boundary before any mathematical authority implementation. A dedicated non-runtime architecture-test crate inspects workspace manifests/source boundaries and proves the dependency firewalls. P0 introduces no D1 schemas, search algorithms, certificates, promotion behavior, or mathematical primitives; those begin in P1/P2.

**Tech Stack:** Stable Rust pinned to `1.98.0` for First-Light reproducibility, Cargo resolver 2, Rust edition 2024, Git. P0 intentionally uses no third-party runtime crates.

**Spec:** `docs/roadmap/2026-08-28-implementation-roadmap.md` P0; canonical design authority `docs/design/README.md`; D3 First-Light concrete crate/runtime boundary in `docs/design/2026-08-28-d3-first-light-build-architecture.md`.

## Global Constraints

- Work on an isolated branch/worktree; never implement P0 directly on `main`.
- Pin Rust exactly to `1.98.0` in `rust-toolchain.toml`; this is a First-Light implementation pin, not a constitutional language commitment.
- Create the canonical P0 workspace crates: `formula-core`, `formula-store`, `formula-check`, `formula-engine`, `formula-packages`, `formula-realize`, `formula-first-light`, `formula-cli`.
- `formula-check` may not depend on `formula-engine`, `formula-realize`, discovery/search crates, or sealed target definitions.
- `formula-engine` and `formula-packages` may not depend on or include sealed First-Light fixtures.
- Canonical First-Light runtime paths are network-free; P0 dependency policy must make any future runtime dependency addition explicit.
- Search/engine code cannot gain authority by importing checker implementation internals.
- Sealed fixtures are source-separated from active discovery code.
- `Cargo.lock`, toolchain pin, source commit, and architecture-test fixture identities are part of the P0 proof record.
- P0 does not implement SHA-256 authority identity; that belongs to P1. P0 fixture identity uses frozen Git blob OIDs only as a deterministic source-fixture proof and must not be mistaken for `ArtifactDigest`.
- TDD is mandatory for behavioral/architectural checks: write the test, observe RED for the intended missing boundary, then add the minimum structure/policy to make it GREEN.
- A P0 PASS may be claimed only from an environment with Rust 1.98.0 and Cargo available and after all prescribed commands have actually run.

---

## Final File Structure

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
.gitignore
crates/
  formula-core/
    Cargo.toml
    src/lib.rs
  formula-store/
    Cargo.toml
    src/lib.rs
  formula-check/
    Cargo.toml
    src/lib.rs
  formula-engine/
    Cargo.toml
    src/lib.rs
  formula-packages/
    Cargo.toml
    src/lib.rs
  formula-realize/
    Cargo.toml
    src/lib.rs
  formula-first-light/
    Cargo.toml
    src/lib.rs
  formula-cli/
    Cargo.toml
    src/main.rs
tests/
  authority-boundary/
    Cargo.toml
    src/lib.rs
    tests/workspace_shape.rs
    tests/dependency_firewall.rs
    tests/sealed_boundary.rs
    tests/runtime_network_policy.rs
    tests/fixture_identity.rs
    runtime-allowlist.txt
  first-light/
    public/P0_PUBLIC_SENTINEL.txt
    sealed/P0_SEALED_SENTINEL.txt
    negative/.gitkeep
```

The `tests/authority-boundary` crate is a development-only workspace member and is not part of the runtime/product architecture.

---

### Task 1: Bootstrap the complete workspace shape

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.gitignore`
- Create: `tests/authority-boundary/Cargo.toml`
- Create: `tests/authority-boundary/src/lib.rs`
- Create: `tests/authority-boundary/tests/workspace_shape.rs`
- Create: all eight canonical crate manifests and minimal `src/lib.rs`/`src/main.rs`

**Interfaces:**
- Produces workspace package names exactly: `formula-core`, `formula-store`, `formula-check`, `formula-engine`, `formula-packages`, `formula-realize`, `formula-first-light`, `formula-cli`, plus dev-only `formula-archtest`.
- Production crates expose no behavior beyond one crate-role constant at P0.

- [ ] **Step 1: Create only the toolchain pin, a workspace containing `tests/authority-boundary`, and the failing workspace-shape test**

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.98.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

Initial `Cargo.toml`:

```toml
[workspace]
members = ["tests/authority-boundary"]
resolver = "2"

[workspace.package]
edition = "2024"
license = "MIT"
publish = false
```

`tests/authority-boundary/Cargo.toml`:

```toml
[package]
name = "formula-archtest"
version = "0.0.0"
edition.workspace = true
license.workspace = true
publish = false
```

`tests/authority-boundary/src/lib.rs`:

```rust
pub const ARCHTEST_ONLY: bool = true;
```

`tests/authority-boundary/tests/workspace_shape.rs`:

```rust
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
        assert!(dir.join("Cargo.toml").is_file(), "missing {name}/Cargo.toml");
        assert!(dir.join("src").is_dir(), "missing {name}/src");
    }
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test -p formula-archtest --test workspace_shape
```

Expected: FAIL because the canonical P0 crate directories do not yet exist.

- [ ] **Step 3: Create the eight minimal crates and switch the root workspace to the final member list**

Final root `Cargo.toml`:

```toml
[workspace]
members = [
  "crates/formula-core",
  "crates/formula-store",
  "crates/formula-check",
  "crates/formula-engine",
  "crates/formula-packages",
  "crates/formula-realize",
  "crates/formula-first-light",
  "crates/formula-cli",
  "tests/authority-boundary",
]
resolver = "2"

[workspace.package]
edition = "2024"
license = "MIT"
publish = false
```

Every library crate manifest initially contains only package metadata; `formula-cli` is a binary crate. Each library `src/lib.rs` exports exactly one `CRATE_ROLE` constant. Example:

```rust
pub const CRATE_ROLE: &str = "immutable semantic schemas and identity";
```

Use these exact role strings:

```text
formula-core        immutable semantic schemas and identity
formula-store       local authority persistence boundary
formula-check       independent evidence checking boundary
formula-engine      query campaign and search orchestration
formula-packages    mathematical package implementations
formula-realize     native realization generation boundary
formula-first-light sealed First-Light harness boundary
formula-cli         local command-line entry point
```

`formula-cli/src/main.rs` initially contains:

```rust
fn main() {}
```

`.gitignore`:

```gitignore
/target/
/.formula/
```

- [ ] **Step 4: Run workspace-shape test and clean workspace build**

Run:

```bash
cargo test -p formula-archtest --test workspace_shape
cargo build --workspace
```

Expected: PASS and successful build.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock rust-toolchain.toml .gitignore crates tests/authority-boundary

git commit -m "build: bootstrap canonical P0 workspace"
```

---

### Task 2: Enforce authority/checker/search dependency firewalls

**Files:**
- Modify: crate `Cargo.toml` files
- Create: `tests/authority-boundary/tests/dependency_firewall.rs`

**Interfaces:**
- `formula-store -> formula-core`
- `formula-check -> formula-core`
- `formula-packages -> formula-core`
- `formula-engine -> formula-core + formula-store + formula-packages`
- `formula-realize -> formula-core`
- `formula-first-light -> formula-core + formula-engine`
- `formula-cli -> formula-core + formula-engine + formula-first-light`
- There is no normal dependency path `formula-check -> formula-engine|formula-realize|formula-first-light`.

- [ ] **Step 1: Write the failing dependency-firewall test**

`dependency_firewall.rs` runs `cargo tree --prefix none --edges normal -p <package>` from the workspace root and checks package-name membership.

Use this helper:

```rust
use std::{path::PathBuf, process::Command};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn cargo_tree(package: &str) -> String {
    let out = Command::new("cargo")
        .args(["tree", "--prefix", "none", "--edges", "normal", "-p", package])
        .current_dir(root())
        .output()
        .expect("cargo tree must execute");
    assert!(out.status.success(), "cargo tree failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("cargo tree output must be UTF-8")
}
```

Required tests:

```rust
#[test]
fn checker_depends_on_core_but_not_search_or_realization() {
    let tree = cargo_tree("formula-check");
    assert!(tree.contains("formula-core"), "checker must share immutable schema through formula-core");
    for forbidden in ["formula-engine", "formula-realize", "formula-first-light"] {
        assert!(!tree.contains(forbidden), "checker must not depend on {forbidden}");
    }
}

#[test]
fn engine_cannot_link_checker_implementation() {
    let tree = cargo_tree("formula-engine");
    assert!(!tree.contains("formula-check"), "engine may submit artifacts to an independent checker process but may not link checker implementation");
}
```

- [ ] **Step 2: Run and verify RED**

Run:

```bash
cargo test -p formula-archtest --test dependency_firewall
```

Expected: FAIL because `formula-check` does not yet depend on `formula-core`.

- [ ] **Step 3: Add only the minimum workspace-path dependencies described above**

Example `formula-check/Cargo.toml` dependency:

```toml
[dependencies]
formula-core = { path = "../formula-core" }
```

Do not add `formula-check` as an engine dependency.

- [ ] **Step 4: Verify GREEN and entire workspace build**

```bash
cargo test -p formula-archtest --test dependency_firewall
cargo build --workspace
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates tests/authority-boundary/tests/dependency_firewall.rs Cargo.lock

git commit -m "test: enforce checker and search dependency firewalls"
```

---

### Task 3: Seal First-Light fixture ownership from discovery code

**Files:**
- Create: `tests/first-light/public/P0_PUBLIC_SENTINEL.txt`
- Create: `tests/first-light/sealed/P0_SEALED_SENTINEL.txt`
- Create: `tests/first-light/negative/.gitkeep`
- Create: `tests/authority-boundary/tests/sealed_boundary.rs`

**Interfaces:**
- Sealed fixture token: `formula-p0-sealed-fixture-v1`.
- Public fixture token: `formula-p0-public-fixture-v1`.
- Active discovery packages (`formula-engine`, `formula-packages`) may not contain the sealed token, the sealed filename, or `tests/first-light/sealed` in source files.

- [ ] **Step 1: Write failing sealed-boundary test before creating fixtures**

The test must first assert the sealed/public fixture files exist, then recursively scan UTF-8 source/manifests under `crates/formula-engine` and `crates/formula-packages` for the forbidden sealed token/path.

Fixture contents are exactly:

```text
formula-p0-public-fixture-v1
```

and:

```text
formula-p0-sealed-fixture-v1
```

with one trailing LF in each file.

- [ ] **Step 2: Run and verify RED**

```bash
cargo test -p formula-archtest --test sealed_boundary
```

Expected: FAIL because the sealed/public fixture files have not been created.

- [ ] **Step 3: Create the fixture directories/files only**

No discovery source code changes are required; the source scan should now pass.

- [ ] **Step 4: Verify GREEN**

```bash
cargo test -p formula-archtest --test sealed_boundary
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add tests/first-light tests/authority-boundary/tests/sealed_boundary.rs

git commit -m "test: enforce sealed First-Light fixture boundary"
```

---

### Task 4: Freeze runtime dependency/network policy and deterministic fixture identities

**Files:**
- Create: `tests/authority-boundary/runtime-allowlist.txt`
- Create: `tests/authority-boundary/tests/runtime_network_policy.rs`
- Create: `tests/authority-boundary/tests/fixture_identity.rs`

**Interfaces:**
- Canonical First-Light runtime package closure at P0 is explicit and contains only workspace packages.
- P0 fixture IDs use Git blob OIDs only for deterministic source-fixture binding; they are not D1/P1 authority digests.

- [ ] **Step 1: Write the runtime-policy test and observe RED on missing allowlist**

The test runs:

```bash
cargo tree --prefix none --edges normal -p formula-first-light
cargo tree --prefix none --edges normal -p formula-cli
```

It compares every package line against `runtime-allowlist.txt` and fails on any package not explicitly listed.

Initial allowlist contents after RED:

```text
formula-cli
formula-core
formula-engine
formula-first-light
formula-packages
formula-store
```

`formula-check` and `formula-realize` are intentionally not linked into the canonical execution path at P0; later phases may invoke independent checker/realizer processes with separate manifests.

- [ ] **Step 2: Verify runtime policy GREEN**

```bash
cargo test -p formula-archtest --test runtime_network_policy
```

Expected: PASS.

- [ ] **Step 3: Write fixture-identity test with frozen expected Git blob OIDs**

The test invokes `git hash-object <fixture>` and requires:

```text
tests/first-light/public/P0_PUBLIC_SENTINEL.txt
aa80868b5e7eba07725cc68c22a5e31116e44648

tests/first-light/sealed/P0_SEALED_SENTINEL.txt
e879c8f2756306a1a7cf29084de267ce65fa3a37
```

Also assert both fixture files contain only the exact expected UTF-8 bytes.

- [ ] **Step 4: Run fixture test and verify GREEN**

```bash
cargo test -p formula-archtest --test fixture_identity
```

Expected: PASS. If an OID differs, treat it as fixture-byte drift and inspect the exact bytes; do not update the expected OID casually.

- [ ] **Step 5: Commit**

```bash
git add tests/authority-boundary/runtime-allowlist.txt tests/authority-boundary/tests/runtime_network_policy.rs tests/authority-boundary/tests/fixture_identity.rs

git commit -m "test: freeze P0 runtime and fixture identity policy"
```

---

### Task 5: Produce the reproducible P0 proof record and gate

**Files:**
- Create after successful execution: `docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md`
- No production source change is allowed in this task unless a failing gate exposes a real P0 defect; if so, return to the appropriate RED/GREEN task instead of patching around the gate.

**Interfaces:**
- Produces the P0 proof note containing exact source commit, toolchain output, lockfile/workspace Git OIDs, architecture test commands/results, and offline build result.
- Does not claim P1 authority identity.

- [ ] **Step 1: Require a clean branch before proof capture**

```bash
git status --porcelain
```

Expected: empty output.

- [ ] **Step 2: Capture exact source/toolchain/dependency metadata**

Run:

```bash
git rev-parse HEAD
rustc -vV
cargo -V
cargo metadata --locked --offline --format-version 1 > target/p0-cargo-metadata.json
git hash-object Cargo.toml Cargo.lock rust-toolchain.toml
```

Record exact stdout/digests in the checkpoint. Do not add timestamps to any structural identity claim.

- [ ] **Step 3: Run the complete architectural gate**

```bash
cargo test -p formula-archtest --locked --offline
cargo test --workspace --locked --offline
cargo build --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
```

Expected: all commands succeed with no warnings/errors.

- [ ] **Step 4: Independently inspect the checker/runtime dependency trees**

```bash
cargo tree --locked --offline -p formula-check
cargo tree --locked --offline -p formula-engine
cargo tree --locked --offline -p formula-first-light
```

Required evidence:

```text
P0-01 pinned toolchain/source dependency manifest       PASS
P0-02 formula-check isolated from engine/search         PASS
P0-03 sealed fixtures unavailable to discovery crates  PASS
P0-04 canonical runtime dependency allowlist satisfied PASS
P0-05 deterministic P0 fixture identities              PASS
```

- [ ] **Step 5: Write the checkpoint with exact command evidence**

The checkpoint must contain:

```text
source commit SHA
branch
rustc -vV exact output
cargo -V exact output
Cargo.toml Git blob OID
Cargo.lock Git blob OID
rust-toolchain.toml Git blob OID
public fixture Git blob OID
sealed fixture Git blob OID
all five P0 proof markers
all gate commands and exit results
explicit statement: no P1 mathematical authority implementation exists yet
```

- [ ] **Step 6: Commit the P0 proof checkpoint**

```bash
git add docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md

git commit -m "checkpoint: prove P0 reproducible build skeleton"
```

- [ ] **Step 7: Verify final branch against the gate one more time**

Repeat Step 3 after the checkpoint commit. P0 is complete only if the post-checkpoint branch remains green.

---

## Self-Review Against Canonical P0

- P0-01 pinned toolchain/source dependency manifest → Task 1 + Task 5.
- P0-02 `formula-check` cannot depend on engine/search → Task 2.
- P0-03 sealed targets cannot be imported by discovery packages → Task 3.
- P0-04 no network dependency in canonical runtime path → Task 4 exact allowlist + Task 5 offline gate.
- P0-05 deterministic fixture identities → Task 4 frozen fixture bytes/OIDs.
- Clean local build with exact metadata → Task 5.
- Full canonical crate separation including later-added `formula-store` → Task 1, following the later roadmap rather than the superseded B01 precursor plan.

## Execution Boundary

The current ChatGPT execution container does not have `rustc` or `cargo`, so this plan must not be marked executed here. The next execution environment must provide Rust `1.98.0` and Git, then begin at **Task 1 / Step 1 / RED**. Do not skip the observed RED step because this plan contains the expected code.
