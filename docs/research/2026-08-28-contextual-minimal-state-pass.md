# Contextual Minimal State and Hankel Realization Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

Automatic dynamic-programming synthesis, black-boxing, automata minimization, and interface extraction all ask a closely related question:

> What is the smallest state/summary that preserves everything future composition can observe?

Automata theory gives a precise canonical answer for important classes.

## 1. Myhill–Nerode: state as future distinguishability

For a language `L`, two prefixes `u` and `v` are equivalent when every possible continuation behaves identically:

```
u ≡_L v
iff
for every suffix/context w:
    uw ∈ L  <=>  vw ∈ L
```

The Myhill–Nerode theorem states:

- `L` is regular iff this equivalence has finitely many classes;
- the number of classes equals the minimum number of states in any DFA recognizing `L`;
- the quotient gives a canonical minimal automaton up to isomorphism.

Sources:
- https://planetmath.org/myhillnerodetheorem
- https://www.maths.tcd.ie/~stalker/11602/notes/10.8-the-myhill-nerode-theorem.html

This is a major conceptual result for the project:

> **A minimal sufficient state can be defined by all future contexts, not by implementation variables.**

## 2. Tree/context generalization

The Myhill–Nerode theorem generalizes to tree languages.

Two trees `u,v` are equivalent when for every one-hole tree context `C[-]`:

```
C[u] ∈ L  <=>  C[v] ∈ L
```

and finite contextual congruence yields the unique minimal deterministic finite tree automaton.

Source:
- https://www.eecs.harvard.edu/~shieber/Projects/Transducers/Papers/comon-tata.pdf

This matters because the project's mathematical objects are likely composed as trees/hypergraphs/networks rather than strings.

So the deeper principle is not suffix equivalence; it is **contextual indistinguishability**.

## 3. Connection to synthesized DP state

Suppose a brute-force specification recursively builds partial objects.

A DP algorithm needs a summary `S(x)` such that no relevant continuation distinguishes two partial objects with the same summary.

Formally, if:

```
S(x) = S(y)
```

then for every admissible context `C`:

```
Outcome(C[x]) = Outcome(C[y])
```

for the declared observer/objective.

This is exactly a Nerode-like congruence.

Therefore DP-state synthesis can be reframed as:

```
find / approximate / learn contextual equivalence classes
       ↓
use class ID or sufficient statistic as state
       ↓
compose states instead of full partial objects
```

This gives a mathematical foundation for the “synthesize missing summary” idea from AutoLifter/MetHyl.

## 4. Observer-relative minimality

Contextual equivalence depends on what future contexts are allowed and what result is observed.

The same internal object may admit different minimal summaries for:

```
existence
counting
minimum cost
probability
exact reconstruction
boundary behavior
```

So minimal state is **observer/query-relative**.

This connects directly to open-system blackboxing:

```
internal objects x,y
are equivalent
iff
no permitted boundary experiment distinguishes them
```

## 5. Weighted automata and Hankel matrices

For quantitative/formal power-series behavior:

```
f : Σ* -> Field
```

construct the infinite Hankel matrix:

```
H_f(u,v) = f(uv)
```

Fliess/Carlyle-Paz-style results establish that the rank of the Hankel matrix equals the minimum number of states in an exact weighted-automaton realization over appropriate fields.

Sources:
- https://games-automata-play.github.io/blog/fliess_theorem/
- https://arxiv.org/abs/2009.01217

Thus:

```
rank(H_f) = minimal exact state dimension
```

This generalizes binary indistinguishability into linear algebra.

A finite-dimensional behavioral state exists exactly when the behavior has finite Hankel rank in the relevant setting.

## 6. Weighted/tree generalization

Weighted-tree automata have analogous Hankel-rank/minimality results.

Source:
- https://www.sciencedirect.com/science/article/abs/pii/S0890540120301425

This is highly relevant to mathematical programs represented as compositional trees:

```
complex structured function
      ↓
Hankel/context behavior
      ↓
finite rank?
      ↓
minimal finite-dimensional realization
```

## 7. Arithmetic-circuit connection

Hankel-rank methods also characterize important classes of arithmetic branching programs/circuits.

Source:
- https://eccc.weizmann.ac.il/report/2018/038/

This links:

```
program/formula succinctness
      ↕
weighted automata state
      ↕
Hankel rank
```

and gives lower bounds: sometimes no smaller state representation exists within the declared realization class.

Negative/minimality evidence is just as important as successful compression.

## 8. Architecture-changing hypothesis: contextual quotient metaprimitive

Potential project operation:

```
CONTEXTUAL_QUOTIENT(
    object_family,
    composition_contexts,
    observer
)
```

returns or searches for:

```
equivalence relation ~
state representation S
composition operation δ
observer/recovery map h
```

with obligation:

```
x ~ y
=>
forall admissible C:
    Obs(C[x]) = Obs(C[y])
```

If the quotient is finite/small, a large problem may collapse into a finite-state DP/automaton/transfer-matrix primitive.

## 9. Exact versus approximate state minimization

In many domains the exact contextual quotient may be infinite or too expensive.

Then the project may search:

```
exact finite quotient
      ↓ if unavailable
sound abstraction
      ↓
observer-specific approximate state
      ↓
certified error/refinement contract
```

Weighted-automata research also studies approximate minimization when exact state is too large.

Source:
- https://www.cambridge.org/core/journals/mathematical-structures-in-computer-science/article/optimal-approximate-minimization-of-oneletter-weighted-finite-automata/9733CA9F3079F9186103F8515DE86D96

Approximate state must carry explicit error semantics rather than silently replacing exact equivalence.

## 10. Active learning connection

Myhill–Nerode states can be learned through membership/equivalence queries (Angluin-style automata learning) without enumerating the entire input language.

This connects to the prior symbolic-query-learning checkpoint:

```
candidate state machine
      ↓
equivalence query
      ↓
counterexample
      ↓
refine contextual partition
```

So minimal-state discovery can potentially use a counterexample-driven learning campaign.

## 11. Relation to blackboxing

The black-box/minimal state of a component can be interpreted as the quotient of internal histories/states by boundary-observable future behavior.

Thus these previously separate research threads may unify:

```
DP summary synthesis
behavioral blackboxing
automata minimization
interface extraction
state abstraction
```

under:

> **contextual observational equivalence**.

This is one of the strongest conceptual unifications found so far.

## 12. Relation to theory interfaces

A semantic interface may not only list exposed symbols.

It may expose a **minimal behavioral state** sufficient for all certified downstream operations.

A huge theory/component could therefore publish:

```
interface vocabulary
+ contextual state space
+ transition/composition law
+ observer/query algebra
```

rather than its full internal derivation graph.

## 13. Core law

> **Two mathematical states are interchangeable for a task exactly when no admissible future mathematical context can distinguish them under the requested observer.**

Whenever this equivalence has a finite/low-dimensional quotient, that quotient is a candidate optimal state representation.

## 14. Open research

1. Generalizing contextual equivalence from trees/automata to typed semantic hypergraphs/open systems.
2. Learning contextual quotients with counterexamples in infinite/algebraic domains.
3. Hankel/operator-rank methods for continuous, probabilistic, or semiring-valued mathematical behaviors.
4. Certified lower bounds proving that no smaller summary exists in a declared representation class.
5. Automatically deriving DP summaries from contextual experiments rather than syntax.
6. Observer-specific quotient reuse across multiple semiring evaluation algebras.
7. Incremental maintenance of a learned contextual quotient as new mathematical contexts/theorems are added.
8. Interaction with theory morphisms: transport minimal-state representations between structurally equivalent domains.
9. Approximate/abstract contextual equivalence with rigorous error bounds.
10. Whether contextual quotienting should become a central mathematical compression metaprimitive in the first architecture design.
