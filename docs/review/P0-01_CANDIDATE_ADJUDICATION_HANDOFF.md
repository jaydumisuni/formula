# P0-01 Candidate Adjudication Handoff

Status: **CANDIDATE EVIDENCE HANDOFF — NOT FORMULA AUTHORITY**

This document collects the currently verified engineering evidence for P0-01 without changing the frozen Formula authority, promoting any milestone, or modifying the exact implementation head that produced the evidence.

## Frozen authority inputs

- Frozen architecture checkpoint: `50c0beb021d8bf02d59a177049b9c2cf1783b26a`
- Frozen roadmap: `docs/roadmap/2026-08-28-implementation-roadmap.md`
- P0 is the first implementation stage.
- P0-01 is the first P0 proof obligation: pinned toolchain/source dependency manifest.
- Gate P0 is not satisfied merely by worker or CI success.

## Exact candidate implementation

- Construction branch: `impl/p0-first-light`
- Exact candidate head: `0eb2dbbb289ae4b5eb6206007d8ee61e5fc5719b`
- Candidate PR: #3, `P0: establish reproducible workspace and source manifest`
- Protected `main` at the time this handoff was prepared: `413a1b20c0227d6851f1595cc2ce12bbbc69f8d0`

The candidate head must remain the identity of the captured P0-01 evidence. Updating the construction branch requires a fresh target-host capture before that new head can be considered equivalent evidence.

## Repository verification already available

GitHub Actions run `34591298093` completed successfully for exact head `0eb2dbbb289ae4b5eb6206007d8ee61e5fc5719b` and exercised the non-authoritative P0 candidate checks, including:

- Rust `1.98.0` installation;
- static P0 authority-boundary checks;
- capture-procedure syntax/safety assertions;
- atomic P0 host-capture exercise;
- independent source-proof verification and fail-closed tests;
- formatting verification;
- locked workspace compilation.

This CI result is repository evidence only. It is not a Formula proof or promotion event.

## Target-host candidate evidence already available

On KRATOS, the exact candidate head has a clean host evidence bundle under `.formula/evidence/p0-01`.

Independent verification returns:

```text
frozen_architecture: 50c0beb021d8bf02d59a177049b9c2cf1783b26a
git_branch: impl/p0-first-light
git_head: 0eb2dbbb289ae4b5eb6206007d8ee61e5fc5719b
rust_toolchain: 1.98.0
status: CANDIDATE_HOST_EVIDENCE_NOT_AUTHORITY
```

The verifier explicitly states that no Formula authority or P0 promotion is created.

## Cookpit evidence boundary

Shared THETECHGUY Cookpit may repeatedly run repository checks against the candidate head. Current Cookpit workers are supplementary engineering evidence only.

Cookpit success MUST NOT:

- adjudicate P0-01;
- promote P0-01, P0, or any later stage;
- create mathematical authority;
- weaken or replace the frozen Formula roadmap gate;
- convert candidate evidence into a `PROVEN` milestone.

## Adjudication question

The protected authority reviewer must decide only whether the existing exact-head candidate evidence satisfies the frozen P0-01 obligation.

The evidence package currently binds:

1. exact Git HEAD;
2. frozen architecture checkpoint ancestry;
3. exact Rust toolchain identity;
4. Cargo identity and locked metadata;
5. hashes for the P0 source/toolchain manifests;
6. clean worktree state;
7. independent fail-closed verification.

If the authority reviewer finds any required field missing, ambiguous, stale, or outside the frozen roadmap, P0-01 remains UNPROVEN and the missing requirement must be named precisely.

If the authority reviewer accepts the evidence, that authority decision must be recorded separately. This handoff document does not make that decision and must never be treated as proof by itself.

## Current boundary

Until an authorized Formula adjudication is recorded:

- P0-01: **UNPROVEN**
- P0: **UNPROVEN**
- latest proven P-stage: **none**

No later P-stage may inherit authority from this handoff, GitHub CI, KRATOS host verification, or Cookpit worker success alone.
