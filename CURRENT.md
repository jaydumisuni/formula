# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

This file is the shortest authoritative recovery path for a new chat/session. Recover repository evidence before reasoning. Do not reconstruct state from chat memory when these files are available.

## Exact current state

Broad research is closed as the lead activity. The canonical D1–D5 architecture, First-Light specification, and implementation roadmap are frozen.

Primary authorities:

1. [`docs/design/README.md`](docs/design/README.md) — canonical D1–D5 design index and precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen implementation phases P0 onward.
3. [`docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md`](docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md) — proved P1 milestone and current implementation authority.
4. [`docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md`](docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md) — proved P0 milestone retained as predecessor evidence.
5. [`docs/superpowers/plans/2026-08-29-p1-core-identity-authority-store.md`](docs/superpowers/plans/2026-08-29-p1-core-identity-authority-store.md) — executed P1 RED/GREEN plan.
6. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for concrete design/implementation gaps.

## Current implementation milestone

**P1 — Core Identity + Authority Store: PROVED on the isolated implementation branch.**

Canonical branch:

```text
implementation/p1-core-identity-authority-store
```

Canonical source-under-test proof boundary:

```text
5218ce5cd35636c080ad569391d48aa62f5d3cc0
workflow run 33236548151
job 99058414531
```

P1 proof checkpoint:

```text
docs/checkpoints/2026-08-29-p1-core-identity-authority-store.md
```

Pinned toolchain:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

### P1 proof markers

```text
P1-01 deterministic structural identity               PASS
P1-02 structural identity separate from equivalence   PASS
P1-03 immutable content-addressed backing             PASS
P1-04 atomic generation publication                   PASS
P1-05 injected publication failures preserve U0       PASS
P1-06 historical roots replay after fresh reopen      PASS
P1-07 replay rejects manifest/blob corruption         PASS
P1-08 explicit locked runtime dependency closure      PASS
P1-09 P0 architecture/build firewall remains green    PASS
```

## What P1 now proves

P1 adds the minimum durable D1/D2 authority substrate on top of proved P0:

```text
deterministic canonical encoding
SHA-256 ArtifactDigest structural identity
immutable semantic artifact schemas
structural identity != semantic equivalence
UniverseGeneration structural roots
immutable content-addressed blob storage
SQLite authority index
atomic generation publication
rollback-safe failure semantics
historical root reconstruction/replay
manifest/blob corruption detection during replay
explicit locked runtime dependency closure
```

Canonical identity examples include:

```text
SHA-256(b"abc")
sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad

{z:1,a:2} canonical object bytes
{"a":2,"z":1}

2/-4 -> -1/2
0/-9 -> 0/1
```

Publication failure has been injected both before active-root movement and after active-root movement but before commit. Both failures roll back and preserve the previous authoritative generation.

Historical generations replay after a fresh store reopen, and replay rejects corruption in either the generation manifest file or its content-addressed blob backing.

## Dependency authority

The exact P1 `Cargo.lock` is frozen and the normal First-Light/CLI runtime closure is explicitly allowlisted.

```text
Cargo.lock Git blob SHA:
ccf6e1cb9e64e5ff0cf80ce6bdcc92e9a594ad4d

Cargo.lock byte SHA-256:
b9e8452c3d354de5c98e36492c9117fb88aa8b2d234ac3286cf8899d5edd56db
```

P1 runtime allowlist contains 26 package names and is enforced by the existing authority-boundary dependency firewall.

## P0 remains proved

P1 does not replace P0. The P0 repository/build and architecture boundaries remain required and remained green in the P1 canonical proof:

```text
formula-check -> formula-core only

formula-engine -> formula-core
               -> formula-store -> formula-core
               -> formula-packages -> formula-core

formula-first-light -> formula-core
                    -> formula-engine
```

`formula-engine` still does not link checker implementation. Sealed First-Light fixtures remain source-separated from discovery crates.

## Not proved yet

P1 intentionally stops before later mathematical/execution semantics. Do **not** claim any of the following from P1:

```text
P2 dimension/unit algebra correctness
P2 affine/delta semantics
parser correctness
evaluator correctness
discovery/search correctness
certificate checker semantics
full promotion semantics beyond the P1 generation transaction
First Light
native realization correctness
Ptah/distributed execution
```

## Next implementation boundary

**Next phase: P2 — Dimensions, Units, Affine/Delta Semantics.**

P2 must build on, and preserve, the proved P1 identity/authority substrate. Do not bypass structural identity, immutable backing, or atomic generation authority to simplify later semantic work.

Use the frozen roadmap for P2 scope and proof obligations before implementation. Mandatory RED→GREEN TDD remains in force.

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
4. Read the P1 checkpoint and inspect the current branch/head before assuming the checkpoint is still the latest proved state.
5. Verify the post-checkpoint P1 proof remains green before treating P1 as the final branch candidate.
6. Start P2 only from the proved P1 substrate using a fresh isolated implementation boundary and mandatory RED→GREEN TDD.
7. Do not return to the older B01 precursor as canonical authority; the later frozen roadmap supersedes it.
8. Do not reopen broad research or redesign frozen milestones unless concrete implementation evidence exposes a contradiction or unsupported obligation.
9. Preserve the temporary project-name rule until the mathematical product identity is mature enough to name.

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
