# P2 — Independent Checker + Certificate Core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish an independent, content-addressed certification path that validates frozen candidate artifacts and certificate envelopes without depending on search, realization, or producer implementation code.

**Architecture:** Preserve P1 identity/authority primitives unchanged, then add the minimum P2 immutable checker-facing schemas in `formula-core` and all checking logic in `formula-check`. The checker operates only on frozen structural digests plus explicit domain-native certificate bodies; it never imports `formula-engine` or `formula-realize`, and no checker verdict writes authority. Authority-contract matching is fail-closed, candidate identity is frozen before certification, and realization/promotion checks validate exact digests rather than trusting producer/compiler identity.

**Tech Stack:** Rust 1.98.0, `formula-core`, `formula-check`, exact integer arithmetic through existing workspace `num-bigint`/`num-traits`, SHA-256 through existing `formula-core::ArtifactDigest`, locked/offline Cargo proof.

**Spec:** `docs/roadmap/2026-08-28-implementation-roadmap.md` P2, plus `docs/design/2026-08-28-d1-mathematical-constitution.md` §10, `docs/design/2026-08-28-d2-core-system-architecture.md` §22, `docs/design/2026-08-28-d2-operational-mathematical-machine.md` §13, `docs/design/2026-08-28-d3-first-light-build-architecture.md` §§3.2, 8–11, `docs/design/2026-08-28-d4-native-execution-architecture.md` §22/§25, and `docs/design/2026-08-28-d5-self-expansion-architecture.md` §3/§29.

## Global Constraints

- P0/P1 authority separation remains intact: `formula-check` may depend on `formula-core` and exact arithmetic only; it must not depend on `formula-engine`, `formula-realize`, candidate search implementations, or search heuristics.
- Do not mutate or reinterpret existing P1 structural identities. Add accessors or new P2 schema types instead of changing canonical bytes for existing P1 artifacts.
- Checker input is frozen/content-addressed. A changed candidate is a different digest and requires a new certification attempt.
- Checker package/version identity is explicit and envelope-bound; mismatch fails closed.
- Authority Contract matching is explicit and monotone: insufficient evidence returns rejection/insufficient-authority, never a weaker PASS.
- No checker verdict can directly publish authority or admit generated/native code.
- Canonical proof commands run with `--locked --offline` after dependencies are frozen.
- Runtime dependency allowlist remains explicit; any added normal runtime package is a reviewed authority-boundary change.

---

## File Structure

### `formula-core`

- Modify `crates/formula-core/src/artifacts.rs`
  - add read-only accessors to existing `EvidenceEnvelope`, `AuthorityContract`, `Observer`, `RealizationMetadata`, and `Judgement` fields needed by an independent checker;
  - do **not** alter their canonical encoding.
- Create `crates/formula-core/src/certification.rs`
  - define P2 structural types that did not exist in P1: `FrozenCandidate`, `CertificateEnvelope`, `PromotionManifest`, and `RealizationCheckManifest`;
  - each implements `StructuralIdentity` and binds exact candidate, target, generation, world, dependencies, checker identity/version, certificate body digest, and authority/observer digests.
- Modify `crates/formula-core/src/lib.rs`
  - export the new certification module/types.
- Create `crates/formula-core/tests/certification_identity.rs`
  - prove sorting/deduplication, candidate immutability by digest, envelope binding, and that existing P1 artifact digests remain unchanged when accessors are added.

### `formula-check`

- Modify `crates/formula-check/Cargo.toml`
  - add only exact arithmetic dependencies already present in the workspace if required (`num-bigint`, `num-traits`); no engine/realizer dependency.
- Replace `crates/formula-check/src/lib.rs`
  - keep crate role constant and export focused checker modules.
- Create `crates/formula-check/src/verdict.rs`
  - define fail-closed `CheckVerdict`, `CheckFailure`, and `AuthorityMatch` result types.
- Create `crates/formula-check/src/identity.rs`
  - define deterministic checker descriptor/version identity and family registry.
- Create `crates/formula-check/src/envelope.rs`
  - validate envelope structural bindings, checker identity/version, target/world/generation/dependencies, certificate body digest, and Authority Contract compatibility.
- Create `crates/formula-check/src/polynomial.rs`
  - exact integer polynomial normalization/identity checking.
- Create `crates/formula-check/src/gf2.rs`
  - Boolean-XOR → GF(2) row/witness checking against the original frozen constraints.
- Create `crates/formula-check/src/u8.rs`
  - finite exhaustive U8 semantic-equivalence checker over all 256 inputs for the bounded First-Light expression subset.
- Create `crates/formula-check/src/promotion.rs`
  - structural promotion-manifest checker proving candidate/evidence/generation/world/dependency bindings; no publication.
- Create `crates/formula-check/src/realization.rs`
  - realization-equivalence harness interface and exact exhaustive U8 output comparison; compiler identity alone cannot PASS.
- Create integration tests under `crates/formula-check/tests/` for each proof/negative boundary.
- Modify `tests/authority-boundary/architecture.rs` only if needed to strengthen the P0 firewall so new `formula-check` dependencies remain within the explicit allowlist.
- Modify `tests/authority-boundary/runtime-allowlist.txt` only if the actual frozen normal runtime closure changes.
- Add `.github/workflows/p2-branch-ci.yml` only after the source tests exist; it must be read-only proof automation and run the canonical P0+P1+P2 gate.

---

### Task 1: Freeze P2 checker-facing structural contracts

**Files:**
- Modify: `crates/formula-core/src/artifacts.rs`
- Create: `crates/formula-core/src/certification.rs`
- Modify: `crates/formula-core/src/lib.rs`
- Create: `crates/formula-core/tests/certification_identity.rs`

**Interfaces:**
- Consumes: existing `ArtifactDigest`, `CanonicalValue`, `StructuralIdentity`, `UniverseGeneration` identities.
- Produces:
  - `FrozenCandidate::new(candidate_class, semantic_artifacts, world, universe_generation, dependencies, proposed_judgements, authority_contract, observer)`
  - `CertificateEnvelope::new(frozen_candidate, target_judgement, universe_generation, world, semantic_scope, outcome_class, verification_mode, certificate_family, certificate_family_version, certificate_body_digest, producer, checker, checker_trust_root, dependencies, authority_contract, observer, replay_binding)`
  - `PromotionManifest::new(parent_generation, frozen_candidate, evidence_envelopes, proposed_admissions, proposed_authority_bindings)`
  - `RealizationCheckManifest::new(semantic_target, realization, universe_generation, world, authority_contract, observer, realization_artifact_digest)`
  - read-only getters for the existing P1 artifacts used by later checker tasks.

- [ ] **Step 1: Write failing structural-identity tests**

Tests must assert:

```rust
#[test]
fn frozen_candidate_digest_changes_when_candidate_content_changes() { /* two semantic digests -> different FrozenCandidate digest */ }

#[test]
fn frozen_candidate_normalizes_set_like_dependencies() { /* reordering/duplicates -> same digest */ }

#[test]
fn certificate_envelope_binds_generation_world_checker_and_body() { /* change each -> different digest */ }

#[test]
fn accessors_do_not_change_existing_p1_structural_identity() { /* known existing constructors -> canonical bytes/digest unchanged */ }
```

- [ ] **Step 2: Run the focused tests and observe RED**

Run:

```bash
cargo test -p formula-core --test certification_identity --locked --offline
```

Expected: FAIL because `certification` types/accessors do not exist.

- [ ] **Step 3: Implement only the immutable P2 schemas/accessors**

Use the existing `canonical_object("...")`, sorted digest/string helpers, and `StructuralIdentity` pattern. Do not change any existing P1 `canonical_value()` field set.

- [ ] **Step 4: Run focused + existing P1 identity tests**

```bash
cargo test -p formula-core --test certification_identity --locked --offline
cargo test -p formula-core --locked --offline
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-core
git commit -m "feat: freeze P2 certification identities"
```

---

### Task 2: Checker identity, envelope validation, and no-downgrade Authority Contract routing

**Files:**
- Modify: `crates/formula-check/Cargo.toml`
- Replace: `crates/formula-check/src/lib.rs`
- Create: `crates/formula-check/src/verdict.rs`
- Create: `crates/formula-check/src/identity.rs`
- Create: `crates/formula-check/src/envelope.rs`
- Create: `crates/formula-check/tests/envelope_validation.rs`

**Interfaces:**
- Consumes: `CertificateEnvelope`, `FrozenCandidate`, `AuthorityContract`, `Observer`, exact body bytes.
- Produces:
  - `CheckerDescriptor::current() -> CheckerDescriptor`
  - `validate_envelope(request: &CheckRequest<'_>) -> Result<AuthorityMatch, CheckFailure>`
  - `CheckRequest` binds expected candidate/target/generation/world/dependencies/authority/observer and certificate body bytes.

- [ ] **Step 1: Write RED tests for required rejection boundaries**

Cover all of:

```rust
forged_certificate_body_digest_is_rejected
mismatched_target_digest_is_rejected
mismatched_world_is_rejected
mismatched_generation_is_rejected
checker_identity_or_version_mismatch_is_rejected
changed_candidate_after_envelope_creation_is_rejected
missing_dependency_binding_is_rejected
strict_deterministic_contract_rejects_probabilistic_or_empirical_evidence
allowed_exact_exhaustive_evidence_satisfies_exact_contract
```

- [ ] **Step 2: Run and observe RED**

```bash
cargo test -p formula-check --test envelope_validation --locked --offline
```

Expected: FAIL because router/descriptor/verdict types do not exist.

- [ ] **Step 3: Implement deterministic checker identity and fail-closed router**

Rules:

```text
body SHA-256 must equal envelope certificate_body_digest
candidate/target/generation/world/dependencies must match exactly
checker digest + family version must equal the running checker descriptor
requested authority class must be satisfied explicitly
exactness requirement may not be weakened
observer binding must match exactly when present
unknown evidence family/version => reject
```

- [ ] **Step 4: Run focused tests and P0 architecture firewall**

```bash
cargo test -p formula-check --test envelope_validation --locked --offline
cargo test -p formula-archtest --locked --offline
```

Expected: PASS and dependency firewall unchanged.

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check crates/formula-core tests/authority-boundary
git commit -m "feat: add fail-closed certificate router"
```

---

### Task 3: Exact polynomial identity checker

**Files:**
- Create: `crates/formula-check/src/polynomial.rs`
- Create: `crates/formula-check/tests/polynomial_identity.rs`

**Interfaces:**
- Consumes: frozen integer coefficient vectors or exact polynomial certificate bodies.
- Produces: `check_polynomial_identity(expected: &IntegerPolynomial, candidate: &IntegerPolynomial) -> CheckVerdict`.

- [ ] **Step 1: Write RED tests**

Required cases:

```rust
expanded_difference_of_seventh_powers_passes
sample_fitting_near_miss_fails_universal_identity
trailing_zero_coefficients_normalize
coefficient_change_fails
```

Use exact integers only; no floating-point sampling may establish PASS.

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-check --test polynomial_identity --locked --offline
```

- [ ] **Step 3: Implement minimal exact normalization/equality**

Normalize coefficient vectors by trimming only trailing exact zeros. Equality of normalized exact coefficient vectors is the universal polynomial-identity proof for this bounded family.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-check --test polynomial_identity --locked --offline
```

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check/src/polynomial.rs crates/formula-check/tests/polynomial_identity.rs crates/formula-check/src/lib.rs
git commit -m "feat: add exact polynomial identity checker"
```

---

### Task 4: GF(2)/Boolean translation and witness checker

**Files:**
- Create: `crates/formula-check/src/gf2.rs`
- Create: `crates/formula-check/tests/gf2_witness.rs`

**Interfaces:**
- Consumes: original Boolean XOR rows, claimed GF(2) rows, and Boolean witness.
- Produces: `check_gf2_witness(problem: &BooleanXorSystem, translated: &Gf2System, witness: &[bool]) -> CheckVerdict`.

- [ ] **Step 1: Write RED tests**

Required cases:

```rust
exact_translation_and_valid_witness_pass
changed_rhs_translation_fails
missing_variable_in_translated_row_fails
witness_bit_outside_declared_width_fails
witness_that_satisfies_gf2_but_not_original_boolean_problem_fails
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-check --test gf2_witness --locked --offline
```

- [ ] **Step 3: Implement exact row canonicalization and dual verification**

The checker must independently canonicalize both row sets and then evaluate the returned witness against the **original** Boolean XOR constraints, not merely trust the translated system.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-check --test gf2_witness --locked --offline
```

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check/src/gf2.rs crates/formula-check/tests/gf2_witness.rs crates/formula-check/src/lib.rs
git commit -m "feat: add GF2 witness checker"
```

---

### Task 5: U8 exhaustive semantic equivalence checker

**Files:**
- Create: `crates/formula-check/src/u8.rs`
- Create: `crates/formula-check/tests/u8_equivalence.rs`

**Interfaces:**
- Consumes: frozen bounded `ByteExpr`/`BoolExpr` candidate and independent U8 specification predicate.
- Produces: `check_u8_equivalence(candidate: &BoolExpr, specification: fn(u8) -> bool) -> CheckVerdict` plus counterexample input on failure.

- [ ] **Step 1: Write RED tests**

Required cases:

```rust
power_of_two_candidate_with_nonzero_guard_passes_all_256_inputs
classic_missing_zero_guard_near_miss_fails_at_zero
integer_subtraction_semantics_cannot_alias_u8_wrapping_subtraction
one_changed_operator_returns_exact_counterexample
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-check --test u8_equivalence --locked --offline
```

- [ ] **Step 3: Implement the bounded independent evaluator and exhaustive loop**

Allowed expression subset:

```text
ByteExpr: X | Const(u8) | SubWrap | BitAnd | BitOr | BitXor | BitNot
BoolExpr: EqZero | NeqZero | And | Or | Not
```

Check exactly `0..=255`; PASS only if every input agrees.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-check --test u8_equivalence --locked --offline
```

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check/src/u8.rs crates/formula-check/tests/u8_equivalence.rs crates/formula-check/src/lib.rs
git commit -m "feat: add exhaustive U8 semantic checker"
```

---

### Task 6: Frozen-candidate promotion-manifest structural checker

**Files:**
- Create: `crates/formula-check/src/promotion.rs`
- Create: `crates/formula-check/tests/promotion_manifest.rs`

**Interfaces:**
- Consumes: `PromotionManifest`, referenced `FrozenCandidate`, and checked envelope digests.
- Produces: `check_promotion_manifest(manifest: &PromotionManifest, candidate: &FrozenCandidate, evidence: &[ArtifactDigest]) -> CheckVerdict`.

- [ ] **Step 1: Write RED tests**

Required cases:

```rust
manifest_bound_to_exact_frozen_candidate_passes
candidate_changed_after_certification_fails
unreferenced_or_missing_evidence_fails
wrong_parent_generation_fails_expected_binding
proposed_admission_not_covered_by_candidate_fails
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-check --test promotion_manifest --locked --offline
```

- [ ] **Step 3: Implement structural checking only**

This task does **not** publish a generation. It proves D5-P01 and the P2 promotion-manifest checker boundary by requiring exact frozen identities before a later P7 promotion service may act.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-check --test promotion_manifest --locked --offline
```

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check/src/promotion.rs crates/formula-check/tests/promotion_manifest.rs crates/formula-check/src/lib.rs
git commit -m "feat: check frozen promotion manifests"
```

---

### Task 7: Realization-equivalence harness and compiler self-admission rejection

**Files:**
- Create: `crates/formula-check/src/realization.rs`
- Create: `crates/formula-check/tests/realization_equivalence.rs`

**Interfaces:**
- Consumes: `RealizationCheckManifest`, admitted semantic U8 candidate/evaluator, and executable-output adapter supplied as data/function by the test harness.
- Produces: `check_u8_realization_equivalence(...) -> CheckVerdict`.

- [ ] **Step 1: Write RED tests**

Required cases:

```rust
compiler_claim_without_independent_output_check_is_not_authority
exact_realization_outputs_for_all_256_inputs_pass
mutated_realization_missing_zero_guard_fails
binary_or_artifact_digest_mismatch_fails_before_execution_comparison
```

- [ ] **Step 2: Run RED**

```bash
cargo test -p formula-check --test realization_equivalence --locked --offline
```

- [ ] **Step 3: Implement independent comparison boundary**

The checker validates manifest digests first, then compares all 256 outputs. A producer/compiler-provided success flag is ignored and has no authority surface.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-check --test realization_equivalence --locked --offline
```

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check/src/realization.rs crates/formula-check/tests/realization_equivalence.rs crates/formula-check/src/lib.rs
git commit -m "feat: add realization equivalence checker"
```

---

### Task 8: Prove producer/checker isolation and malicious-producer rejection end to end

**Files:**
- Create: `crates/formula-check/tests/malicious_producer.rs`
- Modify if needed: `tests/authority-boundary/architecture.rs`
- Modify if needed: `tests/authority-boundary/runtime-allowlist.txt`

**Interfaces:**
- Consumes all Task 1–7 public checker APIs.
- Produces a single adversarial P2 proof fixture demonstrating that producer claims cannot bypass exact frozen checking.

- [ ] **Step 1: Write adversarial test fixture**

The fixture must attempt all of:

```text
forge PASS verdict while body digest mismatches
reuse a valid certificate on a different target
reuse a valid certificate after candidate mutation
claim a weaker probabilistic/empirical route for a deterministic exact contract
claim compiler success without realization equivalence
```

Every path must end in `CheckFailure`/non-PASS.

- [ ] **Step 2: Run RED if any bypass remains**

```bash
cargo test -p formula-check --test malicious_producer --locked --offline
cargo test -p formula-archtest --locked --offline
```

- [ ] **Step 3: Make only minimal fixes required by observed failures**

Do not add search/engine coupling to make tests convenient.

- [ ] **Step 4: Run GREEN**

```bash
cargo test -p formula-check --test malicious_producer --locked --offline
cargo test -p formula-archtest --locked --offline
```

- [ ] **Step 5: Commit**

```bash
git add crates/formula-check tests/authority-boundary
git commit -m "test: prove malicious producer cannot self-certify"
```

---

### Task 9: Freeze dependency closure and canonical P2 proof workflow

**Files:**
- Modify if generated: `Cargo.lock`
- Modify if required: `tests/authority-boundary/runtime-allowlist.txt`
- Create: `.github/workflows/p2-branch-ci.yml`

**Interfaces:**
- Consumes all P0/P1/P2 tests.
- Produces one canonical proof transcript for an exact source SHA.

- [ ] **Step 1: Generate/freeze lockfile from the pinned toolchain**

Do not hand-write `Cargo.lock`. Accept only the exact generated lock and actual runtime closure.

- [ ] **Step 2: Verify dependency firewall explicitly**

```bash
cargo tree --locked --offline -p formula-check
```

Expected: `formula-check` contains only `formula-core` plus the approved exact-arithmetic/hash transitive closure; no engine, realizer, network, model, solver, or external process dependency.

- [ ] **Step 3: Add read-only P2 branch proof workflow**

Canonical commands:

```bash
git rev-parse HEAD
rustc -vV
cargo -V
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test -p formula-core --locked --offline
cargo test -p formula-store --locked --offline
cargo test -p formula-check --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --locked --offline -p formula-check
git status --porcelain
```

- [ ] **Step 4: Require exact P2 proof markers in the workflow log/checkpoint input**

```text
P2-01 certificate envelope exact binding                     PASS
P2-02 no silent Authority Contract downgrade                 PASS
P2-03 independent checker isolated from producer/search      PASS
P2-04 polynomial exact identity checker                      PASS
P2-05 GF2/Boolean translation+witness checker                PASS
P2-06 U8 finite exhaustive semantic equivalence              PASS
P2-07 frozen candidate required before certification         PASS
P2-08 promotion manifest structural binding                  PASS
P2-09 compiler/optimizer cannot self-admit realization       PASS
P2-10 forged/mismatched/stale evidence rejected              PASS
P2-11 P0/P1 architecture and authority-store gates preserved PASS
```

- [ ] **Step 5: Run the complete proof gate and fix only observed defects**

Any failure returns to RED→GREEN for the smallest responsible task.

- [ ] **Step 6: Commit proof workflow/dependency freeze**

```bash
git add Cargo.lock tests/authority-boundary/runtime-allowlist.txt .github/workflows/p2-branch-ci.yml
git commit -m "ci: freeze canonical P2 proof gate"
```

---

### Task 10: Review, checkpoint, and post-checkpoint proof

**Files:**
- Create: `docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md`
- Modify: `CURRENT.md`

**Interfaces:**
- Consumes exact successful P2 source SHA/run/job/toolchain/dependency evidence.
- Produces repository recovery authority for P2 and identifies P3 as next only if P2 remains green after the documentation commit.

- [ ] **Step 1: Review the complete P1→P2 diff against the roadmap**

Verify every P2 build-scope item and proof obligation has a concrete implementation/test; reject unrelated refactors.

- [ ] **Step 2: Record canonical source-under-test evidence**

Checkpoint must include exact:

```text
branch
source SHA
workflow run/job
rustc/cargo versions
Cargo.lock Git blob + byte SHA-256
formula-check dependency tree
P2 proof markers
negative-control results
P0/P1 preservation statement
explicit non-claims
```

- [ ] **Step 3: Update `CURRENT.md`**

Set P2 as current proved milestone only after the canonical source gate is green; identify P3 from the frozen roadmap.

- [ ] **Step 4: Commit documentation only**

```bash
git add docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md CURRENT.md
git commit -m "docs: freeze P2 independent checker checkpoint"
```

- [ ] **Step 5: Re-run the complete canonical P2 gate on the exact documentation-bearing head**

The post-checkpoint run must succeed. Compare the source-under-test SHA to final head and verify only checkpoint/current-state documentation changed after the canonical implementation proof.

- [ ] **Step 6: Stop at the P2 freeze boundary**

Do not merge `main` and do not start P3 until the P2 final state is proved and reported.
