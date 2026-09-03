# P6 First-Light Target Harness + Blindness Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the frozen FL-A/FL-B/FL-C target harness so search can exercise only declared public/oracle interfaces while sealed answers remain inaccessible, every hidden comparison consumes a frozen candidate, and target/grammar/package bindings are deterministic.

**Architecture:** `formula-first-light` owns target manifests, narrow oracle interfaces, sealed semantics, public FL-B fixture data, and P6 runtime evidence. `formula-engine` and `formula-packages` remain unchanged search producers and must not depend on or reference sealed target implementations. Authority-boundary tests statically prove import/path/token separation; runtime tests prove oracle interaction reveals only allowed observations/counterexamples and requires frozen candidates before hidden comparison.

**Tech Stack:** stable Rust pinned by repository `rust-toolchain.toml`; existing `formula-core` canonical identity; existing P5 CandidateSpace/FrozenCandidate APIs; Cargo architecture tests; GitHub Actions canonical proof.

**Spec:** `docs/design/2026-08-28-first-light-specification.md` and P6 section of `docs/roadmap/2026-08-28-implementation-roadmap.md`

## Global Constraints

- P6 starts from frozen P5 head `d2bd250c4b4419316292845a44849747d9e01113`.
- Canonical First Light remains local CPU-only, model-free, Ptah-free, and network-free during execution.
- Search/discovery code may not import `formula-first-light` or sealed fixture definitions.
- Hidden expected formulas/programs may not appear in `formula-engine` or `formula-packages` source.
- Every hidden semantic comparison accepts an immutable/frozen candidate artifact, never an engine-internal mutable pointer.
- P6 does not implement promotion, native realization, or second-query reuse; those belong to P7–P9.
- No new external runtime dependency is required.

---

### Task 1: Blind target identity and manifest contracts

**Files:**
- Create: `crates/formula-first-light/src/manifest.rs`
- Modify: `crates/formula-first-light/src/lib.rs`
- Test: `crates/formula-first-light/tests/p6_manifest.rs`
- Create temporarily with first RED test: `.github/workflows/p6-development.yml`

**Interfaces:**
- Produces `FirstLightTarget::{FlA,FlB,FlC}`.
- Produces `BlindnessManifest::new(target, sealed_target_digest, universe_generation, world, query_digest, grammar_or_routes_digest, package_set_digest, oracle_contract_digest)`.
- Produces deterministic `BlindnessManifest::digest()` and getters for all bound digests.
- Produces `FrozenSubmission::new(target, FrozenCandidate)` / `digest()` / `candidate()`.

- [ ] Write tests proving manifest digest changes when target, sealed digest, generation, world, query, grammar/routes, package set, or oracle contract changes; construction order cannot affect identity.
- [ ] Run `cargo test -p formula-first-light --test p6_manifest --locked` and observe RED because `manifest` API does not exist.
- [ ] Implement canonical-value encoding with a versioned P6 schema and frozen-candidate submission binding.
- [ ] Re-run focused test and existing architecture tests; require GREEN.
- [ ] Commit as `feat(p6): add blind target manifest contracts`.

### Task 2: FL-A sealed polynomial sample/comparison oracle

**Files:**
- Create: `crates/formula-first-light/src/fl_a.rs`
- Test: `crates/formula-first-light/tests/p6_fl_a.rs`

**Interfaces:**
- Produces opaque `FlAOracle` through `fl_a_oracle()`.
- `FlAOracle::sample(n: i128) -> i128` exposes only exact relation samples for `(n+1)^7-n^7`.
- `FlAOracle::first_counterexample(candidate: &AffinePolynomialCandidate, domain: &[i128]) -> Option<(i128,i128)>` requires a frozen P5 candidate and returns only a discriminating input/expected value.
- `fl_a_target_digest()` binds the sealed semantic definition/version, not the expanded coefficient answer.

- [ ] RED test: sample values are exact; candidate comparison accepts an extracted frozen affine candidate; an insufficient-sample near-miss is eliminated by an unseen point; no public API returns the expanded coefficient vector.
- [ ] Implement sealed semantic evaluator privately in `fl_a` and expose only oracle methods.
- [ ] Verify focused tests GREEN and no P4/P5 regression.
- [ ] Commit as `feat(p6): add sealed FL-A oracle`.

### Task 3: FL-B public XOR fixture and route-binding contract

**Files:**
- Create: `crates/formula-first-light/src/fl_b.rs`
- Test: `crates/formula-first-light/tests/p6_fl_b.rs`

**Interfaces:**
- Produces `PublicXorRow { variables: Vec<usize>, rhs: bool }` and `PublicXorSystem { width: usize, rows: Vec<PublicXorRow> }` with read-only getters.
- `fl_b_public_problem()` returns one deterministic satisfiable 24-variable XOR system.
- `fl_b_problem_digest()` is content-addressed over the public system.
- `fl_b_route_contract_digest()` binds exact Boolean-XOR -> GF(2) preservation/reconstruction contract identity without naming a winning route in search code.

- [ ] RED test: fixture is 24 variables, deterministic, satisfiable by a private test witness, and public digest is stable under fresh construction; direct-route and GF(2)-route identities remain separate from target data.
- [ ] Implement fixture/identity only; do not implement route search or checker logic in `formula-first-light`.
- [ ] Verify focused test GREEN.
- [ ] Commit as `feat(p6): add public FL-B xor fixture`.

### Task 4: FL-C sealed U8 specification/counterexample oracle

**Files:**
- Create: `crates/formula-first-light/src/fl_c.rs`
- Test: `crates/formula-first-light/tests/p6_fl_c.rs`

**Interfaces:**
- Produces opaque `FlCOracle` through `fl_c_oracle()`.
- `FlCOracle::first_counterexample(candidate: &FrozenExprCandidate) -> Option<(u8,bool)>` exhaustively scans U8 and returns only the first differing observation.
- `fl_c_target_digest()` binds the sealed semantic target/version.
- `fl_c_grammar_digest()` must equal `U8BoolGrammar::minimal().digest()` for the canonical campaign.
- `fl_c_zero_near_miss()` exposes only the mandatory plausible near-miss `(x & (x-1)) == 0`, not the final answer.

- [ ] RED test: near-miss is accepted for powers of two but oracle returns `0 -> false`; a correct frozen candidate has no counterexample; grammar/target digests are stable and distinct.
- [ ] Implement private sealed membership semantics and exhaustive counterexample oracle over all 256 inputs.
- [ ] Verify focused test GREEN.
- [ ] Commit as `feat(p6): add sealed FL-C oracle`.

### Task 5: Static blindness and dependency firewall

**Files:**
- Modify: `tests/authority-boundary/tests/sealed_boundary.rs`
- Add: `tests/authority-boundary/tests/p6_blindness.rs`

**Interfaces:**
- Architecture test reads repository source/Cargo manifests only; it does not participate in runtime authority.

- [ ] RED tests must fail if `formula-engine` or `formula-packages` references `formula-first-light`, `tests/first-light/sealed`, FL-A expanded answer literals, or the final FL-C compact answer as a source literal.
- [ ] Add checks that `formula-first-light` owns sealed semantics and discovery crates do not depend on it in Cargo manifests.
- [ ] Keep public near-miss literals permitted while final hidden answer literals remain forbidden.
- [ ] Run `cargo test -p formula-archtest --locked`; require GREEN.
- [ ] Commit as `test(p6): enforce first-light blindness firewall`.

### Task 6: Runtime blindness/adversarial harness

**Files:**
- Add: `crates/formula-first-light/tests/p6_blindness_runtime.rs`
- Add: `crates/formula-first-light/tests/p6_adversarial.rs`

**Interfaces:**
- Consumes only public manifests/oracle interfaces and P5 frozen candidates.

- [ ] Prove two fresh manifests with identical inputs have identical digest.
- [ ] Prove modified sealed-target digest changes manifest and fails exact manifest-match validation.
- [ ] Prove FL-A/FL-C hidden comparisons cannot be invoked without frozen candidate types at the public API boundary.
- [ ] Prove FL-C mandatory zero-accepting near-miss is rejected by counterexample `0`.
- [ ] Prove target/oracle APIs reveal observations/counterexamples only; no method exposes hidden expanded FL-A coefficients or final FL-C expression.
- [ ] Commit as `test(p6): prove runtime blindness boundary`.

### Task 7: Canonical P6 proof, review, and freeze

**Files:**
- Create: `.github/workflows/p6-canonical-proof.yml`
- Delete after canonical workflow is installed: `.github/workflows/p6-development.yml`
- After source proof only: create `docs/checkpoints/2026-09-04-p6-first-light-target-harness-blindness.md`
- After source proof only: modify `CURRENT.md`

**Canonical gate:**

```bash
cargo metadata --locked --offline --format-version 1
cargo test -p formula-archtest --locked --offline
cargo test -p formula-core --locked --offline
cargo test -p formula-store --locked --offline
cargo test -p formula-check --locked --offline
cargo test -p formula-packages --locked --offline
cargo test -p formula-engine --locked --offline
cargo test -p formula-first-light --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo build --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo tree --workspace --locked --offline
```

**Required P6 markers:**

```text
PASS P6_BLIND_MANIFEST_BINDING
PASS P6_FLA_SEALED_ORACLE
PASS P6_FLB_PUBLIC_ROUTE_FIXTURE
PASS P6_FLC_FROZEN_COUNTEREXAMPLE_ORACLE
PASS P6_FALSE_NEARMISS_VISIBLE
PASS P6_DISCOVERY_SEALED_DEPENDENCY_FIREWALL
PASS P6_HIDDEN_ANSWER_LITERAL_FIREWALL
PASS P6_RUNTIME_BLINDNESS
```

- [ ] Run the canonical workflow on the exact source-under-test SHA; require all commands/markers PASS.
- [ ] Review exact P5-final -> P6-source diff; reject any P7 promotion, P8 realization, or P9 reuse implementation leakage.
- [ ] Write checkpoint + `CURRENT.md` only after review/source proof.
- [ ] Require the unchanged canonical P6 workflow to pass on the exact documentation-bearing SHA.
- [ ] Freeze P6 only after the documentation-bearing proof succeeds; then cut P7 from that exact head.
