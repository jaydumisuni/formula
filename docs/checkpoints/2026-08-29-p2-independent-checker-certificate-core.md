# P2 Checkpoint — Independent Checker + Certificate-Envelope Core

**Date:** 2026-08-29  
**Status:** PROVED — independent certificate validation, exact evidence families, frozen-candidate binding, promotion preflight, and realization equivalence  
**Branch:** `implementation/p2-independent-checker-certificate-core`  
**P1 exact predecessor branch head:** `0247ed7db4b9829b7e0f2f3095e87fbf4a0f39a1`  
**Source-under-test commit:** `6f42bc31158fe9121270d36561b86c02e8e956ab`  
**Canonical source proof run:** `33245330788`  
**Canonical source proof job:** `99081697417`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P2  
**Implementation plan:** `docs/superpowers/plans/2026-08-29-p2-independent-checker-certificate-core.md`

---

## 1. Scope

This checkpoint proves canonical roadmap phase **P2 — Independent checker and certificate-envelope core**.

P2 establishes the first authority-producing checking path that is structurally independent from candidate generation/search and realization production.

Frozen P2 surfaces include:

- immutable `FrozenCandidate` identity;
- exact `CertificateEnvelope` structural binding;
- checker identity, checker trust-root, evidence-family, and evidence-family-version binding;
- fail-closed Authority Contract matching with no silent exactness downgrade;
- exact polynomial identity checking over integer coefficients;
- independent Boolean-to-GF(2) translation validation plus witness validation;
- exhaustive U8 semantic equivalence over all 256 inputs;
- structural promotion-manifest preflight against a frozen candidate and exact checked-evidence set;
- independent realization artifact/binding/equivalence validation;
- malicious-producer negative controls proving producer-local success cannot manufacture authority;
- architecture firewalls preventing `formula-check` from depending on producer/search paths and preventing `formula-engine` from linking checker implementation.

P2 does **not** publish Universe generations itself. Atomic generation authority remains a P1 store responsibility, and later promotion orchestration remains a later phase. P2 also does not claim discovery/search correctness, package/capability closure, parser/evaluator correctness, First Light, or distributed execution.

---

## 2. Exact predecessor and review boundary

P2 was isolated from the exact final P1 branch head:

```text
0247ed7db4b9829b7e0f2f3095e87fbf4a0f39a1
```

The reviewed source proof boundary is:

```text
6f42bc31158fe9121270d36561b86c02e8e956ab
```

Repository compare evidence for that exact range reported:

```text
base:   0247ed7db4b9829b7e0f2f3095e87fbf4a0f39a1
head:   6f42bc31158fe9121270d36561b86c02e8e956ab
ahead:  24 commits
behind: 0 commits
```

Review found no blocking authority gap. The implementation diff is confined to the intended P2 certification/checker surfaces, tests, architecture proof, the P2 workflow/plan, and the direct checker dependency edge described below. No `formula-engine`, search/discovery, or `formula-store` production implementation was changed.

P1 artifact-schema changes are accessor-only. Their canonical structural-identity projections remain unchanged, and the canonical P2 proof includes a regression test that read-only P2 access does not change P1 structural identity.

---

## 3. Canonical proof environment

The canonical source proof ran from exact commit `6f42bc31158fe9121270d36561b86c02e8e956ab` on GitHub-hosted Ubuntu 24.04 using the pinned Rust 1.98.0 toolchain.

```text
workflow: P2 canonical proof
run:      33245330788
job:      99081697417
result:   success
runner:   ubuntu-24.04
```

Pinned toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
commit-hash: 88d9e12ae178fab0fb5cc050a94da85685d449ea
host: x86_64-unknown-linux-gnu
release: 1.98.0

cargo 1.98.0
```

The proof workflow has `contents: read`. Network access is used only to checkout/provision the pinned toolchain and prime Cargo's locked cache. Canonical Cargo metadata/tests/build/Clippy/tree operations then execute with `--locked --offline` where applicable.

---

## 4. Frozen candidate and certificate-envelope identity

`FrozenCandidate` structurally commits to the candidate kind, proposed judgements, World, Universe generation, dependencies, targets, Authority Contract, and Observer. Set-like inputs are normalized deterministically before structural hashing.

`CertificateEnvelope` binds:

```text
frozen candidate digest
target judgement
Universe generation
World
scope
verdict
verification mode
evidence family + family version
certificate body digest
producer identity
checker identity
checker trust root
dependency digests
Authority Contract digest
Observer digest
replay binding
```

The checker re-hashes supplied certificate-body bytes and rejects any mismatch before authority matching.

Proved envelope rejection classes include:

- forged certificate body;
- changed candidate after certification;
- wrong target;
- wrong World or generation;
- omitted dependency;
- checker identity/trust-root mismatch;
- unsupported family/version;
- Authority Contract mismatch;
- probabilistic/empirical downgrade under an exact deterministic contract.

---

## 5. Authority Contract is fail-closed

P2 does not infer weaker evidence as “good enough.”

For the frozen exact checker family, authoritative matching requires:

```text
requested authority class = deterministic-proof
requested exactness       = exact
verification mode         = EXHAUSTIVE or DETERMINISTIC-EXACT
evidence family           = explicitly allowlisted by the Authority Contract
```

`PROBABILISTIC` and `EMPIRICAL` modes are rejected under that contract. Resource or producer claims cannot silently weaken the requested authority class.

---

## 6. Exact polynomial identity checker

The polynomial checker compares normalized integer coefficient vectors using arbitrary-precision `BigInt` coefficients.

Proved behavior:

- exact coefficient equality passes;
- trailing zero coefficients normalize away;
- a coefficient change fails;
- the expanded difference-of-seventh-powers identity passes exactly;
- a near-miss polynomial that agrees on sampled values but is not universally identical fails.

No finite sampling path can produce polynomial-identity authority.

---

## 7. Boolean / GF(2) translation and witness checker

The GF(2) checker independently canonicalizes parity rows modulo 2 and verifies the claimed translated system against the original Boolean constraints.

Authority requires both:

1. exact translation agreement; and
2. a correctly sized witness satisfying the original Boolean constraint system.

Proved failures include changed right-hand side, omitted variables in translation, wrong witness width, and a witness that satisfies the claimed GF(2) system while failing the original Boolean semantics.

---

## 8. Exhaustive U8 semantic equivalence

The bounded U8 checker evaluates both semantic expressions for **every input `0..=255`** using explicit U8 wrapping semantics.

Proved results include:

- the canonical power-of-two predicate with nonzero guard passes all 256 inputs;
- removing the zero guard fails at the exact counterexample `x = 0`;
- an operator mutation returns its first exact counterexample;
- subtraction is evaluated with U8 wrapping semantics rather than unbounded-integer semantics.

P2 therefore does not substitute sample agreement for authority on the finite U8 domain.

---

## 9. Promotion-manifest preflight

The P2 promotion checker is intentionally structural and non-publishing.

It verifies that a proposed promotion manifest binds to:

- the exact frozen candidate digest;
- the exact expected parent Universe generation;
- the exact checked-evidence set, normalized as a set;
- admissions already covered by the frozen candidate's proposed judgements.

It has no store publication authority and no dependency on `formula-store`. Atomic generation publication remains outside `formula-check`.

---

## 10. Independent realization equivalence

The realization checker validates bindings before semantic execution:

```text
semantic target
realization identity
Universe generation
World
Authority Contract
Observer
realization artifact digest
```

It independently re-hashes the supplied artifact bytes, requires exactly 256 realized U8 outputs, and compares each output with the admitted semantic expression.

A compiler/optimizer “success” signal has no authoritative input field. Producer success therefore cannot self-admit a realization.

Proved failures include missing output coverage, artifact digest mismatch, binding mismatch, and exact output counterexamples.

---

## 11. Malicious-producer negative control

The dedicated adversarial fixture proves that producer-local success is non-authoritative.

The following attempts all terminate in non-PASS:

```text
forged certificate body
    -> CertificateBodyDigestMismatch

reuse certificate on another target
    -> TargetMismatch

mutate candidate after certification
    -> FrozenCandidateMismatch

downgrade exact proof to PROBABILISTIC / EMPIRICAL
    -> AuthorityInsufficient

compiler reports success without independent realized-output proof
    -> RealizationOutputCoverageMismatch
```

This is the roadmap's P2 malicious-producer negative control: the producer may claim success, but the independent checker owns the proof verdict used for authority.

---

## 12. Architecture firewall

The authority-boundary test executes `cargo tree` and proves both directions of the P2 separation:

```text
formula-check
    -> formula-core
    -> no formula-engine
    -> no formula-realize
    -> no formula-first-light

formula-engine
    -> no formula-check implementation dependency
```

The checker therefore shares immutable schemas through `formula-core` but cannot call the producer/search/realization path that generated the candidate under review. The engine may later submit frozen artifacts to an independent checker process, but it cannot link the checker implementation.

---

## 13. Dependency freeze

P2 adds one direct dependency edge to `formula-check`:

```text
formula-check
├── formula-core
└── num-bigint 0.4.8
```

`num-bigint` already existed in the proved P1 dependency closure through `formula-core`; P2 introduced no new package version for this checker feature. The `Cargo.lock` change was generated by pinned Cargo and records the new direct workspace-package dependency edge.

The canonical source proof's `cargo tree --locked --offline -p formula-check` confirms that the checker closure contains no engine, realization producer, First-Light, or store implementation.

---

## 14. RED -> GREEN and correction evidence

P2 was implemented through observed RED -> GREEN boundaries for each subsystem:

1. certification identities;
2. envelope and Authority Contract router;
3. exact polynomial identity checker;
4. Boolean/GF(2) translation + witness checker;
5. exhaustive U8 semantic checker;
6. promotion-manifest preflight;
7. realization-equivalence checker;
8. malicious-producer adversarial fixture.

One dependency-freeze correction occurred when `formula-check` gained the direct `num-bigint` edge. Pinned Cargo correctly rejected the stale lock metadata under `--locked`; a one-shot pinned runner generated the exact lock change, and its write-enabled helper was immediately removed.

The first canonical P2 proof then reached `cargo fmt --check` after all preceding semantic/build gates passed and failed only on formatting. Rust 1.98.0 `rustfmt` was applied with a one-shot helper whose path guard proved that only the eleven rustfmt-reported source files plus deletion of the helper changed. The canonical proof was rerun from scratch afterward and passed.

---

## 15. Canonical source proof sequence

The successful canonical workflow executed:

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
test -z "$(git status --porcelain)"
```

Result: **PASS** on exact source-under-test commit `6f42bc31158fe9121270d36561b86c02e8e956ab`.

---

## 16. P2 proof markers

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

All eleven markers were emitted by canonical job `99081697417`, which concluded successfully.

---

## 17. P0/P1 preservation

P2 extends rather than replaces the proved predecessor substrate.

The canonical P2 gate reruns:

- P0 architecture and sealed-fixture boundaries;
- P1 canonical identity tests;
- P1 immutable store tests;
- P1 publication rollback/fault-injection tests;
- P1 historical replay and corruption rejection tests.

Those predecessor gates remained green at the P2 source proof boundary.

---

## 18. Milestone boundary

**P2 is proved on the isolated implementation branch for the source-under-test commit recorded above.**

The authoritative claim stops at the independent checker/certificate core and its frozen initial evidence families.

This branch has **not** been merged to `main`.

A post-checkpoint canonical proof must remain green with this checkpoint and `CURRENT.md` present before the documentation-bearing branch head is treated as the final P2 branch candidate.

---

## 19. Next phase

The frozen roadmap names the next boundary:

```text
P3 — Theory packages, structure inference, capability closure, federation contracts
```

P3 has **not** been started by this checkpoint and requires separate authorization. It must preserve the P0/P1/P2 authority boundaries rather than weakening them.