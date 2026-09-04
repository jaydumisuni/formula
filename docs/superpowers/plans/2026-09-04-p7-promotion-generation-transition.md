# P7 Promotion + Generation Transition Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the frozen D5 P7 boundary so only independently checked, frozen candidates can advance immutable authority from `U_g` to `U_(g+1)`, with explicit lifecycle/quarantine/supersession/freshness semantics and derived capability-closure delta.

**Architecture:** Extend the existing P1/P2/P3 authority path instead of creating a new trust root. `formula-core` owns immutable promotion/lifecycle artifacts; `formula-check` is the sole constructor of an opaque `PromotionAuthorization`; `formula-store` consumes that authorization to execute the already-proved atomic generation transaction; `formula-packages` computes closure deltas from admitted inputs. Search and sealed-target code receive no store publication handle.

**Tech Stack:** Rust 1.98.0, existing canonical SHA-256 structural identity, SQLite-backed `formula-store`, existing `formula-check` exact promotion-manifest checker, existing `formula-packages` generation-scoped closure.

**Spec:** `docs/roadmap/2026-08-28-implementation-roadmap.md` P7 + frozen `docs/design/2026-08-28-d5-self-expansion-architecture.md`

## Global Constraints

- Start from exact frozen P6 documentation-bearing SHA `035953854f33fe47dc884850dec4fdee7a3571e7`.
- Preserve `U_g` history byte-for-byte; successful promotion creates a new generation and never mutates the parent.
- Search/compiler/sealed-target code cannot publish authority.
- Candidate identity must be frozen before certification/promotion.
- `CERTIFIED`, `ADMITTED`, `ACTIVATED`, and `QUARANTINED` remain structurally distinct states.
- Proof freshness is generation-bound; dependency-cone entries must already be admitted by the parent generation.
- Proposed authority bindings must be exactly independently checked evidence for this promotion.
- Supersession is lineage, not destructive deletion; older admitted artifacts remain replayable.
- Capability closure is derived from the resulting admitted generation; closure data is never authority itself.
- Failed, raced, quarantined, mismatched, or partial promotion cannot expose a new active generation.
- No P8 native compilation or P9 end-to-end First-Light execution in this phase.
- Canonical proof is read-only, Rust 1.98.0, locked/offline after dependency priming.

---

## File Structure

```text
crates/formula-core/src/promotion.rs
    Durable canonical P7 promotion candidate/state/quarantine/lineage artifacts.

crates/formula-core/src/lib.rs
    Export promotion vocabulary.

crates/formula-check/src/promotion.rs
    Extend existing structural manifest checker with PromotionPolicyV1 and opaque PromotionAuthorization.

crates/formula-check/tests/p7_promotion_policy.rs
    Exact policy/freshness/dependency/evidence/quarantine tests.

crates/formula-store/Cargo.toml
    Add formula-check dependency so the authority store accepts checker-issued authorization directly.

crates/formula-store/src/authority_store.rs
    Make raw generation publication internal and add authorized atomic promotion entry point.

crates/formula-store/tests/p7_promotion_transaction.rs
    U0->U1 atomicity, race, rollback, replay, changed-candidate, and immutable-history tests.

crates/formula-packages/src/closure.rs
    Add deterministic CapabilityClosureDelta::between.

crates/formula-packages/tests/p7_closure_delta.rs
    Prove closure changes only from admitted generation-scoped inputs.

crates/formula-first-light/Cargo.toml
    Dev-only P7 integration dependencies; no production authority-store dependency.

crates/formula-first-light/tests/p7_fl_c_promotion.rs
    First-Light FL-C frozen-candidate -> independent authorization -> U1 admission/activation fixture.

tests/authority-boundary/tests/p7_promotion_authority.rs
    Reject search/sealed-harness raw authority-write paths and raw public publication API.

.github/workflows/p7-development.yml
    Temporary focused RED/GREEN gate.

.github/workflows/p7-canonical-proof.yml
    Final read-only P0-P7 exact-head proof.

docs/checkpoints/2026-09-04-p7-promotion-generation-transition.md
CURRENT.md
    Written only after source proof + review.
```

---

### Task 1: Durable Promotion Identity + Lifecycle States

**Files:**
- Create: `crates/formula-core/src/promotion.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Test: `crates/formula-core/tests/p7_promotion_identity.rs`
- Create temporary runner: `.github/workflows/p7-development.yml`

**Interfaces:**
- Consumes: `ArtifactDigest`, `CanonicalValue`, `StructuralIdentity`.
- Produces:
  - `PromotionCandidate::new(frozen_candidate, promotion_manifest, parent_generation, proof_generation, dependency_cone, supersedes)`
  - `PromotionState::{Certified,Admitted,Activated,Quarantined}`
  - `PromotionRecord::new(candidate_digest, state, generation, policy_digest, evidence, semantic_artifacts)`
  - `QuarantineRecord::new(candidate_digest, parent_generation, reason_digest, conflicts)`
  - deterministic structural digests for all artifacts.

- [ ] **Step 1: Write failing identity/state tests** proving set-like normalization, relevant-field sensitivity, and pairwise-distinct lifecycle record digests for `Certified`, `Admitted`, `Activated`, `Quarantined`.
- [ ] **Step 2: Run `cargo test -p formula-core --test p7_promotion_identity --locked` and require RED only because promotion API is absent.**
- [ ] **Step 3: Implement minimal canonical artifacts.** `dependency_cone`, `supersedes`, `evidence`, and `semantic_artifacts` sort/dedup; state string is semantic identity.
- [ ] **Step 4: Run focused core test + existing core suite; require GREEN.**
- [ ] **Step 5: Commit `feat(p7): add promotion lifecycle artifacts`.**

### Task 2: Checker-Owned Promotion Policy + Opaque Authorization

**Files:**
- Modify: `crates/formula-check/src/promotion.rs`
- Test: `crates/formula-check/tests/p7_promotion_policy.rs`

**Interfaces:**
- Consumes: existing `check_promotion_manifest`, `FrozenCandidate`, `PromotionManifest`, `UniverseGeneration`, `PromotionCandidate`.
- Produces:
  - `PromotionPolicyV1::digest()`
  - `PromotionDecision::{Authorized(PromotionAuthorization),Quarantined(QuarantineRecord)}`
  - `authorize_promotion_v1(...)`
  - `PromotionAuthorization` with private fields/constructor and read-only accessors.

**Policy V1 exact rules:**
```text
manifest structural check must PASS
candidate.universe_generation == parent.digest
PromotionCandidate.parent_generation == parent.digest
PromotionCandidate.proof_generation == parent.digest
PromotionCandidate.frozen_candidate == FrozenCandidate.structural_digest
PromotionCandidate.promotion_manifest == PromotionManifest.structural_digest
every dependency-cone digest is admitted in parent
candidate.dependencies is a subset of dependency_cone
proposed_authority_bindings == normalized checked_evidence
supersedes entries are admitted in parent
non-empty conflict set => QUARANTINED, never Authorized
```

- [ ] **Step 1: Write RED tests** for exact authorization plus candidate-generation mismatch, stale proof generation, missing dependency, evidence mismatch, changed frozen candidate, invalid supersession, and quarantine-on-conflict.
- [ ] **Step 2: Prove RED is missing policy/authorization API only.**
- [ ] **Step 3: Implement Policy V1 and opaque authorization.** No lint suppression and no boolean `trusted=true` escape hatch.
- [ ] **Step 4: Run formula-check focused + full tests GREEN.**
- [ ] **Step 5: Commit `feat(p7): authorize promotion through independent checker`.**

### Task 3: Close Raw Publication Bypass + Authorized Atomic U_g -> U_(g+1)

**Files:**
- Modify: `crates/formula-store/Cargo.toml`
- Modify: `crates/formula-store/src/authority_store.rs`
- Modify if required: `crates/formula-store/src/lib.rs`
- Test: `crates/formula-store/tests/p7_promotion_transaction.rs`

**Interfaces:**
- Consumes: `formula_check::promotion::PromotionAuthorization`, current active `UniverseGeneration`.
- Produces:
  - public `AuthorityStore::promote(&PromotionAuthorization) -> Result<PromotionOutcome, AuthorityStoreError>`
  - `PromotionOutcome { parent_generation, new_generation, admitted_record }`
  - raw `publish_generation` becomes crate-private/internal; existing P1 atomic transaction remains unchanged beneath the public P7 entry point.

**Authorized promotion algorithm:**
```text
read active parent
require active == authorization.parent_generation
replay parent
union parent.admitted + authorization.proposed_admissions
union parent.authority_bindings + authorization.authority_bindings
construct generation_number + 1 with parent digest
publish through existing atomic transaction
return exact new generation digest
```

- [ ] **Step 1: Write RED transaction tests** proving successful U0->U1, U0 replay unchanged, candidate admission/evidence visible only in U1, active head advances once, parent race fails, and stale authorization fails.
- [ ] **Step 2: Add negative raw-bypass compile/source architecture test expectation before changing visibility.**
- [ ] **Step 3: Make raw publication internal and implement `promote`.**
- [ ] **Step 4: Re-run P1 atomicity tests and P7 transaction tests; require GREEN.**
- [ ] **Step 5: Commit `feat(p7): require checked authorization for generation advance`.**

### Task 4: Promotion Failpoints + History/Rollback Preservation

**Files:**
- Modify: `crates/formula-store/src/authority_store.rs`
- Test: `crates/formula-store/tests/p7_promotion_transaction.rs`

**Interfaces:**
- Adds test-only promotion failpoint entry mirroring proven P1 transaction failpoints without exposing it publicly.

- [ ] **Step 1: Write RED tests** for failure after generation rows before active switch and failure after active switch before commit using a valid `PromotionAuthorization`.
- [ ] **Step 2: Implement internal `promote_inner(..., PublishFailpoint)` that delegates to the existing publication transaction.**
- [ ] **Step 3: Prove both failures preserve active U0, make U1 unreplayable, and keep U0 replay byte-identical.**
- [ ] **Step 4: Commit `test(p7): prove promotion rollback and history preservation`.**

### Task 5: Derived Capability Closure Delta

**Files:**
- Modify: `crates/formula-packages/src/closure.rs`
- Test: `crates/formula-packages/tests/p7_closure_delta.rs`

**Interfaces:**
- Produces `CapabilityClosureDelta::between(before, after)` with deterministic `added()` / `removed()` sets.
- Delta itself is derived/non-authoritative and carries both closure context digests.

- [ ] **Step 1: Write RED tests** showing an admitted, authority-bound witness in U1 unlocks a capability absent in U0; the computed delta contains it in `added`, not `removed`.
- [ ] **Step 2: Prove a witness not admitted/bound in U1 cannot create the delta.**
- [ ] **Step 3: Implement deterministic delta.**
- [ ] **Step 4: Run P3 closure tests + new P7 closure tests GREEN.**
- [ ] **Step 5: Commit `feat(p7): derive capability closure delta`.**

### Task 6: FL-C Semantic Primitive Promotion Integration

**Files:**
- Modify: `crates/formula-first-light/Cargo.toml` (dev-dependencies only)
- Test: `crates/formula-first-light/tests/p7_fl_c_promotion.rs`

**Interfaces:**
- Dev/test orchestration consumes P5 frozen candidate, P6 FL-C oracle, P2 checker authorization, P7 store promotion.
- Production `formula-first-light` remains without `formula-store` / `formula-check` dependencies.

**Test sequence:**
```text
construct/load U0
construct the final bounded FL-C semantic candidate from public grammar
freeze candidate before sealed-oracle comparison
require sealed oracle returns Equivalent/no counterexample
construct core FrozenCandidate + exact checked evidence fixture
construct PromotionManifest and PromotionCandidate bound to U0
checker authorizes Policy V1
store promotes atomically to U1
assert promoted semantic primitive admitted in U1
construct Certified/Admitted/Activated records and prove state identities distinct
assert U0 replay unchanged
```

- [ ] **Step 1: Write failing integration test.**
- [ ] **Step 2: Add only dev-dependencies needed for integration; regenerate lock with pinned Cargo if workspace package metadata changes.**
- [ ] **Step 3: Make test GREEN using existing P5/P6/P7 APIs; do not add hidden answer data to P5.**
- [ ] **Step 4: Commit `test(p7): prove FL-C semantic primitive promotion`.**

### Task 7: Authority Firewalls + Adversarial Promotion Tests

**Files:**
- Create: `tests/authority-boundary/tests/p7_promotion_authority.rs`
- Extend relevant check/store tests.

**Required negatives:**
```text
formula-engine cannot depend on formula-store publication path
formula-engine cannot construct PromotionAuthorization
formula-first-light production dependency graph cannot depend on formula-store/formula-check
raw public publish_generation entry point is absent
changed candidate digest after authorization cannot be promoted
parent-generation race cannot publish
mismatched checked evidence cannot authorize
conflicted candidate is quarantined and cannot promote
superseded parent artifact remains replayable/admitted historically
failed transaction exposes no U1
```

- [ ] **Step 1: Add adversarial architecture/runtime tests.**
- [ ] **Step 2: Run architecture + check + store + package + First-Light suites.**
- [ ] **Step 3: Fix only demonstrated boundary defects; do not weaken assertions.**
- [ ] **Step 4: Commit `test(p7): lock promotion authority boundaries`.**

### Task 8: Canonical P7 Proof, Review, Freeze

**Files:**
- Create: `.github/workflows/p7-canonical-proof.yml`
- Delete: `.github/workflows/p7-development.yml`
- After source proof/review: create `docs/checkpoints/2026-09-04-p7-promotion-generation-transition.md`
- After source proof/review: update `CURRENT.md`

**Canonical gate:**
```bash
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test -p formula-core --all-targets --locked --offline
cargo test -p formula-store --all-targets --locked --offline
cargo test -p formula-check --all-targets --locked --offline
cargo test -p formula-packages --all-targets --locked --offline
cargo test -p formula-engine --all-targets --locked --offline
cargo test -p formula-first-light --all-targets --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --all-targets --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-store --edges normal
cargo tree --locked --offline -p formula-check --edges normal
cargo tree --locked --offline -p formula-engine --edges normal
test -z "$(git status --porcelain)"
```

**Required P7 proof markers:**
```text
PASS P7_FROZEN_BEFORE_CERTIFICATION
PASS P7_LIFECYCLE_STATES_DISTINCT
PASS P7_CHECKER_AUTHORIZATION_REQUIRED
PASS P7_RAW_PUBLICATION_BYPASS_CLOSED
PASS P7_ATOMIC_U0_TO_U1
PASS P7_U0_HISTORY_REPLAY_PRESERVED
PASS P7_PARENT_RACE_REJECTED
PASS P7_QUARANTINE_FAILS_CLOSED
PASS P7_CAPABILITY_CLOSURE_DERIVED
PASS P7_FLC_PRIMITIVE_ADMITTED
PASS P7_SEARCH_AUTHORITY_FIREWALL
```

- [ ] **Step 1: Install read-only canonical workflow and retire development workflow.**
- [ ] **Step 2: Prove exact source SHA; require every command and marker PASS.**
- [ ] **Step 3: Review exact P6-final -> P7-source diff; reject P8 realization or P9 full-campaign leakage.**
- [ ] **Step 4: Write P7 checkpoint + `CURRENT.md` only after source proof/review.**
- [ ] **Step 5: Require unchanged canonical P7 workflow to pass exact documentation-bearing SHA.**
- [ ] **Step 6: Freeze P7 only after docs-head proof; cut P8 from that exact SHA.**

---

## Self-Review

- **Spec coverage:** P7 roadmap requirements are mapped: PromotionCandidate (Task 1), lifecycle states (1), policy/check authorization (2), atomic generation transaction (3/4), closure delta (5), quarantine/conflict (2/7), supersession lineage (1/2/7), proof freshness/dependency cone (1/2), FL-C semantic primitive promotion (6), negative race/mismatch/search-write tests (3/4/7), exact source/docs freeze (8).
- **No placeholders:** no TBD/TODO/"implement later" steps.
- **Type consistency:** `PromotionCandidate` is core structural identity; `PromotionAuthorization` is checker-owned/opaque; `AuthorityStore::promote` accepts only authorization; `CapabilityClosureDelta` remains derived; First-Light authority orchestration is test/dev-only.
- **Scope:** no P8 generated native source/binary work and no P9 second-query/reuse campaign.
