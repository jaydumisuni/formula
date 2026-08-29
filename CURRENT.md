# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

This file is the shortest authoritative recovery path for a new chat/session. Recover repository evidence before reasoning. Do not reconstruct state from chat memory when these files are available.

## Exact current state

Broad research is closed as the lead activity. The canonical D1–D5 architecture, First-Light specification, and implementation roadmap are frozen.

Primary authorities:

1. [`docs/design/README.md`](docs/design/README.md) — canonical D1–D5 design index and precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen implementation phases P0 onward.
3. [`docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md`](docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md) — proved P2 milestone and current implementation authority.
4. [`docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md`](docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md) — proved P1 predecessor retained as authority-store evidence.
5. [`docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md`](docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md) — proved P0 predecessor evidence.
6. [`docs/superpowers/plans/2026-08-29-p2-independent-checker-certificate-core.md`](docs/superpowers/plans/2026-08-29-p2-independent-checker-certificate-core.md) — executed P2 RED/GREEN plan.
7. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for concrete design/implementation gaps.

## Current implementation milestone

**P2 — Independent Checker + Certificate-Envelope Core: PROVED on the isolated implementation branch.**

Canonical branch:

```text
implementation/p2-independent-checker-certificate-core
```

Exact P1 predecessor branch head:

```text
0247ed7db4b9829b7e0f2f3095e87fbf4a0f39a1
```

Canonical P2 source-under-test proof boundary:

```text
6f42bc31158fe9121270d36561b86c02e8e956ab
workflow: P2 canonical proof
workflow run: 33245330788
job: 99081697417
conclusion: success
```

P2 proof checkpoint:

```text
docs/checkpoints/2026-08-29-p2-independent-checker-certificate-core.md
```

Pinned toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0
```

### P2 proof markers

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

## What P2 now proves

P2 adds the minimum D2/D4/D5 independent certification substrate on top of proved P0/P1:

```text
immutable frozen-candidate identity
exact certificate-envelope binding
checker identity + trust-root binding
fail-closed Authority Contract matching
no silent probabilistic/empirical downgrade
exact polynomial identity checking
independent Boolean/GF(2) translation verification
independent GF(2) witness verification against original constraints
exhaustive U8 equivalence over all 256 inputs
promotion-manifest structural preflight
independent realization artifact + binding verification
independent realization semantic equivalence
malicious-producer rejection
checker/producer architecture firewall
```

The checker is structurally isolated from the producer/search path:

```text
formula-check -> formula-core
              -> num-bigint

formula-check -/-> formula-engine
formula-check -/-> formula-realize
formula-check -/-> formula-first-light
formula-check -/-> formula-store

formula-engine -/-> formula-check implementation
```

A producer-local success claim cannot create authority. Certification requires the independent checker path and exact frozen inputs.

## Negative controls proved

The dedicated malicious-producer fixture rejects:

```text
forged certificate body
certificate reused for another target
candidate mutation after certification
PROBABILISTIC/EMPIRICAL downgrade under exact contract
compiler success without independent realized-output proof
```

The exhaustive U8 near-miss without the zero guard fails at exact counterexample `x = 0`.

## Dependency authority

P2 adds only one direct checker dependency edge:

```text
formula-check -> num-bigint 0.4.8
```

`num-bigint` already existed in the proved P1 dependency closure through `formula-core`; P2 introduced no new package version for this checker feature. The exact lock metadata change was generated by pinned Cargo.

The canonical P2 dependency-tree proof confirms that `formula-check` contains no engine, realization-producer, First-Light, or store implementation dependency.

## P0 and P1 remain proved

P2 does not replace predecessor authority. The canonical P2 workflow reruns and preserves:

```text
P0 repository/build architecture firewall
P0 sealed First-Light fixture boundary
P1 deterministic structural identity
P1 immutable content-addressed backing
P1 atomic generation publication
P1 rollback/fault-injection semantics
P1 historical generation replay
P1 replay corruption rejection
```

P1 artifact canonical identity was not redesigned for P2. P2 adds read-only accessors needed by the checker; canonical structural projections remain unchanged.

## Not proved yet

P2 intentionally stops at the independent checker/certificate core and initial exact evidence families. Do **not** claim any of the following from P2:

```text
P3 theory-package semantics
P3 structure inference
P3 capability closure
P3 federation contracts
full discovery/search correctness
full promotion orchestration beyond structural preflight
parser correctness
evaluator correctness
First Light
native realization generation correctness beyond the P2 independent comparison harness
Ptah/distributed execution
```

## Next implementation boundary

The frozen roadmap names the next phase:

**P3 — Theory packages, structure inference, capability closure, federation contracts.**

P3 is **not started** by this checkpoint and requires separate authorization.

P3 must build on and preserve the proved P0/P1/P2 authority substrate. It may consume P2 checker/certificate interfaces but must not let search, package logic, realization production, or resource pressure manufacture or weaken authority.

## Canonical First-Light end target remains unchanged

First Light must eventually prove the complete growth loop on ordinary local hardware:

```text
U_0
 -> blind target
 -> structured discovery
 -> reject false candidates
 -> independently certify
 -> admit + activate
 -> U_1
 -> generate native realization
 -> independently validate realization
 -> solve a related second query using the promoted primitive
 -> prove reuse without rediscovery
```

Canonical First Light remains:

```text
local
CPU-only
model-free
network-free during execution
exact/replay-bound
independently checked
content-addressed
bound to exact Universe/package/grammar/source digests
```

Ptah remains explicitly deferred until the local First-Light proof requires distributed execution.

## Constitutional laws to preserve

1. Search may propose mathematics. Only Certification + Promotion can create mathematical authority.
2. Execution may consume authority. Execution cannot manufacture authority.
3. No representation, implementation, proof language, model, or solver is the mathematics itself.
4. Mathematical correctness and realization correctness are separate proof obligations.
5. Candidate/search state is outside admitted `U_g` authority.
6. Resource exhaustion never weakens the requested Authority Contract.
7. Models may generate candidates later but have no mathematical authority.
8. Promotion is generation-producing and atomic; accepted history is immutable.
9. A false/broken realization cannot invalidate already admitted mathematics.
10. Research reopens only when an implementation/design choice is unsupported, contradicted, or materially under-specified by preserved evidence.

## Recovery procedure for any new chat

1. Read this file.
2. Read [`docs/design/README.md`](docs/design/README.md).
3. Read the frozen implementation roadmap.
4. Read the P2 checkpoint and inspect the current P2 branch/head before assuming the source-proof commit is still the latest proved state.
5. Verify the post-checkpoint P2 proof remains green before treating the documentation-bearing branch head as the final P2 branch candidate.
6. Preserve the exact P1 predecessor evidence at `0247ed7db4b9829b7e0f2f3095e87fbf4a0f39a1` when reviewing P2 history.
7. Do not start P3 without separate authorization and a fresh isolated implementation boundary.
8. Do not return to the older B01 precursor as canonical authority; the later frozen roadmap supersedes it.
9. Do not reopen broad research or redesign frozen milestones unless concrete implementation evidence exposes a contradiction or unsupported obligation.
10. Preserve the temporary project-name rule until the mathematical product identity is mature enough to name.

## Evidence precedence

When sources disagree, use this order:

```text
current repository implementation/proof evidence
    > later frozen design amendment explicitly superseding an older milestone
    > D5/D4/D3 for later operational/build/self-expansion choices
    > D2 for operational machine contracts
    > D1 for mathematical constitution
    > preserved research checkpoints
    > chat recollection
```

No chat should claim a later phase is complete merely because a design or implementation plan exists. Every roadmap phase requires its own proof gate.

## Freeze state

P2 is frozen on its isolated implementation branch pending only the post-checkpoint proof of the documentation-bearing branch head.

This branch has **not** been merged to `main`. P3 has **not** been started.