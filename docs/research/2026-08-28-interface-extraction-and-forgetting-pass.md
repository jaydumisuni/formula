# Research Pass — Semantic Interface Extraction, Forgetting, and Weakest Assumptions

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates whether the unnamed project can automatically derive the **smallest or strongest useful semantic boundary** between mathematical packages, rather than exposing an entire theory whenever two components interact.

The strongest conclusion is:

> **A mathematical interface can itself be derived as a semantics-preserving projection of a larger theory, but exact finite projection is not always available or cheap.**

This suggests interface extraction should become a first-class mathematical operation with its own Theory Profile, certificate route, exact/approximate result classes, and search-economy value.

---

## 1. Uniform interpolation projects a theory onto a chosen vocabulary

Uniform interpolation/forgetting asks:

```text
Given theory T
and vocabulary/signature Σ
compute T|Σ
```

such that `T|Σ` uses only symbols in `Σ` while preserving all consequences expressible in the target logic over `Σ`.

LETHE implements this for several expressive description logics.

Source:

https://link.springer.com/article/10.1007/s13218-020-00655-w

### Architectural implication

If package `A` contains a huge theory but package `B` only interacts through symbols:

```text
Σ_shared
```

then rather than composing `B` against all of `A`, the system may derive:

```text
Interface(A -> B)
    = forget/private-project(A, Σ_shared)
```

and reason against the projected interface.

This can reduce:

- proof size;
- composition search;
- dependency exposure;
- accidental interference;
- rechecking after private implementation/theory changes.

---

## 2. The projected interface can preserve future composition behavior

For appropriate logics, a uniform interpolant over `Σ` can replace the original theory in another context that shares only vocabulary in `Σ` while preserving all relevant consequences over `Σ`.

LETHE documents this reuse property for supported fragments/target logics.

Source:

https://link.springer.com/article/10.1007/s13218-020-00655-w

### Architectural implication

A semantic package interface can be stronger than a hand-written list of exported theorems.

It may represent **all consequences visible through the declared boundary**, including consequences nobody explicitly listed when the package was built.

This makes interface projection valuable for robust future composition.

---

## 3. Forgetting can expose hidden relationships

Eliminating private/intermediate symbols can require deriving new relationships among the remaining public symbols.

Thus:

```text
forget internal symbols
```

is not merely deletion.

It can produce a new compact theory containing **implicit mathematical structure** that was distributed across private definitions.

Source:

https://link.springer.com/article/10.1007/s13218-020-00655-w

### Architectural implication

Interface extraction may itself be a discovery operation:

```text
large internal theory
    -> eliminate private vocabulary
    -> derive strongest visible relations
    -> expose reusable public semantic interface
```

A package may therefore gain a cleaner/more powerful interface automatically than its original human-authored exports.

---

## 4. Exact finite uniform interpolants do not always exist

Uniform interpolation has serious expressivity and size limits.

Recent 2025 work on large ontologies notes that:

- finite uniform interpolants may not exist in the source logic;
- existence can be computationally expensive to determine;
- when finite interpolants exist, minimal representations can be exponentially or even triple-exponentially larger than the source in some logics;
- recursive/cyclic axioms can induce infinite projected consequence families.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-99984-0_34

### Architectural implication

The project must never assume:

```text
project theory onto shared symbols
    -> finite exact interface always exists
```

Instead the Theory Profile may need:

```text
uniform_projection:
    exact_finite: yes/no/unknown
    target_logic: ...
    worst_case_growth: ...
    approximation_available: ...
```

When exact finite projection is unavailable, possible outcomes include:

- infinite symbolic representation;
- richer target logic;
- sound approximation;
- selected theorem module;
- explicit `UNAVAILABLE`.

---

## 5. Module extraction provides a cheaper approximation to full semantic projection

Ontology/module research extracts a fragment `M` from a larger theory `T` that preserves selected entailments over a signature.

Exact minimal modules are often computationally hard, so practical methods compute provably sufficient approximations.

Sources:

https://arxiv.org/abs/1411.5313

https://link.springer.com/article/10.1007/s13740-020-00114-7

### Architectural implication

The project may need several interface strengths:

```text
EXACT UNIFORM INTERFACE
    strongest consequences in target vocabulary

CERTIFIED MODULE
    sufficient subset preserving declared entailment class

APPROXIMATE / SOUND INTERFACE
    declared over/under-approximation

MANUAL EXPORTED INTERFACE
    explicitly declared public claims only
```

These must not be conflated.

---

## 6. Interface minimization is not the same as interface correctness

A module can preserve every required consequence while still containing many unnecessary axioms.

Finding a minimal module can be computationally difficult.

Source:

https://arxiv.org/abs/1411.5313

### Architectural implication

As with MUS/compact-witness research, interface claims should separate:

```text
semantic sufficiency / preservation
```

from:

```text
minimality / compactness
```

A nonminimal certified interface is still mathematically valid and useful.

Interface compression can be an optional optimization governed by expected reuse value.

---

## 7. Craig interpolation can derive a boundary between conflicting theories

Earlier compact-witness research identified Craig interpolation:

```text
A and B incompatible
    -> derive I

A => I
I conflicts with B
I uses only shared vocabulary
```

Recent overview:

https://arxiv.org/abs/2602.08532

### Architectural implication

Uniform interpolation and Craig interpolation address complementary package-interface problems:

```text
UNIFORM INTERPOLATION
    project one theory onto public vocabulary

CRAIG INTERPOLATION
    summarize a proof/conflict at the shared boundary between two sides
```

Both can generate semantic interfaces automatically.

---

## 8. Weakest-assumption synthesis derives the broadest safe environment interface

Assume-guarantee verification research can synthesize a **weakest environment assumption** under which a component satisfies a target property.

Recent timed-automata work formulates the task explicitly as:

```text
Given component M and property φ,
synthesize weakest environment E
such that M satisfies φ in every environment conforming to E.
```

Source:

https://link.springer.com/chapter/10.1007/978-3-032-32526-6_16

Related learning-based assume-guarantee verification:

https://link.springer.com/chapter/10.1007/978-3-031-30820-8_21

### Architectural implication

A package need not always be authored with a perfect `requires:` contract.

The project may attempt:

```text
synthesize_weakest_assumption(P, guarantee G)
```

and obtain the largest admissible environment/interface under which `P` retains `G`.

This can automate part of package composition-contract discovery.

---

## 9. Counterexample-guided learning can synthesize interfaces without enumerating environments

Assume-guarantee learning uses an implicit candidate space of assumptions and refines it with counterexamples until a suitable assumption is found.

Sources:

https://link.springer.com/chapter/10.1007/978-3-031-30820-8_21

https://link.springer.com/article/10.1007/s10009-022-00669-9

### Architectural implication

This directly connects to the project's symbolic-query-learning pass:

```text
candidate interface/assumption
    -> test package guarantee
    -> counterexample environment
    -> refine candidate language
    -> repeat
```

No explicit enumeration of all possible environments is required when the interface class has a compact symbolic representation.

---

## 10. Interface extraction can support privacy and encapsulation as a mathematical property

Forgetting is used not only for reuse but also information hiding/privacy: eliminate vocabulary that should not be exposed while preserving the public consequences that matter.

Source:

https://link.springer.com/article/10.1007/s13218-020-00655-w

### Architectural implication

The mathematical package layer can enforce **semantic encapsulation**:

```text
private theory internals
    -> public projected interface
```

Clients need not gain access to all internal assumptions/representations simply because they consume a primitive.

This may also reduce coupling between future package versions.

---

## 11. Interface extraction can shrink change impact

Suppose package `B` depends only on projected interface `I_A` of package `A`.

If `A` changes internally but the certified projected interface remains semantically equivalent:

```text
I_A(old) == I_A(new)
```

then `B` may require no mathematical revalidation.

### Architectural implication

The dependency graph should distinguish:

```text
implementation dependency
private-theory dependency
public semantic-interface dependency
```

This could dramatically reduce invalidation in a large self-expanding universe.

---

## 12. Interface equivalence/logical difference becomes a versioning primitive

LETHE and related systems compute **logical difference** between two theory versions over a chosen vocabulary.

Source:

https://link.springer.com/article/10.1007/s13218-020-00655-w

### Architectural implication

Instead of textual diff:

```text
A_v1 vs A_v2
```

the package system may ask:

```text
What consequences over public interface Σ changed?
```

If the logical difference is empty over a dependent package's vocabulary, that dependent may remain valid even if the internal source changed substantially.

This is a strong candidate mechanism for **semantic diff**.

---

## 13. Current semantic-interface hypothesis

A package may eventually expose multiple interface artifacts:

```text
DECLARED INTERFACE
    author/project-selected exports

DERIVED UNIFORM INTERFACE
    strongest exact visible consequences where computable

CERTIFIED MODULE INTERFACE
    sufficient preserved subset for selected query class

ASSUME/GUARANTEE CONTRACT
    required environment + provided guarantees

CONFLICT INTERPOLANT
    learned boundary from failed composition

LOGICAL-DIFFERENCE ARTIFACT
    exact/approximate semantic changes across versions
```

Each interface has different authority and use.

---

## 14. Search-economy implication

A work cell that discovers a reusable small interface can unlock much more than one theorem.

Possible value dimensions:

```text
number of dependent packages decoupled
proof obligations eliminated
future invalidation reduced
composition search reduced
private vocabulary hidden
morphism/transport opportunities exposed
```

Thus **interface discovery itself should be a high-value mathematical campaign type**.

---

## 15. New research obligations

1. Study uniform interpolation/forgetting across candidate logics beyond description logics: first-order fragments, SMT theories, equational logic, temporal logic, type theory.
2. Determine which package classes admit finite exact semantic interfaces and what worst-case blowups apply.
3. Investigate proof/certificate formats for uniform interpolants and module-preservation claims.
4. Study automated module extraction over heterogeneous development graphs.
5. Investigate weakest-assumption synthesis for mathematical package contracts rather than transition-system components.
6. Study exact logical-difference computation as a semantic versioning primitive.
7. Determine how to represent infinite projected interfaces compactly using automata, recursive equations, fixed points, or other symbolic structures.
8. Investigate automatic discovery of the smallest shared signature/vocabulary needed for a target theorem/primitive transport.
9. Study interpolation over heterogeneous logics connected by theory morphisms/comorphisms.
10. Determine how an interface artifact binds to package generation, assumptions, and certificate freshness.
11. Investigate whether public-interface equivalence can authorize downstream proof reuse without rechecking private package internals.
12. Study semantic privacy: prove that forgotten/private vocabulary cannot influence observable consequences beyond the declared interface.
13. Investigate interface extraction from e-graph/provenance structures, not only axiom-based theories.
14. Study automatic interface refinement when composition counterexamples reveal missing assumptions.
15. Determine whether interface synthesis can be partially distilled into reusable metaprimitives by theory class.
