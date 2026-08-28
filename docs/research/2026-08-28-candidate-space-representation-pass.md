# Research Pass — Compact Candidate-Space Representations

**Date:** 2026-08-28  
**Status:** RESEARCH supplement  

This pass investigates how the unnamed mathematical project can represent enormous families of candidate executable constructions, formulas, transformations, or proof terms without enumerating each candidate individually.

---

## 1. Version Space Algebras show that candidate programs can be manipulated as sets

Version Space Algebras (VSAs) compactly represent sets of programs using operations such as:

- union;
- intersection;
- Cartesian/product-style join;
- extraction/ranking.

Microsoft PROSE uses version-space representations to synthesize ranked programs consistent with examples.

Sources:

https://www.microsoft.com/en-us/research/project/prose-framework/

https://www.microsoft.com/en-us/research/project/prose-framework/usage/

### Architectural implication

The project should not assume candidate discovery means:

```text
generate C1
check C1
generate C2
check C2
...
```

For appropriate mathematical families it can instead maintain:

```text
CandidateSpace S
```

and apply operations that eliminate or combine whole families at once.

Potential abstract operations:

```text
union(S1, S2)
intersect(S1, S2)
restrict(S, constraint)
compose_spaces(S1, S2)
extract_best(S, cost_model)
```

This is a research-level abstraction, not a frozen interface.

---

## 2. Finite Tree Automata generalize compact program-space representations

Program ASTs form trees, so finite tree automata (FTAs) can recognize sets of possible programs.

FTA-based synthesis can represent many programs sharing substructure compactly, and intersections can represent programs satisfying several examples simultaneously.

Source:

https://www.microsoft.com/en-us/research/publication/synthesis-data-completion-scripts-using-finite-tree-automata/

A theoretical result shows that reified Version Space Algebras embed into acyclic tree automata, making tree automata a broader formal substrate for these compact term/program spaces.

Source:

https://arxiv.org/abs/2107.12568

### Architectural implication

The project should treat tree-automata-like structures as a serious donor for candidate mathematical construction spaces, especially where the candidate language is grammar-generated.

---

## 3. Candidate-space intersection can apply evidence to entire families

In FTA-based synthesis, an automaton can be constructed for each example/specification and automata can be intersected so the result accepts only programs satisfying every specification.

### Mathematical analogue

```text
S0 = all constructions in grammar G
S1 = restrict S0 by witness/evidence E1
S2 = restrict S1 by witness/evidence E2
...
```

Each new certified witness can therefore reduce a symbolic candidate set without enumerating its members.

This fits directly with the symbolic-query-learning research checkpoint.

---

## 4. Ordinary VSA/FTA/e-graph sharing has an important limitation: entangled choices

Many compact representations exploit independence between subterms.

That breaks down when choices at distant positions must be equal or otherwise related.

Example family:

```text
f(t) + f(t)
```

where both occurrences must use exactly the same `t`.

Representing the two branches independently admits invalid mixed combinations.

---

## 5. Equality-Constrained Tree Automata address entangled candidate spaces

**Equality-Constrained Tree Automata (ECTAs)** extend tree automata with equality constraints between paths/subterms.

They were introduced specifically to compactly represent program spaces where choices of subterms are entangled, and are described as generalizing major sharing-based structures including VSAs, FTAs, and e-graph-related representations.

Sources:

https://arxiv.org/abs/2206.07828

https://www.jameskoppel.com/publication/ecta/

The Hectare synthesizer built on ECTA reported an average 8x speedup over a state-of-the-art type-directed synthesizer while having a much smaller implementation.

### Architectural implication

Candidate mathematical constructions frequently contain global structural requirements such as:

```text
same subexpression used twice
same basis/parameter used across branches
inverse pair must refer to same transformation
shared invariant across several substructures
matching domain/codomain identity
```

A candidate-space representation that cannot encode these dependencies compactly may explode even if ordinary subterm sharing is excellent.

Therefore **entanglement constraints must be first-class in the candidate-space research**.

---

## 6. Weighted/constrained tree automata suggest search cost can live with the space representation

Weighted tree automata and tree automata with equality/inequality constraints are established research areas.

Representative source:

https://doi.org/10.1007/s00224-023-10144-w

### Architectural implication

A future candidate-space structure may be able to attach values such as:

- execution cost;
- construction size;
- proof/checking cost;
- search priority;
- complexity estimate;
- novelty/generalization score;

without extracting every represented construction first.

This connects candidate-space representation directly to the search-economy layer.

---

## 7. Deductive inverse semantics can prune from the target backward

PROSE supports witness functions / inverse semantics: given a desired output property of an operator, derive constraints/specifications for the operator's arguments.

Source:

https://www.microsoft.com/en-us/research/project/prose-framework/tutorial/

### Architectural implication

This is a narrow but concrete analogue of the project's relational/inverse-search vision:

```text
desired relation/output
    -> propagate admissible requirements backward
    -> construct compact child candidate spaces
```

Not every mathematical operator admits practical complete inverse semantics, so this should be treated as an optimization/capability property rather than a universal requirement.

---

## 8. Candidate spaces may need multiple representations

Current evidence suggests no single compact representation dominates all spaces:

```text
VSA
    good for union/join structured independent spaces

FTA
    broader grammar/tree-language representation

ECTA
    adds entangled equality constraints

e-graph
    particularly strong for equivalence/rewrite closure

versioned e-graph
    multiple contextual equality worlds

e-hypergraph
    richer multi-input/output semantic target abstraction
```

### Architectural implication

The project should probably avoid constitutionally binding candidate search to one concrete data structure.

It may instead need a semantic candidate-space contract with several backend representations selected by Theory Profile / problem structure.

---

## 9. Current hypothesis

A discovery campaign may operate over **symbolic candidate spaces** as first-class mathematical work artifacts:

```text
Candidate Space
    |
    +-- grammar / semantic family
    +-- domain/codomain constraints
    +-- equality/disequality constraints
    +-- assumption/world identity
    +-- accepted/rejected witnesses
    +-- compact representation backend
    +-- ranking/cost annotations
    +-- provenance
```

Then new evidence transforms the space itself:

```text
S
 -> intersect with constraint
 -> quotient by proven equivalence
 -> remove certified nogood family
 -> add required entanglement
 -> extract candidate only when useful
```

This may be far more scalable than candidate-by-candidate search.

---

## 10. New research obligations

1. Compare ECTA constraints with e-graph/e-hypergraph equality semantics and determine whether these structures can interoperate or should remain separate backends.
2. Investigate whether candidate spaces can carry assumptions/world versions without full duplication.
3. Study intersection/emptiness/member/extraction complexity for the candidate-space families being considered.
4. Determine how certified nogoods and distinguishing witnesses can be compiled into candidate-space restrictions.
5. Investigate weighted tree automata / semiring-weighted candidate spaces for search-economy integration.
6. Determine whether candidate-space operations themselves can emit certificates or independently checkable witnesses.
7. Investigate compact representations for non-tree mathematical constructions such as cyclic graphs, state machines, tensor networks, and hypergraph relations.
8. Determine how candidate-space representations can evolve when a newly promoted primitive extends the mathematical grammar itself.
