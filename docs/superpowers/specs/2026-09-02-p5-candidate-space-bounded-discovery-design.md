# P5 Design — CandidateSpace + Bounded Discovery

**Date:** 2026-09-02  
**Status:** IMPLEMENTATION DESIGN  
**Branch:** `implementation/p5-candidate-space-bounded-discovery`  
**Exact predecessor:** `d6f92a3a5872d9634d74a5fef688e28adfcec2cf`  
**Roadmap authority:** `docs/roadmap/2026-08-28-implementation-roadmap.md`, phase P5  
**First-Light authority:** `docs/design/2026-08-28-first-light-specification.md`  
**D3 authority:** `docs/design/2026-08-28-d3-mathematical-compiler-architecture.md`

## 1. Purpose

P5 implements the smallest exact symbolic CandidateSpace/discovery substrate required by the frozen First-Light suite. It is not the final universal search substrate.

P5 lives in `formula-engine`. Search state remains downstream of P4 compiler/campaign state and outside mathematical authority.

P5 must support P6 without importing sealed target definitions.

## 2. Constitutional boundary

```text
P4 Obligation/Campaign/WorkCell inputs
        ↓
CandidateSpace context
        ↓
exact symbolic restriction/refinement
        ↓
heuristic/non-authoritative ranking
        ↓
frozen candidate
        ↓
independent oracle/checker later
```

P5 cannot certify, admit, activate, publish, or mutate authority. It must not depend on `formula-check` or `formula-first-light` implementation.

A search candidate is always `CANDIDATE_ONLY` until a later independent authority path certifies it.

## 3. Common CandidateSpace contract

P5 introduces immutable/shared search contracts:

```text
CandidateSpaceContext {
    universe_generation
    world
    query_digest
    obligation_digest
    grammar_or_route_set_digest
    policy_digest
}

CandidatePolarity {
    Exact
    SoundOverApproximation
    SoundUnderApproximation
    HeuristicProposal
}

SearchAuthority {
    CandidateOnly
}

FrozenCandidateSpace {
    context_digest
    backend_kind
    polarity
    completeness_class
    state_digest
}

FrozenCandidate {
    space_digest
    candidate_digest
    cost
    authority = CandidateOnly
}
```

The context intentionally binds only semantic inputs local to the CandidateSpace. Unrelated campaign changes cannot perturb its identity, satisfying D3-P10.

Backend-neutral operations required in P5 are represented by equivalent bounded methods across the three backends:

```text
restrict
refine
partition
empty
extract
freeze/serialize
```

No backend operation may silently strengthen its polarity/completeness class.

## 4. AffinePolynomialSpace

FL-A requires an exact polynomial coefficient space:

```text
P(n) = c0 + c1*n + ... + cd*n^d
```

P5 implements bounded exact rational arithmetic internally using normalized signed `i128` numerator/positive denominator pairs. This is sufficient for the sealed First-Light degree/sample bounds and introduces no external dependency.

State is a linear system over exact rationals, not enumeration of coefficient tuples.

Operations:

```text
new(max_degree, context)
add_exact_sample(n, y)
restrict_degree(d)
solve_affine_space()
partition_by_degree()
empty()
extract_min_degree_unique()
freeze()
```

`solve_affine_space` returns deterministic row-reduced affine information including rank/free-variable dimension and, when unique, exact coefficients.

Adding one sample refines the whole affine family. The P5 proof must show a counterexample reduces affine dimension or eliminates a prior class, not merely rejects one enumerated polynomial.

## 5. ReductionRouteSpace

FL-B requires a bounded exact route graph rather than hard-coded algorithm dispatch.

A route candidate binds:

```text
route_digest
source_semantics
target_semantics
preserved_result_classes
required_capabilities
exactness
cost
```

The space supports:

```text
restrict_result_class
restrict_capabilities
subtract_scoped_failure
partition_by_target
empty
extract_min_cost
freeze
```

Cost may rank only already admissible routes. A low-cost route that does not preserve the requested result class remains unavailable.

Failure pruning is scoped by an explicit applicability set. A failure for route A cannot delete unrelated route B unless the failure artifact's declared scope includes B.

## 6. ObservationalExprSpace

FL-C uses the frozen grammar family:

```text
ByteExpr:
    x
    0
    1
    sub_wrap(ByteExpr, ByteExpr)
    bit_and(ByteExpr, ByteExpr)

BoolExpr:
    eq_zero(ByteExpr)
    neq_zero(ByteExpr)
    and(BoolExpr, BoolExpr)
```

P5 defines typed ASTs, deterministic canonical encoding/digests, exact U8 wrapping evaluator, and structural cost.

The bounded space is generated bottom-up by cost and stores the lowest-cost deterministic representative for each behavior vector on the current sample set.

Important correctness rule: when a new counterexample/sample is added, P5 **regenerates/rebuckets from the grammar bound**, rather than refining only the retained representatives. This preserves the exact bounded candidate family because two expressions previously observationally equivalent may split under the new sample.

Operations:

```text
new(grammar, max_cost, context)
restrict_exact_sample(x, expected)
refine_counterexample(x, expected)
partition_by_behavior
empty
extract_min_cost
freeze
```

## 7. Generic bounded CEGIS loop

P5 provides a generic oracle interface that reveals only:

```text
output_for_sample(input)
validate_frozen_candidate(candidate) -> Equivalent | Counterexample(input, expected)
```

The oracle is supplied by callers. P5 does not import sealed First-Light fixtures.

Loop:

```text
initial exact samples
 -> build/refine CandidateSpace
 -> extract lowest-cost candidate
 -> structurally freeze candidate
 -> ask oracle only after freeze
 -> on counterexample refine the whole space
 -> repeat within explicit iteration/resource bound
```

Candidate freeze is therefore observable before validation.

## 8. Minimal-cost extraction and fair fallback

Ranking is non-authoritative.

Within one exact space, extraction order is deterministic by:

```text
structural cost
then candidate digest
```

For portfolios/routes, P5 includes deterministic fair fallback/round-robin scheduling so a high heuristic score cannot permanently starve another admissible exact route.

Heuristic scores may reorder work; they cannot remove sound candidates, discharge Judgements, or change CandidateSpace polarity.

## 9. Failure scope and nogoods

P5 distinguishes:

```text
exact counterexample restriction
scoped exact nogood
heuristic rejection
```

Only exact restrictions/nogoods may soundly remove candidate families, and only inside their declared applicability scope.

A heuristic rejection may affect ranking only.

## 10. Deterministic freeze/replay

Every backend freeze binds:

```text
CandidateSpaceContext
backend schema/version
polarity/completeness
semantic constraints/samples
backend-specific grammar/routes/degree bound
current exact candidate-space state
```

Insertion order of set-like samples/routes/constraints is non-semantic.

Identical exact inputs produce identical frozen-space and extracted-candidate identities.

## 11. P6 readiness without answer leakage

P5 public integration tests may use structurally similar public fixtures, but P5 source/tests must not import:

```text
crates/formula-first-light
/tests/first-light/sealed
hidden FL-A expanded coefficients
hidden FL-C known compact answer as a privileged search constant
```

P6 will provide sealed oracle implementations later.

The observational grammar may naturally enumerate the mandatory FL-C near-miss because it is part of the public frozen grammar; P5 must not hard-code it as the answer or privileged candidate.

## 12. Proof obligations

### D3-P06 — polarity/completeness preservation

All exact restrictions preserve `Exact`; heuristic operations never claim exact pruning.

### D3-P07 — scoped pruning

A counterexample or nogood removes only candidates to which its semantics apply. Tests prove unrelated routes/classes survive.

### D3-P08 — heuristic non-authority

Search/heuristic APIs expose no Judgement/PASS/admission result. Extracted outputs are `CandidateOnly`.

### D3-P10 — unaffected identity

Recreating an unchanged CandidateSpace context/state yields the same identity even if unrelated campaign state changes elsewhere.

### Gate P5

PASS only when:

1. one exact counterexample refines/removes a whole candidate class;
2. all three bounded backends serialize/freeze deterministically;
3. minimal-cost extraction is deterministic;
4. bounded CEGIS freezes candidates before oracle validation;
5. heuristic ranking cannot discharge authority;
6. P5 source cannot import checker/sealed-target implementations;
7. all P0–P4 canonical regression gates remain green.

## 13. Explicit exclusions

P5 does not implement:

- sealed FL-A/B/C fixtures;
- independent target certification;
- promotion/admission;
- U0->U1 transition;
- native realization;
- models/LLMs;
- external SAT/SMT/CAS;
- Ptah;
- general e-graphs/FTA/ECTA;
- final universal scheduler.

Those are later phases or post-First-Light work.

## 14. Freeze boundary

P5 freezes only after:

```text
RED/GREEN implementation
canonical exact-head source proof
exact P4->P5 scope/authority review
checkpoint + CURRENT update
canonical exact-head proof with documentation present
```

No P6 work begins before that exact documentation-bearing P5 head is proved.
