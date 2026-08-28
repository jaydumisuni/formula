# Research Pass — Conjugacy, Linearization, Normal Forms, and Automatic Difficulty-Changing Transformations

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates one of the closest mathematical analogues to the project's original ambition: automatically discover a transformation that changes the representation of a hard nonlinear problem so that the transformed problem becomes substantially simpler.

The central pattern is:

```text
hard dynamics/problem F
    -> discover transformation T
    -> G = T o F o T^-1
    -> G belongs to simpler/solvable class
    -> solve/analyze G
    -> transport result back through T^-1
```

The central finding is:

> **This is a real and active mathematical program: conjugacy/normal-form methods, feedback linearization, Koopman eigenfunctions, and Carleman lifting all seek transformations that replace nonlinear dynamics with simpler linear/normal forms under declared conditions. What remains difficult is discovering the right transformation and certifying its domain/quality.**

---

## 1. Conjugacy is the exact form of “same mathematics, easier representation”

Two dynamical systems can be related through a change of coordinates/transformation `T` so that their evolution is conjugate:

```text
T(F(x)) = G(T(x))
```

or equivalently:

```text
G = T o F o T^-1
```

when the inverse exists on the relevant domain.

Source:

https://epubs.siam.org/doi/10.1137/17M116207X

### Architectural implication

The project should have a first-class transformation claim stronger than ordinary rewrite equivalence:

```text
Conjugacy
    source_dynamics
    target_dynamics
    map T
    inverse/partial_inverse
    domain
    exactness
    certificate
```

Once established, solution/proof machinery for `G` can be transported back to `F`.

---

## 2. Koopman theory searches for coordinates in which nonlinear dynamics become linear

The Koopman operator acts linearly on observables even when the underlying state dynamics are nonlinear. Suitable Koopman eigenfunctions can provide coordinates/embeddings in which dynamics become linear.

Sources:

https://epubs.siam.org/doi/10.1137/21M1401243

https://www.nature.com/articles/s41467-017-00030-8

### Architectural implication

Representation search can explicitly target:

```text
find observables phi(x)
```

such that:

```text
phi(F(x)) = K phi(x)
```

for a simpler linear operator `K`.

This is not merely approximating the output; it is searching for a mathematical coordinate system that changes the structure of the problem.

---

## 3. Koopman research explicitly targets discovery of rectifying transformations

Work on Koopman eigenfunctions describes the goal as systematic discovery of transformations analogous to Cole-Hopf transformations that can match or rectify nonlinear systems.

Source:

https://epubs.siam.org/doi/10.1137/17M116207X

### Architectural implication

This validates a major project metaprimitive family:

```text
discover_conjugacy(F, target_class)
discover_linearizing_coordinates(F)
discover_rectifying_transform(F)
```

The target class can be chosen from mathematical structures for which the project already owns strong primitives.

Thus the search objective can be:

> Find a map from this unknown/hard class into any known class with cheap certified solving.

---

## 4. Exact finite-dimensional linearization is not always possible

Modern Koopman literature emphasizes that finite-dimensional invariant coordinate systems are difficult to obtain and may not exist for broad nonlinear/chaotic systems.

Sources:

https://epubs.siam.org/doi/10.1137/21M1401243

https://www.nature.com/articles/s42005-024-01626-5

### Architectural implication

The result taxonomy must distinguish:

```text
EXACT_GLOBAL_CONJUGACY
EXACT_LOCAL_CONJUGACY
EXACT_EMBEDDING_HIGHER_DIMENSION
APPROXIMATE_WITH_CERTIFIED_BOUND
HEURISTIC_APPROXIMATION
NO_TRANSFORM_IN_SEARCHED_CLASS
UNKNOWN
```

Failure to find a finite linearization must never be interpreted as proof that no useful representation change exists unless a completeness theorem applies.

---

## 5. Exact analytical nonlinear solutions can emerge from discovered Koopman eigenfunctions

Recent work constructs exact/global Koopman eigenfunctions from invariant manifolds for restricted one- and two-dimensional nonlinear ODEs, yielding analytical solutions for previously unsolved examples.

Source:

https://epubs.siam.org/doi/10.1137/22M1516622

### Architectural implication

Earlier invariant-discovery research and transformation discovery reinforce each other:

```text
discover invariant manifolds
    -> construct Koopman eigenfunctions
    -> obtain linearizing representation
    -> derive analytical solution
```

One mathematical discovery can unlock the next transformation family automatically.

---

## 6. Carleman linearization trades nonlinearity for dimension

Carleman linearization embeds polynomial/analytic nonlinear dynamics into an infinite-dimensional linear system. Finite truncation gives a practical approximate linear realization, and modern research provides explicit truncation-error bounds under declared conditions.

Sources:

https://arxiv.org/abs/2207.07755

https://epubs.siam.org/doi/10.1137/1.9781611976847.1

### Architectural implication

Representation search should understand a three-way trade:

```text
nonlinearity
representation dimension
approximation error
```

A larger representation can make operations structurally simpler.

This is an important warning against minimizing representation size blindly.

Sometimes:

```text
small nonlinear problem
```

is computationally worse than:

```text
larger sparse linear problem
```

on ordinary hardware.

---

## 7. Approximate transformation can still be mathematically certified

Carleman finite-section research derives explicit error bounds between the truncated linearized dynamics and original nonlinear system.

Source:

https://arxiv.org/abs/2207.07755

### Architectural implication

A transformation certificate can state:

```text
semantic_relation: approximate_conjugacy / embedding
valid_domain
valid_time_horizon
error_bound
truncation_order
```

This connects directly to the certified-progress and numerical-realization passes: adding compute/order can refine the representation and reduce the certified error.

---

## 8. Normal-form theory systematically removes irrelevant nonlinear terms

Poincare/normal-form methods apply successive polynomial coordinate transformations to eliminate nonlinear terms when algebraic/resonance conditions permit.

Source:

https://www.cambridge.org/core/services/aop-cambridge-core/content/view/1B52E027A2FAC21A69359E94D6A3F035/9780511564161c4_p74-99_CBO.pdf/normal_forms.pdf

### Architectural implication

The search compiler should include:

```text
simplify_by_conjugacy
```

not only:

```text
simplify_expression
```

A normal form can preserve the essential dynamics while removing coordinate-dependent complexity.

This is a deeper notion of simplification than expression length.

---

## 9. Resonances/obstructions are valuable negative structure

Normal-form and linearization theory identifies terms/structures that cannot be removed because of resonance or other obstruction conditions.

### Architectural implication

Failed transformation attempts should derive obstruction knowledge:

```text
term family R cannot be eliminated by transformation class T
under spectrum/structure condition C
```

Such an obstruction is a durable pruning theorem preventing repeated attempts to linearize an impossible region in the same way.

---

## 10. Feedback linearization extends transformation search to controlled systems

Nonlinear control theory gives conditions and constructions for feedback equivalence to linear systems. Symmetry-based approaches can systematize dynamic feedback linearization, with some constructions automated in symbolic differential-geometry software.

Sources:

https://arxiv.org/abs/2103.05078

https://arxiv.org/abs/2104.02141

### Architectural implication

The target transformation need not act only on state coordinates.

More general equivalence searches can include:

```text
coordinate change
input transformation
state augmentation
feedback
higher-dimensional embedding
```

provided every transformation's semantics and invertibility/reconstruction contract is explicit.

---

## 11. Transformation discovery should target known capability classes

Suppose the primitive registry already has exceptionally strong algorithms for:

```text
linear systems
convex optimization
finite automata
polynomial ideals
SAT/SMT fragments
sparse tensor contractions
```

### Architectural implication

Rather than searching arbitrary transformations with no destination, the compiler can formulate:

```text
find T such that T(P) is recognized as class C
```

where `C` is a class with cheap certified solvers.

This converts representation discovery into **goal-directed reduction synthesis**.

---

## 12. The cost of transformation and inverse must be included

A problem may become trivial in transformed space but expensive to encode/decode.

### Architectural implication

Transformation search utility should include:

```text
cost(T)
cost(solve target)
cost(T^-1 / reconstruction)
certificate cost
reuse/amortization value
```

A transformation can still be valuable if expensive once but reusable across many later queries.

---

## 13. Current conjugacy/linearization hypothesis

```text
HARD MATHEMATICAL OBJECT P
    -> profile structure/invariants/symmetries
    -> choose promising target classes C1...Cn
    -> synthesize/search transformations T
    -> check exactness/invertibility/domain
    -> classify transformed problem T(P)
    -> solve using mature primitive for target class
    -> reconstruct result
    -> compose certificates
    -> promote reusable transformation when generalizable
```

This is perhaps the clearest current formalization of the original vision:

> **do not merely calculate the hard problem faster; discover the representation in which it belongs to mathematics we already know how to solve.**

---

## 14. New research obligations

1. Study automated conjugacy/normal-form synthesis with certificate-producing transformations.
2. Investigate exact versus approximate Koopman invariant-subspace discovery and how to certify closure.
3. Study representation-search target selection from the capability registry.
4. Investigate automatic obstruction/resonance extraction and durable pruning rules.
5. Connect symmetry/invariant discovery directly to conjugacy search.
6. Study automatic reconstruction/inverse synthesis when `T` is injective only on a restricted domain.
7. Investigate transformations into non-linear-but-easier classes, not only linear targets.
8. Study cost models that include transform, solve, inverse, proof, and future reuse.
9. Investigate generalization of successful problem-specific transformations into promoted cross-domain primitives.
10. Study whether e-graphs/e-hypergraphs can search families of conjugacies/embeddings without materializing every transformed problem.
