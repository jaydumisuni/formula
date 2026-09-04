# P6 Checkpoint — First-Light Target Harness + Blindness

**Recorded:** 2026-09-04  
**Status:** PROVED SOURCE — sealed First-Light harness/blindness boundary; documentation-bearing head still requires exact-head canonical proof  
**Branch:** `implementation/p6-first-light-target-harness-blindness`  
**P5 exact frozen predecessor:** `d2bd250c4b4419316292845a44849747d9e01113`  
**Source-under-test commit:** `3d50226f51066d3b3fd2562080d67105c004ea92`  
**Canonical source proof run:** `33854085182`  
**Canonical source proof job:** `100963233751`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P6  
**Implementation plan:** `docs/superpowers/plans/2026-09-04-p6-first-light-target-harness-blindness.md`

---

## 1. Scope

P6 adds the sealed First-Light target harness required before promotion work begins. It consumes P5 candidate-only search outputs while keeping sealed target semantics outside discovery authority.

Frozen P6 source surfaces include:

- deterministic `BlindnessManifest` identity binding target, sealed-target digest, generation, World, query, grammar/routes, package set, and oracle contract;
- deterministic `FrozenSubmission` identity binding target + frozen candidate;
- FL-A sealed oracle and target identity;
- FL-B public Boolean/XOR system plus exact direct/GF(2) route contract identities;
- FL-C sealed U8 target oracle, public grammar identity, and explicit visible near-miss;
- runtime blindness tests;
- architecture firewalls preventing `formula-engine` / `formula-packages` from depending on the sealed First-Light harness, sealed fixture paths, target schemas, or hidden final answers.

P6 does **not** certify/promote discovered mathematics, perform `U0 -> U1`, generate native CPU realizations, prove realization equivalence, or complete the final First-Light campaign. Those remain P7–P9.

---

## 2. Exact predecessor and review boundary

P6 was implemented from exact frozen P5 documentation-bearing head:

```text
d2bd250c4b4419316292845a44849747d9e01113
```

The final proved P6 source boundary is:

```text
3d50226f51066d3b3fd2562080d67105c004ea92
```

Exact compare evidence:

```text
base:    d2bd250c4b4419316292845a44849747d9e01113
head:    3d50226f51066d3b3fd2562080d67105c004ea92
status:  ahead
ahead:   31 commits
behind:  0 commits
```

The final delta is confined to the P6 canonical workflow, `formula-first-light` sealed-target/harness surfaces, P6 tests, P6 authority-boundary tests, and the P6 plan. The temporary P6 development workflow and one-shot rustfmt helper were retired before the final source proof.

No P7 promotion, P8 native-realization, or P9 full First-Light implementation is present in the reviewed range.

---

## 3. Canonical source proof

Exact source proof:

```text
workflow: P6 canonical proof
run:      33854085182
job:      100963233751
head:     3d50226f51066d3b3fd2562080d67105c004ea92
result:   success
runner:   ubuntu-24.04
Rust:     1.98.0
```

The read-only canonical workflow passed:

```text
identity/toolchain
locked offline metadata
architecture tests
formula-core tests
formula-store tests
formula-check tests
formula-packages tests
formula-engine tests
formula-first-light tests
workspace tests
workspace build
rustfmt
Clippy -D warnings
dependency trees
discovery/sealed dependency firewall
clean worktree
```

---

## 4. Blindness and authority law

P6 keeps sealed target semantics in `formula-first-light`; discovery implementation remains blind.

The architecture tests reject:

```text
formula-engine -> formula-first-light
formula-packages -> formula-first-light
sealed fixture path references from discovery source
sealed FL-A / FL-C target schema references from discovery source
embedded final FL-C expression in discovery source
embedded expanded FL-A coefficient sequence in discovery source
```

The hidden oracle may judge a frozen candidate, but it cannot feed target implementation details backward into P5 search code.

`FrozenSubmission` is still candidate-only. P6 creates no mathematical authority.

---

## 5. FL-A / FL-B / FL-C boundary

### FL-A

The target oracle is sealed inside `formula-first-light` and exposes only evaluation behavior plus a target digest to the blindness manifest.

### FL-B

The Boolean/XOR system is intentionally public. P6 exposes deterministic identities for the public problem, direct route, GF(2) route, and exact-preserving route contract so P7/P9 can prove route behavior without pretending the public fixture is sealed.

### FL-C

The exact target predicate remains sealed in `formula-first-light`. The public P5 grammar remains independent. A known false near-miss is intentionally visible so the eventual campaign can prove that search/certification rejects a plausible but wrong candidate.

---

## 6. Canonical-proof correction history

The semantic P6 implementation and development gates were already green when the first canonical workflow failed closed at rustfmt.

Pinned Rust 1.98.0 `cargo fmt --all` showed the remaining formatting delta touched only legitimate Rust surfaces:

```text
crates/formula-first-light/src/fl_c.rs
crates/formula-first-light/tests/p6_blindness_runtime.rs
crates/formula-first-light/tests/p6_fl_a.rs
crates/formula-first-light/tests/p6_manifest.rs
tests/authority-boundary/tests/p6_blindness.rs
```

A one-shot write-enabled formatter helper was scope-guarded to those Rust surfaces, applied canonical rustfmt, and was removed. The temporary P6 development workflow was also removed before final source proof.

No semantic change, authority weakening, blindness weakening, dependency change, or target leakage was used to satisfy the canonical gate.

---

## 7. P6 proof markers

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

---

## 8. P0–P5 remain authority

P6 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable generation authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
P4 deterministic query compiler + campaign/obligation/work-cell planning
P5 bounded candidate-only CandidateSpace + discovery
```

---

## 9. Not proved by P6

Do not claim from P6:

```text
independent certification of discovered First-Light candidates
promotion/admission of discovered mathematics
atomic U0 -> U1 transition
native CPU realization generation
independent realization-equivalence proof
reuse from U1 without rediscovery
full P9 First-Light completion
external SAT/SMT/CAS authority
model/LLM mathematical authority
Ptah/distributed execution
```

---

## 10. Next boundary

After the documentation-bearing P6 head passes the unchanged P6 canonical workflow, P6 becomes frozen/proved and P7 may begin from that exact SHA.

P7 is the atomic promotion/admission phase for the independently certified First-Light mathematics. Search and sealed-target harness code must not acquire authority-store publication power.
