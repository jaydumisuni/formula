# P9 Canonical First-Light Proof Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prove the frozen First-Light loop end-to-end, including U1 capability expansion, a second query that reuses the promoted FL-C primitive with zero primitive rediscovery, and one independently replayable manifest that earns all fifteen canonical PASS markers.

**Architecture:** Preserve all P0-P8 discovery/promotion/realization paths. Add durable semantic activation to `AuthorityStore`, derive the activated primitive into U1 capability closure, compile the second query through a separate reuse-only D3 path that creates no discovery CandidateSpace or WorkCell, dispatch the already-admitted P8 native realization, then bind the complete run into a core proof manifest verified by an independent checker path.

**Tech Stack:** Rust 1.98.0, Cargo locked workspace, SQLite/rusqlite authority store, SHA-256 `ArtifactDigest`, existing formula-core/check/engine/packages/store/realize/first-light crates, GitHub Actions Ubuntu 24.04 canonical proof.

**Spec:** `docs/superpowers/specs/2026-09-04-p9-canonical-first-light-proof-design.md`

## Global Constraints

- Exact P8 recovery predecessor: `02af51ded5cbc8732017b57300f79e7fbe8cc80c`.
- Frozen P8 proof head remains `fa369b6241c0c069176e5939acf4d5ec74eb8085`; P9 may consume but never rewrite P8 authority.
- QueryIR schema v1 and ordinary `CompilerV1::compile` behavior remain unchanged.
- Canonical P9 execution is local CPU only, model free, network free during proof, Ptah free, GPU free.
- Reuse must create zero FL-C primitive-discovery CandidateSpaces and zero primitive-discovery Work Cells.
- Canonical second query uses `RequestedResultClass::Count` over a content-addressed U8 vector.
- Second-query reuse must select the already-admitted P8 realization; it must not regenerate source, invoke rustc, reauthorize, or readmit a realization.
- Final markers are emitted only after independent replay/verification of the full manifest.
- P10 generalized self-expansion machinery is excluded.

---

### Task 1: Durable semantic activation and U1 capability closure

**Files:**
- Create: `crates/formula-store/src/authority_store/activation_store.rs`
- Modify: `crates/formula-store/src/authority_store.rs`
- Modify: `crates/formula-packages/src/closure.rs`
- Test: `crates/formula-store/tests/p9_semantic_activation_store.rs`
- Test: `crates/formula-packages/tests/p9_semantic_capability_closure.rs`

**Interfaces:**
- Consumes: `PromotionRecord`, `PromotionState::Activated`, `UniverseGeneration`, existing authority SQLite state.
- Produces: `AuthorityStore::admit_semantic_activation(&PromotionRecord, ArtifactDigest)`, `AuthorityStore::resolve_semantic_activation(ArtifactDigest, ArtifactDigest)`, and `derive_capabilities_with_semantic_activations(...)`.

- [ ] **Step 1: Write the activation-store failing tests**

```rust
#[test]
fn activated_semantic_primitive_is_persisted_only_for_active_admitted_generation() {
    // Build U0 -> U1 through the existing checked promotion path.
    // Construct the exact ACTIVATED record from that promotion outcome.
    // Assert admit_semantic_activation(record, primitive) succeeds.
    // Re-open the store and assert resolve_semantic_activation(U1, primitive)
    // returns the same PromotionRecord structural digest.
}

#[test]
fn activation_rejects_non_activated_wrong_generation_unadmitted_or_unbound_evidence() {
    // Assert typed fail-closed errors for each invalid binding.
}
```

- [ ] **Step 2: Run the store tests and prove RED**

Run:

```bash
cargo test -p formula-store --test p9_semantic_activation_store --locked
```

Expected: compile failure because `admit_semantic_activation` / `resolve_semantic_activation` and activation error variants do not exist.

- [ ] **Step 3: Implement the private durable activation registry**

Add `mod activation_store;` and create a private SQLite registry using the existing `AuthorityStore` connection. The public API is:

```rust
pub fn admit_semantic_activation(
    &mut self,
    record: &PromotionRecord,
    primitive: ArtifactDigest,
) -> Result<PromotionRecord, AuthorityStoreError>;

pub fn resolve_semantic_activation(
    &self,
    generation: ArtifactDigest,
    primitive: ArtifactDigest,
) -> Result<Option<PromotionRecord>, AuthorityStoreError>;
```

Admission checks in this exact order:

```rust
record.state() == PromotionState::Activated
self.active_generation()? == Some(record.generation())
self.replay_generation(record.generation())?.admitted().contains(&primitive)
record.semantic_artifacts().contains(&primitive)
record.evidence().iter().all(|e| generation.authority_bindings().contains(e))
```

Persist enough canonical fields to reconstruct the record exactly and compare its reconstructed `structural_digest()` with the stored activation digest before returning it.

Add typed `AuthorityStoreError` variants:

```rust
SemanticActivationStateMismatch,
SemanticActivationGenerationMismatch { expected: ArtifactDigest, actual: ArtifactDigest },
SemanticActivationPrimitiveNotAdmitted(ArtifactDigest),
SemanticActivationEvidenceNotAuthorityBound(ArtifactDigest),
SemanticActivationDigestMismatch { stored: ArtifactDigest, reconstructed: ArtifactDigest },
```

- [ ] **Step 4: Run the store tests and prove GREEN**

Run the Task 1 store test plus all formula-store tests.

- [ ] **Step 5: Write capability-closure RED tests**

```rust
#[test]
fn u1_activation_derives_promoted_primitive_as_capability_but_u0_does_not() {
    // Resolve the durable ACTIVATED record from AuthorityStore.
    // derive_capabilities_with_semantic_activations(U0, []) excludes primitive.
    // derive_capabilities_with_semantic_activations(U1, [record]) includes primitive.
}

#[test]
fn missing_or_invalid_activation_does_not_create_reuse_capability() {
    // ADMITTED alone is not enough; only exact ACTIVATED state contributes.
}
```

- [ ] **Step 6: Implement activation-aware closure without changing legacy closure behavior**

Keep `derive_capabilities(...)` unchanged. Add:

```rust
pub fn derive_capabilities_with_semantic_activations(
    context: &ClosureContext,
    activated: &ActivatedPackageSet,
    witnesses: &[AdmittedStructureWitness],
    packages: &[TheoryPackageManifest],
    generation: &UniverseGeneration,
    activations: &[PromotionRecord],
) -> Result<CapabilityClosure, ClosureError>
```

It first calls existing `derive_capabilities`, then validates each activation against `generation.digest()`, `PromotionState::Activated`, admitted primitive membership, and authority-bound evidence before inserting each activated semantic artifact into the returned capability set.

Add closure errors:

```rust
SemanticActivationGenerationMismatch,
SemanticActivationStateMismatch,
SemanticActivationPrimitiveNotAdmitted,
SemanticActivationEvidenceNotAuthorityBound,
```

- [ ] **Step 7: Run Task 1 full gate and commit**

Run formula-store, formula-packages, workspace build, rustfmt, and Clippy `-D warnings`. Commit only Task 1 files.

---

### Task 2: Explicit D3 reuse compilation with structural zero-rediscovery proof

**Files:**
- Create: `crates/formula-engine/src/reuse.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Modify: `crates/formula-engine/src/compiler.rs`
- Modify: `crates/formula-engine/src/campaign.rs`
- Test: `crates/formula-engine/tests/p9_reuse_compiler.rs`

**Interfaces:**
- Consumes: `QueryIR`, `CompilerAuthoritySnapshot`, `CompilerInputs`, `RelevantRegion` capability set.
- Produces: `ReuseRequest`, `ResolvedCapability`, `ReuseExecutionPlan`, `ReuseMetrics`, `CompiledReuseCampaign`, `CompilerV1::compile_reuse`.

- [ ] **Step 1: Write the reuse compiler RED tests**

```rust
#[test]
fn u1_reuse_compiles_without_candidate_space_or_discovery_work_cell() {
    let request = ReuseRequest::new(&query, primitive);
    let compiled = CompilerV1::compile_reuse(&query, &snapshot_with_primitive, inputs, &request).unwrap();
    assert_eq!(compiled.resolved_capability().primitive(), primitive);
    assert!(compiled.work_cells().is_empty());
    assert_eq!(compiled.execution_plans().len(), 1);
    assert_eq!(compiled.metrics().primitive_discovery_candidate_spaces(), 0);
    assert_eq!(compiled.metrics().primitive_discovery_work_cells(), 0);
}

#[test]
fn reuse_fails_when_capability_is_missing_or_context_changes() {
    // U0/missing capability, wrong generation, world, observer, or authority all reject.
}
```

- [ ] **Step 2: Run the test and prove RED**

Expected: unresolved `formula_engine::reuse` and missing `CompilerV1::compile_reuse`.

- [ ] **Step 3: Implement canonical reuse identities**

In `reuse.rs` implement deterministic canonical artifacts:

```rust
pub struct ReuseRequest {
    query_digest: ArtifactDigest,
    required_semantic_capability: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
    result_class: RequestedResultClass,
}

pub struct ResolvedCapability {
    reuse_request: ArtifactDigest,
    primitive: ArtifactDigest,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

pub struct ReuseExecutionPlan {
    query: ArtifactDigest,
    resolved_capability: ArtifactDigest,
    primitive: ArtifactDigest,
    result_class: RequestedResultClass,
    universe_generation: ArtifactDigest,
    world: ArtifactDigest,
    authority_contract: ArtifactDigest,
    observer: ArtifactDigest,
}

pub struct ReuseMetrics {
    primitive_discovery_candidate_spaces: u64,
    primitive_discovery_work_cells: u64,
    resolved_capability_count: u64,
    execution_work_items: u64,
}
```

Each exposes read-only getters and deterministic `digest()`/canonical identity. `ReuseRequest::new(&QueryIR, primitive)` copies exact context from the query rather than accepting separate spoofable context arguments.

- [ ] **Step 4: Add an execution-plan campaign node kind without changing old digests**

Extend `CampaignNodeKind` with `ExecutionPlanRef` serialized as `EXECUTION_PLAN_REF`. Existing variants/serialization remain byte-identical.

- [ ] **Step 5: Implement `CompilerV1::compile_reuse`**

Signature:

```rust
pub fn compile_reuse(
    query: &QueryIR,
    snapshot: &CompilerAuthoritySnapshot,
    inputs: CompilerInputs,
    request: &ReuseRequest,
) -> Result<CompiledReuseCampaign, CompilerError>
```

Add `CompilerError::ReuseRequestMismatch` and `CompilerError::RequiredCapabilityUnavailable`.

The method validates request/query identity, observer/authority, and `RelevantRegion::from_snapshot`; compiles the `TheoryProfile`; requires `region.admitted_capabilities().contains(&request.required_semantic_capability())`; then builds a campaign containing Goal, ArtifactRef(promoted primitive), Obligation, and ExecutionPlanRef nodes. It returns `work_cells = []`, one `ReuseExecutionPlan`, and internally constructed metrics exactly `(0,0,1,1)`.

It must not import or call `candidate_space`, `observational`, `discovery`, or FL-C sealed code.

- [ ] **Step 6: Prove Task 2 GREEN and commit**

Run targeted engine tests, all formula-engine tests, authority-boundary tests, workspace build, rustfmt, and Clippy.

---

### Task 3: Canonical second-query input and admitted native realization reuse

**Files:**
- Create: `crates/formula-first-light/src/reuse.rs`
- Modify: `crates/formula-first-light/src/lib.rs`
- Modify: `crates/formula-first-light/Cargo.toml`
- Test: `crates/formula-first-light/tests/p9_second_query_reuse.rs`
- Lockfile: `Cargo.lock` only if the new dev dependency changes the package entry.

**Interfaces:**
- Consumes: P7 promotion, durable activation, activation-aware closure, `CompilerV1::compile_reuse`, P8 admitted realization dispatch.
- Produces: canonical second-query vector/result artifacts and end-to-end `SecondQueryReuseProof` evidence for Task 5.

- [ ] **Step 1: Add a canonical second-query data artifact**

In `reuse.rs` define:

```rust
pub struct CanonicalU8Vector {
    values: Vec<u8>,
}

pub struct SecondQueryResult {
    input_digest: ArtifactDigest,
    primitive: ArtifactDigest,
    realization: ArtifactDigest,
    matching_count: u64,
}
```

Use a fixed public vector containing power-of-two, zero, adjacent non-powers, and duplicates, e.g.:

```rust
[0, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 129, 255, 1, 3]
```

The expected count is derived by admitted semantics/checker logic in the test, never embedded as an authority artifact.

- [ ] **Step 2: Write the end-to-end reuse RED test**

The test must reproduce the existing P7/P8 path to create U1 and admit the native realization, then persist the exact ACTIVATED record and derive U1 capabilities. It creates a `QueryIR` with `RequestedResultClass::Count`, compiles via `compile_reuse`, and asserts:

```rust
compiled.work_cells().is_empty()
compiled.metrics().primitive_discovery_candidate_spaces() == 0
compiled.metrics().primitive_discovery_work_cells() == 0
store.resolve_realization(&dispatch).unwrap().is_some()
```

Before activation, the same reuse request must fail with `RequiredCapabilityUnavailable`.

- [ ] **Step 3: Add `formula-packages` only as a dev-dependency to `formula-first-light`**

Production dependencies remain exactly `formula-core + formula-engine`; P9 orchestration dependencies remain dev-only.

- [ ] **Step 4: Execute the already-admitted binary, never rebuild it**

The test retrieves P8 admitted bytes through `resolve_realization`, writes those verified bytes to a temporary executable path, preserves executable permission on Unix, and executes it once per vector input. It must not call `generate_u8_bool_rust_source`, `rustc`, `authorize_native_u8_realization_v1`, or `admit_realization` after the reuse query begins.

Construct `SecondQueryResult` from the counted outputs and independently compare the count against evaluation of the already-admitted semantic expression over the same vector.

- [ ] **Step 5: Add negative reuse cases**

Prove U0/missing activation, wrong observer/authority/world, missing realization, and tampered admitted binary cannot produce canonical reuse success.

- [ ] **Step 6: Prove Task 3 GREEN and commit**

Run the targeted First-Light test, all First-Light tests, workspace build, rustfmt, Clippy, and architecture dependency checks.

---

### Task 4: Canonical P9 proof and negative-control manifest schemas

**Files:**
- Create: `crates/formula-core/src/first_light.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Test: `crates/formula-core/tests/p9_first_light_manifest.rs`

**Interfaces:**
- Consumes: content-addressed digests from P0-P9 evidence.
- Produces: `FirstLightTargetEvidence`, `FirstLightReuseEvidence`, `NegativeControlEvidence`, `NegativeControlManifest`, `FirstLightProofManifest`.

- [ ] **Step 1: Write canonical identity RED tests**

Tests prove deterministic ordering/deduplication where inputs are set-like, mutation of any authority-bearing digest changes manifest digest, and non-semantic runtime metadata has no field in the schema.

- [ ] **Step 2: Implement negative-control identities**

```rust
pub enum NegativeControlId {
    ModifiedSealedTarget,
    SealedImportAttempt,
    FlASampleNearMiss,
    FlBCorruptedTranslation,
    FlCZeroNearMiss,
    ForgedEvidence,
    CandidateMutationAfterCertificate,
    SearchAuthorityWrite,
    MutatedRealizationBinary,
    ActivationRemoved,
    StricterAuthorityWithoutEvidence,
    PromotionParentRace,
}

pub struct NegativeControlEvidence {
    id: NegativeControlId,
    evidence_digest: ArtifactDigest,
}

pub struct NegativeControlManifest {
    controls: Vec<NegativeControlEvidence>,
}
```

The manifest constructor sorts by ID and rejects duplicates/missing canonical IDs through `NegativeControlManifest::complete(...) -> Result<Self, FirstLightManifestError>`.

- [ ] **Step 3: Implement target/reuse evidence sections**

```rust
pub struct FirstLightTargetEvidence {
    query: ArtifactDigest,
    campaign: ArtifactDigest,
    candidate: ArtifactDigest,
    certification: ArtifactDigest,
    auxiliary: Vec<ArtifactDigest>,
}

pub struct FirstLightReuseEvidence {
    query: ArtifactDigest,
    campaign: ArtifactDigest,
    resolved_capability: ArtifactDigest,
    execution_plan: ArtifactDigest,
    result: ArtifactDigest,
    reuse_metrics: ArtifactDigest,
    realization: ArtifactDigest,
}
```

- [ ] **Step 4: Implement `FirstLightProofManifest`**

Bind source commit string plus exact U0/U1/world/package, FL-A/B/C sections, promotion/closure/native digests, reuse section, negative-control manifest digest, and verifier/checker identity. The constructor requires every section; there is no partial/default constructor.

- [ ] **Step 5: Prove Task 4 GREEN and commit**

Run formula-core tests, workspace build, rustfmt, and Clippy.

---

### Task 5: Independent First-Light verifier and complete marker set

**Files:**
- Create: `crates/formula-check/src/first_light.rs`
- Modify: `crates/formula-check/src/lib.rs`
- Test: `crates/formula-check/tests/p9_first_light_verifier.rs`

**Interfaces:**
- Consumes: core `FirstLightProofManifest` plus independently reconstructed/replayed evidence summary.
- Produces: opaque `FirstLightVerification` and ordered canonical marker strings only on complete success.

- [ ] **Step 1: Define verifier inputs as checked/replayed facts, not search claims**

```rust
pub struct FirstLightReplayEvidence {
    pub source_commit: String,
    pub u0_digest: ArtifactDigest,
    pub u1_digest: ArtifactDigest,
    pub u1_parent: ArtifactDigest,
    pub fl_a: FirstLightTargetEvidence,
    pub fl_b: FirstLightTargetEvidence,
    pub fl_c: FirstLightTargetEvidence,
    pub promotion_digest: ArtifactDigest,
    pub closure_before: ArtifactDigest,
    pub closure_after: ArtifactDigest,
    pub closure_delta: ArtifactDigest,
    pub native_realization: ArtifactDigest,
    pub reuse: FirstLightReuseEvidence,
    pub reuse_candidate_spaces: u64,
    pub reuse_discovery_work_cells: u64,
    pub reuse_result_exact: bool,
    pub negative_controls: NegativeControlManifest,
}
```

This evidence is assembled by the independent replay integration after it re-runs existing checker/store functions; the verifier does not import `formula-engine`, `formula-store`, or `formula-first-light`.

- [ ] **Step 2: Write RED tests for complete and incomplete evidence**

Mutation/missing evidence, U1 parent mismatch, nonzero rediscovery metrics, wrong reuse result, or incomplete negative controls must return typed `FirstLightVerificationError` and no marker set.

- [ ] **Step 3: Implement opaque successful verification**

`verify_first_light_manifest_v1(&FirstLightProofManifest, &FirstLightReplayEvidence)` checks exact manifest/evidence digest equality, U1 parent == U0, complete negative controls, and zero rediscovery metrics before constructing private `FirstLightVerification`.

`FirstLightVerification::markers()` returns exactly, in frozen order:

```text
PASS D1_AUTHORITY_SEPARATION
PASS D2_IDENTITY_GENERATION_REPLAY
PASS D2_CERTIFICATE_ROUTING
PASS D2_SEARCH_STATE_SEPARATION
PASS D3_BLIND_SEMANTIC_ELABORATION
PASS D3_REPRESENTATION_REDUCTION
PASS D3_SYMBOLIC_CANDIDATE_SPACE
PASS D3_FALSE_NEARMISS_REJECTION
PASS D4_NATIVE_REALIZATION_EQUIVALENCE
PASS D4_CPU_LOCAL_OFFLINE
PASS D5_ATOMIC_PROMOTION
PASS D5_CAPABILITY_CLOSURE_EXPANDED
PASS D5_SECOND_QUERY_REUSE
PASS NEGATIVE_CONTROLS
PASS FIRST_LIGHT_COMPLETE
```

No public constructor exists for `FirstLightVerification`.

- [ ] **Step 4: Prove checker authority firewall**

Update/add authority-boundary tests so `formula-check` still depends only on `formula-core` and cannot import engine/search/store/sealed First-Light implementation.

- [ ] **Step 5: Prove Task 5 GREEN and commit**

Run formula-check tests, authority-boundary tests, workspace build, rustfmt, and Clippy.

---

### Task 6: Canonical clean-state First-Light assembly, replay, proof workflow, and freeze

**Files:**
- Create: `crates/formula-first-light/tests/p9_canonical_first_light.rs`
- Create: `.github/workflows/p9-canonical-proof.yml`
- Create: `docs/checkpoints/2026-09-04-p9-canonical-first-light-proof.md`
- Modify: `CURRENT.md`

**Interfaces:**
- Consumes: all P0-P9 APIs and predecessor tests.
- Produces: exact canonical P9 proof run, proof-manifest digest, complete marker set, frozen recovery checkpoint.

- [ ] **Step 1: Build one clean-state integration test following the frozen sequence**

The test must execute in order:

```text
create/load U0
FL-A blind discovery -> independent certify
FL-B representation/reduction -> independent certify
FL-C blind synthesis
mandatory zero-accepting near-miss rejection
freeze/certify FL-C
promote -> U1
persist ACTIVATED semantic primitive
prove capability closure expansion
compile + independently validate native realization
submit canonical second COUNT query under U1
compile_reuse -> zero candidate spaces/work cells
resolve and execute already-admitted native realization
verify exact second-query result
run/bind all 12 negative controls
assemble FirstLightProofManifest
replay U0/U1 + evidence independently
verify_first_light_manifest_v1
assert all fifteen ordered markers
```

- [ ] **Step 2: Prove the integration test RED before any missing Task 6 glue is added**

Run only `p9_canonical_first_light` and confirm failure is specific to missing assembly/replay glue, not predecessor regression.

- [ ] **Step 3: Complete the minimum assembly/replay glue**

Reuse existing FL-A/B/C tests/helpers/checkers rather than reimplementing search algorithms. Any helper promoted into `formula-first-light/src` must remain sealed-harness code and must not create a dependency from engine/checker production crates back into sealed fixtures.

- [ ] **Step 4: Create the read-only canonical workflow**

`.github/workflows/p9-canonical-proof.yml` uses:

```yaml
permissions:
  contents: read
runs-on: ubuntu-24.04
```

It installs Rust 1.98.0, primes locked dependencies, then runs targeted P9 gates, all predecessor crate/workspace tests, build, `cargo fmt --all -- --check`, Clippy `--all-targets -- -D warnings`, normal dependency trees, authority dependency firewall, and clean-worktree check.

Each canonical PASS marker is printed only after the test/check that earns it succeeds. `PASS FIRST_LIGHT_COMPLETE` is printed last.

- [ ] **Step 5: Run canonical source proof on an exact source head**

Record exact source SHA, workflow run ID, job ID, toolchain identity, proof-manifest digest, U0/U1 digests, and all marker evidence.

- [ ] **Step 6: Record checkpoint and CURRENT as docs-only candidate**

Checkpoint must state exact predecessor P8 head/run, exact P9 source proof, reviewed file delta, manifest digest, U0/U1, negative-control manifest, markers, exclusions, and P10 as next boundary. `CURRENT.md` must state source proved but docs-head proof pending until the final unchanged run succeeds.

- [ ] **Step 7: Run the unchanged canonical workflow on the documentation-bearing head**

Only after that exact docs head succeeds may P9 be labeled `FINALLY FROZEN` and `FIRST_LIGHT_COMPLETE` be treated as recovered authority.

- [ ] **Step 8: Final structural review**

Compare P8 recovery predecessor to frozen P9 source/docs boundaries. Confirm no temporary development workflow/helper remains, no P10 implementation leaked in, and the final recovery metadata names the exact successful canonical run/job.
