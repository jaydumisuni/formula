# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

This file is the shortest authoritative recovery path for a new chat/session. Recover repository evidence before reasoning. Do not reconstruct state from chat memory when these files are available.

## Exact current state

Broad research is closed as the lead activity. The canonical D1–D5 architecture, First-Light specification, and implementation roadmap are frozen.

Primary authorities:

1. [`docs/design/README.md`](docs/design/README.md) — canonical D1–D5 design index and precedence.
2. [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md) — frozen implementation phases P0 onward.
3. [`docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md`](docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md) — proved P0 milestone.
4. [`docs/superpowers/plans/2026-08-29-p0-repository-reproducible-build-skeleton.md`](docs/superpowers/plans/2026-08-29-p0-repository-reproducible-build-skeleton.md) — executed P0 plan and RED/GREEN sequence.
5. [`docs/research/`](docs/research/) — preserved research evidence; reopen only for concrete design/implementation gaps.

## Current implementation milestone

**P0 — Repository and reproducible build skeleton: PROVED on the isolated implementation branch.**

Canonical branch used for P0 development:

```text
implementation/p0-reproducible-skeleton
```

P0 proof checkpoint:

```text
docs/checkpoints/2026-08-29-p0-repository-reproducible-build-skeleton.md
```

The checkpoint records exact source/toolchain/blob identities, observed RED→GREEN boundaries, dependency trees, runtime allowlist, sealed-fixture boundaries, and the locked/offline proof commands.

The post-checkpoint branch gate also passed with the checkpoint present.

### P0 proof markers

```text
P0-01 pinned toolchain/source dependency manifest       PASS
P0-02 formula-check isolated from engine/search         PASS
P0-03 sealed fixtures unavailable to discovery crates  PASS
P0-04 canonical runtime dependency allowlist satisfied PASS
P0-05 deterministic P0 fixture identities              PASS
```

Pinned First-Light implementation toolchain at this milestone:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

P0 proves only the repository/build and architectural authority boundaries. It does **not** claim mathematical authority implementation, First Light, self-expansion, certificate semantics, promotion, or native mathematical realization.

## Canonical P0 workspace now exists

```text
crates/
  formula-core/
  formula-store/
  formula-check/
  formula-engine/
  formula-packages/
  formula-realize/
  formula-first-light/
  formula-cli/

tests/
  authority-boundary/
  first-light/
```

Constitutional dependency separation established at P0:

```text
formula-check -> formula-core only

formula-engine -> formula-core
               -> formula-store -> formula-core
               -> formula-packages -> formula-core

formula-first-light -> formula-core
                    -> formula-engine
```

`formula-engine` does not link the checker implementation. Sealed First-Light fixtures are source-separated from discovery crates. The canonical P0 runtime closure is explicitly allowlisted and contains no third-party runtime package.

## Next implementation boundary

**Next phase: P1 — Core structural identity and authority store.**

Do not return to the older B01 precursor plan as the canonical next task. The later frozen roadmap superseded it.

P1 begins the minimum D1/D2 durable authority implementation:

```text
Entity
Relation
World
Judgement
EvidenceEnvelope metadata
Realization metadata
ArtifactDigest
UniverseGeneration
AuthorityContract
Observer

canonical encoding v1
SHA-256 structural digests
content-addressed immutable blob store
local authority index/transaction layer
generation manifest build/load/replay
```

Required P1 proof obligations from the frozen roadmap:

```text
D2-P01 deterministic structural identity replay
D2-P02 atomic generation publication
D2-P03 semantic equivalence separate from digest identity
D2-P11 historical generation replay
```

Required negative boundaries include:

```text
field-order/canonicalization variation cannot change normalized digest
non-semantic timestamp/path cannot enter structural digest
blob mutation changes digest and is rejected
semantic equivalence never aliases structural digest identity
```

No P1 mathematical authority implementation is claimed by P0.

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
4. Inspect current `main`, branches, and checkpoint evidence before assuming the last phase is still current.
5. If P0 has not landed on the branch being used, recover `implementation/p0-reproducible-skeleton` and its proof checkpoint before doing P1 work.
6. If P0 has landed, start P1 from the frozen roadmap using a fresh isolated implementation branch/worktree and mandatory RED→GREEN TDD.
7. Do not reopen broad research or redesign frozen milestones unless concrete implementation evidence exposes a contradiction or unsupported obligation.
8. Preserve the temporary project-name rule until the mathematical product identity is mature enough to name.

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
