# Research Pass — Open-System Compositional Semantics, Boundaries, Cospans, and Wiring Algebra

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a principled mathematical layer for composing independently defined mathematical components/packages through explicit interfaces rather than merging their entire internal theories or representations.

The central finding is:

> **Applied-category-theory frameworks for open systems model a component as internal structure together with typed boundary ports, and compose components by wiring/identifying those boundaries. Structured/decorated cospans, hypergraph categories, and wiring diagrams therefore provide a serious donor for the project's mathematical interface/composition algebra.**

---

## 1. Open systems separate internal mathematics from external boundary

Open-system frameworks represent systems with interfaces through which they interact with an environment or other systems.

Structured cospans have the form conceptually:

```text
boundary_in -> internal_system <- boundary_out
```

and larger systems are constructed by composing along compatible boundaries.

Sources:

https://arxiv.org/abs/2008.02394

https://www.researchgate.net/publication/410740405_Double_Categories_of_Open_Systems_The_Cospan_Approach

### Architectural implication

A mathematical package/construction can expose:

```text
INTERNAL SEMANTICS
    arbitrary rich theory/representation

BOUNDARY SEMANTICS
    typed variables/relations/ports visible to composition
```

Other packages need not inspect or merge all internals merely to compose with it.

---

## 2. Composition is wiring, not inheritance

Decorated/structured cospan categories compose components by identifying/connecting their interface variables/ports. Current open-systems research calls this the variable-sharing paradigm.

Source:

https://arxiv.org/abs/2509.22584

### Architectural implication

This reinforces the project's rejection of one giant mathematical inheritance hierarchy.

Composition can instead be:

```text
Package A boundary
    <-> certified wiring/morphism <->
Package B boundary
```

with the composed object retaining internal modularity.

---

## 3. Hypergraph categories formalize arbitrary network-style composition

Hypergraph categories are used across circuits, tensor networks, automata, databases, linear relations, graph rewriting, and belief propagation. They give algebraic laws for wiring diagrams where values/ports can split, merge, create, and discard according to declared Frobenius structure.

Sources:

https://www.sciencedirect.com/science/article/pii/S0022404919300489

https://arxiv.org/abs/1609.05382

### Architectural implication

The project's semantic composition layer needs operations beyond ordinary sequential function composition:

```text
sequential composition
parallel/tensor composition
shared-variable connection
fan-out / merge where structure permits
hiding/internalization of boundary variables
feedback/closure in richer fragments
```

Hypergraph/wiring algebra is a strong mathematical donor for these operations.

---

## 4. Wiring diagrams are a compact executable representation

Catlab represents symmetric-monoidal morphism compositions as typed port graphs/wiring diagrams because tree-form expression syntax becomes too verbose for large compositions.

Source:

https://github.com/AlgebraicJulia/Catlab.jl/blob/main/docs/literate/wiring_diagrams/wd_cset.jl

### Architectural implication

This independently supports the project's typed semantic hypergraph direction.

A large mathematical construction can be stored as:

```text
boxes = mathematical components/relations
ports = typed interfaces
wires = composition/sharing
```

rather than a huge nested AST.

This is naturally compatible with decomposition and parallel Work Cells.

---

## 5. Composition laws can be separated from component semantics

The same wiring syntax can be interpreted into different semantic categories through functors. Fong's framework explicitly separates diagrammatic composition from semantic interpretation.

Source:

https://arxiv.org/abs/1609.05382

### Architectural implication

The project can distinguish:

```text
COMPOSITIONAL SKELETON
    how components are wired

SEMANTIC INTERPRETATION
    algebra, probability, dynamics, relation, program, etc.
```

This connects directly to semiring-parametric evaluation and factorized FAQ execution: one structure may support several certified interpretations.

---

## 6. Different open-system theories can be mapped into one another

The 2026 open-systems overview studies maps between categories of open systems, including maps from open Petri nets with rates to open dynamical systems.

Sources:

https://www.researchgate.net/publication/410740405_Double_Categories_of_Open_Systems_The_Cospan_Approach

https://math.ucr.edu/home/baez/double.pdf

### Architectural implication

A representation transformation may operate compositionally:

```text
component semantics A
    -> mapped semantics B
```

while preserving wiring structure.

This could allow a whole assembled system to be transformed/reduced by mapping each component rather than synthesizing a monolithic transform from scratch.

---

## 7. Boundary composition complements theory-interface extraction

Earlier research on uniform interpolation/forgetting sought the smallest logical interface visible to another theory.

Open-system composition provides a complementary operational view:

```text
logical/semantic interface
    what facts are visible

open-system boundary
    what variables/ports are connected
```

### Architectural implication

A promoted mathematical component may need both:

```text
semantic interface theory
boundary signature/ports
```

so the compiler knows both what can be inferred and how components physically/mathematically connect.

---

## 8. Boundary hiding is a natural form of abstraction

After composing two open systems, internal boundary variables used only for their connection can be hidden from the external interface.

### Architectural implication

This gives a principled route from:

```text
large composition with many internal variables
```

to:

```text
small externally visible relation
```

without destroying internal proof/provenance.

It connects to existential projection, variable elimination, interface forgetting, and knowledge compilation.

---

## 9. Compositional certification can follow the wiring graph

If each component has certified semantics and the wiring/composition operation has a theorem, the composite can derive a certificate by composition rather than re-proving the internal meaning of every component.

### Architectural implication

The certificate envelope should support hierarchical/compositional proofs:

```text
component certificate A
component certificate B
boundary compatibility proof
composition theorem
    -> composite certificate
```

This is critical for building very large mathematical constructions from previously promoted primitives.

---

## 10. Open-system decomposition aligns with Work Cells

A large wiring diagram already exposes component boundaries and dependency structure.

### Architectural implication

The campaign compiler can map diagram components/subdiagrams directly to Mathematical Work Cells when:

```text
interfaces are small
components independent given boundaries
composition theorem permits separate evaluation
```

Thus compositional semantics can generate mathematically justified parallel execution automatically.

---

## 11. Black-boxing can turn a complex component into a boundary behavior

A mature component may be replaceable by a smaller semantic relation describing only externally visible behavior, while its internal implementation remains hidden.

### Architectural implication

The project should investigate a `black_box`/behavioral-abstraction metaprimitive:

```text
open system S
    -> externally equivalent boundary relation B(S)
```

with proof that every observable interaction is preserved.

This can dramatically reduce future composition/search size while retaining an exact reference to the internal certified implementation.

---

## 12. Current open-system hypothesis

```text
CERTIFIED MATHEMATICAL COMPONENT
    internal semantic graph/theory
    typed boundary/interface

COMPOSITION
    -> typed wiring / shared-variable identification
    -> boundary compatibility proof
    -> hierarchical composite semantic object

OPTIONAL BLACK-BOXING
    -> derive compact external behavior

INTERPRETATION / LOWERING
    -> relation / factor graph / dynamics / program / other target semantics
```

This may provide the principled middle layer between individual mathematical primitives and the full semantic e-hypergraph universe.

---

## 13. New research obligations

1. Study structured/decorated cospans versus e-hypergraphs and determine whether one can serve as the composition API over the other.
2. Investigate double-category/2-cell semantics for transformations/proofs between mathematical components.
3. Study automatic black-box behavioral extraction for linear, relational, probabilistic, and dynamical components.
4. Connect boundary hiding to uniform interpolation, existential projection, and FAQ variable elimination.
5. Investigate typed boundary compatibility through the mathematical structure/type system.
6. Study compositional certificate generation and minimal rechecking when one component changes.
7. Investigate cost/resource composition along wiring diagrams.
8. Study decomposition/parallelization directly from open-system boundaries.
9. Determine how theory morphisms/reductions transform whole wiring diagrams compositionally.
10. Evaluate Catlab/C-Set/wiring-diagram implementation ideas as donors without committing the core to Julia or category-theory-specific runtime representations.
