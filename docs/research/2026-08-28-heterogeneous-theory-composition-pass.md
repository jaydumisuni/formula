# Research Pass — Heterogeneous Theory Composition, Conservativity, and Proof-Carrying Mathematical Packages

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how a self-expanding mathematical system can combine independently certified theories, primitive families, solvers, rewrite systems, and logical fragments without silently destroying properties such as termination, confluence, consistency, decidability, or previously established theorems.

The strongest conclusion is:

> **Certification is not automatically compositional. A combined mathematical package is a new mathematical object with new proof obligations.**

This is not a rare edge case. Existing theory-combination, rewriting, specification, and heterogeneous-logic research repeatedly demonstrates that useful properties have different modularity behavior.

---

## 1. Individually decidable theories can have an undecidable combination

Modern SMT theory-combination research studies exactly this problem: two theories can each have decision procedures, while the satisfiability problem for their union requires additional model-theoretic conditions.

Recent work analyses properties including:

- stable infiniteness;
- smoothness;
- finite witnessability;
- strong finite witnessability;
- convexity.

Sources:

https://link.springer.com/article/10.1007/s10817-025-09746-5

https://link.springer.com/chapter/10.1007/978-3-031-99984-0_2

The 2025 CADE work gives explicit negative results: removing assumptions from established theory-combination theorems can yield pairs of individually decidable theories whose combined theory is undecidable.

### Architectural implication

The project must never infer:

```text
Package A:
    decidable = PROVEN

Package B:
    decidable = PROVEN

therefore

A + B:
    decidable = PROVEN
```

Instead:

```text
combine(A, B)
    -> inspect signatures/shared sorts
    -> inspect combination properties
    -> select applicable combination theorem/procedure
    -> create composition proof obligations
    -> certify or declare UNKNOWN/UNSAFE
```

Decidability is a property of the composition as well as of its parts.

---

## 2. Theory-combination conditions belong in the Theory Profile

Nelson-Oppen-style and polite/shiny/gentle combination methods rely on model-theoretic properties of the component theories.

Recent work shows the interaction among properties such as stable infiniteness, smoothness, finite witnessability, strong finite witnessability, and convexity is itself nontrivial.

Source:

https://link.springer.com/article/10.1007/s10817-025-09746-5

### Architectural implication

Theory Profile should eventually include **combination capabilities**, not merely intrinsic properties.

Possible research-level structure:

```text
combination_profile:
    signature_sharing:
        disjoint_functions_predicates: ...
        shared_sorts: ...

    model_properties:
        stable_infinite: ...
        convex: ...
        strongly_finitely_witnessable: ...
        smooth: ...

    supported_combination_methods:
        - method: nelson_oppen
          obligations: [...]
        - method: polite
          obligations: [...]

    known_incompatible_classes:
        ...
```

This is not a frozen schema.

---

## 3. Rewrite-system properties have sharply different modularity behavior

Term rewriting provides an especially clear warning.

**Confluence** is modular for disjoint term rewrite systems: the disjoint union of confluent TRSs remains confluent.

Sources:

https://link.springer.com/chapter/10.1007/978-3-030-02508-3_10

https://ir.cwi.nl/pub/2653/2653D.pdf

By contrast, **termination is not modular in general**, even for disjoint systems. There are two individually terminating rewrite systems whose direct sum admits an infinite rewrite sequence.

Sources:

https://www.sciencedirect.com/science/article/pii/030439759400039L

https://www.sciencedirect.com/science/article/abs/pii/0020019094001874

A large research literature gives sufficient modularity criteria for restricted classes such as left-linear/r-consistent systems and constructor-sharing systems.

### Architectural implication

Properties cannot be assigned one global flag such as:

```text
property.modular = true
```

They need a declared **combination theorem scope**:

```text
property: confluence
preserved_under:
    - disjoint_TRS_union

property: termination
preserved_under:
    - only_if: modularity_criterion_X
    - only_if: modularity_criterion_Y
```

A primitive package should carry enough structure for the composition analyzer to determine which theorem is applicable.

---

## 4. Hets provides a major donor architecture for heterogeneous mathematics

The **Heterogeneous Tool Set (Hets)** was built to manage specifications and proofs using many different logics and languages rather than forcing everything into one universal logic.

Hets is based on:

- a graph of logics/languages;
- formalizations of logics as **institutions**;
- logic translations/morphisms/comorphisms;
- heterogeneous specifications;
- development graphs;
- logic-specific theorem provers and tools.

Sources:

https://www.dfki.de/en/web/research/projects-and-publications/publication/3902

https://github.com/spechub/Hets

https://www.dfki.de/en/web/research/projects-and-publications/publication/4480

### Institution lesson

An institution abstracts a logic through:

```text
signatures
sentences over signatures
models over signatures
satisfaction relation
```

with a satisfaction condition ensuring truth is preserved under appropriate changes of notation/signature.

Source:

https://www.researchgate.net/publication/221138151_Towards_Logical_Frameworks_in_the_Heterogeneous_Tool_Set_Hets

### Architectural implication

The unnamed project should seriously consider **heterogeneous mathematical semantics** rather than inventing one mega-logic expected to encode every mathematical domain optimally.

Potential principle:

```text
one semantic package protocol
!=
one universal internal logic
```

Different mathematical fragments may retain their strongest native semantics and proof technology while participating in one common theory/package graph.

---

## 5. Development graphs provide a strong model for the permanent mathematical ledger

Hets uses **development graphs** as a logic-independent kernel formalism for structured specifications and proof management.

Nodes represent theories/specification fragments. Links include structural dependencies and theorem/proof obligations.

Sources:

https://www.sciencedirect.com/science/article/pii/S1567832605000810

https://www.dfki.de/web/forschung/projekte-publikationen/publikation/3904

Development graphs are explicitly designed for:

- large modular theories;
- proof reuse;
- proof-state tracking;
- hiding/imports;
- change management;
- decomposition of global proof obligations into local ones.

### Architectural implication

The project's permanent mathematical universe may need a structure closer to a **heterogeneous development/theory graph** than one monolithic theorem database.

Possible node families:

```text
Theory / Fragment
Primitive Package
Representation Theory
Decision Procedure Contract
Rewrite Theory
Certificate Theory
Domain Adapter Theory
```

Possible link families:

```text
imports / definition dependency
conservative extension
interpretation / theory morphism
logic translation
refinement
proof obligation
theorem transport
combination result
```

This complements rather than replaces the earlier semantic e-graph/hypergraph search substrate.

The development graph would govern durable theory/package relationships; e-graphs/automata/constraint stores remain active search representations.

---

## 6. Conservative extension is a crucial safe-growth relation

A model-theoretically conservative extension ensures that every model of the original theory can be expanded to a model of the extended theory. This implies consequence conservativity over the old language: the extension does not create new theorems purely in the old vocabulary.

Sources:

https://theo.cs.ovgu.de/lehre/lehre13w/modularity/lcc.pdf

https://openreview.net/forum?id=uT798pTkvc

Hets treats conservativity annotations as proof obligations in development graphs.

Source:

https://www.researchgate.net/publication/221350774_Change_Management_for_Heterogeneous_Development_Graphs

### Architectural implication

When adding a new mathematical primitive/theory package, a particularly strong promotion result is:

```text
T -> T'
status: CONSERVATIVE EXTENSION
```

This can justify retaining old theorems and interpretations without re-proving every old statement individually.

If the extension is **not** conservative, that is not automatically forbidden. It means the changed old-vocabulary consequences must be exposed and dependent knowledge may need invalidation/review.

---

## 7. Conservativity enables proof reuse and change localization

Development-graph proof calculi exploit conservativity to shift/borrow proof obligations and preserve proof work across structured theories.

Source:

https://www.researchgate.net/publication/221350774_Change_Management_for_Heterogeneous_Development_Graphs

Change-management work uses graph dependencies to confine rechecking to affected regions rather than recomputing every proof in a development.

Sources:

https://www.researchgate.net/publication/221350774_Change_Management_for_Heterogeneous_Development_Graphs

https://user.informatik.uni-bremen.de/autexier/pub/LPAR2002.pdf

### Architectural implication

This is a stronger model for the project's dependency invalidation problem:

```text
change package P
    -> determine changed semantic links
    -> retain proofs protected by conservative/unaffected paths
    -> invalidate only theorem/primitive descendants whose support changed
    -> generate local open proof obligations
```

The mathematical universe should therefore track proof support at package/link level as well as individual claim level.

---

## 8. Theory morphisms can transport whole bodies of mathematics

Theory morphisms/interpretations are truth-preserving maps between theories. Hets/MMT-style theory graphs use these mappings to move results between mathematical structures.

MMT represents theories and truth-preserving theory morphisms as first-class structures and builds large theory graphs from them.

Sources:

https://kwarc.info/people/dmueller/pubs/thesis.pdf

https://kwarc.info/projects/latin/

https://kwarc.info/people/frabe/Research/MR_implicit_19.pdf

### Architectural implication

A successful structural map can have much higher value than proving one theorem:

```text
certify morphism M : Theory A -> Theory B
    -> transport applicable theorems/constructions of A into B
```

This strengthens the earlier theory-transfer research: search-economy scoring should include **unlock value of candidate morphisms**.

---

## 9. Colimits provide a formal theory-combination operation—but not a free correctness guarantee

Hets computes colimits of theories in heterogeneous development graphs, and institution research generalizes colimits to multi-logic settings.

Sources:

https://github.com/spechub/Hets

https://www.dfki.de/web/forschung/projekte-publikationen/publikation/4468

https://www.dfki.de/en/web/research/projects-and-publications/publication/3897

Related Hets work uses category-theoretic diagrams, pushouts, and colimits to combine aligned theories/ontologies.

Source:

https://www.dfki.de/en/web/research/projects-and-publications/publication/3849

### Architectural implication

A theory merge can be represented explicitly:

```text
A <- Shared -> B
       |
       v
   combined colimit C
```

But constructing `C` does not by itself establish every desired operational property of `C`.

The composition layer must separately analyze/certify:

- consistency/satisfiability;
- conservativity where claimed;
- decidability where claimed;
- termination/confluence/coherence where relevant;
- transported theorem validity;
- runtime realization compatibility.

---

## 10. Proof-carrying code suggests a package-admission pattern

Proof-Carrying Code (PCC) separates an untrusted producer from a small trusted checker: optimized native code arrives with proof evidence that it satisfies the consumer's policy.

Sources:

https://www.cs.cornell.edu/courses/cs513/2007fa/L14.html

https://csd.cs.cmu.edu/academics/doctoral/degrees-conferred/george-ciprian-necula

https://www.cs.cmu.edu/~fox/pcc.html

### Architectural implication

A mathematical primitive package can be treated similarly:

```text
UNTRUSTED / EXTERNAL PACKAGE
    semantic declarations
    implementations
    claimed properties
    claimed combination properties
    certificates
        |
        v
PACKAGE ADMISSION CHECKER
    validate semantic identity
    validate certificate envelopes
    validate declared dependencies
    validate combination contract
    create open obligations for unproved claims
        |
        v
ADMITTED PACKAGE GENERATION
```

The package producer can be human-written, generated by the unnamed project, generated by a model, or imported from an external mathematics system.

The admission semantics should be identical.

---

## 11. Mathematical package correctness has several independent layers

This research suggests that package admission should distinguish at least:

```text
INTRINSIC SEMANTIC CORRECTNESS
    claims inside the package are valid under its assumptions

EXTENSION SAFETY
    how the package changes an existing theory

COMBINATION SAFETY
    which properties survive composition with other packages

TRANSPORT SAFETY
    whether theory morphisms/translations preserve declared claims

REALIZATION SAFETY
    native implementation realizes the certified mathematics
```

A package may pass one layer and fail/leave another unknown.

Example:

```text
P:
    intrinsic mathematics = CERTIFIED
    CPU realization = CERTIFIED
    conservative extension of Core = CERTIFIED
    termination when combined with Q = UNKNOWN
```

This is more honest and more useful than one global `certified=true` flag.

---

## 12. A composition contract may become a first-class package artifact

Current research supports a possible **Composition Contract** attached to a theory/primitive package.

Research-level sketch:

```text
package P

semantic_signature:
    ...

imports:
    ...

assumptions:
    ...

guarantees:
    ...

extension_properties:
    conservativity: certificate/unknown

combination_properties:
    termination:
        preserved_if: [...]
    confluence:
        preserved_if: [...]
    decidability:
        methods_and_conditions: [...]
    propagation_completeness:
        ...

logic/foundation:
    institution_or_native_semantics: ...

morphisms:
    exported/imported theory maps

certificates:
    universal envelopes to native proof bodies

realizations:
    CPU/GPU/etc
```

This is a research hypothesis, not a frozen schema.

---

## 13. Heterogeneous semantics may be preferable to a universal logic

Hets, DOL, institutions, MMT, and LATIN all provide evidence for a key architectural principle:

> **Interoperability can be built by explicitly representing logics, theories, and truth-preserving translations instead of forcing all formal knowledge into one foundation.**

Sources:

https://www.dfki.de/en/web/research/projects-and-publications/publication/3902

https://wiki.dol-omg.org/index.php/DOL

https://www.omg.org/spec/DOL/1.0/

https://kwarc.info/projects/latin/

### Architectural implication

The unnamed project may eventually own a **mathematical semantic/package metaprotocol** rather than one universal foundational logic.

A domain may use:

- first-order logic;
- higher-order dependent type theory;
- equational/rewrite logic;
- constraint semantics;
- algebraic geometry certificates;
- interval/numerical semantics;
- automata semantics;

provided the package declares how claims, models/meaning, translations, and certificates connect to the project's common trust envelope.

This keeps the project broad without weakening native mathematics.

---

## 14. Theory graphs and active search structures should remain separate

Current evidence now suggests two graph-like structures with different jobs:

```text
DURABLE HETEROGENEOUS THEORY / DEVELOPMENT GRAPH
    packages
    theories
    morphisms
    conservative extensions
    imports
    proof obligations
    certification lineage
    change impact

ACTIVE SEARCH STRUCTURES
    e-graphs / versioned e-graphs
    automata / VSA / ECTA
    propagation stores
    abstract domains
    AND/OR proof graphs
    candidate worlds
```

### Architectural implication

The durable graph answers:

> What mathematics do we currently accept, under what theories and translations?

Active search structures answer:

> How do we efficiently explore this particular mathematical problem right now?

Collapsing those into one mutable graph would mix permanent truth with disposable search representation.

---

## 15. Current safe-composition hypothesis

The strongest current growth path is:

```text
Certified Universe Generation G
    |
    +-> candidate package/theory P
            |
            +-> certify intrinsic claims
            +-> certify/declare logic semantics
            +-> check imports and theory morphisms
            +-> analyze conservativity
            +-> analyze combination properties
            +-> create new composition obligations
            +-> certify executable realizations
            |
            v
        isolated candidate development graph G'
            |
            +-> all required promotion obligations closed
            v
        promote G' as next accepted generation
```

A new primitive is therefore not merely appended to a global function table.

It enters through a **proof-managed theory/package graph**.

---

## 16. New research obligations

1. Study the institution abstraction in detail and determine the minimum semantic interface a mathematical package would need without adopting Hets wholesale.
2. Investigate development graphs as a donor for durable mathematical proof/change management.
3. Define which extension relations matter beyond conservativity: definitional, weakly definitional, monomorphic, refinement, interpretation, equivalence.
4. Study automated conservativity checking across major candidate mathematical logics.
5. Build a catalogue of combination theorems/conditions for rewrite systems, SMT theories, abstract domains, constraint propagators, and theorem libraries.
6. Investigate automatic derivation of **composition certificates** rather than only component certificates.
7. Study heterogeneous colimits/pushouts and how their construction should generate proof obligations rather than automatically authorize a merge.
8. Investigate package-level assume/guarantee reasoning so large compositions can be certified locally.
9. Study MMT theory graphs and implicit morphisms as donors for automatic theorem/primitive transport.
10. Investigate semantic versioning of mathematical packages: what changes are conservative, breaking, strengthening, weakening, or realization-only?
11. Define change-impact analysis over the mathematical dependency/theory graph so only affected proofs/primitives are rechecked.
12. Investigate how a package can expose multiple native logics/certificate families through one project-level semantic envelope.
13. Study conflicts between two individually conservative extensions when both are applied to the same base theory.
14. Determine how the search economy should value candidate morphisms, conservative abstractions, and reusable combination theorems.
15. Investigate whether a small package-admission kernel inspired by PCC/Hets can remain independent of all large theorem provers and solvers.
