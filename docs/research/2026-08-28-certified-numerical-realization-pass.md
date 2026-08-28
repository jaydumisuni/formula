# Research Pass — Certified Numerical Realization, Stability, and Adaptive Precision

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can execute exact/real-valued semantics at high speed using finite-precision hardware without confusing algebraic equivalence with numerical reliability.

The central finding is:

> **A mathematically correct real-valued construction and a fast floating-point realization are separate artifacts. Numerical realization requires its own error/stability contract, and equivalent real formulas may have radically different finite-precision behavior.**

---

## 1. Real equivalence does not imply floating-point equivalence

Herbie demonstrates that two expressions equal over the reals can behave very differently under floating-point rounding. It searches algebraic rewrites to reduce observed numerical error.

Sources:

https://herbie.uwplse.org/

https://herbie.uwplse.org/doc/latest/error.html

### Architectural implication

The project must preserve at least three semantic layers:

```text
EXACT / IDEAL MATHEMATICAL SEMANTICS

FINITE-PRECISION SEMANTICS

CONCRETE MACHINE REALIZATION
```

An equality valid in the first layer does not automatically authorize replacement in the second.

---

## 2. Numerical stability can itself be searched

Herbie automatically discovers algebraically equivalent rewrites that often improve floating-point accuracy, including avoiding catastrophic cancellation.

Source:

https://herbie.uwplse.org/papers.html

### Architectural implication

Primitive realization search should include objectives beyond runtime:

```text
roundoff error
relative/absolute error
ULP error
overflow/underflow risk
exception risk
latency
memory
```

This produces a Pareto search over numerical quality and speed rather than blindly choosing the shortest expression.

Heuristic search can propose candidates; certification remains separate.

---

## 3. Rigorous roundoff-error analysis already exists

FPTaylor rigorously estimates floating-point roundoff errors using symbolic Taylor forms and optimization.

Source:

https://github.com/soarlab/FPTaylor

Daisy integrates multiple sound finite-precision error analyses and optimization techniques including interval subdivision, FPTaylor-style bounds, mixed precision, and rewriting.

Sources:

https://link.springer.com/chapter/10.1007/978-3-319-89960-2_15

https://github.com/malyzajko/daisy

### Architectural implication

A numerical realization can carry a certificate/analysis envelope such as:

```text
input_domain
ideal_semantics_digest
machine_format
rounding_mode
absolute_error_bound
relative_error_bound
range_bound
exception guarantees
analysis/certificate lineage
```

rather than merely `uses f64`.

---

## 4. Numerical proofs can be independently checked

Gappa proves properties about floating/fixed-point computations and can generate formal proof artifacts checkable by Coq.

Sources:

https://arxiv.org/abs/0801.0523

https://gappa.gitlabpages.inria.fr/gappa/tools.html

### Architectural implication

The project's solver/verifier separation extends naturally to numerical kernels:

```text
aggressive numerical optimizer
    -> candidate floating kernel
    -> error/range certificate
    -> independent checker/formal proof path
    -> admitted numerical realization
```

The optimizer does not need truth authority.

---

## 5. Precision should be selected, not globally maximized

Precimonious searches mixed-precision assignments subject to an accuracy specification and performance objective, demonstrating that using maximum precision everywhere can be unnecessarily expensive.

Sources:

https://github.com/corvette-berkeley/precimonious

https://people.eecs.berkeley.edu/~ksen/papers/precimonious.pdf

### Architectural implication

A numerical primitive may compile to a **mixed-precision realization**:

```text
operation A -> f32
operation B -> f64
critical accumulator -> f128 / MPFR
```

provided the resulting whole-program error contract is certified.

Precision becomes an optimization variable.

---

## 6. Adaptive precision can preserve ordinary-hardware performance

Earlier exact-real and interval research established that a computation can refine precision only where necessary.

This pass strengthens that into an execution policy:

```text
try cheap precision
    -> certify requested bound?
        yes: stop
        no: refine precision / representation
```

### Architectural implication

The runtime need not choose between:

```text
fast unsafe float
```

and

```text
huge exact arithmetic everywhere
```

It can use escalating precision under a certified stop condition.

---

## 7. Conditioning belongs in the Theory/Problem Profile

Roundoff error and computational difficulty may be inherent to the mathematical problem rather than to a bad implementation. Ill-conditioned problems amplify small input/rounding perturbations.

### Architectural implication

The mathematical compiler should distinguish:

```text
ALGORITHMIC INSTABILITY
    realization can be improved

PROBLEM CONDITIONING
    input-output map itself amplifies uncertainty
```

A request for precision beyond what input uncertainty/conditioning supports should not waste arbitrary compute.

The result should expose the limiting mathematical cause.

---

## 8. Numerical realization should support regime splitting

One formula may be stable in one input region and poor in another. Herbie and Daisy-style work supports region/precondition-specific optimization.

### Architectural implication

The compiler may generate certified piecewise realizations:

```text
if x in regime R1 -> kernel K1
if x in regime R2 -> kernel K2
...
```

where every regime boundary and kernel error contract is explicit.

This is another form of automatic mathematical decomposition.

---

## 9. Error should compose through the construction graph

A large mathematical construction may have thousands of numerical suboperations. Local error bounds need composition rules so the final result contract is derived from the entire graph.

### Architectural implication

Numerical metadata should be attached compositionally to executable realization graphs rather than recorded only at top-level APIs.

The compiler can then identify which internal operation dominates the error budget and spend additional precision only there.

---

## 10. Correctness and numerical quality are separate claims

A realization can be:

```text
semantically exact but slow

finite-precision with certified error ≤ e

correctly rounded

probabilistically evaluated

heuristically accurate only
```

These are different assurance classes.

### Architectural implication

Primitive selection should honor the client's required result class:

```text
exact
correctly-rounded
rigorous-enclosure
certified-relative-error
heuristic-fast
```

A Wolf-Coin exploratory computation may accept a different realization than a theorem-certification path while both use the same semantic construction.

---

## 11. Current numerical-realization hypothesis

```text
CERTIFIED MATHEMATICAL CONSTRUCTION
    -> identify numeric domain / conditioning / requested accuracy
    -> generate algebraically equivalent realizations
    -> search stability + precision + performance
    -> rigorous error/range analysis
    -> realization certificate
    -> compile native SIMD/CPU/GPU kernel
    -> translation/error validation
    -> publish realization under explicit numeric contract
```

This lets the project use ordinary floating hardware aggressively without making floating point the mathematical truth model.

---

## 12. New research obligations

1. Study automatic conditioning analysis and condition-number estimation/certification.
2. Investigate verified mixed-precision synthesis rather than dynamic testing-only approaches.
3. Map Gappa/FPTaylor/Daisy certificate outputs into the universal certificate envelope.
4. Study correctly rounded elementary-function synthesis and tools such as Sollya/Metalibm.
5. Investigate numerical regime synthesis with formally verified branch boundaries.
6. Study error propagation through loops/fixed points and relational propagators.
7. Determine when interval/ball arithmetic beats finite-precision error analysis and when it does not.
8. Investigate automatic escalation from float -> extended precision -> modular/exact methods based on certification failure.
9. Study hardware-specific FMA/SIMD semantics and realization validation.
10. Define numerical-result classes that domain clients can request without understanding the underlying implementation.
