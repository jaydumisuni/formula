# Research Pass — Computability Boundaries, Semi-Decisions, and Honest Unknowns

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates the fundamental limits of a general mathematical problem solver. The purpose is not to reduce ambition, but to prevent the architecture from confusing computational hardness with mathematical undecidability.

The central result is:

> **Some mathematical/problem classes cannot have a total general decision algorithm. The system must represent that fact explicitly and route search accordingly.**

---

## 1. Undecidable does not mean insufficient compute

Classical computability theory proves that no algorithm decides the halting problem in general. Rice's theorem strengthens this for programs: every nontrivial semantic/extensional property of arbitrary computable programs is undecidable.

Sources:

https://plato.stanford.edu/entries/recursive-functions/

https://link.springer.com/chapter/10.1007/978-3-319-96142-2_8

### Architectural implication

The project must not interpret:

```text
search did not terminate
```

as merely:

```text
need more CPU/RAM
```

A Theory Profile should classify known computability boundaries such as:

```text
decidable
semi-decidable / recursively enumerable
co-semi-decidable
undecidable
open/unknown classification
```

where mathematically justified.

---

## 2. Program equivalence/property checking is undecidable in general

Rice's theorem implies that nontrivial semantic properties of arbitrary programs cannot be decided universally, including many equivalence/correctness-style questions.

Source:

https://plato.stanford.edu/entries/recursive-functions/

### Architectural implication

The project's translation/equivalence validation layer must always be **fragment/profile scoped**.

It can provide total equivalence checking for restricted languages/IRs (e.g. finite-state, bitvector fragments, specific LLVM subsets, algebraic domains) without claiming a universal program-equivalence oracle.

A realization certificate must include the semantic fragment under which equivalence is established.

---

## 3. Hilbert's Tenth Problem separates linear integer arithmetic from nonlinear integer polynomial equations

Presburger arithmetic (integer arithmetic with addition/order but without multiplication of variables) is decidable.

By contrast, the Davis-Putnam-Robinson-Matiyasevich theorem shows there is no general algorithm deciding whether an arbitrary multivariate integer polynomial equation has an integer/natural-number solution.

Representative sources:

https://www.cs.ox.ac.uk/people/christoph.haase/home/publication/haa-18/haa-18.pdf

https://arxiv.org/abs/1812.00990

### Architectural implication

A small change in permitted mathematical vocabulary can cross a fundamental computability boundary:

```text
linear integer arithmetic
    -> decidable

arbitrary Diophantine polynomial equations
    -> undecidable in general
```

Theory Profile must therefore be based on actual mathematical structure/operators, not broad labels such as `integer mathematics`.

---

## 4. Proof search can be semi-decisive even when truth decision is impossible

For recursively axiomatized complete fragments, finite proofs can be enumerated. More generally, theorem-proving systems often provide a semi-decision procedure: if a statement is provable in the chosen formal system, exhaustive fair proof search will eventually find a proof, while failure to find one may never terminate.

General proof-enumeration discussion:

https://plato.stanford.edu/archives/sum2026/entries/logic-propositional/

### Architectural implication

A work-cell contract should distinguish:

```text
total decision procedure
```

from:

```text
semi-decision search
```

and include directionality such as:

```text
halts_if_provable
halts_if_refutable
may_diverge_on_other_cases
```

The scheduler can then decide whether to run proof and refutation searches in parallel, bound them, or switch to sound approximations.

---

## 5. Gödel incompleteness means formal proof authority is always theory-relative

Sufficiently expressive consistent recursively axiomatized formal systems cannot prove every true arithmetic statement expressible in the system.

The project therefore cannot equate:

```text
not provable in current theory
```

with:

```text
false
```

nor can it equate:

```text
formally proven
```

with an assumption-free absolute result outside the axiomatic/contextual scope recorded by the proof.

### Architectural implication

Every proof certificate already needs:

```text
theory / axiom-world identity
```

The incompleteness boundary makes that field constitutional rather than bookkeeping.

---

## 6. Decidability borders can be extremely sharp and domain-specific

Research on string/concatenation theories gives examples where a broad first-order theory is undecidable while restricted existential equation fragments are decidable.

Source:

https://link.springer.com/article/10.1007/s00153-020-00735-6

### Architectural implication

The project should maintain a **fragment lattice**, not one property per backend:

```text
Theory T
    full fragment: undecidable
    existential fragment: decidable
    bounded fragment: decidable
    special structural subfragment: polynomial/FPT/etc.
```

Query normalization should attempt to recognize the smallest strong decidable fragment containing the obligation before falling back to general search.

---

## 7. Reductions can transfer impossibility/hardness classifications

Undecidability proofs and complexity theory routinely use reductions: if problem A reduces to problem B and A is undecidable/hard, a hypothetical too-strong solver for B would solve A.

### Architectural implication

The project's Theory Graph should carry not only theorem-transport morphisms but also **problem reductions with hardness/computability consequences**.

A certified reduction may establish:

```text
Problem family P is at least as hard as Q
```

or:

```text
a total solver for P would decide known-undecidable family Q
```

This can stop futile universal-solver campaigns and redirect work toward:

- restricted fragments;
- bounded instances;
- semi-decision search;
- approximation;
- additional assumptions;
- human/domain reformulation.

---

## 8. `UNKNOWN` needs multiple meanings

The result taxonomy should distinguish reasons that currently collapse into one human word:

```text
UNKNOWN_RESOURCE_BOUND
    search stopped because declared resources expired

UNKNOWN_INCOMPLETE_METHOD
    method is sound but incomplete

UNKNOWN_SEMI_DECISION_OPEN
    search may legitimately diverge on this polarity

UNKNOWN_UNDECIDABLE_GENERAL_CLASS
    no total general decision procedure exists for the class

UNKNOWN_FORMAL_INDEPENDENCE_SUSPECTED/ESTABLISHED
    statement not decidable from current axioms under established conditions

UNKNOWN_OPEN_MATHEMATICS
    no known proof/refutation yet
```

Names are provisional; the distinction is the important part.

---

## 9. Computability profiling can save enormous search resources

Before a large mathematical campaign begins, Theory Profile should ask:

```text
Is the query inside a known decidable fragment?
Is only one polarity semi-decidable?
Is the unrestricted class undecidable?
Can additional structural assumptions recover decidability?
Can the obligation reduce to a stronger tractable theory?
```

This can prevent billions of work-cell operations being spent searching for a nonexistent universal decision procedure.

---

## 10. Undecidable general class does not make concrete instances useless

An undecidable problem family may still contain many instances that are easy, certifiable, or solvable by incomplete procedures.

For example:

```text
general program semantic property: undecidable
specific finite-state abstraction: decidable
specific bounded execution: decidable
specific proof supplied: cheaply checkable
```

### Architectural implication

Computability status belongs to the **problem family/fragment**, while individual obligations can still be attempted by:

- proof search;
- counterexample search;
- bounded search;
- abstraction/refinement;
- certificate checking;
- structural reductions.

The system should not reject a concrete problem solely because its unrestricted parent class is undecidable.

---

## 11. Current computability-aware routing hypothesis

```text
PROBLEM OBLIGATION
    -> identify theory/fragment
    -> consult computability profile

if decidable:
    use strongest complete route

if semi-decidable:
    launch fair directional search with explicit divergence semantics

if undecidable general class:
    search for restricted fragment / reduction / bounded proof / certificate

if status unknown:
    treat classification itself as research obligation
```

This should sit before generic large-scale search.

---

## 12. New research obligations

1. Build a machine-readable catalogue of decidability/semi-decidability boundaries for candidate foundational mathematical theories.
2. Study formal certificates for fragment membership so a query is not routed to a decision procedure based only on parser heuristics.
3. Investigate reductions/hardness certificates as first-class Theory Graph edges.
4. Study proof-search dovetailing/fair-search mechanisms for semi-decision procedures.
5. Determine how undecidability classifications interact with model-generated candidate heuristics without allowing the model to overclaim solvability.
6. Investigate independence/relative-consistency proof certificates and how they should be represented distinctly from ordinary `UNKNOWN`.
7. Study bounded/model-checking reductions that turn undecidable infinite obligations into decidable finite approximations with explicit scope.
8. Determine how search economy should price semi-decision cells whose expected runtime has heavy/infinite tails.
9. Investigate automatic discovery of tractable/decidable subfragments inside broader undecidable theories.
10. Define operator/vocabulary changes that trigger Theory Profile recomputation because they may cross a computability boundary.
