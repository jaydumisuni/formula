# Research Pass — Self-Specialization, Semantic Compression, and Residual Primitive Generation

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how a general mathematical semantic relation/search procedure can become a fast permanent primitive after repeated use, without making the optimized implementation the source of mathematical truth.

The emerging loop is:

```text
repeated solved structures
    -> semantic abstraction/generalization
    -> certified new relation/construction
    -> specialization for common query direction
    -> native optimized residual implementation
    -> independent realization validation
```

---

## 1. Partial evaluation turns a generic interpreter plus fixed information into a specialized program

Partial evaluation specializes a program with respect to known/static inputs and emits a residual program containing only the computations dependent on the remaining dynamic inputs.

The Futamura-projection tradition shows that specializing an interpreter with respect to a source program can yield a compiled program, and self-applicable specializers can derive compiler-generation behavior.

Sources:

https://link.springer.com/book/10.1007/3-540-16446-4

https://link.springer.com/chapter/10.1007/b94823_8

### Architectural implication

If the project owns a generic semantic relation/interpreter:

```text
R(problem, assumptions, query_direction, result)
```

then fixing stable parts such as:

```text
theory
representation
query direction
assumptions class
primitive vocabulary
```

may allow generation of a much smaller residual solver:

```text
specialize(R, fixed_context)
    -> fast residual primitive
```

This supports the earlier concept that one relational semantic object can generate several directional realizations without using one slow universal relational interpreter at runtime.

---

## 2. Supercompilation performs deeper whole-program semantic transformation

Supercompilation symbolically executes/generalizes programs to remove interpretive overhead, intermediate structures, and redundant computation while preserving semantics under declared conditions.

Sources:

https://link.springer.com/book/10.1007/3-540-16446-4

https://arxiv.org/abs/1005.5278

Supercompilation has also been used as a verification technique by specializing interpreters/models and exposing simplified residual structures whose safety properties can be proved.

Sources:

https://arxiv.org/abs/1708.09002

https://arxiv.org/abs/1705.06738

### Architectural implication

The project's compiler/distillation layer may eventually use transformation mechanisms more powerful than conventional constant folding/JIT specialization.

A discovered construction may be **semantically collapsed** into a simpler residual algorithm.

The resulting transformation still requires independent equivalence/realization checking before primitive admission.

---

## 3. Library learning compresses repeated solved programs into reusable abstractions

DreamCoder grows a domain language by inventing abstractions that explain recurring structures in solved programs. Stitch performs scalable top-down library learning and compresses program corpora into reusable abstractions.

Sources:

https://arxiv.org/abs/2006.08381

https://github.com/mlb2251/stitch

### Architectural implication

Repeated mathematical constructions can be post-processed for common semantic structure:

```text
C1, C2, C3, ...
    -> candidate abstraction/generalization G
    -> mathematical falsification/certification
    -> promoted construction G
```

The important difference from ordinary library learning is:

> Compression or reuse frequency does not grant mathematical authority.

A compressed abstraction becomes a mathematical primitive only after its semantics/domain and proof obligations are independently established.

---

## 4. Rewrite-rule learning can make the transformation language itself grow

Ruler infers compact general rewrite rules from a semantic interpreter using equality saturation.

Sources:

https://arxiv.org/abs/2108.10436

https://github.com/uwplse/ruler

### Architectural implication

Primitive growth may occur at several levels:

```text
value-level primitive
    computes a recurring mathematical relation

rewrite primitive
    establishes a reusable semantic equivalence

metaprimitive
    changes/generalizes/searches mathematical structures
```

A newly certified rewrite can reduce future equality/candidate search globally without being invoked as a conventional function.

---

## 5. Equality saturation avoids prematurely choosing transformation order

Equality saturation retains many equivalent versions simultaneously and extracts an implementation only after building the equivalence space.

Source:

https://arxiv.org/abs/1012.1802

### Architectural implication

Primitive distillation should avoid a rigid pipeline where an early local optimization destroys a representation needed by a later transformation.

Possible route:

```text
certified semantic construction
    -> bounded equivalence expansion
    -> global extraction under realization cost model
    -> translation validation
```

This keeps mathematical identity independent of optimization order.

---

## 6. Inverse transformations can expose hidden structure before being eliminated

Szalinski uses equality saturation plus **inverse transformations** to decompile low-level CAD programs into higher-level structured programs. Inverse transformations can be speculatively introduced to expose patterns that ordinary forward rewriting cannot see.

Sources:

https://arxiv.org/abs/1909.12252

https://github.com/uwplse/szalinski

### Architectural implication

Representation discovery may intentionally introduce temporary inverse/canceling structure:

```text
X
    -> T^-1(T(X))
    -> expose factorization/pattern in T(X)
    -> transform at higher level
    -> eliminate temporary inverse pair
```

This is a concrete mechanism for the project's earlier intuition that inversion should be first-class in mathematical search.

Such speculative transformations must remain semantically justified and bounded to avoid search explosion.

---

## 7. Search strategy itself may become an explicit synthesized artifact

Recent equality-saturation work represents search/optimization strategy explicitly rather than only rewrite rules, because unrestricted saturation can explode in RAM/time.

Example:

https://arxiv.org/abs/2604.17364

The reported system uses models for strategy synthesis, which is not a dependency model we need to adopt. The important donor lesson is that **search strategies can themselves be represented as reusable programs/artifacts**.

### Architectural implication

The project may eventually distill successful campaign policies into deterministic strategy primitives such as:

```text
when TheoryProfile matches P
apply representation sequence R
use rewrite subset W
saturate to budget B
extract using cost C
```

These are operational/search capabilities, not mathematical truths.

They should be versioned, benchmarked, replayable, and replaceable.

---

## 8. Specialization can erase the cost of generality

The project wants a rich general semantic substrate while ordinary invocation remains cheap.

Partial evaluation/supercompilation provide an explicit mechanism:

```text
very general semantics
    -> fix common theory/direction/representation
    -> compile away generic dispatch/search
    -> native residual implementation
```

Thus the architecture does not necessarily face a permanent tradeoff between:

```text
general mathematical semantics
```

and:

```text
fast ordinary execution
```

provided frequently used cases can be specialized and validated.

---

## 9. Primitive promotion should separate three identities

Current research now suggests at least:

```text
SEMANTIC IDENTITY
    what mathematical relation/construction is established

SPECIALIZATION IDENTITY
    which assumptions/direction/theory/context are fixed

REALIZATION IDENTITY
    exact executable CPU/GPU implementation
```

Example:

```text
Relation R
    semantic_digest = ...

R specialized to integer domain D and forward query
    specialization_digest = ...

AVX2 implementation v7
    realization_digest = ...
```

Changing the implementation does not change the mathematics.

Changing fixed assumptions may create a different specialization while preserving the parent semantic relation.

---

## 10. Current self-expansion loop

A stronger end-to-end hypothesis is:

```text
SOLVED CAMPAIGNS
    -> detect recurring semantics
    -> abstraction / anti-unification / theory intersection
    -> candidate generalized relation G
    -> falsify / certify G
    -> promote semantic primitive G
    -> infer common query directions/contexts
    -> partial-evaluate / supercompile G
    -> search optimized realization
    -> independent realization validation
    -> publish residual primitives
    -> future campaigns start stronger
```

This is closer to genuine self-expanding mathematical capability than merely adding library functions manually.

---

## 11. New research obligations

1. Investigate modern partial-evaluation/supercompilation systems that can operate on relational/logic DSLs rather than only functional programs.
2. Determine how specialization should preserve universal certificate-envelope lineage back to the unspecialized semantic relation.
3. Study termination/control of supercompilation so specialization itself cannot diverge uncontrollably.
4. Investigate equivalence checking between a general relation and a specialized directional residual implementation.
5. Determine which mathematical metadata should remain runtime-visible after specialization and which can be erased safely.
6. Study abstraction/library-learning methods over graph/hypergraph relations rather than only tree/program ASTs.
7. Investigate whether inverse-transform introduction can be generalized beyond CAD into algebraic/graph/constraint representations.
8. Determine how often primitive promotion should occur and how to avoid vocabulary bloat from over-specialized primitives.
9. Investigate semantic subsumption: when a new primitive makes older primitives redundant or representable as special cases.
10. Determine how search-policy primitives should be evaluated, garbage-collected, and distilled without coupling their success metrics to mathematical truth.
