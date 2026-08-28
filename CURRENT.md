# CURRENT — Cross-chat recovery authority

**Repository name:** `formula` is temporary only; it is not the final product name.

This file is the shortest authoritative recovery path for a new chat/session. Recover repository evidence before reasoning. Do not reconstruct state from chat memory when these files are available.

## Exact current state

The research-led phase is complete enough for implementation. Broad research is no longer the lead activity.

Three design milestones are already frozen:

1. [`docs/design/2026-08-28-d1-mathematical-constitution.md`](docs/design/2026-08-28-d1-mathematical-constitution.md)
   - frozen D1 mathematical constitution;
   - six durable artifact classes: `Entity`, `Relation`, `World`, `Judgement`, `Evidence`, `Realization`;
   - immutable Universe generations `U_g`;
   - Problem/Campaign Compiler;
   - Discovery Fabric;
   - Certification + Promotion boundary;
   - Execution/Realization boundary;
   - search may propose mathematics; only Certification + Promotion creates authority.

2. [`docs/design/2026-08-28-d2-operational-mathematical-machine.md`](docs/design/2026-08-28-d2-operational-mathematical-machine.md)
   - frozen D2 operational contracts;
   - Package/Capability contracts;
   - Structure Goal IR and generation-scoped capability closure;
   - Theory Profiles;
   - CandidateSpace contract;
   - Campaign/Work Cell IR;
   - shared mathematical facts / solver federation;
   - search economy;
   - certificate routing and realization planning;
   - canonical First-Light proof obligations.

3. [`docs/design/2026-08-28-d3-first-light-build-architecture.md`](docs/design/2026-08-28-d3-first-light-build-architecture.md)
   - frozen D3 concrete First-Light build architecture;
   - stable Rust for First Light only, not a permanent constitutional language choice;
   - exact integer/rational authority arithmetic;
   - SQLite + immutable SHA-256 content-addressed blob store;
   - independent checker boundary;
   - three different CandidateSpace backends;
   - blind targets FL-A / FL-B / FL-C;
   - negative controls N1-N7;
   - atomic `U_0 -> U_1` promotion protocol;
   - native realization + independent equivalence validation;
   - canonical proof manifest and PASS markers;
   - implementation sequence B01-B13.

Design index: [`docs/design/README.md`](docs/design/README.md).
Research evidence authority: [`docs/research/`](docs/research/).

## Current milestone

**Next milestone: F0 — First-Light implementation campaign.**

No implementation completion is claimed by the frozen design documents. A new chat must inspect the current repository tree/commits for any implementation work added after this handoff before assuming F0 is untouched.

At the handoff boundary represented by this file, the canonical design says F0 should implement D3 stages in this exact order:

```text
B01 canonical identity + blob store
B02 D1 durable schemas + U_0 generation
B03 Evidence envelope + independent checker process
B04 Package/Capability contracts + closure resolver
B05 Query + minimal Campaign IR + Work Cell runner
B06 FL-A polynomial CandidateSpace + checker
B07 FL-B route/reduction CandidateSpace + GF2 package/checker
B08 FL-C observational synthesis CandidateSpace
B09 promotion transaction U_0 -> U_1
B10 generated native realization + independent finite-domain validation
B11 reuse query / no-rediscovery proof
B12 negative controls
B13 canonical verifier + full PASS manifest
```

Do **not** skip ahead to distributed execution, GPU work, models, large CAS/proof-assistant federation, Ptah integration, UI/API work, advanced e-graphs, or a full roadmap before the canonical F0 proof requires them.

## Canonical First-Light PASS set

The independent verifier must eventually emit all constitutional markers in one proof manifest:

```text
PASS_UNIVERSE_BINDING
PASS_BLIND_DISCOVERY
PASS_FALSE_CANDIDATE_REJECTION
PASS_CERTIFICATION
PASS_PROMOTION_ATOMICITY
PASS_CAPABILITY_CLOSURE_DELTA
PASS_REALIZATION_EQUIVALENCE
PASS_REUSE_WITHOUT_REDISCOVERY
PASS_AUTHORITY_NOT_DOWNGRADED
PASS_REPLAY_BINDING
```

D3 also defines target diagnostics:

```text
PASS_TARGET_A_EXACT_IDENTITY
PASS_TARGET_B_REPRESENTATION_REDUCTION
PASS_TARGET_C_SYNTHESIZED_PRIMITIVE
```

## Non-negotiable First-Light boundary

Canonical First Light is:

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

The purpose is to prove the architecture's complete self-expansion loop on ordinary hardware:

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
3. Recover D1, D2, and D3; D1 wins on constitution, D2 on operational contracts, D3 on First-Light implementation choices.
4. Inspect current `main` and repository tree before acting; later implementation commits may supersede this handoff's no-F0-work observation.
5. If F0 has not started, resume at **B01**.
6. If F0 has started, recover the latest completed B-stage and its proof evidence; continue from the first unproven stage.
7. Do not reopen broad research or redesign frozen milestones unless concrete implementation evidence exposes a contradiction or unsupported obligation.
8. Preserve the temporary project-name rule until the mathematical product identity is mature enough to name.

## Evidence precedence

When sources disagree, use this order:

```text
current repository implementation/proof evidence
    > later frozen design amendment explicitly superseding an older milestone
    > D3 for First-Light build choices
    > D2 for operational machine contracts
    > D1 for mathematical constitution
    > preserved research checkpoints
    > chat recollection
```

No chat should claim work is complete merely because a design exists. F0 completion requires the independently replayable proof manifest and full PASS set defined by D3.
