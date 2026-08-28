# Research Pass — Substrate Integration, Contexts, Provenance, and Maturity

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This pass tests the earlier four-plane substrate hypothesis against concrete equality-saturation, relational, contextual, and provenance systems.

The result is important: the planes are individually well-supported, but **their combination is not plug-and-play**. There are real semantic tensions that must be handled explicitly.

---

## 1. egglog proves equality + relational deduction can coexist

`egglog` combines:

- equality saturation;
- congruence closure;
- term rewriting;
- Datalog-style relations;
- incremental fixed-point execution;
- cooperating analyses;
- lattice-valued information.

Sources:

https://doi.org/10.1145/3591239

https://github.com/egraphs-good/egglog

As of August 2026 the Rust crate is actively maintained (`egglog` 3.0.0 released 2026-08-19).

### Architectural implication

The project does **not** need to assume equality reasoning and relational facts require completely separate engines.

A substrate can natively express both:

```text
A = B
```

and:

```text
preserves(T, I)
requires(T, A)
refutes(C, E)
path(X, Y)
```

with rules that match modulo equality.

---

## 2. Contextual equality introduces non-monotonic canonicalized views

Work on relational contextual equality saturation identifies a major issue.

If a database stores terms modulo a finer equivalence and later a context adds more equalities, canonicalization can **shrink** the materialized database because previously distinct terms collapse.

Datalog-style relational engines normally depend on monotonic growth for efficient and correct fixed-point evaluation.

Source:

https://inst.eecs.berkeley.edu/~cs294-260/sp24/projects/contextual-eqsat/

### Architectural implication

A naive architecture such as:

```text
one mutable canonical database
+ contextual equalities
+ ordinary monotone Datalog
```

is unsafe as a design assumption.

The project may need to distinguish:

```text
append-only semantic/provenance truth
```

from:

```text
context-specific canonicalized indexes/views
```

so that collapsing representations does not destroy derivation history or violate monotonic assumptions relied on by the deduction engine.

This is a **design hypothesis requiring proof**, not yet a frozen architecture.

---

## 3. Versioned e-graphs are now a mature enough concrete donor

**Versioned E-Graphs (PLDI 2026)** encode a hierarchy of equivalence relations over one shared term space.

The work includes:

- formal correctness results;
- versioned union-find;
- persistent sharing;
- e-class analysis support;
- an optimized standalone Rust implementation called **Veg**;
- theorem-prover/EUF evaluation;
- measured memory reductions and up to 4x runtime improvement in reported cases.

Sources:

https://pldi26.sigplan.org/details/pldi-2026-papers/6/Versioned-E-Graphs

https://programming-group.com/assets/pdf/papers/2026_Versioned-EGraphs.pdf

### Architectural implication

Assumption/world branching does not need to begin from a speculative custom data structure.

A practical early substrate can study or prototype against versioned e-graph machinery while the broader e-hypergraph design remains under research.

---

## 4. Proof extraction over branching e-graphs is progressing

The Smart E-Graphs / Vegie line of work reports proof primitives for equality-saturation-based automated proving over versioned e-graphs, including reasoning involving:

- rewrites;
- congruence;
- injectivity;
- contradictions;
- induction;
- case splits.

Source:

https://prg-grp.github.io/egraphs-extensions-website/

This is relevant because the project wants assumption branches to remain certifiable rather than becoming opaque speculative states.

---

## 5. E-hypergraphs are mathematically strong but less mature operationally

**Equivalence Hypergraphs: DPO Rewriting for Monoidal E-Graphs (LICS 2025)** gives a sound-and-complete combinatorial representation for monoidal e-graph semantics and DPO rewriting.

Source:

https://doi.org/10.1109/LICS65433.2025.00023

The theory is highly relevant for multi-input/multi-output mathematical relations and composition.

However, the current research evidence does **not** show a production ecosystem at the maturity level of `egg`, `egglog`, or the 2026 `Veg` versioned-e-graph implementation.

### Architectural implication

The project should distinguish:

```text
best mathematical target abstraction
```

from:

```text
best First-Light implementation substrate
```

It may be reasonable for an early implementation to use mature term/e-graph/relational pieces while preserving a migration/evolution path toward a richer hypergraph semantic model.

This is explicitly not a decision to abandon e-hypergraphs.

---

## 6. Disequality already has efficient direct e-graph implementations

**Dis/Equality Graphs** adds native disequality support instead of encoding disequality indirectly.

Its artifact contains both an `egg` extension and a Scala implementation, with evaluations in SMT/theorem-proving settings.

Source:

https://zenodo.org/records/13938878

### Architectural implication

The mathematical world should likely treat:

```text
A = B
```

and:

```text
A != B
```

as first-class structural knowledge rather than storing contradiction only as generic facts outside the equality substrate.

How that integrates with versioned worlds and e-hypergraphs remains open.

---

## 7. Provenance for recursive relational reasoning is itself subtle

Semiring provenance is a powerful abstraction for representing alternate derivation paths and attaching meta-information to relational facts.

However, provenance semantics for recursive Datalog is not trivial: different proposed semantics disagree in the presence of recursive/infinite derivations, and the research literature explicitly studies these distinctions.

Source:

https://doi.org/10.24963/kr.2022/10

### Architectural implication

The project must not simply annotate every recursive fact with a naive algebraic provenance expression and assume it is canonical/finite.

The provenance layer may require:

- declared semantics by relation family;
- cycle/recursion handling;
- compact DAG/fixpoint representations;
- support for alternative sufficient derivations;
- explicit infinite/unknown provenance states.

This remains a major research obligation.

---

## 8. Equality explanations already provide a certifiable provenance-like boundary

Modern e-graph systems can generate equality explanations showing which rewrites/congruence steps establish equivalence.

A July 2026 Isabelle AFP development verifies executable checkers for egg merge and extraction certificates.

Source:

https://isa-afp.org/entries/Equality_Saturation_Checker.html

### Architectural implication

For equality-derived facts, the project may be able to rely on proof/explanation DAGs rather than trying to force all support through generic Datalog provenance.

Different fact families may need different native derivation objects beneath the common certificate envelope.

---

## 9. Current substrate synthesis

The earlier four-plane hypothesis survives, but with a stronger separation of responsibilities:

```text
SEMANTIC OBJECT / RELATION SPACE
        |
        +-- persistent identities
        +-- typed composition
        |
        v
CONTEXTUAL EQUALITY / WORLD INDEXES
        |
        +-- equivalence
        +-- disequality
        +-- assumption branches
        +-- canonicalized views
        |
        v
RELATIONAL DEDUCTION
        |
        +-- monotone facts where valid
        +-- lattice analyses
        +-- fixed-point rules
        |
        v
DERIVATION / CERTIFICATE ARTIFACTS
        |
        +-- equality explanations
        +-- relational provenance
        +-- domain certificates
        +-- counterexamples
```

A likely law is:

> **Canonicalization may change views/indexes, but it must not erase semantic identity or admitted provenance.**

This resembles an append-only truth layer plus rebuildable derived indexes rather than one mutable database being both truth and optimization structure.

This is a design hypothesis to test, not a freeze.

---

## 10. New research obligations

1. Determine a sound provenance model for recursive mathematical relations and cyclic derivations.
2. Determine whether versioned equality contexts can expose monotone relational interfaces without leaking context-specific canonicalization shrinkage.
3. Investigate whether semantic e-graph values can bridge multiple mathematical representations without collapsing distinct proof scopes.
4. Investigate a migration path from mature e-graph/egglog/Veg machinery to monoidal e-hypergraph semantics.
5. Determine whether equality explanations, domain certificates, and relational provenance should remain separate derivation families under one certificate envelope.
6. Prototype conceptually how semantic object identities survive canonicalization, quotienting, and representation replacement.

---

## 11. Current maturity judgement

```text
egg / egglog           -> mature donor / practical prototype substrate
Versioned E-Graphs/Veg -> strong 2026 donor / practical research substrate
Dis/Equality Graphs    -> concrete research implementation
Semantic E-Graphs      -> important 2026 semantic donor
E-Hypergraphs          -> strong formal target, operational maturity still to establish
Generic provenance     -> mathematically useful but integration semantics unresolved
```

This maturity gradient should influence First-Light engineering later without constraining the final mathematical architecture.