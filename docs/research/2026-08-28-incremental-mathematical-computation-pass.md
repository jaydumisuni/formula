# Incremental Mathematical Computation Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project should not recompute a complete mathematical result when a new problem instance differs only slightly from a previously solved one.

The relevant question is deeper than caching:

> Can a mathematical construction itself carry a principled update law that transforms a prior certified result into the new result after an input change?

The research says yes for broad classes of computations, including recursive/fixed-point computations.

## 1. Self-adjusting computation

Self-adjusting computation records control/data dependencies so changes to inputs can be propagated only through affected portions of a previous execution.

Umut Acar's work provides:

- dynamic dependence graphs;
- memoization;
- change propagation;
- formal semantics proving consistency with from-scratch execution.

Sources:
- https://www.umut-acar.org/self-adjusting-computation
- https://csd.cmu.edu/academics/doctoral/degrees-conferred/umut-a-acar
- https://www.cambridge.org/core/journals/journal-of-functional-programming/article/consistent-semantics-of-selfadjusting-computation/441A28C813BDA23B57F1ED2BB1A7E36E

Core semantic property:

```
update(previous_execution, input_change)
      ≡
recompute_from_scratch(new_input)
```

when the self-adjusting semantics applies.

This is exactly the kind of realization-equivalence contract the project needs.

## 2. Program derivatives / change actions

Incremental-computation research formalizes a derivative-like operation for programs:

```
F : X -> Y

ΔF : X × ΔX -> ΔY
```

such that:

```
F(x ⊕ dx) = F(x) ⊕ ΔF(x, dx)
```

under the declared change structure/action.

This is not ordinary calculus differentiation. `ΔX` and `ΔY` may describe:

- insertions/deletions in sets;
- relation updates;
- graph edge changes;
- map changes;
- lattice information changes;
- numeric deltas.

Source:
- https://arxiv.org/abs/1811.06069

This creates a candidate mathematical metaprimitive:

```
INCREMENTALIZE(C)
    -> delta-construction ΔC
```

with its own correctness proof.

## 3. Derivatives of fixed points

A particularly important result is that change-action theory can derive **derivatives of least fixed points**, enabling incremental maintenance of recursive computations such as Datalog transitive closure.

Sources:
- https://arxiv.org/abs/1811.06069
- https://link.springer.com/chapter/10.1007/978-3-030-17184-1_19

This ties directly to earlier project research:

```
recursive mathematical construction
        ↓
least fixed point μF
        ↓
input changes
        ↓
derivative/update of fixed point
        ↓
new μF without full recomputation
```

So fixed-point semantics and incremental execution can share one mathematical framework.

## 4. Differential dataflow

Differential dataflow maintains large, iterative, recursive computations as input collections change.

It supports:

- nested iteration;
- dynamic graph computations;
- indexed shared arrangements;
- incremental changes represented explicitly as `(data, time, diff)` updates;
- reuse across multiple queries/dataflows.

Sources:
- https://www.microsoft.com/en-us/research/publication/differential-dataflow/
- https://timelydataflow.github.io/differential-dataflow/
- https://timelydataflow.github.io/differential-dataflow/chapter_5/chapter_5.html

Its `arrangements` are especially relevant: expensive indexed state is built once and shared across multiple downstream queries rather than reconstructed independently.

This resembles what the mathematical universe will need for reusable semantic indexes.

## 5. Higher-order deltas

DBToaster applies recursive finite differencing to database queries, materializing not only a query result but also higher-order delta queries that help maintain the result at very high update rates.

Sources:
- https://researchconnect.buffalo.edu/en/publications/dbtoaster-higher-order-delta-processing-for-dynamic-frequently-fr-2/
- https://researchconnect.buffalo.edu/en/publications/dbtoaster-higher-order-delta-processing-for-dynamic-frequently-fr/

So the system can in principle compile:

```
F
ΔF
Δ²F
...
```

where higher-order changes make later incremental maintenance cheaper.

This is a powerful donor for repeatedly changing domain data such as market state, engineering telemetry, graph state, or evolving mathematical assumptions.

## Architecture-changing conclusion

A promoted primitive should not necessarily contain only:

```
solve(input) -> result
```

It may also contain certified update constructions:

```
update(previous_state, delta_input) -> delta_result / new_result
```

Potential primitive package:

```
C184
  |- semantic construction F
  |- full evaluator R_full
  |- incremental evaluator R_delta
  |- dependency schema
  |- persistent reusable index/state
  |- delta correctness certificate
  |- threshold policy for delta vs rebuild
```

## Semantic identity versus cached state

The cached execution state is **not mathematical authority** by itself.

The semantic construction remains authoritative.

The incremental realization must establish:

```
apply_delta(cache_for_x, dx)
   produces semantics equivalent to
F(x ⊕ dx)
```

If cached state is corrupt/stale or dependency digests mismatch, the engine must rebuild or revalidate rather than trusting it.

This aligns with Tenfold-style proof freshness.

## Change provenance

A delta should itself be a mathematical object:

```
Delta {
  base_generation_digest,
  operation,
  affected_objects,
  semantic_change,
  provenance,
  timestamp/logical_version
}
```

This lets the engine know whether a cached derivation remains reusable.

## Incremental theorem/capability closure

This pass also strengthens the earlier incremental-capability-closure hypothesis.

If a new certified fact is added:

```
Field(D)
```

then only capability closure depending on that property should be updated.

Likewise if one graph edge, constraint, coefficient, market observation, or boundary condition changes, only mathematically dependent structures should be revisited when an incremental law exists.

## Deletions and non-monotone changes

Insertions are easier than deletions in many monotone systems.

The Theory Profile must therefore distinguish:

```
insert_delta_supported
remove_delta_supported
replacement_delta_supported
nonmonotone_update_supported
```

and whether updates require:

- reference counts/provenance;
- truth maintenance;
- recomputation of affected strongly-connected region;
- new generation rather than mutation.

## Cost policy

Incremental execution is not always cheaper.

The runtime should compare:

```
Cost(delta update)
vs
Cost(rebuild)
```

using certified or measured cost models.

A large or structurally destructive change may trigger a rebuild even if a mathematically valid delta algorithm exists.

Thus:

```
semantic correctness of ΔF
   !=
performance preference for ΔF
```

## Repeated-query leverage

Incrementalization is especially powerful for clients such as:

```
Wolf-Coin:
 market state changes continuously

engineering systems:
 sensor/state updates

graph/network problems:
 edges/nodes added or removed

proof/search campaigns:
 new lemmas/counterexamples/constraints

interactive mathematical use:
 user adjusts one assumption/parameter
```

The entire mathematical campaign need not restart.

## Interaction with work cells

A prior campaign can leave reusable certified artifacts:

```
- decomposition
- compiled representation
- e-class index
- bound hierarchy
- proof graph
- oracle cache
- active constraints
```

When the problem changes, the scheduler computes the dependency cone and launches only the cells whose obligations became stale or whose delta rules can improve the result.

This is much stronger than ordinary memoization.

## Core law

> **When mathematics changes locally and the Theory Profile provides a sound change action, update the mathematics locally rather than rediscovering the unaffected world.**

## Open research

1. Certified incrementalization of algebraic/symbolic constructions beyond relational/Datalog workloads.
2. Dynamic algorithms for Gröbner bases, factorizations, decompositions, and theorem indexes.
3. Incremental maintenance of e-graphs/versioned semantic equivalence views.
4. Incremental proof/certificate repair rather than total re-proving.
5. Differential updates through chains of certified reductions and theory morphisms.
6. Higher-order delta construction synthesis.
7. Out-of-core/persistent incremental state under memory pressure.
8. Deterministic replay of incremental computations and change streams.
9. Automatic break-even policies for delta update versus from-scratch rebuild.
10. Whether `INCREMENTALIZE` can become a certifying compiler transform similar to automatic differentiation.
