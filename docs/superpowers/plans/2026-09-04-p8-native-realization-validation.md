# P8 Native Realization and Validation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement and canonically prove the bounded P8 D4 native CPU realization path for the P7-admitted FL-C semantic primitive.

**Architecture:** Keep generation and authority roles separated. `formula-realize` is an untrusted deterministic source/binary producer, `formula-check` independently validates the exact source/toolchain/binary/semantic bindings and issues opaque `RealizationAuthorization`, and `formula-store` admits/resolves only checker-authorized realizations while preserving the underlying P7 mathematical admission. The P8 end-to-end proof discovers/promotes FL-C exactly as P7 did, generates standalone Rust, compiles with pinned Rust 1.98.0 `-O`, executes all 256 U8 inputs, validates independently, admits the realization, and proves mutation/identity controls fail closed.

**Tech Stack:** Rust 1.98.0, Cargo workspace, SHA-256 `ArtifactDigest`, canonical structural identities, `rustc -O`, SQLite/blob authority store, GitHub Actions Ubuntu 24.04.

**Spec:** `docs/superpowers/specs/2026-09-04-p8-native-realization-validation-design.md`

## Global Constraints

- Exact predecessor is `e82f7b0535694285baeeb4baae37edc27b6864b8`.
- Canonical P8 runner is `ubuntu-24.04` with Rust `1.98.0`.
- Canonical native backend is standalone Rust compiled by pinned `rustc -O`.
- P8 realization semantics are bounded to forward `U8 -> Bool` and exhaustive inputs `0..=255`.
- Mathematical authority and realization authority remain separate; false realization evidence cannot invalidate P7 mathematics.
- `formula-realize` may generate candidate code but cannot issue or construct realization authority.
- `formula-check` is the only component that can issue opaque `RealizationAuthorization`.
- `formula-store` consumes authorization; it does not decide semantic correctness.
- Canonical proof is local, CPU-only, model-free, network-free during proof execution, Ptah-free, and GPU-free.
- P9 second-query reuse, synthesis-skipping proof, and `FIRST_LIGHT_COMPLETE` are out of scope.

---

### Task 1: Canonical P8 Realization Identities

**Files:**
- Create: `crates/formula-core/src/realization.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/tests/p8_realization_identity.rs`

**Interfaces:**
- Consumes: `ArtifactDigest`, `CanonicalValue`, `StructuralIdentity`.
- Produces: `SpecializationIdentity`, `NativeToolchainIdentity`, `NativeRealizationManifest`, and `RealizationDispatchContext`.

- [ ] **Step 1: Write failing structural-identity tests**

Create tests that instantiate each type and assert:

```rust
assert_eq!(same.structural_digest(), same_again.structural_digest());
assert_ne!(base.structural_digest(), changed_semantic.structural_digest());
assert_ne!(base.structural_digest(), changed_source.structural_digest());
assert_ne!(base.structural_digest(), changed_binary.structural_digest());
assert_ne!(base.structural_digest(), changed_toolchain.structural_digest());
assert_ne!(base.structural_digest(), changed_specialization.structural_digest());
```

Also assert the fixed P8 contract strings through getters:

```rust
assert_eq!(specialization.query_direction(), "u8_to_bool_forward");
assert_eq!(specialization.input_domain(), "u8:0..=255");
assert_eq!(specialization.output_domain(), "bool");
assert_eq!(specialization.lowering_class(), "EXACT_EQUIVALENCE");
assert_eq!(manifest.fallback_semantics(), "semantic_execution");
```

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test -p formula-core --test p8_realization_identity --locked
```

Expected: compile failure because the P8 realization types/module do not exist.

- [ ] **Step 3: Implement minimal canonical identities**

`SpecializationIdentity::new(...)` binds semantic target, generation, world, authority contract, observer and the fixed P8 direction/domain/lowering strings.

`NativeToolchainIdentity::new(rust_release, host_target)` binds:

```text
compiler = rustc
rust_release = caller supplied exact release
optimization = -O
host_target = caller supplied host
backend_family = standalone-rust-native
```

`NativeRealizationManifest::new(...)` binds semantic target, generation, world, authority contract, observer, specialization/source/toolchain/binary digests and fixed representation/lowering/fallback strings.

`RealizationDispatchContext::new(...)` binds semantic target, generation, world, authority contract and observer for exact resolution.

All authority-bearing fields remain private and expose read-only getters.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test -p formula-core --test p8_realization_identity --locked
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-core/src/lib.rs crates/formula-core/src/realization.rs crates/formula-core/tests/p8_realization_identity.rs
git commit -m "feat(p8): define native realization identities"
```

---

### Task 2: Deterministic Standalone Rust Generation

**Files:**
- Modify: `crates/formula-realize/Cargo.toml`
- Modify: `crates/formula-realize/src/lib.rs`
- Create: `crates/formula-realize/src/rust_native.rs`
- Create: `crates/formula-realize/tests/p8_rust_generation.rs`

**Interfaces:**
- Consumes: `formula_engine::observational::{BoolExpr, ByteExpr}`, `SpecializationIdentity`.
- Produces: `NativeSourceArtifact { source: String, source_digest: ArtifactDigest }` via `generate_u8_bool_rust_source`.

- [ ] **Step 1: Write failing deterministic-generation tests**

Use a fixed semantic expression such as:

```rust
BoolExpr::neq_zero(ByteExpr::bit_and(
    ByteExpr::x(),
    ByteExpr::sub_wrap(ByteExpr::x(), ByteExpr::one()),
))
```

Assert:

```rust
let a = generate_u8_bool_rust_source(&expr, &specialization).unwrap();
let b = generate_u8_bool_rust_source(&expr, &specialization).unwrap();
assert_eq!(a.source(), b.source());
assert_eq!(a.source_digest(), b.source_digest());
assert!(a.source().contains("wrapping_sub"));
assert!(a.source().contains("std::process::exit(2)"));
assert!(a.source().contains("println!(\"{}\", if result { 1 } else { 0 });"));
```

Add a mismatch test where `specialization.semantic_target() != expr.digest()` and expect `NativeGenerationError::SemanticTargetMismatch`.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test -p formula-realize --test p8_rust_generation --locked
```

Expected: compile failure because generator APIs do not exist.

- [ ] **Step 3: Implement minimal generator**

Add normal dependency:

```toml
formula-engine = { path = "../formula-engine" }
```

Render every existing bounded observational expression variant recursively:

```text
ByteExpr::X -> x
ByteExpr::Zero -> 0u8
ByteExpr::One -> 1u8
ByteExpr::SubWrap(a,b) -> (a).wrapping_sub(b)
ByteExpr::BitAnd(a,b) -> (a) & (b)
BoolExpr::EqZero(v) -> (v) == 0
BoolExpr::NeqZero(v) -> (v) != 0
BoolExpr::And(a,b) -> (a) && (b)
```

Generated `main` requires exactly one argument, parses `u8`, exits 2 for invalid input/arity, computes the expression, and prints exactly `0` or `1` plus newline.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test -p formula-realize --test p8_rust_generation --locked
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-realize/Cargo.toml crates/formula-realize/src/lib.rs crates/formula-realize/src/rust_native.rs crates/formula-realize/tests/p8_rust_generation.rs Cargo.lock
git commit -m "feat(p8): generate deterministic native Rust source"
```

---

### Task 3: Checker-Issued Realization Authorization

**Files:**
- Modify: `crates/formula-check/src/realization.rs`
- Modify: `crates/formula-check/src/verdict.rs`
- Create: `crates/formula-check/tests/p8_realization_authorization.rs`

**Interfaces:**
- Consumes: `NativeRealizationManifest`, `SpecializationIdentity`, `NativeToolchainIdentity`, existing `RealizationCheckManifest`, exact source/binary bytes, checker `BoolExpr`, and 256 realized outputs.
- Produces: opaque `RealizationAuthorization` and `authorize_native_u8_realization_v1(...) -> Result<RealizationAuthorization, RealizationPolicyFailure>`.

- [ ] **Step 1: Write failing authorization tests**

Build a valid fixture and assert authorization exposes only read-only bindings:

```rust
let authorization = authorize_native_u8_realization_v1(...).unwrap();
assert_eq!(authorization.realization_manifest(), native_manifest.structural_digest());
assert_eq!(authorization.binary_digest(), native_manifest.binary_digest());
assert_eq!(authorization.universe_generation(), native_manifest.universe_generation());
```

Add one-variable negative tests for changed source bytes, changed binary bytes, changed specialization digest, changed toolchain digest, wrong generation/world/authority/observer, 255 outputs, and one wrong output.

Expected typed failures include source digest mismatch, native binding mismatch, inherited output coverage mismatch, and exact `RealizationCounterexample(input)` propagation.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test -p formula-check --test p8_realization_authorization --locked
```

Expected: compile failure because authorization APIs do not exist.

- [ ] **Step 3: Implement minimal authorization policy**

Extend `CheckFailure` with `RealizationSourceDigestMismatch` and `RealizationNativeBindingMismatch`.

`authorize_native_u8_realization_v1` must:

1. verify native manifest fields match specialization and existing checker manifest bindings;
2. hash source bytes and require `source_digest`;
3. hash binary bytes and require `binary_digest` / existing realization artifact digest;
4. require provided specialization/toolchain structural digests to match the native manifest;
5. call existing `check_u8_realization_equivalence` for exact 256-value semantic validation;
6. return opaque authorization only on `CheckVerdict::Pass`.

`RealizationAuthorization` fields are private and it exposes no public constructor.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test -p formula-check --test p8_realization_authorization --locked
cargo test -p formula-check --test realization_equivalence --locked
```

Expected: PASS for both new and inherited realization checker tests.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check/src/realization.rs crates/formula-check/src/verdict.rs crates/formula-check/tests/p8_realization_authorization.rs
git commit -m "feat(p8): issue checked realization authorization"
```

---

### Task 4: Authorized Realization Admission and Dispatch

**Files:**
- Modify: `crates/formula-store/src/lib.rs`
- Modify: `crates/formula-store/src/authority_store.rs`
- Create: `crates/formula-store/src/realization_store.rs`
- Create: `crates/formula-store/tests/p8_realization_store.rs`

**Interfaces:**
- Consumes: checker `RealizationAuthorization`, binary bytes, `RealizationDispatchContext`.
- Produces: `AuthorityStore::admit_realization(...)`, `AuthorityStore::resolve_realization(...)`, `AdmittedRealization`.

- [ ] **Step 1: Write failing store tests**

Initialize U0, promote a primitive into U1 with existing P7 authorization, construct a valid realization authorization bound to U1, and assert:

```rust
let admitted = store.admit_realization(&authorization, binary_bytes).unwrap();
assert_eq!(admitted.manifest_digest(), authorization.realization_manifest());
let resolved = store.resolve_realization(&context).unwrap().unwrap();
assert_eq!(resolved.binary_bytes(), binary_bytes);
```

Add negatives:

```text
wrong active/admission generation -> rejected
binary bytes changed at admission -> rejected
wrong dispatch context -> None
stored binary blob missing/tampered -> error/fail closed
```

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test -p formula-store --test p8_realization_store --locked
```

Expected: compile failure because realization admission/dispatch APIs do not exist.

- [ ] **Step 3: Implement minimal durable registry**

Add SQLite table `realizations` with manifest digest primary key and exact semantic/generation/world/authority/observer/binary bindings.

`admit_realization` must require the authorization generation equals the active generation, verify binary bytes hash to the authorized binary digest, store bytes in the existing immutable blob store, and insert exact bindings.

`resolve_realization` queries only exact `RealizationDispatchContext` matches, loads the bound blob, rehashes bytes, and returns `None` for context mismatch. Missing/tampered bytes return a store error rather than a guessed result.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test -p formula-store --test p8_realization_store --locked
cargo test -p formula-store --all-targets --locked
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-store/src/lib.rs crates/formula-store/src/authority_store.rs crates/formula-store/src/realization_store.rs crates/formula-store/tests/p8_realization_store.rs
git commit -m "feat(p8): admit and resolve authorized realizations"
```

---

### Task 5: P8 Authority Firewall

**Files:**
- Create: `tests/authority-boundary/tests/p8_realization_authority.rs`

**Interfaces:**
- Consumes: repository source/manifests.
- Produces: mechanical proof that generator/compiler code cannot self-admit and authorization remains opaque.

- [ ] **Step 1: Write authority-boundary tests**

Assert `formula-realize/Cargo.toml` contains neither `formula-check` nor `formula-store`.

Scan `crates/formula-realize/src/**/*.rs` and reject these tokens:

```text
formula_check
formula_store
RealizationAuthorization
AuthorityStore
admit_realization
```

Inspect `formula-check/src/realization.rs` and assert `RealizationAuthorization` has no public authority-bearing fields and no `pub fn new(` constructor.

Inspect `formula-store/src/realization_store.rs` and assert the public admission signature consumes `&RealizationAuthorization` rather than a native manifest/checker semantic expression.

- [ ] **Step 2: Verify RED against incomplete P8 implementation**

Run before Task 4 production code if possible, otherwise temporarily make the expected API assertion stricter so it fails until the final admission signature exists:

```bash
cargo test -p formula-archtest --test p8_realization_authority --locked
```

Expected: FAIL until all P8 authority boundaries are present.

- [ ] **Step 3: Make only boundary-required adjustments**

Do not weaken tests. If production dependency/API structure violates the asserted boundary, move the authority-bearing operation to checker/store rather than adding exceptions.

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test -p formula-archtest --test p8_realization_authority --locked
cargo test -p formula-archtest --all-targets --locked
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add tests/authority-boundary/tests/p8_realization_authority.rs
git commit -m "test(p8): enforce realization authority firewall"
```

---

### Task 6: End-to-End FL-C Native CPU Proof

**Files:**
- Modify: `crates/formula-first-light/Cargo.toml`
- Create: `crates/formula-first-light/tests/p8_fl_c_native_realization.rs`

**Interfaces:**
- Consumes: existing P7 FL-C discovery/promotion path, generator, pinned `rustc`, checker authorization, realization store.
- Produces: executable end-to-end P8 evidence for all 256 U8 values.

- [ ] **Step 1: Write failing integration test**

Reproduce the P7 FL-C bounded discovery/promote flow to obtain the final engine `BoolExpr`, primitive digest and U1.

Then:

1. create exact `SpecializationIdentity` bound to the admitted primitive/U1;
2. generate deterministic source;
3. derive/check `NativeToolchainIdentity` from test-supplied canonical Rust release/host evidence;
4. write source to a temporary file and invoke `rustc -O -o <binary>`;
5. hash executable bytes;
6. execute the binary for every input `0..=255`, parse only `0\n` or `1\n`, and collect 256 booleans;
7. independently translate the discovered engine expression into checker `BoolExpr` structurally, not by using the realized outputs;
8. issue `RealizationAuthorization`;
9. admit into the store;
10. resolve exact dispatch context and verify bound executable bytes.

Negative sections in the same integration test must prove source mutation and binary mutation reject without changing the U1 generation bytes/digest.

- [ ] **Step 2: Verify RED**

Run:

```bash
cargo test -p formula-first-light --test p8_fl_c_native_realization --locked -- --nocapture
```

Expected: compile/test failure until Tasks 1-5 interfaces are implemented.

- [ ] **Step 3: Add only required dev-dependencies and integration helpers**

Keep `formula-first-light` production dependencies unchanged. Add `formula-realize` only under `[dev-dependencies]`; reuse existing dev-only checker/store/tempfile/engine access.

The engine-to-checker expression translation covers exactly the existing bounded variants and must preserve semantics structurally:

```text
engine X -> checker X
Zero -> Const(0)
One -> Const(1)
SubWrap -> SubWrap
BitAnd -> BitAnd
EqZero -> EqZero
NeqZero -> NeqZero
And -> And
```

- [ ] **Step 4: Verify GREEN**

Run:

```bash
cargo test -p formula-first-light --test p8_fl_c_native_realization --locked -- --nocapture
```

Expected: PASS, with 256 native executions independently equivalent and mutation controls rejected.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-first-light/Cargo.toml crates/formula-first-light/tests/p8_fl_c_native_realization.rs Cargo.lock
git commit -m "test(p8): prove FL-C native realization end to end"
```

---

### Task 7: P8 Canonical Proof and Freeze Documentation

**Files:**
- Create: `.github/workflows/p8-canonical-proof.yml`
- Create: `docs/checkpoints/2026-09-04-p8-native-realization-validation.md`
- Modify: `CURRENT.md`

**Interfaces:**
- Consumes: complete P8 implementation.
- Produces: unchanged exact-head canonical proof and cross-chat recovery authority.

- [ ] **Step 1: Create canonical read-only workflow**

Use:

```yaml
runs-on: ubuntu-24.04
permissions:
  contents: read
```

Install/select Rust 1.98.0, prime dependencies once, then use `--locked --offline` where applicable.

Run at minimum:

```bash
cargo metadata --locked --offline --format-version 1 >/dev/null
cargo test -p formula-archtest --all-targets --locked --offline
cargo test -p formula-core --all-targets --locked --offline
cargo test -p formula-check --all-targets --locked --offline
cargo test -p formula-store --all-targets --locked --offline
cargo test -p formula-realize --all-targets --locked --offline
cargo test -p formula-first-light --all-targets --locked --offline -- --nocapture
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --all-targets --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
```

Derive the P8 proof markers only after the corresponding commands/tests pass:

```text
PASS P8_SEMANTIC_BINDING
PASS P8_SPECIALIZATION_IDENTITY
PASS P8_EXACT_LOWERING_CLASS
PASS P8_SOURCE_TOOLCHAIN_BINARY_BOUND
PASS P8_COMPILER_CANNOT_SELF_ADMIT
PASS P8_ALL_256_INPUTS_EQUIVALENT
PASS P8_MUTATED_SOURCE_REJECTED
PASS P8_MUTATED_BINARY_REJECTED
PASS P8_DISPATCH_IDENTITY_ENFORCED
PASS P8_CPU_LOCAL_OFFLINE
PASS P8_P7_MATH_AUTHORITY_PRESERVED
```

Finish with `git diff --exit-code` and `git status --porcelain` clean checks.

- [ ] **Step 2: Run canonical workflow on exact source head**

Push/commit workflow and inspect the GitHub Actions run. If it fails, use the exact failing job/step/log as evidence and correct only the demonstrated defect; rerun unchanged proof after corrections.

- [ ] **Step 3: Record source-proof checkpoint**

After a successful exact source-head run, write the checkpoint with source SHA, workflow/run/job IDs, exact P7 predecessor, proof markers, scope, negative controls, and explicit P9 exclusions.

- [ ] **Step 4: Update `CURRENT.md`**

Set the next implementation boundary to P9 only after the documentation-bearing P8 head itself passes the unchanged P8 canonical workflow.

- [ ] **Step 5: Run canonical workflow on documentation-bearing head**

Require success on that exact final head before declaring P8 frozen.

- [ ] **Step 6: Commit/freeze**

```bash
git add .github/workflows/p8-canonical-proof.yml docs/checkpoints/2026-09-04-p8-native-realization-validation.md CURRENT.md
git commit -m "docs(p8): freeze native realization proof"
```

Do not merge to `main` and do not begin P9 in this task.
