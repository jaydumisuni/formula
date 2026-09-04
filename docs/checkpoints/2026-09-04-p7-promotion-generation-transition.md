# P7 Checkpoint — Promotion + Generation Transition

**Recorded:** 2026-09-04  
**Status:** PROVED SOURCE — D5 promotion/generation transition; documentation-bearing head still requires exact-head canonical proof  
**Branch:** `implementation/p7-promotion-generation-transition`  
**P6 exact frozen predecessor:** `035953854f33fe47dc884850dec4fdee7a3571e7`  
**Source-under-test commit:** `ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2`  
**Canonical source proof run:** `33861803872`  
**Canonical source proof job:** `100987699053`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P7  
**Implementation plan:** `docs/superpowers/plans/2026-09-04-p7-promotion-generation-transition.md`

---

## 1. Scope

P7 implements the bounded D5 promotion path between certified candidate mathematics and immutable generation authority. It composes the already-proved P1 atomic authority store, P2 checker/promotion-manifest validation, P3 capability closure, P5 frozen candidates, and P6 sealed First-Light harness without introducing a second authority root.

Frozen P7 source surfaces include:

- canonical `PromotionCandidate` identity;
- explicit `CERTIFIED`, `ADMITTED`, `ACTIVATED`, and `QUARANTINED` lifecycle states;
- canonical `PromotionRecord` and `QuarantineRecord` identity;
- checker-owned `PromotionPolicyV1`;
- checker-issued opaque `PromotionAuthorization` with no public constructor or mutable authority fields;
- fail-closed `PromotionDecision` and quarantine path;
- `AuthorityStore::promote(&PromotionAuthorization)` as the public generation-advance route;
- closure of the earlier raw public generation-publication bypass;
- atomic `U_g -> U_(g+1)` publication using the existing P1 transaction primitive;
- parent-generation race rejection and existing publication failpoint rollback;
- preserved historical replay of `U_0` after `U_1` creation;
- deterministic `CapabilityClosureDelta` derived from admitted/authority-bound inputs only;
- bounded FL-C semantic primitive promotion integration;
- architecture firewalls keeping search/discovery code outside checker/store/promotion authority.

P7 does **not** implement native CPU realization generation/admission, realization dispatch, the second-query reuse proof, the complete P9 First-Light proof manifest, or Ptah/distributed execution. Those remain P8/P9 and later.

---

## 2. Exact predecessor and review boundary

P7 was cut from exact fully frozen P6 documentation-bearing head:

```text
035953854f33fe47dc884850dec4fdee7a3571e7
```

The final proved P7 source boundary is:

```text
ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
```

Exact compare evidence:

```text
base:    035953854f33fe47dc884850dec4fdee7a3571e7
head:    ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
status:  ahead
ahead:   44 commits
behind:  0 commits
```

The final tree delta is confined to:

```text
.github/workflows/p7-canonical-proof.yml
Cargo.lock
crates/formula-check/src/promotion.rs
crates/formula-check/tests/p7_promotion_policy.rs
crates/formula-core/src/lib.rs
crates/formula-core/src/promotion.rs
crates/formula-core/tests/p7_promotion_identity.rs
crates/formula-engine/Cargo.toml
crates/formula-first-light/Cargo.toml
crates/formula-first-light/tests/p7_fl_c_promotion.rs
crates/formula-packages/src/closure.rs
crates/formula-packages/tests/p7_closure_delta.rs
crates/formula-store/Cargo.toml
crates/formula-store/src/authority_store.rs
crates/formula-store/src/lib.rs
crates/formula-store/src/promotion_store.rs
crates/formula-store/tests/generation_replay.rs
crates/formula-store/tests/p7_promotion_transaction.rs
docs/superpowers/plans/2026-09-04-p7-promotion-generation-transition.md
tests/authority-boundary/tests/p7_promotion_authority.rs
```

The one-line `formula-engine/Cargo.toml` change removes the inherited normal dependency on `formula-store`; it is an authority-boundary hardening required by P7, not unrelated refactoring.

No P8 native-realization implementation or P9 second-query/reuse implementation is present in the reviewed range.

Temporary P7 development/diagnostic/repair workflows were removed before the final source proof and are absent from the final tree delta.

---

## 3. Canonical source proof

Exact source proof:

```text
workflow: P7 canonical proof
run:      33861803872
job:      100987699053
head:     ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
result:   success
runner:   ubuntu-24.04
Rust:     1.98.0
```

The canonical workflow is read-only (`permissions: contents: read`). Dependencies are primed once and all proof execution is locked/offline where applicable.

The exact source head passed:

```text
identity/toolchain
locked offline metadata
formula-archtest
formula-core all-targets
formula-store all-targets
formula-check all-targets
formula-packages all-targets
formula-engine all-targets
formula-first-light all-targets
workspace all-targets tests
workspace all-targets build
cargo fmt --all -- --check
workspace clippy --all-targets -D warnings
dependency trees
authority dependency firewall
clean worktree
all P7 proof markers
```

---

## 4. Promotion identity and lifecycle law

`PromotionCandidate` is a canonical identity binding the exact frozen candidate, promotion manifest, expected parent generation, proof generation, dependency cone, and supersession lineage.

Set-like inputs are normalized and deduplicated before structural identity is computed.

Lifecycle states are explicit and structurally distinct:

```text
CERTIFIED
ADMITTED
ACTIVATED
QUARANTINED
```

A state change therefore cannot be represented as an untracked mutation of one identity.

---

## 5. Checker-owned authorization law

P7 does not allow the store or search engine to decide whether candidate mathematics is admissible.

The checker validates:

```text
frozen candidate identity
promotion-manifest identity
parent generation
candidate generation
proof generation / freshness boundary
dependency cone
checked evidence bindings
supersession lineage
underlying PromotionManifest admissions/evidence contract
```

Successful validation produces opaque `PromotionAuthorization`. Its authority-bearing fields are private and it exposes no public constructor.

Conflicts or invalid bindings fail closed to quarantine/rejection; they cannot become an authorized promotion by caller choice.

---

## 6. Raw-publication bypass closure

P1 originally exposed a generic generation-publication entry point before D5 promotion existed. P7 closes that bypass.

The public transition is now:

```text
checked frozen candidate
 -> PromotionAuthorization
 -> AuthorityStore::promote(&PromotionAuthorization)
 -> atomic new UniverseGeneration
```

The low-level `publish_generation_inner` transaction remains crate-private inside `formula-store` and is reused rather than replaced. This preserves the previously proved P1 atomic transaction mechanics while preventing search/application code from manufacturing a new authoritative generation directly.

Architecture tests also prove production `formula-engine` does not depend on `formula-store`, `formula-check`, or `formula-first-light`.

---

## 7. Atomic U0 -> U1 and history preservation

`AuthorityStore::promote` requires the currently active generation to match the authorization parent exactly, replays that parent, constructs the next generation from existing admitted/bound authority plus the authorized delta, and publishes it through the P1 atomic transaction.

Negative tests cover stale-parent/race rejection and existing injected publication failures before active-root commit.

After successful FL-C promotion:

```text
U0 remains replayable under its original digest
U0 canonical bytes remain unchanged
U1 has U0 as parent
U1 contains the authorized semantic primitive
U1 contains the authorized evidence binding
partial/failed transactions cannot expose U1
```

This is the bounded P7 proof of D5 history preservation and atomic generation growth.

---

## 8. Capability closure remains derived state

`CapabilityClosureDelta::between(before, after)` records deterministic added/removed capability differences between exact closure contexts.

The underlying closure algorithm remains unchanged in authority character: capabilities are derived only from activated packages and generation-admitted, authority-bound structure witnesses.

P7 therefore records the effect of a generation transition without turning capability closure into an authority source.

---

## 9. FL-C semantic primitive promotion

The P7 First-Light integration test preserves the P6 blindness boundary:

```text
bounded public U8/Boolean CandidateSpace
 -> extract frozen candidate
 -> sealed FL-C oracle returns exact counterexample or equivalence
 -> refine whole bounded space
 -> final frozen candidate
 -> independent checker authorization
 -> store promotion
 -> U1 admission
```

`formula-first-light` gains `formula-check`, `formula-store`, and `tempfile` as **dev-dependencies only** for the integration test. Its production dependencies remain `formula-core` + `formula-engine`; it does not become a production authority-store client.

The admitted FL-C primitive is the digest of the independently equivalent final expression. The test additionally proves the lifecycle identities remain distinct and `U0` stays byte-replayable after `U1` creation.

---

## 10. Canonical-proof correction history

The semantic P7 implementation, architecture gates, workspace tests, and build were green before canonical proof reached formatting.

The first canonical source candidate failed closed only at:

```text
cargo fmt --all -- --check
```

A read-only pinned Rust 1.98.0 diagnostic recovered the exact formatter scope. Canonical rustfmt touched exactly 11 Rust files:

```text
crates/formula-check/src/promotion.rs
crates/formula-check/tests/p7_promotion_policy.rs
crates/formula-core/src/promotion.rs
crates/formula-core/tests/p7_promotion_identity.rs
crates/formula-first-light/tests/p7_fl_c_promotion.rs
crates/formula-packages/tests/p7_closure_delta.rs
crates/formula-store/src/lib.rs
crates/formula-store/src/promotion_store.rs
crates/formula-store/tests/generation_replay.rs
crates/formula-store/tests/p7_promotion_transaction.rs
tests/authority-boundary/tests/p7_promotion_authority.rs
```

A one-shot formatter helper enforced that exact 11-path allowlist, ran formatting checks, reran P7 crate/authority tests, and ran full workspace Clippy with `-D warnings` before it was permitted to commit formatter output. The helper then removed itself.

No authority rule, dependency contract, candidate set, promotion policy, or test expectation was weakened to satisfy formatting/lint.

Canonical run `33861803872` subsequently passed the unchanged full P7 proof on exact clean source SHA `ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2`.

---

## 11. P7 canonical proof markers

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

---

## 12. P0–P6 remain authority

P7 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable atomic authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
P4 deterministic query compiler + campaign planning
P5 bounded exact CandidateSpace/discovery
P6 sealed First-Light target harness + blindness gates
```

The P7 canonical workflow reruns predecessor crate/workspace/architecture gates and preserves the sealed-target/search boundary.

---

## 13. Not proved by P7

Do not claim from P7:

```text
native CPU realization generation
native realization equivalence/admission
binary/source dispatch integrity
second-query reuse under U1
proof that synthesis is skipped on reuse
complete P9 First-Light proof manifest
FIRST_LIGHT_COMPLETE
external SAT/SMT/CAS execution
Ptah/distributed execution
P8 completion
P9 completion
```

---

## 14. Next implementation boundary

The frozen roadmap phase after P7 is P8 — D4 native realization and validation.

Do **not** start P8 until the documentation-bearing P7 branch head passes the unchanged P7 canonical workflow.

P8 must consume the P7-admitted FL-C semantic construction, generate a bounded native CPU realization, independently validate it over all 256 U8 inputs, and keep realization authority distinct from mathematical authority. P9 reuse logic remains out of P8 scope.

---

## 15. Freeze state

P7 source is proved and scope/review-clean on:

```text
ca61ed42ab47b3a79a3d258f015b2f6ac9979ec2
```

This checkpoint plus the accompanying `CURRENT.md` update form the documentation-bearing branch candidate. **P7 is not finally frozen until the unchanged P7 canonical workflow succeeds on that exact documentation-bearing head.**

This branch has not been merged to `main`. P8 has not started.
