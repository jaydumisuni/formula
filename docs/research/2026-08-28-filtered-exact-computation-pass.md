# Filtered Exact Computation Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project targets mathematical authority without requiring worst-case exact arithmetic for every ordinary query. The open question was whether an execution path can start with cheap approximate/native arithmetic and still return an exact mathematical verdict without trusting the approximation.

The answer is yes, provided approximation stages are used only as **certified filters** and ambiguity causes escalation rather than an unqualified answer.

## Evidence

### CGAL filtered predicates

CGAL's `Filtered_predicate` explicitly separates:

- an approximate/filtering predicate,
- an exact but slower predicate,
- conversions to each representation.

The approximate predicate may return only when it can establish the correct result; otherwise it signals failure and the exact predicate is invoked.

Source:
- https://doc.cgal.org/latest/Kernel_23/classCGAL_1_1Filtered__predicate.html

CGAL's `Filtered_kernel` adds interval-arithmetic filtering and, for selected predicates, formally proved semi-static filters before the exact path.

Source:
- https://doc.cgal.org/latest/Kernel_23/structCGAL_1_1Filtered__kernel.html

The developer documentation describes the key rule directly: evaluate first with efficient floating-point arithmetic while computing an error bound; if the computed result is not guaranteed correct, re-evaluate with exact arithmetic.

Source:
- https://doc.cgal.org/Manual/3.6/doc_html/Developers_manual/Developers_manual/Chapter_kernels.html

### Shewchuk adaptive predicates

Robust geometric predicates such as orientation/incircle can be evaluated adaptively: ordinary non-degenerate cases remain close to floating-point cost, while near-zero/ill-conditioned cases receive progressively more precision until the sign is certain.

Source:
- https://www.cs.cmu.edu/~quake/robust.html

This establishes an important runtime asymmetry:

```
common easy instance
  -> native arithmetic
  -> certified decisive sign

near-degenerate instance
  -> native arithmetic inconclusive
  -> higher precision
  -> possibly exact arithmetic
```

The exactness contract is unchanged; only the cost varies with mathematical difficulty/conditioning.

### Lazy exact DAGs

CGAL's `Lazy_exact_nt` maintains approximate interval information and an exact expression DAG. Exact values are computed recursively only when required. The implementation explicitly describes DAG-based lazy evaluation and pruning of the DAG after exact evaluation.

Sources:
- https://doc.cgal.org/latest/Number_types/classCGAL_1_1Lazy__exact__nt.html
- https://github.com/CGAL/cgal/blob/main/Number_types/include/CGAL/Lazy_exact_nt.h

This proves that the system does not need to eagerly materialize every exact intermediate value merely because an exact semantic value exists.

## Architecture-changing conclusion

Precision/exactness selection should be represented as a **certified escalation ladder**, not as a global numeric mode.

Conceptually:

```
mathematical obligation
       |
       v
stage 0: static/compile-time filter
       |
       | decisive + proof bound -> answer
       | ambiguous
       v
stage 1: native float / SIMD filter
       |
       | decisive + rigorous error bound -> answer
       | ambiguous
       v
stage 2: interval / ball arithmetic
       |
       | enclosure separates target predicate -> answer
       | ambiguous
       v
stage 3: increased precision / adaptive expansion
       |
       | decisive -> answer
       | ambiguous
       v
stage 4: exact arithmetic / symbolic / certificate authority
```

A stage may only terminate the query if it produces evidence that its result is mathematically decisive for the requested property.

`AMBIGUOUS` is therefore not an incorrect result and not a failure. It is an admissible typed outcome meaning: **this representation cannot yet certify the requested distinction**.

## Candidate runtime contract

A filtered evaluator should expose something structurally like:

```
FilterResult<T> =
    Decisive {
        value: T,
        certificate: C,
        semantic_scope: S
    }
  | Ambiguous {
        enclosure_or_bound: B,
        reason: R,
        recommended_next_stage: StageClass
    }
  | InvalidDomain {
        witness: W
    }
```

The approximate value by itself is never mathematical authority.

## Separation of predicates and constructions

CGAL exposes an important warning: exact predicates do not automatically make approximate constructions exact.

The project must therefore distinguish:

- **decision/predicate exactness** — e.g. sign, ordering, membership, orientation;
- **construction exactness** — the produced mathematical object itself is exact;
- **approximate construction with certified enclosure/error**.

A fast exact predicate around an inexact construction must not silently upgrade the construction's semantic class.

## Lazy exactness as a graph property

The project can preserve a construction as:

```
node
  |- approximate enclosure/cache
  |- exact semantic expression
  |- dependencies
  |- exact-value cache (optional/not-yet-computed)
  |- refinement state
```

This means exact semantics and exact materialization are separate properties.

A large chain may remain cheap until an operation genuinely requires exact realization.

This integrates naturally with the existing research on:

- domain-theoretic certified progress,
- adaptive representations,
- rigorous numerical realization,
- arithmetic circuits,
- out-of-core computation,
- semantic identity versus realization identity.

## Search/runtime law

The new law is:

> **Use the cheapest representation that can certify the current obligation; escalate only when the current representation cannot decide it.**

This is stronger than CPU-first/GPU-last. It is representation-first:

```
cheap proof-producing representation
        before
expensive exact representation
        before
larger hardware
```

A difficult input earns expensive arithmetic; ordinary inputs do not.

## Implications for the Theory Profile

Each operation/predicate should eventually declare candidate evaluation stages such as:

```
filter_stages:
  - representation: f64
    guarantee: rigorous-sign-if-bound-separated
    cost_model: ...

  - representation: interval-f64
    guarantee: enclosure
    cost_model: ...

  - representation: mpfr-128
    guarantee: enclosure
    cost_model: ...

  - representation: exact-rational
    guarantee: exact
    cost_model: ...
```

The profile should also record:

- whether the operation is filterable;
- which result properties can be certified cheaply;
- conditioning/degeneracy indicators;
- whether refinement is monotone;
- whether exact fallback exists;
- whether a lazy expression representation is available;
- worst-case escalation guarantees.

## Why this matters to ordinary hardware

This gives a concrete route to a system that is authoritative but behaves near native speed on the majority of inputs.

The project does **not** need to choose between:

```
fast but unsafe
```

and

```
exact but always expensive
```

It can use:

```
fast
  + certificate of decisiveness
  + exact fallback only when required
```

This should become a major execution policy candidate.

## Open research created by this pass

1. Can filter chains themselves be automatically synthesized from a semantic obligation and available representations?
2. Can the system prove that an escalation ladder is complete, i.e. eventually reaches an authority path when one exists?
3. How should filter success rates and hardware measurements alter stage ordering without changing semantic guarantees?
4. How much of static/semi-static error-bound generation can be automated at primitive-compilation time?
5. How should exact DAGs be pruned, memoized, spilled, or recomputed under memory pressure?
6. Can Ziv-style adaptive-precision strategies be generalized into a common certified refinement protocol across transcendental, algebraic, geometric, and optimization computations?
7. Can a discovered primitive automatically gain a filtered implementation after its exact semantics are certified?

## Current architectural hypothesis

Filtered exact computation should not be a special geometry/numerics subsystem.

It should be one instance of a general **evidence-driven escalation protocol**:

```
cheap candidate computation
        ↓
cheap certificate/check
        ↓
if sufficient: stop
if insufficient: refine/escalate
        ↓
stronger candidate computation
        ↓
stronger certificate/check
        ↓
...
        ↓
exact/foundational authority
```

The mathematics determines when escalation can stop.

Hardware only determines how fast each stage executes.
