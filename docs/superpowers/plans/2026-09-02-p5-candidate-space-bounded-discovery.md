# P5 Implementation Plan — CandidateSpace + Bounded Discovery

**Date:** 2026-09-02  
**Branch:** `implementation/p5-candidate-space-bounded-discovery`  
**Predecessor:** `d6f92a3a5872d9634d74a5fef688e28adfcec2cf`  
**Design:** `docs/superpowers/specs/2026-09-02-p5-candidate-space-bounded-discovery-design.md`

## Execution law

Every task is RED -> minimum GREEN -> exact branch CI evidence. Search outputs remain candidate-only. No P6 sealed target implementation, checker dependency, promotion, or realization may enter P5.

## Task 1 — Common CandidateSpace contracts

Files:
- `crates/formula-engine/src/candidate_space.rs`
- `crates/formula-engine/tests/p5_candidate_contract.rs`
- `crates/formula-engine/src/lib.rs`

RED tests:
- deterministic context/freeze identity;
- set-like constraint order does not alter identity;
- candidate authority is always `CandidateOnly`;
- polarity/completeness cannot be silently upgraded.

GREEN:
- `CandidateSpaceContext`, `CandidatePolarity`, `CompletenessClass`, `SearchAuthority`, `FrozenCandidateSpace`, `FrozenCandidate`, scoped failure/nogood contracts.

## Task 2 — Exact bounded rational + affine polynomial space

Files:
- `crates/formula-engine/src/affine_polynomial.rs`
- `crates/formula-engine/tests/p5_affine_polynomial.rs`

RED tests:
- exact normalized rational arithmetic;
- deterministic Gaussian/RREF result;
- sample addition refines affine dimension;
- degree restriction can empty a space;
- unique minimal-degree extraction deterministic;
- freeze deterministic across sample insertion order.

GREEN:
- bounded normalized `Rational128`;
- exact linear constraint system;
- affine-space solve/rank/nullity;
- exact sample restriction and min-degree extraction.

## Task 3 — Reduction-route CandidateSpace

Files:
- `crates/formula-engine/src/route_space.rs`
- `crates/formula-engine/tests/p5_route_space.rs`

RED tests:
- requested result class filters routes exactly;
- capability restriction exact;
- lower cost cannot rescue inadmissible route;
- scoped route failure removes only declared applicability;
- deterministic partition/extract/freeze.

GREEN:
- bounded route candidates and exact filtering/scoped subtraction.

## Task 4 — Typed U8/Boolean grammar + evaluator

Files:
- `crates/formula-engine/src/observational.rs`
- `crates/formula-engine/tests/p5_observational_grammar.rs`

RED tests:
- typed AST identity deterministic;
- U8 subtraction wraps;
- bitwise/Boolean evaluator exact;
- structural cost deterministic;
- grammar digest changes when operator set/bound changes.

GREEN:
- frozen First-Light-compatible grammar types/evaluator without sealed target knowledge.

## Task 5 — Observational CandidateSpace

RED tests:
- bottom-up generation bounded by cost;
- one lowest-cost deterministic representative per current behavior bucket;
- exact examples restrict whole behavior classes;
- adding a new sample regenerates/rebuckets and can split old classes;
- deterministic extract/freeze.

GREEN:
- bounded exact regeneration from grammar per sample set.

## Task 6 — Generic bounded CEGIS

Files:
- `crates/formula-engine/src/discovery.rs`
- `crates/formula-engine/tests/p5_cegis.rs`

RED tests:
- candidate frozen before validation callback;
- counterexample refines whole space;
- deterministic candidate sequence under same inputs;
- resource/iteration exhaustion returns search/resource unknown, never refutation;
- successful output remains candidate-only.

GREEN:
- generic oracle trait/interface and bounded loop over observational space.

## Task 7 — Fair fallback + heuristic non-authority

Files:
- `crates/formula-engine/src/search_policy.rs`
- `crates/formula-engine/tests/p5_search_policy.rs`

RED tests:
- deterministic minimal-cost ordering;
- fair round-robin floor prevents starvation;
- heuristic rejection cannot delete exact candidate;
- heuristic score cannot create Judgement/PASS/authority.

GREEN:
- bounded deterministic policy helpers only.

## Task 8 — Recompilation/local identity

Files:
- `crates/formula-engine/tests/p5_recompilation.rs`

Tests:
- identical local semantic inputs => same frozen space/candidate identities;
- unrelated campaign digest/state changes do not perturb local CandidateSpace identity;
- changed generation/world/query/grammar/policy does perturb identity.

## Task 9 — P6-readiness integration + adversarial boundary

Files:
- `crates/formula-engine/tests/p5_first_light_ready.rs`
- `crates/formula-engine/tests/p5_adversarial.rs`
- `tests/authority-boundary/tests/p5_search_authority_inert.rs`

Public fixtures only:
- polynomial-like oracle demonstrates affine family refinement;
- route fixture selects certified lower-cost exact route without operation-name special case;
- U8/Bool fixture demonstrates a plausible zero-related near-miss being eliminated by a counterexample without embedding the sealed FL-C answer as privileged data.

Architecture gate:
- P5 engine source cannot import `formula_check`, `formula-first-light`, sealed fixtures, authority publication/promotion APIs.

## Task 10 — Canonical P5 proof

Create `.github/workflows/p5-canonical-proof.yml` with named read-only steps:
- pinned Rust 1.98.0;
- locked cache fetch;
- metadata;
- P0-P5 package/workspace tests;
- build;
- rustfmt;
- Clippy `-D warnings`;
- relevant dependency trees;
- clean worktree;
- P5 markers.

Markers:
```text
P5-01 CandidateSpace identity deterministic                         PASS
P5-02 affine exact space refines whole family                      PASS
P5-03 route failure pruning scoped                                 PASS
P5-04 observational buckets exact under current samples            PASS
P5-05 counterexample regeneration preserves bounded candidate set  PASS
P5-06 bounded CEGIS freezes before validation                      PASS
P5-07 heuristics remain non-authoritative                          PASS
P5-08 unaffected local identity preserved                          PASS
P5-09 checker/sealed-target/search authority firewall              PASS
P5-10 P0-P4 gates preserved                                        PASS
```

## Task 11 — Review / Freeze / Prove

- compare exact frozen P4 docs head -> P5 source proof head;
- review authority/search boundary and no sealed leakage;
- write `docs/checkpoints/2026-09-02-p5-candidate-space-bounded-discovery.md`;
- update `CURRENT.md`;
- require full canonical proof on exact docs-bearing head;
- only then begin P6.
