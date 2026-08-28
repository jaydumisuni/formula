# Research Pass — Compositional Progress, AND/OR Proof Graphs, and Root-Level Mathematical Value

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how certified partial progress in individual mathematical work cells should combine when a larger goal decomposes into alternatives and required subgoals.

The central finding is:

> **Local mathematical progress is not enough for scheduling. Progress must be propagated through the logical/dependency structure of the goal so the system estimates how a cell changes the status or certified bounds of the root problem.**

---

## 1. AND/OR graphs are a direct model of decomposed mathematical problems

An AND/OR graph represents two fundamentally different forms of decomposition:

```text
AND node:
    all required children must succeed

OR node:
    one admissible child may be sufficient
```

Classic AND/OR search treats a solution as a solution subgraph rather than a single path.

Sources:

https://www.sciencedirect.com/science/article/abs/pii/0004370271900063

https://doi.org/10.1145/2455.2459

### Architectural implication

The project's proof/problem graph should make the combination law explicit per node. A generic dependency list is insufficient.

A mathematical obligation may be:

```text
AND
OR
MIN
MAX
weighted aggregate
fixed-point dependency
other certified combination rule
```

and the search economy should propagate child progress using that declared semantics.

---

## 2. Proof-number search turns proof/disproof difficulty into compositional estimates

Proof-number search assigns proof and disproof numbers to nodes in an AND/OR tree. At OR nodes, one child can prove the parent; at AND nodes, all children are required. The dual rules apply to disproof.

Source:

https://webdocs.cs.ualberta.ca/~mmueller/research/proof-set-search.html

Background summary:

https://www.researchgate.net/publication/250821076_Pattern_Knowledge_for_Proof-Number_Search_in_Computer_Go

### Architectural implication

A mathematical campaign can maintain separate estimates:

```text
estimated_remaining_proof_work
estimated_remaining_disproof_work
```

rather than one generic priority.

This fits the project's adversarial principle: proving and refuting a conjecture are different campaigns whose expected work can evolve independently.

---

## 3. Root impact can differ radically from local progress

Consider:

```text
ROOT = A AND B AND C
```

If A and B are essentially solved but C is the only remaining gating obligation, a small advance on C may be worth more to the root than a large local improvement on A.

Likewise:

```text
ROOT = A OR B OR C
```

once A becomes certified, additional work on B and C may have near-zero root value unless the objective asks for all alternatives or a better-cost solution.

### Architectural implication

Scheduling utility should include something conceptually like:

```text
root_impact(cell_result)
```

computed through the mathematical goal graph.

This is different from:

```text
local_information_gain(cell_result)
```

Both matter.

---

## 4. AO* propagates admissible bounds through AND/OR solution graphs

AO* and related AND/OR heuristic-search methods propagate cost estimates from partial subgraphs back to the root. Under admissible lower-bound conditions they can recover optimal solution graphs.

Sources:

https://www.sciencedirect.com/science/article/abs/pii/0004370271900063

https://www.sciencedirect.com/science/article/pii/S0921889021002505

A generalized AND/OR best-first framework can maintain both upper and lower estimates.

Source:

https://www.sciencedirect.com/science/article/pii/0196677492900144

### Architectural implication

For costed mathematical constructions, the root problem can carry recursively derived certified/optimistic bounds rather than waiting for every leaf to finish.

This gives a compositional counterpart to the optimization-gap research in the certified-progress pass.

---

## 5. Cyclic mathematical dependencies require fixed-point treatment

Plain AO* assumes acyclic solution structure. LAO* extends heuristic AND/OR search to cyclic settings using dynamic-programming/value/policy iteration.

Source:

https://cdn.aaai.org/AAAI/1998/AAAI98-058.pdf

### Architectural implication

The project's goal graph cannot assume every decomposition is a DAG.

Recursive mathematics may create:

```text
A depends on B
B depends on C
C depends on A
```

Such regions should be collapsed/profiled as fixed-point components and use the project's least/greatest-fixed-point machinery rather than pretending they are ordinary acyclic work dependencies.

---

## 6. Proof sets can be better than scalar proof numbers when subgraphs share work

Proof-set search was developed because ordinary proof-number estimates can overcount work when the same nodes/subgoals are reachable through multiple paths. It tracks proof/disproof sets to better approximate minimal proof structures.

Source:

https://webdocs.cs.ualberta.ca/~mmueller/research/proof-set-search.html

### Architectural implication

The mathematical universe will contain extensive shared dependencies and equivalent subgoals.

Therefore:

```text
sum(child_costs)
```

can badly overestimate actual remaining work when several branches reuse the same theorem, certificate, representation, or primitive.

Scheduling/cost estimation should be graph-aware and deduplicate shared mathematical obligations.

---

## 7. Progress should compose differently for proof, disproof, optimization, and enumeration

The combination law is query-dependent.

Examples:

### Existential proof

```text
exists x: P(x)
```

One certified witness closes the goal.

### Universal proof

```text
forall x in finite partition: P(x)
```

all required partitions must close.

### Refutation

One valid counterexample may close a universal conjecture.

### Optimization

Children may contribute competing incumbents and global lower/upper bounds.

### Enumeration

All branches may be required even after one solution is found.

### Architectural implication

The requested result class must influence the AND/OR aggregation semantics.

The same underlying mathematical decomposition can have a different completion/progress law depending on what the client asked to establish.

---

## 8. A root-level progress envelope is plausible

A campaign may maintain something like:

```text
GoalProgressEnvelope
    semantic_status
    proof_lower_bound / proof_estimate
    disproof_lower_bound / disproof_estimate
    certified_result_bounds
    unresolved_frontier
    gating_subgoals
    shared_dependency_set
    completion_condition
```

Child updates are incrementally propagated to affected ancestors.

This resembles dynamic programming on the proof graph, but the aggregation operator is mathematical/query-specific.

---

## 9. Search economy should spend compute on marginal root value

A stronger scheduling objective becomes:

```text
expected_root_mathematical_gain
--------------------------------
expected_compute_cost
```

rather than merely:

```text
local_information_gain / cost
```

A cell with modest local progress can be the best investment if it lies on every remaining proof route.

Conversely, a spectacular local improvement on a branch already dominated by a certified alternative may have low marginal value.

---

## 10. Certified progress and heuristic search estimates must remain separate

AO*/proof-number estimates can guide search without constituting mathematical proof.

The architecture should distinguish:

```text
CERTIFIED ROOT STATE
    derived only from accepted mathematical evidence

SEARCH ESTIMATE
    heuristic/learned estimate of remaining cost or likelihood
```

A heuristic estimate may choose which cell to run next.

It must never promote root truth status.

---

## 11. Current compositional-progress hypothesis

The strongest current model is:

```text
MATHEMATICAL GOAL GRAPH
    AND / OR / optimization / fixed-point / other aggregation nodes

LEAF WORK CELLS
    -> certified partial progress + heuristic cost forecasts

INCREMENTAL PROPAGATION
    -> recompute only affected ancestor envelopes
    -> identify gating/frontier obligations

SEARCH ECONOMY
    -> allocate compute by expected marginal root value
       while preserving fairness/exploration guarantees
```

This turns the giant work-cell formation into a coordinated mathematical campaign rather than independent parallel searches.

---

## 12. New research obligations

1. Define a small algebra of goal-combination operators sufficient for proof, refutation, optimization, enumeration, and fixed-point subgoals.
2. Study AND/OR hypergraphs with shared subgraphs so repeated obligations are not double-counted.
3. Investigate exact/certified lower bounds on remaining proof or disproof work in selected domains.
4. Study compositional certificate construction: how child certificates combine into a parent certificate incrementally.
5. Determine how root impact is computed when progress states are partially ordered rather than scalar.
6. Study dynamic decomposition: a work cell may discover a new AND/OR factorization that changes the campaign graph itself.
7. Investigate cycle detection and automatic conversion of recursive dependency regions into fixed-point components.
8. Connect proof/disproof estimates to symbolic version-space elimination and compact-witness learning.
9. Study scheduling when one new theorem simultaneously advances many shared ancestors.
10. Investigate proof-set/minimal-support representations for estimating the true marginal value of shared mathematical work.
