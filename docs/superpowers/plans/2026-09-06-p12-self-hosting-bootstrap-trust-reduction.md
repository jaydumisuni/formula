# P12 Self-Hosting Bootstrap / Trust Reduction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rebuild an authority-critical identity checker through a Formula-owned Bootstrap Core, independently validate Stage1 and Stage2 successors through a distinct checker path, keep bootstrap generation separate from mathematical Universe generation, and freeze the result under an unchanged read-only canonical workflow.

**Architecture:** Canonical bootstrap identities live in `formula-core`; generation and bytecode execution live in `formula-realize`; independent recompilation/equivalence authority lives in `formula-check`; append-only `T_g` admission lives in `formula-store`; a sealed First-Light P12 harness executes Stage0→Stage1→Stage2 and all trusting-trust controls. The external Rust 1.98 toolchain remains an explicit hashed B0 seed.

**Tech Stack:** Rust 1.98.0, existing SHA-256 `ArtifactDigest`, `rusqlite` store patterns, GitHub Actions read-only canonical proof.

**Spec:** `docs/superpowers/specs/2026-09-06-p12-self-hosting-bootstrap-trust-reduction-design.md`

## Global Constraints

- Exact frozen P11 predecessor: `6f8ce7bb6702ea1baf119aab9950aa5ba0f87283`.
- `generator_identity != validator_identity` is mandatory.
- `BootstrapGenerationId` / `T_g` is semantically distinct from `UniverseGeneration` / `U_g`.
- Self-hosting cannot call mathematical Certification/Promotion or mutate active `U_g`.
- B0 Rust toolchain remains explicit seed provenance.
- Canonical Bootstrap Core v1 accepts exactly four instructions and four opcodes.
- Canonical P12 requires byte-for-byte independent rebuild equality plus semantic equivalence.
- All NC-BS-01...NC-BS-10 must execute before final proof-manifest construction.
- Permanent canonical workflow has `contents: read` and no write-back step.

---

### Task 1: Canonical bootstrap identities

**Files:**
- Create: `crates/formula-core/src/bootstrap.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Test: `crates/formula-core/tests/p12_bootstrap_identity.rs`

**Produces:**

```rust
BootstrapGenerationId
BootstrapRole
BootstrapSeedManifest
BootstrapInstruction
BootstrapProgramSource
BootstrapBytecode
BootstrapGeneratorImage
BootstrapDecision
BootstrapEquivalenceLevel
BootstrapValidationState
BootstrapRebuildManifest
BootstrapNegativeControl
BootstrapNegativeControlEvidence
BootstrapNegativeControlManifest
BootstrapProofManifest
```

- [ ] Write failing identity tests proving every seed/rebuild/program field changes structural identity, NC-BS manifest requires exactly 10 controls, `T_g` identity cannot be constructed from `UniverseGeneration` implicitly, and canonical identity-checker source is exactly four instructions.
- [ ] Run `cargo test -p formula-core --test p12_bootstrap_identity --locked` and require RED on missing `formula_core::bootstrap`.
- [ ] Implement canonical structs/accessors/`StructuralIdentity` using existing `CanonicalValue` patterns.
- [ ] Run targeted test and full P12 development gate.
- [ ] Commit only core schema + test.

### Task 2: Formula-owned Bootstrap Core generator/runtime

**Files:**
- Create: `crates/formula-realize/src/bootstrap.rs`
- Modify: `crates/formula-realize/src/lib.rs`
- Test: `crates/formula-realize/tests/p12_bootstrap_generator.rs`

**Produces:**

```rust
pub fn compile_bootstrap_source(
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapGenerationError>;

pub fn canonical_generator_image() -> BootstrapGeneratorImage;

pub fn rebuild_with_generator_image(
    image: &BootstrapGeneratorImage,
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapGenerationError>;

pub fn execute_bootstrap_bytecode(
    bytecode: &BootstrapBytecode,
    actual: ArtifactDigest,
    expected: ArtifactDigest,
) -> Result<BootstrapDecision, BootstrapExecutionError>;
```

- [ ] Write RED tests for exact `FBC1 01 02 03 04` bytecode, image-driven rebuild equality, valid/reject semantics, and malformed/unknown/truncated bytecode rejection.
- [ ] Prove RED.
- [ ] Implement generator/image interpreter/bytecode evaluator without checker dependencies.
- [ ] Prove targeted + workspace GREEN.
- [ ] Commit.

### Task 3: Independent bootstrap validator and authorization

**Files:**
- Create: `crates/formula-check/src/bootstrap.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p12_bootstrap_validator.rs`

**Produces:**

```rust
pub fn reference_compile(
    source: &BootstrapProgramSource,
) -> Result<BootstrapBytecode, BootstrapValidationFailure>;

pub fn reference_execute(
    source: &BootstrapProgramSource,
    actual: ArtifactDigest,
    expected: ArtifactDigest,
) -> Result<BootstrapDecision, BootstrapValidationFailure>;

pub fn validate_bootstrap_candidate(
    rebuild: &BootstrapRebuildManifest,
    source: &BootstrapProgramSource,
    candidate: &BootstrapBytecode,
    seed: &BootstrapSeedManifest,
) -> Result<BootstrapValidationAuthorization, BootstrapValidationFailure>;
```

`BootstrapValidationAuthorization` must expose read-only bindings but keep construction private to `formula-check`.

- [ ] Write RED tests proving generator==validator rejection, seed/source/build mismatch rejection, candidate/reference mismatch rejection, semantic mismatch rejection, malformed bytecode rejection, and successful authorization only after exact independent compilation + two-case identity semantics.
- [ ] Prove RED.
- [ ] Implement a separate opcode match/reference compiler and source evaluator; do not import `formula-realize`.
- [ ] Prove targeted + workspace GREEN and dependency firewall.
- [ ] Commit.

### Task 4: Separate bootstrap generation authority store

**Files:**
- Create: `crates/formula-store/src/authority_store/bootstrap_store.rs`
- Modify: `crates/formula-store/src/authority_store.rs`
- Test: `crates/formula-store/tests/p12_bootstrap_store.rs`

**Produces:**

```rust
pub fn create_bootstrap_root(
    &mut self,
    seed: &BootstrapSeedManifest,
) -> Result<BootstrapGenerationId, AuthorityStoreError>;

pub fn admit_bootstrap_successor(
    &mut self,
    authorization: &BootstrapValidationAuthorization,
    candidate: &BootstrapBytecode,
) -> Result<BootstrapGenerationId, AuthorityStoreError>;

pub fn active_bootstrap_generation(
    &self,
) -> Result<BootstrapGenerationId, AuthorityStoreError>;

pub fn replay_bootstrap_generation(
    &self,
    id: BootstrapGenerationId,
) -> Result<AdmittedBootstrapGeneration, AuthorityStoreError>;

pub fn select_bootstrap_generation(
    &mut self,
    id: BootstrapGenerationId,
) -> Result<(), AuthorityStoreError>;
```

- [ ] Write RED tests for T0 creation, authorized T1 admission, unauthorized candidate rejection, append-only T2 admission, rollback/select, and exact proof that active `UniverseGeneration` is unchanged across every bootstrap operation.
- [ ] Prove RED.
- [ ] Implement normalized bootstrap tables and pointer separate from `meta.active_generation`.
- [ ] Prove targeted + workspace GREEN.
- [ ] Commit.

### Task 5: Stage1→Stage2 canonical self-host proof and NC-BS-01...10

**Files:**
- Create: `crates/formula-first-light/src/p12.rs`
- Modify: `crates/formula-first-light/src/lib.rs`
- Create: `crates/formula-first-light/tests/p12_bootstrap_trust_reduction.rs`

**Consumes:** Tasks 1-4.

**Proof sequence:**

```text
exact P11 predecessor
 -> B0 seed manifest from workflow-provided executable hashes
 -> T0 bootstrap root
 -> canonical identity-checker Bootstrap Core source
 -> Stage0 generator image builds Stage1 candidate
 -> independent checker validates Stage1
 -> T1 admission
 -> admitted Stage1 image rebuilds same checker
 -> independent checker validates Stage2
 -> T2 admission
 -> T1/T2 byte equality + semantic equality
 -> U-before == U-after
 -> NC-BS-01...NC-BS-10 complete
```

- [ ] Write the integration test with source/toolchain digest environment variables required (`P12_RUSTC_SHA256`, `P12_CARGO_SHA256`, `P12_RUST_TOOLCHAIN_SHA256`, `GITHUB_SHA`), plus explicit fallback test values only for noncanonical local tests.
- [ ] Execute all 10 concrete negative controls before constructing `BootstrapNegativeControlManifest`.
- [ ] Assert Stage1 and Stage2 artifacts equal byte-for-byte and both validate identity equality/rejection semantics.
- [ ] Assert Universe generation digest before and after P12 store operations is identical.
- [ ] Print seed/T0/T1/T2/source/artifact/NC/proof identities for canonical workflow capture.
- [ ] Prove GREEN.
- [ ] Commit.

### Task 6: Independent final bootstrap proof replay

**Files:**
- Create: `crates/formula-check/src/bootstrap_verifier.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p12_bootstrap_verifier.rs`

**Produces:**

```rust
pub const P12_CANONICAL_MARKERS: [&str; 10];

pub fn verify_bootstrap_proof_manifest(
    manifest: &BootstrapProofManifest,
    replay: &BootstrapReplayEvidence,
) -> Result<VerifiedBootstrapProof, BootstrapProofFailure>;
```

Canonical markers:

```text
PASS P12_B0_SEED_EXPLICIT
PASS P12_BOOTSTRAP_CORE_DETERMINISTIC
PASS P12_GENERATOR_VALIDATOR_DIVERSE
PASS P12_STAGE1_INDEPENDENTLY_VALIDATED
PASS P12_STAGE2_SELF_REBUILD_VALIDATED
PASS P12_BYTE_EQUIVALENCE
PASS P12_SEMANTIC_EQUIVALENCE
PASS P12_UNIVERSE_AUTHORITY_UNCHANGED
PASS P12_NEGATIVE_CONTROLS
PASS BOOTSTRAP_TRUST_REDUCED
```

- [ ] Write RED verifier tests for exact replay success, manifest mutation, incomplete NC manifest, generator/validator identity equality, T-stage mismatch, artifact mismatch, and U-generation mutation.
- [ ] Prove RED.
- [ ] Implement checker-only replay verification; no store/realize/engine dependency.
- [ ] Integrate the canonical P12 test with final verifier so markers are emitted only from verified evidence.
- [ ] Prove GREEN.
- [ ] Commit.

### Task 7: Canonical P12 workflow, source proof and final freeze

**Files:**
- Create temporarily then remove: `.github/workflows/p12-development.yml`
- Create permanent: `.github/workflows/p12-canonical-proof.yml`
- Create: `docs/checkpoints/2026-09-06-p12-self-hosting-bootstrap-trust-reduction.md`
- Modify: `CURRENT.md`

- [ ] During Tasks 1-6 use read-only development CI pinned to Rust 1.98.0; no write-back steps.
- [ ] Remove development workflow before source proof.
- [ ] Create canonical workflow with `contents: read`.
- [ ] In canonical workflow compute:

```bash
sha256sum "$(command -v rustc)"
sha256sum "$(command -v cargo)"
sha256sum rust-toolchain.toml
```

and export the digest values to the canonical P12 test.
- [ ] Prime dependencies with `cargo fetch --locked`, then run all proof/regression/static gates with `CARGO_NET_OFFLINE=true` / `--offline`.
- [ ] Run canonical P12 integration + independent verifier + frozen P11 canonical predecessor proof.
- [ ] Run all crates, workspace tests/build, rustfmt, Clippy `-D warnings`, dependency/source firewalls and clean-tree.
- [ ] Record exact source SHA/run/job/toolchain/seed/T-stage/artifact/NC/proof identities.
- [ ] Update only P12 checkpoint + `CURRENT.md` to source-proved/docs-pending state.
- [ ] Prove source→docs delta is exactly those two files and workflow blob unchanged.
- [ ] Run unchanged canonical workflow on exact docs head and require identity stability except source-bound proof manifest.
- [ ] Record exact docs SHA/run/job as non-recursive recovery metadata.
- [ ] Mark `BOOTSTRAP_TRUST_REDUCED` final recovery authority.
- [ ] Immediately inspect P13 entry conditions; do not implement P13 unless roadmap evidence proves the explicit cluster/remoting threshold has been met.
