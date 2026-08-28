# Research Pass — Generalization, Representation Change, and Canonicalization

**Date:** 2026-08-28  
**Status:** RESEARCH supplement to `2026-08-28-research-checkpoint.md`  

This note records architecture-changing findings discovered after the first checkpoint. Nothing here freezes the final design.

---

## 1. Exact relation-space discovery is stronger than formula guessing

A July 2026 paper, **Discovery of Exact Equations via Computing the Gröbner Basis**, formulates exact equation discovery over noise-free integer/rational data as computation of the **vanishing ideal** of the observed points.

Instead of guessing one equation at a time, the method computes algebraic structure representing the full family of polynomial relations satisfied by the data. A Gröbner basis then gives a finite basis for that potentially infinite relation space, canonical once a monomial order is fixed.

Source:

https://link.springer.com/article/10.1007/s10994-026-07112-z

The reported MoadeeB method was evaluated over more than 30,000 OEIS sequences, recovering known recurrences and discovering previously undocumented ones.

### Architectural implication

The project should not assume discovery means:

```text
search candidate formula
-> test formula
```

For some domains, discovery can mean:

```text
observations / constraints
-> derive entire valid relation space
-> compute finite generators / canonical basis
-> search or compile inside that basis
```

This is a much stronger form of mathematical compression.

---

## 2. Canonicalization/completion may be a cross-domain metaprinciple

Several independent mathematical/computational areas perform the same high-level operation:

```text
large equivalence/relation space
-> completion/canonicalization
-> compact normal-form machinery
```

Examples:

- Gröbner bases for polynomial ideals;
- Knuth-Bendix-style completion for equational rewrite theories;
- graph canonical labeling modulo isomorphism;
- congruence closure/e-graphs for term equivalence;
- Maude canonical forms modulo equational axioms.

The project should research whether these can be exposed through one **canonicalization/completion metainterface** while preserving the domain-specific mathematics underneath.

This does **not** imply one universal canonicalization algorithm exists.

The potential shared contract is closer to:

```text
semantic theory / equivalence relation
+ admissible orientation/order/cost
-> canonical or normal-form mechanism
+ certificate/conditions
```

A major open question is which mathematical theories admit finite, terminating, complete canonicalization and which do not.

---

## 3. Maude proves a powerful relation between canonical forms, inversion, and symbolic reachability

Maude supports order-sorted unification modulo equational axioms such as associativity and commutativity.

Its **variant** concept pairs:

```text
substitution
+
canonical form of the instantiated term
```

Variant generation and folding-variant narrowing can then be used for:

- equational unification;
- symbolic evaluation;
- symbolic reachability;
- inverse-style solving.

Sources:

https://maude.lcc.uma.es/maude30-manual-html/maude-manualch14.html

https://maude.lcc.uma.es/maude31-manual-html/maude-manualch15.html

### Important limitation

This search is not automatically finite.

Maude distinguishes theories with the **finite variant property**, under which a finite complete set of most-general variants exists, from theories where the variant/unifier space can be infinite.

Even narrowing modulo common algebraic laws such as associativity-commutativity can fail to terminate under naive strategies.

### Architectural implication

Relational meaning may support forward/inverse/search behavior, but the project must classify mathematical theories by operational properties such as:

```text
finite complete search?
termination known?
confluent?
canonical form available?
finite variant property?
approximation required?
```

This classification should influence search planning before work is launched.

---

## 4. Symbolic theory exploration already grows its own rewrite/search knowledge

Theory-exploration systems such as IsaCoSy, HipSpec, QuickSpec, and TheSy provide strong deterministic donors.

### IsaCoSy / HipSpec

Previously proved theorems can be oriented into rewrite rules. Future conjecture generation then avoids terms reducible by those rules.

This means:

```text
new theorem
-> new simplification/rewrite knowledge
-> fewer future terms generated
-> changed future search geometry
```

Sources:

https://link.springer.com/article/10.1007/s10489-017-0954-8

https://github.com/danr/hipspec

### TheSy

TheSy uses e-graphs plus **symbolic observational equivalence** rather than only concrete random tests. It generates symbolic examples containing uninterpreted leaves, reasons about equivalence symbolically, selects compact/general conjectures, and then verifies them with induction/proving machinery.

Source:

https://link.springer.com/chapter/10.1007/978-3-030-81688-9_6

### Architectural implication

The project should distinguish at least:

```text
candidate relation
observational/symbolic-equivalence support
counterexample status
formal proof status
promotion to rewrite/search primitive
```

A proved result should be able to alter future enumeration immediately.

---

## 5. Generalization is mathematically constrained, not a free operation

Anti-unification computes least general generalizations of terms, but current research shows that once algebraic theories and binding are included, the problem becomes much more complex.

### 2026 nominal anti-unification modulo equational theories

Research extending nominal anti-unification to associativity, commutativity, and AC shows that unrestricted settings may fail to admit finite solution sets; finitary algorithms require restrictions such as a finite atom vocabulary.

Source:

https://doi.org/10.1016/j.jlamp.2025.101100

### 2026 complexity of generalization with commutative functions

Least-general generalization becomes substantially harder when commutative symbols are admitted.

Source:

https://link.springer.com/article/10.1007/s10472-026-10014-4

### Architectural implication

A metaprimitive such as:

```text
generalize(C1...Cn)
```

cannot mean unrestricted generalization.

It needs parameters/metadata such as:

```text
active equational theory
allowed constructors
allowed variables/atoms
binding rules
maximum generality class
finiteness expectations
counterexample domain
proof obligation
```

Generalization itself is a mathematical campaign with a declared theory and search contract.

---

## 6. Exact symmetry discovery can be algorithmic

A 2026 COLT paper, **Efficient Learning and Symmetry Discovery under Exact Invariances**, gives polynomial-time results for exact group-invariant learning and, for finite-group subgroup lattices, recovery of an unknown exact symmetry from data.

Source:

https://proceedings.mlr.press/v336/soleymani26a.html

### Architectural implication

`discover_symmetry` need not always mean heuristic pattern recognition.

For suitable domains the system can have exact algorithms that:

```text
identify symmetry
-> quotient/canonicalize states
-> reduce future search space
```

The project should maintain a library of **representation-change algorithms with declared validity domains**, rather than one generic representation-search heuristic.

---

## 7. Graph canonicalization is a useful concrete representation-normalization donor

Tools such as bliss/nauty compute:

- automorphism groups;
- canonical labelings;
- canonical graph forms.

A canonical labeling map ensures isomorphic graphs receive identical canonical forms.

Source:

https://users.aalto.fi/~tjunttil/bliss/definitions.html

### Important warning

bliss documentation notes that the chosen canonical form can depend on implementation version/options, even though it remains a valid canonicalizer.

### Architectural implication

The project must distinguish:

```text
mathematical equivalence identity
```

from:

```text
chosen implementation-specific canonical representative
```

A canonical representative may change between implementation generations while the underlying mathematical equivalence class does not.

This reinforces the earlier separation between mathematical identity and implementation identity.

---

## 8. Abstract interpretation supplies a sound theory of representation simplification

Abstract interpretation replaces expensive concrete semantics with computation in a simpler **abstract domain**.

Galois connections relate abstraction and concretization and provide a mathematical basis for soundness/precision.

Sources:

https://doi.org/10.1145/3728905

https://www.sciencedirect.com/science/article/pii/S0304397520301122

This becomes directly relevant to program/mathematical search because **Absynthe** demonstrates synthesis guided by user-defined abstract semantics. More expressive abstract domains prune more candidates but cost more to evaluate.

Source:

https://arxiv.org/abs/2302.13145

### Architectural implication

A representation-change search may distinguish:

```text
exact equivalence-preserving representation
```

from:

```text
sound abstraction that deliberately loses information
```

Both can be useful, but they need different truth semantics.

A sound abstraction may solve or disprove a class of questions cheaply even though it cannot reconstruct every concrete detail.

This creates a potential formal contract for:

```text
abstract(problem, domain A)
solve in A
refine if insufficient
concretize/certify result
```

and connects naturally to CEGAR-style refinement.

---

## 9. Algebraic invariant discovery can derive whole invariant ideals

Classical work on polynomial loop invariants shows that, for suitable program classes, **all polynomial invariants** form an ideal and can be computed using Gröbner-basis machinery.

Source:

https://www.sciencedirect.com/science/article/pii/S0747717107000107

The method is correct and complete for its declared class, with termination bounds for specific solvable mappings.

Related work combines polynomial ideals with abstract interpretation and widening to handle broader control flow while trading completeness for guaranteed termination.

Source:

https://www.sciencedirect.com/science/article/pii/S0167642306001572

### Architectural implication

`discover_invariant` should not be a single algorithm.

It should be a capability family whose implementations declare:

```text
invariant language/class
completeness class
termination conditions
exact/approximate status
certificate route
```

In some domains, the output can be the **strongest representable invariant**, not merely one useful invariant.

---

## 10. New design hypothesis: theory-property profiling before search

The findings above suggest that each mathematical theory/problem region may need an automatically derived **operational profile** before expensive search begins.

Conceptual fields:

```text
available canonical form?
known normal form?
confluence known?
termination known?
finite variant property?
finite basis theorem available?
symmetry group computable?
exact abstraction available?
complete invariant class available?
certificate/checker available?
search space finite / finitely generated / infinite?
```

This profile could determine which metaprimitives are safe and efficient.

For example:

```text
if finite Gröbner basis route exists:
    derive relation ideal instead of enumerate equations

if finite variant property holds:
    use complete variant-based inverse search

if exact canonical labeling exists:
    quotient symmetry before enumeration

if only sound abstraction exists:
    abstract -> solve -> refine
```

This is a stronger idea than generic tool routing: the machine chooses a search strategy based on **mathematical structural properties of the theory itself**.

---

## 11. Revised next research obligations

1. Investigate completion/canonicalization as a shared metainterface across rewrite systems, Gröbner bases, graph canonization, and e-graphs.
2. Investigate automatic detection of theory properties: confluence, termination, finite variant property, finite basis conditions, symmetry/canonicalization availability.
3. Investigate CEGAR/abstract interpretation as a sound representation-change/refinement engine for mathematical search.
4. Investigate exact/symbolic invariant generation beyond polynomial ideals and how invariant classes can be composed.
5. Investigate whether provenance can preserve alternate derivation paths through canonicalization and abstraction changes.
6. Continue automatic generalization research with explicit theory-bounded anti-unification rather than unrestricted abstraction.

---

## 12. Current synthesis

The new evidence suggests that the project should not treat all search as enumeration.

Before searching candidates, it should ask whether the mathematical domain admits a stronger structural operation:

```text
Can the whole relation space be generated by a finite basis?
Can equivalence classes be canonicalized?
Can symmetry be quotiented out?
Can the theory be completed into a terminating rewrite system?
Can a sound abstraction collapse the problem?
Can all invariants of a declared class be generated directly?
Can inverse solutions be represented by finitely many most-general variants?
```

If the answer is yes, use that structure first.

This may be one of the central ways the unnamed project becomes dramatically more powerful than brute-force mathematical search.