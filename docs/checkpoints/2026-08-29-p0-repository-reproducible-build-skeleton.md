# P0 Checkpoint — Repository + Reproducible Build Skeleton

**Date:** 2026-08-29  
**Status:** PASS — clean isolated checkout, pinned Rust 1.98.0, locked/offline Cargo proof gate  
**Branch:** `implementation/p0-reproducible-skeleton`  
**Source-under-test commit:** `12262d2af9f1b72af610a8afd4c561b859f61bb7`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P0  
**Implementation plan:** `docs/superpowers/plans/2026-08-29-p0-repository-reproducible-build-skeleton.md`

---

## 1. Scope

This checkpoint proves only canonical roadmap phase **P0 — Repository and reproducible build skeleton**.

P0 establishes:

- the eight canonical First-Light crate boundaries;
- pinned source/toolchain/dependency identity;
- checker/search/realization dependency separation;
- sealed First-Light fixture ownership outside discovery crates;
- an explicit network-free canonical runtime dependency closure at P0;
- deterministic P0 source-fixture identities;
- a clean locked/offline workspace build and architecture-test gate.

This checkpoint does **not** implement P1 mathematical authority. There is no canonical D1 artifact encoding, SHA-256 `ArtifactDigest`, authority store, generation replay, certificate checker semantics, discovery engine, mathematical package implementation, promotion mechanism, or native realization implementation yet.

P0 fixture identities below are Git blob OIDs used only to freeze source fixtures. They are **not** D1/P1 mathematical authority digests.

---

## 2. Execution environment

The final proof candidate ran from a clean checkout of the isolated implementation branch on a GitHub-hosted Ubuntu runner.

```text
OS: Ubuntu 24.04.4 LTS
runner image: ubuntu-24.04 / 20260823.283.1
source commit: 12262d2af9f1b72af610a8afd4c561b859f61bb7
workflow run: 33234184747
job: 99052136521
```

Exact pinned compiler metadata:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
binary: rustc
commit-hash: 88d9e12ae178fab0fb5cc050a94da85685d449ea
commit-date: 2026-08-18
host: x86_64-unknown-linux-gnu
release: 1.98.0
LLVM version: 22.1.8
```

Exact Cargo metadata:

```text
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

The workflow checkout and toolchain provisioning necessarily used the hosted runner's network before the proof commands. The canonical Cargo metadata/test/build/clippy/tree commands themselves were executed with `--locked --offline`, and the P0 runtime closure contains only local workspace packages. This checkpoint therefore proves the P0 runtime/dependency boundary; it does not claim that the GitHub workflow bootstrap itself is network-free.

A separate replay on a user workstation has not been recorded by this checkpoint and is not claimed.

---

## 3. Frozen source identities

Exact Git blob OIDs captured from the clean source tree:

```text
Cargo.toml
ce5be581439c871ca19b2b79c4d92bc2c1cafa05

Cargo.lock
205e391948d2f12a13078c68684743daecd036ad

rust-toolchain.toml
a1598ccbb34d9332eb1e2e5dd70fe129acbad594

public P0 fixture
aa80868b5e7eba07725cc68c22a5e31116e44648

sealed P0 fixture
e879c8f2756306a1a7cf29084de267ce65fa3a37
```

Exact frozen fixture bytes:

```text
tests/first-light/public/P0_PUBLIC_SENTINEL.txt
formula-p0-public-fixture-v1\n

tests/first-light/sealed/P0_SEALED_SENTINEL.txt
formula-p0-sealed-fixture-v1\n
```

`cargo metadata --locked --offline --format-version 1` showed only workspace/path packages and no registry/Git/runtime third-party dependency.

---

## 4. Canonical P0 workspace

The proved workspace contains:

```text
crates/
  formula-core/
  formula-store/
  formula-check/
  formula-engine/
  formula-packages/
  formula-realize/
  formula-first-light/
  formula-cli/

tests/
  authority-boundary/
  first-light/
```

P0 crate roles are deliberately minimal:

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

No mathematical behavior beyond these architectural boundaries is claimed at P0.

---

## 5. Dependency firewall proof

Exact final dependency tree for `formula-check`:

```text
formula-check v0.0.1
└── formula-core v0.0.1
```

Exact final dependency structure for `formula-engine`:

```text
formula-engine v0.0.1
├── formula-core v0.0.1
├── formula-packages v0.0.1
│   └── formula-core v0.0.1
└── formula-store v0.0.1
    └── formula-core v0.0.1
```

Exact final dependency structure for `formula-first-light`:

```text
formula-first-light v0.0.1
├── formula-core v0.0.1
└── formula-engine v0.0.1
    ├── formula-core v0.0.1
    ├── formula-packages v0.0.1
    │   └── formula-core v0.0.1
    └── formula-store v0.0.1
        └── formula-core v0.0.1
```

Consequences:

- `formula-check` shares immutable schema only through `formula-core`;
- `formula-check` does not depend on engine/search/realization/First-Light code;
- `formula-engine` does not link `formula-check` implementation;
- canonical P0 First-Light runtime does not link `formula-check` or `formula-realize` implementation;
- checker/realizer invocation can therefore remain an explicit independent-process boundary in later phases.

---

## 6. Canonical P0 runtime allowlist

The canonical P0 runtime closure is explicitly frozen to:

```text
formula-cli
formula-core
formula-engine
formula-first-light
formula-packages
formula-store
```

The architecture test obtains the actual normal-dependency closure with `cargo tree` and fails if any package outside this list appears.

This is the P0 proof for the roadmap's network/runtime dependency constraint. It intentionally makes any later addition of a runtime dependency an explicit reviewed change rather than an invisible transitive dependency.

---

## 7. TDD evidence

P0 architectural boundaries were implemented through observed RED → GREEN cycles.

### Task 1 — workspace shape

RED commit:

```text
5d2a76ba56b0d30cb8116a972d6e0e8578b349e2
```

Observed failure:

```text
missing formula-core/Cargo.toml
```

GREEN after adding only the eight canonical minimal crates and final workspace membership.

### Task 2 — checker/search dependency firewall

RED commit:

```text
3877e9f7898e5f735e83ee03ef51858448d6a93c
```

Observed failure:

```text
checker must share immutable schema through formula-core
```

GREEN after adding only the canonical path dependencies. The independent test that `formula-engine` cannot link `formula-check` remained green.

### Task 3 — sealed First-Light boundary

RED commit:

```text
8d7e2fd31a9e5614687d7798d2c5a1a1a1117e78
```

Observed failure:

```text
missing public First-Light P0 fixture
```

GREEN after adding only the frozen public/sealed sentinel fixtures and the negative-fixture directory marker. Discovery crate source remained unchanged.

### Task 4 — runtime dependency allowlist

RED commit:

```text
f47b34a83dc898af6a611fa5e47fc2f904ea7a5d
```

Observed failure:

```text
missing runtime allowlist .../tests/authority-boundary/runtime-allowlist.txt
```

GREEN after adding only the six-package allowlist.

### Full gate formatting defect

The first complete proof candidate passed architecture tests, full tests, and locked/offline build, then failed `cargo fmt --check` only.

The exact pinned `rustfmt` diff was recovered before modification. Root cause was source formatting drift in the architecture-test files, not logic or dependency behavior.

The formatter-only correction is:

```text
12262d2af9f1b72af610a8afd4c561b859f61bb7
```

No test semantics or production architecture changed in that correction.

---

## 8. Final proof commands

All commands below succeeded at source-under-test commit `12262d2af9f1b72af610a8afd4c561b859f61bb7`:

```bash
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test --workspace --locked --offline
cargo build --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-check
cargo tree --locked --offline -p formula-engine
cargo tree --locked --offline -p formula-first-light
git status --porcelain
```

The final `git status --porcelain` produced no output.

---

## 9. P0 proof markers

```text
P0-01 pinned toolchain/source dependency manifest       PASS
P0-02 formula-check isolated from engine/search         PASS
P0-03 sealed fixtures unavailable to discovery crates  PASS
P0-04 canonical runtime dependency allowlist satisfied PASS
P0-05 deterministic P0 fixture identities              PASS
```

### P0-01 — PASS

Evidence:

- source commit captured exactly;
- Rust pinned exactly to `1.98.0`;
- Cargo pinned through that toolchain;
- root manifest, lockfile and toolchain file blob identities captured;
- Cargo metadata resolved offline;
- lockfile contains only local workspace packages.

### P0-02 — PASS

Evidence:

- `formula-check -> formula-core` only;
- architecture tests forbid checker dependencies on engine/realizer/First-Light;
- architecture tests forbid engine linkage to checker implementation.

### P0-03 — PASS

Evidence:

- sealed/public fixture ownership is outside active discovery crates;
- architecture test scans `formula-engine` and `formula-packages` for the sealed token, filename and path;
- frozen sealed/public fixture byte identities are independently checked.

### P0-04 — PASS

Evidence:

- explicit normal-runtime package allowlist;
- `formula-first-light` and `formula-cli` normal-dependency closures are checked against that allowlist;
- Cargo metadata contains no external runtime dependency;
- canonical Cargo proof commands run with `--offline`.

### P0-05 — PASS

Evidence:

- source fixture bytes are fixed;
- Git blob OIDs are fixed in an architecture test;
- byte drift or source identity drift fails the test.

---

## 10. Milestone result

**P0 is proved for the isolated implementation branch at the source boundary recorded above.**

The proof establishes the reproducible repository/build skeleton and architectural authority boundaries only. It does not imply First Light, mathematical correctness, self-expansion, or native realization has been implemented.

A post-checkpoint branch gate must remain green before this checkpoint is treated as the branch's final P0 candidate.

---

## 11. Next phase

Only after the post-checkpoint P0 gate remains green may work advance to:

```text
P1 — Core structural identity and authority store
```

P1 begins canonical D1/D2 authority implementation:

```text
Entity
Relation
World
Judgement
EvidenceEnvelope metadata
Realization metadata
ArtifactDigest
UniverseGeneration
AuthorityContract
Observer
canonical encoding v1
SHA-256 structural digests
content-addressed immutable blob store
generation manifest build/load/replay
```

No P1 implementation exists in this checkpoint.
