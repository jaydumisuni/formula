# Research Pass — Mathematical Identity, Content Addressing, Canonical Forms, and Certified Equivalence

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project should identify immutable mathematical objects while acknowledging that semantic equivalence is hard or undecidable in broad classes.

The central finding is:

> **Do not define content identity as “same mathematical meaning.” Define a deterministic structural/content identity that is always computable, then represent stronger equality/equivalence as separately certified, theory- and scope-specific relations. Canonical forms may collapse identities only in fragments whose canonicalization properties are established.**

---

## 1. Content addressing is proven practical for immutable executable definitions

Unison identifies each term/type by a SHA3 hash of its internal structure, excluding human names. Bound variables are normalized and dependencies are referenced by their hashes. Mutually recursive cycles receive a deterministic canonical hashing treatment.

Sources:

https://www.unison-lang.org/docs/language-reference/hashes/

https://www.unison-lang.org/docs/the-big-idea/

### Architectural implication

The project can define a **Structural Identity**:

```text
canonical structural serialization
+ exact semantic dependencies by digest
+ declared semantics/version/domain metadata where identity-relevant
    -> cryptographic digest
```

Names, comments, source-file location, and human notation remain metadata unless explicitly semantic.

---

## 2. Alpha-equivalent binder syntax can be normalized before hashing

Unison uses positional/binder normalization; research also provides efficient hashing modulo context-sensitive alpha-equivalence for open terms.

Source:

https://dl.acm.org/doi/10.1145/3656459

### Architectural implication

Safe low-level canonicalization should remove representation-only differences such as:

```text
bound-variable names
irrelevant source ordering where semantics proves order-insensitive
formatting
human aliases
```

before structural hashing.

This is not the same as algebraic/semantic simplification.

---

## 3. General semantic program equivalence is undecidable

Program-equivalence results and Rice-style undecidability prevent a total algorithm from deciding arbitrary nontrivial semantic equivalence for general programs.

Representative source:

https://www.csc.kth.se/utbildning/kth/kurser/DD2352/algokomp16/F11_16.pdf

### Architectural implication

The project must not require:

```text
semantic canonical form for all executable mathematics
```

as a prerequisite for storage, hashing, or retrieval.

If identity depended on solving semantic equivalence, content-addressing itself could fail to terminate.

---

## 4. Structural identity and semantic equivalence are different relations

Two structurally different constructions may later be proved equivalent:

```text
hash(A) != hash(B)
BUT
A ≡ B under theory T and assumptions Γ
```

### Architectural implication

The ledger should preserve both immutable nodes and certified edges:

```text
StructuralObject A
StructuralObject B

EquivalenceCertificate E:
    A == B
    under theory/profile/scope
```

No node has to be rewritten/deleted merely because a stronger relationship was later discovered.

---

## 5. Canonical forms are powerful where a theory supports them

Earlier research established that confluent + terminating rewrite systems, Gröbner bases, graph canonization, and other completion procedures can provide canonical representatives for specific equivalence theories/classes.

### Architectural implication

A Theory Profile may declare a **Canonicalization Domain**:

```text
canonicalizer C
scope/class F
equivalence relation E
termination/confluence/completeness certificates
```

Then:

```text
C(A) = C(B)
```

can be a cheap exact equivalence decision for objects inside `F`.

Outside that declared fragment, no such conclusion is permitted.

---

## 6. Equality saturation gives generation-scoped equivalence classes, not timeless content identity

E-graphs merge terms as new equalities are established. Current Isabelle work can independently check egg merge/extraction certificates.

Source:

https://isa-afp.org/entries/Equality_Saturation_Checker.html

### Architectural implication

An e-class identifier belongs to a particular accepted/candidate equality context:

```text
Generation G
World W
Theory T
```

It should not replace the permanent structural digest of each member.

When a new theorem merges two classes, permanent object hashes stay unchanged; the derived equivalence view changes.

---

## 7. Semantic equivalence is always scoped

Possible equivalence notions include:

```text
exact function equality
relation equality
refinement
observational equivalence
bisimulation
same steady-state boundary behavior
same count
same optimum
same distribution
same result under assumptions Γ
approximate equivalence within bound ε
```

### Architectural implication

There should be no generic unqualified edge:

```text
A SAME_AS B
```

Instead:

```text
EquivalenceRelationType
scope/observer
direction if refinement
assumptions
certificate
```

This prevents a behavioral black-box equivalence from being mistaken for full internal equivalence.

---

## 8. Equivalence can be directional

Compiler refinement such as Alive2's source/target semantics is directional: a target may refine the source without being fully equivalent under undefined/nondeterministic behavior.

Source:

https://github.com/AliveToolkit/alive2

### Architectural implication

The identity graph needs first-class relations such as:

```text
equivalent
refines
implies
subsumes
specializes
implements
observationally_equivalent_under O
```

not merely equality.

These relations can participate differently in search/replacement/optimization.

---

## 9. Dependency hashing gives immutable semantic closure identity

Unison replaces dependencies inside a term with their content hashes so a term's hash pins its exact dependency meanings rather than names.

Source:

https://www.unison-lang.org/docs/the-big-idea/

### Architectural implication

A mathematical structural digest should normally be Merkle-like:

```text
object encoding
references -> dependency structural digests
```

Thus changing a dependency creates a new dependent structural identity automatically.

This reproduces Tenfold-style exact-input freshness at arbitrary graph scale.

---

## 10. Cyclic mathematics requires component-level hashing

Recursive definitions/theories can contain dependency cycles. Unison handles mutually recursive definitions by hashing the cycle as a canonical component and assigning indexed identities inside it.

Sources:

https://www.unison-lang.org/docs/language-reference/hashes/

https://www.unison-lang.org/docs/usage-topics/general-faqs/

### Architectural implication

The project should content-address strongly connected semantic components rather than assume the mathematical dependency graph is acyclic.

This aligns with earlier fixed-point/cyclic-goal research.

---

## 11. Identity should form a hierarchy

Current research suggests at least:

```text
SOURCE IDENTITY
    exact external artifact/version

STRUCTURAL IDENTITY
    canonical internal syntax/graph + dependency digests

THEORY-CANONICAL IDENTITY
    canonical representative under certified theory fragment

CERTIFIED EQUIVALENCE CLASS
    generation/world-scoped equalities

BEHAVIORAL IDENTITY
    equivalence under declared observer/interface

SPECIALIZATION IDENTITY
    semantic object + fixed assumptions/query direction

REALIZATION IDENTITY
    exact executable implementation
```

### Architectural implication

Different subsystems use the cheapest identity strong enough for the task.

Caching native execution may use realization/specialization identity.

Theorem deduplication may use theory-canonical/equivalence identity.

Permanent evidence always retains structural/source identity.

---

## 12. Certified equivalence should enable deduplication without erasing lineage

If A and B are proved equivalent, the system may choose one preferred representative for search/execution under a given cost model.

### Architectural implication

Do:

```text
A -> equivalence class E <- B
preferred_rep(E) = A
```

not:

```text
delete B
```

because B may have independent provenance, stronger representation properties, cheaper derivative computation, or a different realization path useful later.

---

## 13. Canonical representative selection is separate from equivalence truth

An e-class may contain many equal terms. The 2026 Isabelle equality-saturation checker separately proves equality membership and minimal additive-cost extraction.

Source:

https://isa-afp.org/entries/Equality_Saturation_Checker.html

### Architectural implication

Two claims remain distinct:

```text
A belongs to equivalence class E

A is the preferred/minimum-cost representative under cost C
```

Changing the operational cost model can change the preferred representative without changing mathematical equivalence.

---

## 14. Current mathematical-identity hypothesis

```text
NEW MATHEMATICAL ARTIFACT
    -> normalize only representation-invariant syntax/binders
    -> canonical deterministic serialization
    -> Merkle/dependency-aware structural digest
    -> immutable ledger node

THEORY/SEARCH
    -> attempt certified canonicalization/equivalence/refinement
    -> add relation edges / generation-scoped e-classes

EXECUTION
    -> choose preferred representative/specialization/realization
    -> cache by appropriate identity layer
```

The project therefore gets exact content addressing **without pretending arbitrary mathematical equivalence is decidable**.

---

## 15. New research obligations

1. Define exactly which metadata fields are structural-identity relevant versus provenance-only.
2. Design deterministic canonical serialization for typed semantic hypergraphs with cycles.
3. Study canonical graph labeling/hashing for port/wiring diagrams and e-hypergraph components.
4. Investigate incremental Merkle hashing under immutable-generation construction.
5. Define relation types for equivalence, refinement, implication, specialization, implementation, and behavioral equivalence.
6. Study generation/world-scoped equivalence-class identifiers and cache invalidation after class merges.
7. Investigate proof/certificate storage for equivalence paths and path minimization.
8. Study whether equivalence chains should be composed eagerly into direct certificates or lazily replayed.
9. Define collision/fallback policy for cryptographic structural hashes without sharing raw structure unnecessarily.
10. Build adversarial identity tests where alpha-renaming, algebraic equivalence, behavioral equivalence, and genuinely different semantics must be distinguished correctly.
