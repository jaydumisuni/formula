# Formula — temporary project repository

> **Temporary repository name only. This is not the final product name or mathematical identity.**

This repository preserves the research, architecture, proof specifications, and implementation roadmap for an unnamed self-expanding deterministic mathematical problem-solving system.

The project is deliberately not classified as only a CAS, theorem prover, AI mathematician, solver, programming language, or compiler. Those can all participate as packages, tools, proof systems, or realizations.

## Current status

Broad pre-design research is complete enough to support architecture.

The project now has a frozen canonical architecture:

- **D1 — Mathematical Constitution**
- **D1A — Self-Hosting and Bootstrap Trust Amendment**
- **D2 — Core System Architecture**
- **D3 — Mathematical Compiler Architecture**
- **D4 — Native Execution Architecture**
- **D5 — Self-Expansion Architecture**
- **First Light — Canonical End-to-End Proof Specification**
- **Implementation Roadmap — P0 through P13**

Canonical index: [`docs/design/README.md`](docs/design/README.md)

Implementation roadmap: [`docs/roadmap/2026-08-28-implementation-roadmap.md`](docs/roadmap/2026-08-28-implementation-roadmap.md)

## Defining architectural law

```text
U_g
  -> target
  -> represent / reduce / decompose / search / invent
  -> candidate mathematics
  -> falsify / certify
  -> promote
  -> U_(g+1)
  -> compile / specialize / realize
  -> stronger and cheaper future problem solving
```

Search may propose mathematics. Only Certification + Promotion creates mathematical authority.

Execution consumes admitted mathematics. It cannot create authority directly.

No solver, model, proof language, representation, compiler, backend, or external package is the mathematics itself.

## Research authority

The preserved research under [`docs/research/`](docs/research/) remains the evidence authority behind the design.

Canonical baseline:

- [`docs/research/2026-08-28-research-checkpoint.md`](docs/research/2026-08-28-research-checkpoint.md)

The directory contains the subsequent focused research passes covering substrate integration, mathematical identity, certificate routing, exact/native execution, representation search, theory transfer, candidate-space compression, search economy, self-specialization, computation limits, proof repair, primitive invention, federation, and related architecture-changing evidence.

Broad research is no longer the lead activity. New research should be opened only when:

1. a frozen design/implementation decision lacks adequate evidence;
2. preserved research contradicts the intended design;
3. a roadmap research-spike trigger is reached;
4. implementation evidence exposes a genuinely new architectural uncertainty.

## First-Light boundary

The canonical First-Light proof must remain:

```text
local
ordinary CPU
model-free
network-free during canonical execution
independently checked
content-addressed/replay-bound
Ptah-free
GPU-free
```

First Light must blindly discover a primitive absent from `U_0`, reject false near-misses, independently certify and promote the surviving mathematics into `U_1`, compile and independently validate a native realization, then prove a second query reuses the promoted capability without rediscovery.

## Ptah boundary

Ptah is explicitly deferred until the local First-Light proof passes.

Ptah may later execute serialized Work Cells and large mathematical campaigns, but it does not become mathematical authority and is not required to consume already promoted local mathematics.

## Next executable milestone

**P0 — Repository and reproducible build skeleton.**

The next major proof freeze is **P9 — Canonical First-Light Proof**.
