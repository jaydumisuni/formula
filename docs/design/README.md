# Design authority

The repository name `formula` is temporary and is not the final product identity.

## Canonical frozen architecture

The canonical design sequence is now:

1. [D1 — Mathematical Constitution](2026-08-28-d1-mathematical-constitution.md) — mathematical authority model, six durable artifact classes, Problem/Campaign constitutional semantics, Discovery Fabric, Certification + Promotion, Execution/Realization boundary, constitutional stress tests.
2. [D1A — Self-Hosting and Bootstrap Trust Amendment](2026-08-28-d1a-self-hosting-bootstrap-amendment.md) — normative self-hosting rule: successors may be generated internally but may not self-authorize; explicit bootstrap/independent-validation trust chain required.
3. [D2 — Core System Architecture](2026-08-28-d2-core-system-architecture.md) — universe storage/identity, authority graph, theory packages, capability closure, federation, shared mathematical facts, certificate routing, search-state separation, promotion service, practical bootstrap roles.
4. [D3 — Mathematical Compiler Architecture](2026-08-28-d3-mathematical-compiler-architecture.md) — QueryIR, semantic elaboration, Theory Profile, observer/sufficiency analysis, representation/reduction/decomposition, Campaign/Obligation IR, CandidateSpace ABI, discovery compilation, search economy, replay.
5. [D4 — Native Execution Architecture](2026-08-28-d4-native-execution-architecture.md) — mathematical lowering, specialization, CPU-first exact execution, filtered precision/escalation, exact image/reconstruction, SIMD, incremental/out-of-core execution, optional GPU, translation validation and realization manifests.
6. [D5 — Self-Expansion Architecture](2026-08-28-d5-self-expansion-architecture.md) — primitive/theorem/reduction/morphism promotion, grammar evolution, metaprimitive activation, generation transitions, proof transport/repair, self-hosted evolution and rollback.
7. [First Light — Canonical End-to-End Proof Specification](2026-08-28-first-light-specification.md) — blind rediscovery, false near-miss rejection, independent certification, U0→U1 promotion, native realization validation, and mandatory second-query reuse proof.
8. [Implementation Roadmap](../roadmap/2026-08-28-implementation-roadmap.md) — proof-gated phases P0–P13, targeted research spikes, and explicit deferred Ptah integration.

## Preserved precursor milestones

These files remain design history/evidence but their milestone labels are superseded by the canonical sequence above:

- [Precursor D2 — Operational Mathematical Machine](2026-08-28-d2-operational-mathematical-machine.md) — largely incorporated into canonical D2/D3.
- [Precursor D3 — First-Light Build Architecture](2026-08-28-d3-first-light-build-architecture.md) — concrete bounded implementation research incorporated into the canonical First-Light specification and roadmap.

The precursor files are not deleted or rewritten because they remain useful provenance for how the frozen architecture evolved.

## Current design rule

Research under `../research/` is the evidence authority behind the design. Broad research is no longer the lead activity. A new research pass is permitted only when a design/implementation choice is unsupported, contradicted, or materially under-specified by preserved evidence, or when a roadmap research-spike trigger is reached.

## Current implementation boundary

D1 + D1A + D2 + D3 + D4 + D5 + First Light + roadmap are the frozen architecture authority.

The canonical First-Light implementation remains:

```text
local
ordinary CPU
model-free
network-free during canonical run
independently checked
content-addressed/replay-bound
Ptah-free
GPU-free
```

Advanced infrastructure is downstream unless an explicit frozen proof obligation cannot be satisfied without it.

## Next milestone

**P0 — Repository and reproducible build skeleton**, followed by P1/P2.

The next major implementation freeze is **P9 — Canonical First-Light Proof**. Ptah integration is explicitly deferred until P9 passes and P13 entry conditions are met.
