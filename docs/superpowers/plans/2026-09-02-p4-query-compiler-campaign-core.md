# P4 Query Compiler + Campaign Core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement frozen roadmap phase P4 so an exact query compiles deterministically into a typed, replayable, authority-inert AND/OR mathematical campaign and invalid route compositions fail closed.

**Architecture:** All P4 planning/compiler behavior lives in `formula-engine`, which consumes immutable semantic/authority snapshots from `formula-core`, `formula-store`, and `formula-packages`. P4 defines structure and validation only; it does not execute discovery, solvers, promotion, native realization, models, or Ptah. `formula-engine` must not depend on `formula-check`, and P4 code must not invoke authority publication/mutation APIs.

**Tech Stack:** Rust 1.98.0, existing workspace crates only, canonical encoding/digest primitives already in `formula-core`, locked/offline Cargo proof workflow.

**Spec:** `docs/superpowers/specs/2026-09-02-p4-query-compiler-campaign-core-design.md`

## Global Constraints

- Exact predecessor is frozen P3 head `5c15368440ad9cc387708dae3c3d73135009f053`.
- P4 implementation branch is `implementation/p4-query-compiler-campaign-core`.
- No P5 CandidateSpace/refinement/CEGIS/search algorithm enters P4.
- No new crate and no new external runtime dependency.
- `formula-engine -/-> formula-check` remains true.
- Compiler/WorkCell code may read immutable authority snapshots only; no publish/rollback/promotion/authority mutation API may be reachable from P4 surfaces.
- Every new semantic/planning identity must be deterministic under canonical ordering.
- `REFUTED`, `SEMANTIC_UNKNOWN`, and `RESOURCE_BOUNDED_UNKNOWN` remain distinct.
- Every task follows RED -> observed failure -> minimum GREEN -> exact-head proof -> commit.

---

## File Structure

Create focused files under `crates/formula-engine/src/`:

- `query.rs` — QueryIR, resource/side-effect contracts, metavariable/target bindings.
- `region.rs` — immutable CompilerAuthoritySnapshot and RelevantRegion projection.
- `theory_profile.rs` — certified profile facts vs non-authoritative operational estimates.
- `representation.rs` — RepresentationNode/Edge and preservation validation.
- `reduction.rs` — ReductionEdge, ResultClass and composition validation.
- `decomposition.rs` — Decomposition contract and reconstruction validation.
- `campaign.rs` — CampaignIR typed nodes/edges, AND/OR validation, deterministic digest.
- `obligation.rs` — ObligationIR and terminal states.
- `work_cell.rs` — authority-inert WorkCellPlan.
- `replay.rs` — deterministic ReplayManifest binding all semantic/policy inputs.
- `result_bundle.rs` — structural result envelope.
- `compiler.rs` — deterministic end-to-end CompilerV1.
- `lib.rs` — stable module exports only.

Tests live in `crates/formula-engine/tests/p4_*.rs`. Architecture-policy tests remain in `tests/authority-boundary/` or `formula-archtest` where they inspect dependency/source boundaries rather than P4 behavior.

---

### Task 1: QueryIR Exact Semantic Identity

**Files:**
- Create: `crates/formula-engine/src/query.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_query_ir.rs`

**Interfaces:**
- Produces `QueryIR`, `KnownBinding`, `Metavariable`, `TargetRequest`, `ResourceContract`, `SideEffectPolicy`.
- `QueryIR::canonical_bytes() -> Vec<u8>` and `QueryIR::digest() -> ArtifactDigest`.
- Consumes existing `ArtifactDigest`, `World`, `AuthorityContract`, `Observer` identities from `formula-core` and activated package identity from P3.

- [ ] **Step 1: Write failing tests** proving two identical semantic queries produce the same digest; changing generation, World, Observer, AuthorityContract, known binding, target, package set, resource contract, or side-effect policy changes the digest.
- [ ] **Step 2: Run** `cargo test -p formula-engine --test p4_query_ir --locked --offline` and observe unresolved `formula_engine::query`/types.
- [ ] **Step 3: Implement minimum immutable QueryIR** using sorted/deduplicated set-like fields and existing canonical encoding helpers.
- [ ] **Step 4: Re-run the focused test and `cargo test -p formula-engine --all-targets --locked --offline`**; both must pass.
- [ ] **Step 5: Commit** `feat(p4): add exact QueryIR semantics`.

### Task 2: CompilerAuthoritySnapshot + RelevantRegion + TheoryProfile

**Files:**
- Create: `crates/formula-engine/src/region.rs`
- Create: `crates/formula-engine/src/theory_profile.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_region_profile.rs`

**Interfaces:**
- `CompilerAuthoritySnapshot { generation, world, admitted_artifacts, admitted_capabilities, admitted_morphisms, package_context }`.
- `RelevantRegion::from_snapshot(query: &QueryIR, snapshot: &CompilerAuthoritySnapshot) -> Result<RelevantRegion, RegionError>`.
- `TheoryProfile::compile(region: &RelevantRegion, certified_properties: &[ProfileFact], estimates: &[OperationalEstimate]) -> TheoryProfile`.

- [ ] **Step 1:** Test exact generation/world/package mismatch rejection and deterministic region/profile digest.
- [ ] **Step 2:** Test that `OperationalEstimate` cannot appear in `exact_properties` and cannot satisfy a certified property query.
- [ ] **Step 3:** Run focused test and observe missing modules/types.
- [ ] **Step 4:** Implement bounded deterministic projection; no heuristic retrieval/ranking.
- [ ] **Step 5:** Run focused + engine all-target tests and commit `feat(p4): add relevant region and theory profile`.

### Task 3: Representation Contracts

**Files:**
- Create: `crates/formula-engine/src/representation.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_representation.rs`

**Interfaces:**
- `ExactnessClass::{Exact, SoundOverApproximation, SoundUnderApproximation, HeuristicProposal}`.
- `InformationLoss::{None, Declared}`.
- `RepresentationEdge::validate(requested: ResultClass) -> Result<(), RepresentationError>`.

- [ ] **Step 1:** RED tests reject missing preservation metadata, implicit lossy conversion for exact witness requests, and mismatched World/Observer bindings.
- [ ] **Step 2:** Test that a lossless exact edge with explicit certificate/reconstruction route validates.
- [ ] **Step 3:** Implement minimum node/edge structures and fail-closed validation.
- [ ] **Step 4:** Run tests and commit `feat(p4): add explicit representation contracts`.

### Task 4: Reduction Contracts + Composition

**Files:**
- Create: `crates/formula-engine/src/reduction.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_reduction.rs`

**Interfaces:**
- `ResultClass::{Decision, Witness, Count, Optimum, Bound}`.
- `ReductionEdge` with preserved result classes, encode relation, decode/reconstruct relation, assumptions and evidence reference.
- `compose_reduction_path(path: &[ReductionEdge], requested: ResultClass) -> Result<ComposedReduction, ReductionError>`.

- [ ] **Step 1:** RED tests reject decision-only route for Witness/Count/Optimum and reject Witness route without reconstruction.
- [ ] **Step 2:** Test full-path preservation: one weak edge invalidates the composed class even if every other edge is strong.
- [ ] **Step 3:** Implement intersection-based preservation plus explicit reconstruction checks.
- [ ] **Step 4:** Run tests and commit `feat(p4): validate reduction result classes`.

### Task 5: Decomposition Contracts

**Files:**
- Create: `crates/formula-engine/src/decomposition.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_decomposition.rs`

**Interfaces:**
- `AggregationSemantics::{And, Or}` for P4.
- `Decomposition::validate() -> Result<(), DecompositionError>`.

- [ ] **Step 1:** RED tests reject empty child set, missing reconstruction relation, missing aggregation semantics, and parent/child World mismatch.
- [ ] **Step 2:** Implement immutable contract validation only; no decomposition search.
- [ ] **Step 3:** Run tests and commit `feat(p4): add explicit decomposition contracts`.

### Task 6: CampaignIR AND/OR Graph

**Files:**
- Create: `crates/formula-engine/src/campaign.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_campaign.rs`

**Interfaces:**
- `CampaignNode::{Goal, Route, Obligation, WorldRef, ArtifactRef, FactRef, WorkCellPlanRef, ResultRef}`.
- `CampaignEdgeKind::{Requires, Produces, Satisfies, Refutes, AlternativeTo, DecomposesInto, ReducesTo, TransportsTo, Unlocks}`.
- `CampaignAggregation::{And, Or}`.
- `CampaignIR::validate()`, `canonical_bytes()`, `digest()`.

- [ ] **Step 1:** RED tests deterministic identity under insertion-order variation and reject dangling references, illegal aggregation, mismatched generation/world, or route nodes without obligations.
- [ ] **Step 2:** Implement deterministic node/edge ordering and structural validation.
- [ ] **Step 3:** Run tests and commit `feat(p4): add deterministic AND OR campaign IR`.

### Task 7: ObligationIR + Terminal-State Separation

**Files:**
- Create: `crates/formula-engine/src/obligation.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_obligation.rs`

**Interfaces:**
- `TerminalState::{Satisfied, Refuted, CertifiedBound, SemanticUnknown, ResourceBoundedUnknown, UndecidableGeneralClass, Superseded, BlockedByAuthority}`.
- `ObligationIR::digest()`.

- [ ] **Step 1:** RED tests prove `Refuted != SemanticUnknown != ResourceBoundedUnknown`, including serialization/digest distinction.
- [ ] **Step 2:** Test resource exhaustion can produce only `ResourceBoundedUnknown`, never `Refuted` through the P4 helper API.
- [ ] **Step 3:** Implement immutable obligation identity and typed terminal transition helper.
- [ ] **Step 4:** Run tests and commit `feat(p4): add obligation IR terminal semantics`.

### Task 8: Authority-Inert WorkCellPlan

**Files:**
- Create: `crates/formula-engine/src/work_cell.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_work_cell.rs`
- Test/Modify: authority-boundary architecture source/dependency test that inspects P4 modules.

**Interfaces:**
- `WorkCellPlan` fields exactly from spec.
- No store mutation handle/API in constructor or fields.

- [ ] **Step 1:** RED behavioral tests bind exact obligation, packages, capabilities, authority requirement, budget, side effects, replay key and stop conditions.
- [ ] **Step 2:** RED architecture test fails if P4 modules reference `publish_generation`, authority rollback/update methods, PromotionCandidate admission APIs, or `formula_check` implementation.
- [ ] **Step 3:** Implement WorkCellPlan and read-only snapshot access only.
- [ ] **Step 4:** Run `formula-engine` + `formula-archtest` tests and commit `feat(p4): enforce authority inert work cells`.

### Task 9: ReplayManifest + ResultBundle

**Files:**
- Create: `crates/formula-engine/src/replay.rs`
- Create: `crates/formula-engine/src/result_bundle.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_replay_result.rs`

**Interfaces:**
- `ReplayManifest::digest()` binds generation, World, query, packages, region, profile, compiler/scheduler policy, resource contract, random key and campaign digest.
- `ResultBundle` is structural and authority-inert.

- [ ] **Step 1:** RED tests identical exact inputs -> same replay digest.
- [ ] **Step 2:** Test changing each semantic/policy field independently changes replay identity.
- [ ] **Step 3:** Test creating ResultBundle with evidence refs cannot mutate/create Evidence or authority state.
- [ ] **Step 4:** Implement and commit `feat(p4): bind deterministic campaign replay`.

### Task 10: CompilerV1 End-to-End

**Files:**
- Create: `crates/formula-engine/src/compiler.rs`
- Modify: `crates/formula-engine/src/lib.rs`
- Test: `crates/formula-engine/tests/p4_compiler.rs`

**Interfaces:**
- `CompilerV1::compile(query: &QueryIR, snapshot: &CompilerAuthoritySnapshot, inputs: CompilerInputs) -> Result<CompiledCampaign, CompilerError>`.
- `CompiledCampaign { region, theory_profile, campaign, obligations, work_cells, replay_manifest }`.

- [ ] **Step 1:** RED fixture compiles one exact deterministic query into an AND/OR campaign and recompile produces byte-identical campaign/replay digests.
- [ ] **Step 2:** RED invalid fixtures cover World/generation/Authority/Observer/package mismatch, lossy implicit morphism, ambiguous parent, invalid representation, reduction class loss, and invalid decomposition.
- [ ] **Step 3:** Implement deterministic orchestration only; no CandidateSpace/discovery execution.
- [ ] **Step 4:** Run focused + engine all-target tests and commit `feat(p4): compile deterministic mathematical campaigns`.

### Task 11: Adversarial P4 Integration Gate

**Files:**
- Create: `crates/formula-engine/tests/p4_adversarial.rs`
- Modify only production code if a demonstrated defect exists.

- [ ] **Step 1:** Add adversarial cases attempting: resource timeout -> Refuted; operational estimate -> certified profile fact; lossy morphism implicit elaboration; decision reduction -> witness result; decomposition without reconstruction; WorkCell authority mutation capability; replay with omitted policy field.
- [ ] **Step 2:** Run tests. Existing production should reject all cases; if any passes, enter systematic-debugging and add the minimum correction.
- [ ] **Step 3:** Re-run all P4 tests and commit `test(p4): prove compiler authority and semantic boundaries`.

### Task 12: Canonical P4 Proof, Review, Freeze

**Files:**
- Create: `.github/workflows/p4-canonical-proof.yml`
- Remove any temporary P4 development workflow before source freeze.
- Create after source proof: `docs/checkpoints/2026-09-02-p4-query-compiler-campaign-core.md`
- Update after source proof: `CURRENT.md`

**Interfaces:**
- Canonical workflow must be read-only (`contents: read`) and branch-scoped to P4.

- [ ] **Step 1:** Install pinned Rust 1.98.0; prime locked dependencies only.
- [ ] **Step 2:** Run locked/offline metadata; architecture/core/store/check/packages/engine tests; whole workspace tests/build; `cargo fmt --all -- --check`; Clippy `-D warnings`; relevant `cargo tree`; clean-worktree check.
- [ ] **Step 3:** Emit P4 markers:
  - `P4-01 QueryIR exact semantics preserved`
  - `P4-02 no lossy implicit morphism`
  - `P4-03 representation preservation metadata enforced`
  - `P4-04 reduction result classes preserved`
  - `P4-05 decomposition reconstruction explicit`
  - `P4-06 CampaignIR deterministic AND/OR`
  - `P4-07 terminal states remain distinct`
  - `P4-08 WorkCells authority-inert`
  - `P4-09 replay manifest complete/deterministic`
  - `P4-10 P0-P3 gates preserved`
- [ ] **Step 4:** Require exact source SHA canonical SUCCESS.
- [ ] **Step 5:** Compare exact frozen P3 head `5c15368440ad9cc387708dae3c3d73135009f053` to P4 source head; review scope/authority using requesting-code-review discipline.
- [ ] **Step 6:** Write P4 checkpoint and update `CURRENT.md` without changing implementation.
- [ ] **Step 7:** Require a fresh canonical P4 SUCCESS on the exact documentation-bearing head.
- [ ] **Step 8:** Only then declare P4 frozen and branch P5.
