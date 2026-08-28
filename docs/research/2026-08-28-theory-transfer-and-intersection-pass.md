# Research Pass — Theory Morphisms, Cross-Domain Transfer, and Theory Intersection

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This pass investigates whether the unnamed mathematical project can recognize that apparently different domains instantiate the same mathematical structure and then reuse whole bodies of mathematics rather than rediscovering individual formulas or proofs.

The result is strong: formal mathematics already has mature semantics for **theory morphisms/interpretations**, and there is prior work on automatically discovering such morphisms and even extracting common theories/intersections.

---

## 1. Theory interpretation transports all derivable mathematics through a certified map

In Isabelle locales, once the assumptions/axioms of one abstract locale/theory are proved under an interpretation in another context, theorems from the source context become available in the target under the same morphism.

Sources:

https://isabelle.in.tum.de/~ballarin/publications/TUM-I0607.pdf

https://isabelle.in.tum.de/dist/library/Doc/Locales/Examples1.html

https://www.cl.cam.ac.uk/research/hvg/Isabelle/dist/library/Doc/Isar_Ref/Spec.html

### Architectural implication

Cross-domain reuse should not be modeled primarily as:

```text
find analogous theorem T
copy T
re-prove T in target
```

A stronger route is:

```text
source theory S
    -> candidate morphism M
    -> prove M preserves required axioms/judgments
    -> transport theorems/constructions from S into target T
```

The expensive reusable artifact is therefore often the **certified theory morphism**, not each transported theorem.

---

## 2. MMT/OMDoc explicitly represents theories and meaning-preserving morphisms as a theory graph

MMT/OMDoc organizes mathematical knowledge at object, statement, and theory/context levels. Theory morphisms/views relate theories through meaning/judgment-preserving translations.

Sources:

https://docs.mathhub.info/legacy/omdoc-mmt.html

https://uniformal.github.io/doc/language/index

https://uniformal.github.io/doc/language/modules

In MMT, theories and morphisms form a graph/category across which declarations and theorems can be induced/translated.

### Architectural implication

The unnamed project may need a **Theory Graph** above the lower-level semantic/equivalence substrate:

```text
Theory S
  |\
  | morphism M1
  v
Theory T ----morphism M2----> Theory U
```

This does not replace the semantic mathematical hypergraph/equality structures.

It captures a different scale:

```text
low level: objects / relations / transformations / proofs
high level: reusable mathematical theories + certified maps between them
```

---

## 3. Automated theory interpretation has already been demonstrated at library scale

Immanuel Normann's work on Automated Theory Interpretation presents algorithms for finding reusable knowledge overlaps and interpretations between formal theories using semantic formula matching, normalization, and associative/commutative standardization.

The implementation was evaluated on the Mizar Mathematical Library and reportedly exposed thousands of theory interpretations.

Source:

https://opus.constructor.university/frontdoor/index/index/docId/336

### Architectural implication

The project should treat **morphism search** as a deterministic mathematical work-cell family.

Possible operation:

```text
find_morphisms(source_theory, target_theory_or_library)
```

Candidate morphisms are suggestions only until their preservation obligations are independently certified.

A successful morphism can unlock many existing theorems/primitives at once.

---

## 4. MMT has an implemented theory-view finder, including cross-library settings

Research on automatically finding theory morphisms in MMT implements a view-finder algorithm that searches for maps from one formal theory into another; demonstrations include finding a matroid interpretation for a differently named source theory and cross-library experiments involving different logical foundations.

Sources:

https://kwarc.info/people/mkohlhase/submit/viewfinder-report.pdf

https://kwarc.info/people/frabe/Research/MKR_viewfinder_18.pdf

https://doi.org/10.1007/978-3-319-96812-4_18

### Architectural implication

Domain labels and symbol names should have very low mathematical authority.

The transfer engine should search structure such as:

- arity/types;
- defining equations;
- algebraic properties;
- dependency relationships;
- theorem signatures;
- canonical forms;
- semantic invariants.

A valid map must ultimately be justified by the mathematics, not by lexical similarity.

---

## 5. Theory intersection can extract the common mathematical core of distinct theories

MMT research defines and implements theory intersections along partial views/morphisms.

The motivating idea is mathematically familiar: apparently different structures can share a common abstract theory, and discovering that commonality can itself produce a useful new theory.

Source:

https://kwarc.info/people/dmueller/pubs/TheoryIntersections.pdf

Related automated theory-interpretation work also develops a practical notion of theory intersection.

Source:

https://opus.constructor.university/frontdoor/index/index/docId/336

### Architectural implication

The project's generalization machinery should not be restricted to generalizing terms/programs.

It may generalize at **theory scale**:

```text
Theory A
Theory B
    -> find partial structural correspondences
    -> extract common subtheory G
    -> prove maps G -> A and G -> B
    -> move reusable mathematics into G
```

This can create a new primitive/theory that is more reusable than either original domain-specific version.

---

## 6. The Little Theories method gives a strong reuse architecture

The Little Theories methodology develops mathematics in small theories at the most convenient abstraction/vocabulary level and transports definitions/theorems through morphisms to other theories as needed.

Recent illustration:

https://arxiv.org/abs/2312.05658

Classic discussion:

https://www.researchgate.net/publication/2614291_Little_Theories

### Architectural implication

The project should avoid one giant global axiom/theorem namespace where every theorem is proved against maximal context.

A more scalable architecture may maintain many compact mathematical theory regions/worlds with explicit maps among them.

Benefits include:

- smaller assumption sets;
- stronger reuse;
- cheaper proof obligations;
- clearer provenance;
- easier invalidation;
- better theory-specific search profiling.

---

## 7. Theory transport can cross formal foundations, but this requires explicit meta-level mappings

MMT is foundation-independent and its morphism research includes cross-library/cross-logic cases using meta-theories/meta-views.

Institution theory provides another abstract framework for logical systems and their satisfaction-preserving morphisms/comorphisms; fragments have been mechanized in Coq.

Source:

https://www.sciencedirect.com/science/article/pii/S0167642323001363

### Architectural implication

The project should not assume every imported formal library shares one logic.

Possible future hierarchy:

```text
object-level mathematical morphism
    within same theory/logic

theory morphism
    between theories

meta-theory / logic translation
    between formal foundations
```

Each level needs its own preservation obligations and certificate/provenance chain.

---

## 8. Formal theorem transfer packages prove the practical value of transport

Isabelle's lifting/transfer and locale mechanisms transport definitions/theorems between related structures/types once the relation/morphism is established.

Representative discussion:

https://pmc.ncbi.nlm.nih.gov/articles/PMC9637085/

### Architectural implication

A certified equivalence/isomorphism/morphism can become a **high-value reusable primitive** because it can transport:

- theorems;
- invariants;
- algorithms;
- canonical forms;
- proof strategies;
- candidate-space restrictions.

Search-economy scoring should therefore account for the potentially huge unlock value of discovering a morphism.

---

## 9. Formal conceptual/theory blending goes beyond intersection and may invent new theories

There is prior work formalizing conceptual/theory blending using logical specifications, anti-unification, category-theoretic constructions, and colimits.

Sources:

https://www.sciencedirect.com/science/article/abs/pii/S1389041711000155

https://www.sciencedirect.com/science/article/pii/S000437021730142X

https://www.research.ed.ac.uk/en/publications/formal-conceptual-blending-in-the-co-invention-of-pure-mathematic/

This work demonstrates computational construction of new concept/theory spaces from multiple source theories, including examples in pure mathematics.

### Architectural implication

There may be at least three distinct cross-domain metaprimitives:

```text
MORPHISM
    transport known mathematics between theories

INTERSECTION / COMMON CORE
    extract what two or more theories share

BLEND / PUSHOUT-LIKE CONSTRUCTION
    combine compatible source structure into a candidate new theory
```

The third is substantially more dangerous: a mathematically well-formed blend is not automatically useful, consistent, conservative, or true under intended interpretations.

Any generated theory must enter the ordinary candidate/falsification/certification pipeline.

---

## 10. Theory intersection is a possible mechanism for true mathematical abstraction growth

Term-level anti-unification finds common structure between expressions.

Theory intersection can do an analogous operation at a much larger scale:

```text
many domain-specific solved structures
    -> discover common axiomatic/relational core
    -> move common theorems/constructions upward
    -> create a more general reusable theory
```

This may be one of the principal ways the system's mathematical vocabulary grows in **generality**, not just in raw primitive count.

---

## 11. Cross-domain reuse should be certificate-bearing

A candidate morphism can create enormous downstream consequences.

Therefore it must never become authoritative based only on heuristic matching.

Possible lifecycle:

```text
candidate structural analogy
    -> proposed symbol/object map
    -> derive preservation obligations
    -> prove/check source axioms under target interpretation
    -> certify morphism
    -> transport source mathematics
    -> optionally recheck selected high-risk transported results
```

If the morphism is later invalidated, every result depending solely on that transport path becomes stale through the existing provenance/dependency machinery.

---

## 12. Search-space implication: morphism discovery can collapse whole domains

Suppose two problem families initially appear independent:

```text
Domain A: PA
Domain B: PB
```

If the project establishes a suitable equivalence/reduction/morphism:

```text
M : A -> B
```

then future work on `PA` may become:

```text
PA
 -> M(PA)
 -> use mature solver/construction family in B
 -> transport result back where inverse/adequate reconstruction exists
```

This is the theory-scale counterpart of the earlier representation-change principle:

> Find the mathematical space in which the problem is already understood.

---

## 13. Current Theory Graph hypothesis

The project may eventually maintain a certified high-level graph such as:

```text
Theory Node
    axioms/assumptions
    signature/objects
    known canonical forms
    Theory Profile
    accepted constructions/theorems

Theory Edge
    morphism / embedding / equivalence / reduction / interpretation
    direction
    preservation properties
    inverse/partial inverse if known
    certificate envelope
    transportable artifact classes
```

Theory intersections/generalizations can introduce new nodes.

Certified discoveries update the graph and may unlock previously impossible transfers.

---

## 14. New research obligations

1. Study algorithms for automatic theory-morphism discovery beyond syntactic/simple-symbol mappings, including complex expression assignments.
2. Determine how theory morphisms relate to the lower-level semantic e-hypergraph/equality substrate without duplicating identity/provenance.
3. Investigate automatic theory intersection/common-core extraction for large modern Lean/Isabelle/MMT libraries.
4. Study theory equivalence/isomorphism and partial inverse certificates so algorithms/results can be transported back safely.
5. Investigate reduction notions weaker than full morphism/equivalence where only particular problem classes or properties transport.
6. Define how Theory Profile data transports or changes under morphisms.
7. Investigate automated detection of high-value candidate morphisms from repeated successful cross-domain reductions.
8. Study proof/certificate reuse under theorem transport to avoid regenerating expensive proofs unnecessarily while preserving trust.
9. Investigate how formal conceptual blending can be constrained by consistency, conservativity, satisfiability, and usefulness checks.
10. Determine whether theory-intersection discovery should be part of primitive promotion: repeated parallel constructions -> common theory -> generalized primitive family.
11. Investigate modern statement/dependency graphs such as TheoremGraph only as retrieval/analogy donors; probabilistic semantic matching must remain candidate-generation evidence, not morphism authority.
12. Measure the search-economy value of a morphism by the amount of certified mathematics/candidate-space reduction it unlocks.
