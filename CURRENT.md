# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

Recover repository evidence before reasoning. Do not reconstruct implementation state from chat memory when these files are available.

## Primary authorities

1. [`docs/design/README.md`](docs/design/README.md) — frozen D1–D5 design precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen roadmap P0 onward.
3. [`docs/checkpoints/2026-09-04-p6-first-light-target-harness-blindness.md`](docs/checkpoints/2026-09-04-p6-first-light-target-harness-blindness.md) — current P6 source-proof checkpoint.
4. [`docs/checkpoints/2026-09-02-p5-candidate-space-bounded-discovery.md`](docs/checkpoints/2026-09-02-p5-candidate-space-bounded-discovery.md) — exact frozen P5 predecessor checkpoint.
5. [`docs/superpowers/plans/2026-09-04-p6-first-light-target-harness-blindness.md`](docs/superpowers/plans/2026-09-04-p6-first-light-target-harness-blindness.md) — executed P6 implementation plan.
6. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for a concrete contradiction or missing obligation.

## Exact current implementation state

**P6 — First-Light Target Harness + Blindness: SOURCE PROVED and scope/review-clean; documentation-bearing head awaiting exact-head proof.**

Canonical branch:

```text
implementation/p6-first-light-target-harness-blindness
```

Exact frozen P5 predecessor:

```text
d2bd250c4b4419316292845a44849747d9e01113
workflow: P5 canonical proof
workflow run: 33812388173
conclusion: success
```

Canonical P6 source-under-test proof boundary:

```text
3d50226f51066d3b3fd2562080d67105c004ea92
workflow: P6 canonical proof
workflow run: 33854085182
job: 100963233751
conclusion: success
```

Exact P5 -> P6 source compare:

```text
base:    d2bd250c4b4419316292845a44849747d9e01113
head:    3d50226f51066d3b3fd2562080d67105c004ea92
status:  ahead
ahead:   31 commits
behind:  0 commits
```

Pinned proof toolchain:

```text
Rust 1.98.0
runner: ubuntu-24.04
Cargo proof commands: --locked / --offline where applicable
canonical workflow permissions: contents: read
```

## What P6 now proves

P6 supplies the sealed First-Light target harness required before promotion begins:

```text
BlindnessManifest exact semantic binding
FrozenSubmission exact target/candidate binding
FL-A sealed oracle + target digest
FL-B public XOR fixture + exact direct/GF(2) route identities
FL-C sealed U8 target oracle + public grammar digest
visible false FL-C near-miss
runtime blindness checks
discovery/sealed dependency firewall
hidden-answer literal firewall
```

## Authority boundary

P6 consumes P5 candidate-only search outputs; it does not manufacture mathematical authority.

Production discovery code remains unable to import or depend on the sealed First-Light harness:

```text
formula-engine   -/-> formula-first-light
formula-packages -/-> formula-first-light
```

Architecture tests also reject sealed fixture-path references, target-schema references, and embedded hidden First-Light answers inside discovery source.

The sealed oracle may judge a frozen candidate. It cannot make search code authoritative and cannot leak target implementation details backward into P5.

## Canonical proof correction

The semantic P6 implementation and development gates were already green when the canonical workflow failed closed at rustfmt.

Pinned Rust 1.98.0 `cargo fmt --all` identified only legitimate Rust formatting changes in `formula-first-light` tests/source plus `tests/authority-boundary/tests/p6_blindness.rs`.

A one-shot scope-guarded formatter helper applied only that canonical Rust formatting and was removed. The temporary P6 development workflow was then retired as required by the P6 plan.

Canonical run `33854085182` passed the unchanged full P6 proof on exact source SHA `3d50226f51066d3b3fd2562080d67105c004ea92`.

## P6 proof markers

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

## P0–P5 remain authority

P6 extends rather than replaces predecessor proofs:

```text
P0 reproducible repository/build + architecture firewall
P1 deterministic identity + immutable generation authority store
P2 independent checker/certificate authority
P3 theory packages + generation/world-scoped capability closure
P4 deterministic query compiler + campaign/obligation/work-cell planning
P5 bounded candidate-only CandidateSpace + discovery
```

## Not proved by P6

Do not claim from P6:

```text
P7 certification/promotion/admission completion
atomic U0 -> U1 First-Light growth
P8 native CPU realization + independent realization proof
P9 complete First-Light campaign/reuse proof
external SAT/SMT/CAS authority
model/LLM mathematical authority
Ptah/distributed execution
```

## Next implementation boundary

The frozen roadmap phase after P6 is P7.

Do **not** start P7 until the documentation-bearing P6 branch head passes the unchanged P6 canonical workflow.

P7 must independently certify the frozen First-Light candidates and perform atomic generation-producing promotion without granting search or sealed-target harness code authority-store publication power.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. Mathematical correctness and realization correctness are separate proof obligations.
4. Candidate/search/compiler state is outside admitted `U_g` authority unless explicitly promoted.
5. Resource exhaustion never weakens the requested Authority Contract and never becomes mathematical refutation.
6. Promotion is generation-producing and atomic; accepted history is immutable.
7. A false realization cannot invalidate already admitted mathematics.
8. Capability closure is derived state from exact admitted inputs, not an authority source.
9. Weak Shared Facts cannot silently satisfy stronger obligations.
10. Federation/certificate routing cannot weaken authority for cost or availability.
11. Compiler/campaign/work-cell/search state cannot publish or mutate authority.
12. Replay/provenance/candidate identity must bind every local semantic input capable of changing the result while excluding unrelated state.
13. Heuristic ranking cannot delete exact candidates or create authority.
14. Sealed First-Light targets cannot leak backward into P5 search implementation.

## Recovery procedure

1. Read this file.
2. Read the P6 checkpoint and executed P6 plan.
3. Inspect `implementation/p6-first-light-target-harness-blindness` before assuming the source-proof SHA is still the branch head.
4. Verify the post-checkpoint P6 canonical proof on the **exact documentation-bearing branch head** before treating P6 as finally frozen.
5. Preserve frozen P5 head `d2bd250c4b4419316292845a44849747d9e01113` as the predecessor review boundary.
6. Do not start P7 unless P6's documentation-bearing exact head has passed canonical proof.
7. Do not reopen broad research unless implementation evidence exposes a concrete contradiction or missing requirement.

## Freeze state

P6 source is proved and scope/review-clean on:

```text
3d50226f51066d3b3fd2562080d67105c004ea92
```

The P6 checkpoint and this `CURRENT.md` update now form the documentation-bearing branch candidate. **P6 is not finally frozen until the unchanged P6 canonical workflow succeeds on that exact documentation-bearing head.**

This branch has **not** been merged to `main`. P7 has **not** started.
