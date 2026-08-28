# Research Pass — Executable Mathematical Semantics, Totality, Partiality, Effects, and Machine Arithmetic

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a foundational question created by the project's central premise that newly discovered mathematics may have an executable/program realization:

> **When is an executable program itself a mathematically meaningful object, and which semantic distinctions must remain explicit?**

The strongest conclusion is:

> **“Program” is not one semantic class. Total functions, partial functions, relations, nondeterministic transition systems, productive infinite processes, probabilistic procedures, and IEEE-754 computations require different mathematical semantics and different proof obligations.**

A program being executable does not make its behavior an admissible mathematical primitive.

---

## 1. Total pure functions provide the cleanest program-as-mathematics case

Lean's logical model treats ordinary logical functions as total: for every type-correct input they return a value in finite time. Recursive definitions must satisfy structural/well-founded termination conditions or other accepted recursion principles to participate transparently in mathematical reasoning.

Sources:

https://lean-lang.org/doc/reference/latest/The-Type-System/Functions/

https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/

Agda similarly enforces totality/termination for ordinary programs used in its type theory.

Sources:

https://agda.readthedocs.io/en/latest/getting-started/what-is-agda.html

https://agda.readthedocs.io/en/stable/language/termination-checking.html

### Architectural implication

The strongest primitive class is conceptually:

```text
TOTAL DETERMINISTIC RELATION/FUNCTION
    domain explicitly defined
    result exists for every admissible input
    termination established
    no hidden external effects
```

Such a primitive can often be treated directly as executable mathematics.

---

## 2. Nontermination/partiality must be represented explicitly

Lean permits `partial` definitions for practical programming, but they are opaque to the logic because they may not terminate. Lean also supports more controlled partial-fixpoint constructions whose equations can be reasoned about under a logical wrapper.

Source:

https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/

### Architectural implication

The project must not erase this distinction:

```text
f : X -> Y
```

versus:

```text
f : X ⇀ Y
```

where `f` is only defined/terminating on a subset of `X`.

A partial mathematical primitive needs metadata such as:

```text
domain_of_definition
termination_condition
failure/divergence semantics
proof status of those conditions
```

An unchecked potentially nonterminating implementation cannot be promoted as a total mathematical function.

---

## 3. Termination is part of semantics, not only runtime quality

Lean/Agda's termination discipline exists because unrestricted recursive equations can undermine logical consistency or fail to denote ordinary total functions.

Sources:

https://lean-lang.org/functional_programming_in_lean/Getting-to-Know-Lean/Summary/

https://agda.readthedocs.io/en/stable/language/termination-checking.html

### Architectural implication

This reinforces the Theory Profile:

```text
termination:
    PROVEN_TOTAL
    PROVEN_FOR_DOMAIN D
    PRODUCTIVE_COINDUCTIVE
    UNKNOWN
    NONTERMINATING_FOR_WITNESS
```

A primitive can still be useful when termination is conditional/unknown, but its search and certification route must reflect that fact.

---

## 4. Internal state can remain pure mathematics when modeled explicitly

A stateful computation can be represented mathematically as a transition:

```text
(input, state) -> (output, state')
```

or a relation among pre/post states.

Lean's logical description of `IO` itself uses a state-monad-like model in which the world token is threaded explicitly; this preserves ordering and prevents effectful values from silently escaping into pure values.

Source:

https://lean-lang.org/doc/reference/latest/IO/Logical-Model/

### Architectural implication

The project should distinguish:

```text
MATHEMATICAL STATE
    explicit state variable participating in semantic relation
```

from:

```text
EXTERNAL EFFECT
    filesystem/network/clock/device/random OS state
```

An algorithm with explicit internal state can still have pure mathematical semantics.

External effects belong to execution/workspace infrastructure and must not become hidden mathematical inputs.

---

## 5. Effects should be explicit in the semantic type

Lean's pure language separates effectful `IO α` computations from pure `α`; a value cannot normally escape the IO context without marked unsafe operations.

Source:

https://lean-lang.org/doc/reference/latest/IO/Logical-Model/

Algebraic-effects research similarly models effects as explicit operations with handlers and gives them denotational algebraic semantics.

Source:

https://arxiv.org/abs/1203.1539

### Architectural implication

A future executable-math language/IR should not allow hidden side effects inside a supposedly pure primitive.

Possible semantic effect classes:

```text
PURE
STATE_INTERNAL
NONDETERMINISTIC_SEARCH
PROBABILISTIC_SEARCH
EXTERNAL_IO
MACHINE_NONDETERMINISM
```

A compiled mathematical primitive intended for reusable truth-producing computation should normally expose only effects explicitly authorized by its semantic contract.

---

## 6. Nondeterminism is naturally a relation/set-valued semantics

The K Framework gives executable rewrite-based semantics to nondeterministic systems. Matching-logic/K terms can denote sets of configurations, and K distinguishes one-path from all-path reachability claims.

Source:

https://kframework.org/docs/user_manual/

### Architectural implication

Nondeterministic mathematical computation should not be forced into an arbitrary chosen output.

Instead:

```text
R(x, y)
```

may define a set of valid `y` for `x`.

Queries can ask:

```text
exists-path:
    is some result satisfying P reachable?

all-path:
    do all allowed evolutions satisfy P?

enumeration:
    describe all reachable result classes
```

These are different mathematical claims with different certificates.

---

## 7. Executable transition semantics can support verification directly

K specifications are both executable semantics and logical theories with reachability proof machinery. The same rules defining execution can support state-space exploration and deductive verification.

Sources:

https://kframework.org/docs/user_manual/

https://kframework.org/faq/

### Architectural implication

This is a strong donor pattern for discovered stateful/nondeterministic constructions:

```text
semantic transition theory
    -> execute examples
    -> search reachable states
    -> prove reachability/safety properties
```

The project does not need a separate ad hoc simulator semantics and proof semantics if a fragment can share one formally specified transition relation.

---

## 8. Infinite productive computation needs coinductive semantics, not termination

Earlier coinduction research showed that infinite streams/processes can be mathematically well-defined even though they never terminate as whole computations.

The relevant property is **productivity**: every finite observation can be produced in finite time, with coinductive/bisimulation proof principles governing infinite behavior.

This complements rather than contradicts termination checking.

### Architectural implication

Semantic classification should distinguish:

```text
TOTAL TERMINATING VALUE COMPUTATION
```

from:

```text
PRODUCTIVE INFINITE OBJECT/PROCESS
```

A stream generator or infinite symbolic structure may be a valid mathematical primitive even though evaluating the entire object never terminates.

---

## 9. Machine floating-point semantics is not real-number semantics

Flocq formalizes floating-point formats, rounding, IEEE-754 operations, exactness results, and rounding-error theorems inside Rocq/Coq.

Sources:

https://flocq.gitlabpages.inria.fr/

https://flocq.gitlabpages.inria.fr/theos.html

Gappa automatically proves interval/error properties of floating/fixed-point numerical programs and can emit proof scripts for Rocq/Coq checking.

Sources:

https://gappa.gitlabpages.inria.fr/gappa/index.html

https://gappa.gitlabpages.inria.fr/gappa/tools.html

### Architectural implication

A CPU primitive performing IEEE-754 arithmetic should be semantically classified as something like:

```text
IEEE754Binary64 computation under rounding mode R
```

not simply:

```text
Real arithmetic
```

A separate certificate can establish a relation between the machine result and ideal real mathematics:

```text
|machine_result - exact_real_result| <= ε
```

or a rigorous enclosure.

This preserves truth while allowing extremely fast machine arithmetic.

---

## 10. Approximate realization and exact semantic relation must be separate identities

Suppose semantic construction `C` is exact over reals, but realization `C_f64` uses floating point.

The relationship might be:

```text
C_f64(x) ∈ enclosure(C(x), ε)
```

rather than equality.

Gappa/Flocq demonstrate that such relations can be formally established for numerical programs.

Sources:

https://gappa.gitlabpages.inria.fr/gappa/examples.html

https://flocq.gitlabpages.inria.fr/flocq/html/Flocq.IEEE754.Binary.html

### Architectural implication

Realization validation must support more than exact equivalence:

```text
EXACT_EQUIVALENCE
REFINEMENT
RIGOROUS_ENCLOSURE
ERROR_BOUND
ONE_SIDED_BOUND
PROBABILISTIC_GUARANTEE
```

The semantic primitive and machine realization remain distinct artifacts.

---

## 11. Representation choice can change proof difficulty dramatically

Gappa's documentation shows equivalent mathematical expressions can produce very different enclosure/proof performance; rewriting to a structurally better equivalent form can make tight bounds provable.

Source:

https://gappa.gitlabpages.inria.fr/gappa/examples.html

### Architectural implication

Even numerical-certification work reinforces the project's central representation-search idea:

```text
same mathematical semantics
    -> different executable/algebraic representation
    -> radically different proof/analysis difficulty
```

Representation selection should therefore optimize not only runtime cost but also **certificate/proof cost**.

---

## 12. Program semantics should be first-class package metadata

A discovered executable construction should declare a semantic class before primitive promotion.

Research-level sketch:

```text
semantic_mode:
    total_function
    partial_function
    relation
    nondeterministic_transition
    productive_coinductive
    probabilistic_relation

state:
    none / explicit internal

effects:
    declared classes

termination/productivity:
    property + certificate

numeric_semantics:
    exact / IEEE754 / interval / other

query_modes:
    forward
    inverse
    reachability
    all-path
    enumeration
    optimization
```

This is not a frozen schema.

---

## 13. Search strategies and mathematical primitives have different effect permissions

Search machinery may legitimately use:

- clocks;
- benchmarking;
- randomized exploration;
- filesystem checkpoints;
- GPU queues;
- external solvers.

These are operational effects.

### Architectural implication

The project should maintain a strong boundary:

```text
MATHEMATICAL SEMANTICS
    pure/relation/state semantics under declared assumptions

SEARCH/REALIZATION PROCESS
    may use operational effects to discover/execute candidates
```

The method used to discover a mathematical primitive does not become part of the primitive's semantic truth unless explicitly modeled.

---

## 14. Totality can be promoted after discovery

A candidate algorithm may initially be explored as partial/unrestricted code.

After its behavior is understood, the system may discover/prove a measure or invariant establishing termination over a domain.

Then:

```text
candidate partial implementation
    -> discover termination argument
    -> certify totality on D
    -> specialize/compile
    -> promote total primitive on D
```

### Architectural implication

The project should not require every exploratory work cell to be born fully verified.

It requires **promotion** to establish the semantic class claimed by the permanent primitive.

This is analogous to the candidate-theory promotion boundary.

---

## 15. Current executable-semantics hypothesis

```text
SEMANTIC MATHEMATICAL OBJECT
        |
        +-- total relation/function
        +-- partial relation/function + domain
        +-- nondeterministic relation/transition system
        +-- productive coinductive object/process
        +-- probabilistic relation (future research)
        |
        v
SEMANTIC PROPERTIES
    termination/productivity
    determinism
    domain
    invariants
    exactness
        |
        v
EXECUTABLE REALIZATIONS
    reference interpreter
    native CPU
    SIMD
    GPU
    IEEE-754 approximation
        |
        v
REALIZATION CERTIFICATES
    exact equivalence / refinement / error bound / etc.
```

Program syntax is only one representation within this structure.

---

## 16. New research obligations

1. Study denotational/domain-theoretic semantics for partial recursive mathematical constructions and whether the project needs explicit bottom/divergence objects.
2. Investigate total-functional/corecursive languages as donors for the trusted semantic core without forcing all search machinery to be total.
3. Study effect systems/algebraic effects for cleanly separating internal mathematical state, nondeterminism, probability, and external IO.
4. Investigate K/matching logic as a donor for executable relational/state semantics and reachability certificates.
5. Define semantic classes for stochastic/probabilistic mathematical algorithms and distinguish Las Vegas search from probabilistic mathematical claims.
6. Study verified/coinductive productivity checking for infinite mathematical objects.
7. Investigate partial-function domain inference: automatically derive the weakest domain on which a candidate construction is total/correct.
8. Study automatic termination-measure/ranking-function synthesis as a primitive-promotion tool.
9. Build a machine-arithmetic semantics catalogue: IEEE-754, fixed point, modular overflow, arbitrary precision, SIMD/vector floating behavior.
10. Investigate proof-producing translation from exact real semantics to certified machine implementations using Flocq/Gappa/interval tools.
11. Determine how semantic effects participate in structure typing and primitive applicability.
12. Study equivalence/refinement checking for stateful and nondeterministic constructions, not only pure functions.
13. Investigate when operational nondeterminism can be proved confluent/deterministic at the semantic result level.
14. Define how partial/nonterminating search artifacts can be archived without being mistaken for failed mathematical claims.
15. Determine whether primitive promotion should require a minimal semantic core representation independent of source programming language.
