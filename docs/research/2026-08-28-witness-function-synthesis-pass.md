# Research Pass — Witness-Function Synthesis for Stability, Safety, Termination, and Global Behavior

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates a cross-domain pattern in which a difficult global/infinite-time property is transformed into the search for a finite mathematical witness function satisfying local or algebraic inequalities.

The central finding is:

> **Lyapunov functions, barrier certificates, ranking functions, and related witnesses are instances of a reusable metaproblem: synthesize a finite function whose certified local conditions imply a much larger global property.**

This is exactly the kind of inversion the unnamed project should exploit.

---

## 1. Lyapunov functions turn infinite-time stability into a finite witness search

A Lyapunov function establishes stability through positivity and decrease conditions rather than by enumerating every trajectory for all future time.

Recent formally certified work constructs exact rational sums-of-squares Lyapunov functions and verifies them in Minlog.

Source:

https://link.springer.com/article/10.1007/s10817-024-09717-2

Automated SMT-based synthesis also frames Lyapunov search as a second-order logical existence problem over a function satisfying the stability conditions universally.

Source:

https://link.springer.com/chapter/10.1007/978-3-030-45190-5_6

### Architectural implication

The Problem Compiler can transform:

```text
prove global/asymptotic stability of system S
```

into:

```text
find V in witness language W
such that certified Lyapunov obligations hold
```

The target condition is inverted into witness synthesis.

---

## 2. Barrier certificates do the same for safety

Barrier certificates define an invariant separating initial states from unsafe states using inequalities involving a barrier function and the system dynamics.

Pegasus includes automatic barrier-certificate generation for continuous systems.

Source:

https://keymaerax.org/Pegasus/methods.html

### Architectural implication

Safety can often be attacked as:

```text
find B
    -> B separates safe/unsafe regions
    -> derivative/transition condition preserves separation
    -> certify B
```

rather than:

```text
enumerate all reachable states
```

This can transform an unbounded reachability problem into a finite function-discovery problem.

---

## 3. Ranking functions are termination witnesses

A ranking function maps program/system states into a well-founded ordered set and strictly decreases with each transition. Its existence establishes absence of infinite descent and therefore termination.

Automatic synthesis algorithms exist for linear ranking functions.

Source:

https://arxiv.org/abs/1004.0944

### Architectural implication

The same generic witness-function machinery can solve:

```text
prove process terminates
```

by synthesizing:

```text
R(state)
```

plus a certified well-foundedness/decrease contract.

This directly connects to the fixed-point, progress-measure, and computability research.

---

## 4. Probabilistic systems generalize ranking witnesses

Ranking supermartingales and lexicographic ranking supermartingales generalize termination/progress witnesses to probabilistic programs/processes.

Source:

https://link.springer.com/chapter/10.1007/978-3-031-65633-0_19

### Architectural implication

Witness semantics must be theory-specific but can share a common envelope:

```text
WitnessFunction
    target_property
    state/domain
    codomain/order
    local obligations
    global theorem bridge
    certificate
```

The bridge theorem explains why the local obligations imply the target global claim.

---

## 5. The witness search and witness checker should be separate

The 2025 certified Lyapunov work explicitly uses external computational tools to construct certificates and formally verifies the result inside Minlog.

Source:

https://link.springer.com/article/10.1007/s10817-024-09717-2

### Architectural implication

The project can use extremely aggressive witness discovery:

```text
SDP/SOS solver
SMT
symbolic synthesis
program synthesis
models
random search
GPU optimization
```

as candidate producers.

Admission is through:

```text
exact witness reconstruction
    -> certificate
    -> independent checker / formal theorem bridge
```

This preserves truth while allowing search technology to evolve freely.

---

## 6. Approximate numerical witnesses can be reconstructed into exact ones

The certified Lyapunov framework begins with numerical semidefinite optimization but reconstructs exact rational weighted SOS certificates before formal proof.

Source:

https://link.springer.com/article/10.1007/s10817-024-09717-2

### Architectural implication

Another recurring architectural pattern is:

```text
cheap/fast approximate search
    -> locate promising witness region
    -> exact reconstruction
    -> independent certification
```

The project need not choose between fast numerical discovery and exact mathematical authority.

---

## 7. Sums-of-squares are one witness language, not the universal language

SOS methods convert polynomial nonnegativity obligations into semidefinite feasibility and provide useful certificates, with completeness guarantees for some restricted system classes.

Source:

https://arxiv.org/abs/1801.00070

But SOS can be expensive and is only one positivity certificate family.

### Architectural implication

The Theory Profile should select a witness language based on the system:

```text
linear / affine
polynomial SOS
barrier templates
piecewise polynomial
rational
lexicographic
ordinal/well-founded
probabilistic supermartingale
custom promoted witness family
```

The generic architecture is witness synthesis, not “everything is SOS.”

---

## 8. Witnesses may need to be synthesized jointly

Recent 2026 work on termination shows that ranking-function synthesis and invariant synthesis can mutually constrain each other. Syndicate uses bidirectional decompositional search instead of treating them as independent one-way tasks.

Source:

https://link.springer.com/chapter/10.1007/978-3-032-22723-2_7

Similarly, control research studies compatibility between control Lyapunov and control barrier functions rather than merely synthesizing each independently.

Source:

https://arxiv.org/abs/2406.18914

### Architectural implication

A mathematical campaign may contain **coupled witness unknowns**:

```text
find invariant I
AND ranking function R
such that I makes R valid
and failed R candidates refine I
```

The decomposition is maintained for computational efficiency while information flows bidirectionally.

This is a direct example of coordinated Mathematical Work Cells.

---

## 9. Witness synthesis is another property-bridge theorem family

The global implication has the shape:

```text
exists W satisfying local obligations O
    -> global property G
```

Examples:

```text
Lyapunov witness -> stability
Barrier witness -> safety
Ranking witness -> termination
Ranking supermartingale -> probabilistic termination
```

### Architectural implication

The registry should support **WitnessBridge** objects extending the earlier property-bridge concept:

```text
TargetProperty
WitnessLanguage
ObligationGenerator
GlobalSoundnessTheorem
OptionalCompletenessScope
CertificateRoute
```

When a problem matches the target property, the compiler can automatically create the corresponding synthesis campaign.

---

## 10. Completeness status matters enormously

Some witness languages are only sufficient: failure to find a witness does not imply the global property is false.

Restricted classes may have converse/completeness results establishing that an appropriate witness exists whenever the property holds.

### Architectural implication

Every witness bridge must declare:

```text
sound: yes/no
complete_for: fragment / unknown / no
degree/template completeness: ...
```

A failed SOS search should return:

```text
NO_WITNESS_IN_SEARCHED_CLASS
```

not:

```text
SYSTEM_UNSTABLE
```

unless a completeness theorem justifies that inference.

---

## 11. Witness quality can influence execution even after proof

A certified Lyapunov/barrier/ranking function can provide more than a yes/no theorem. It may expose:

```text
rate of convergence
margin to unsafe region
termination bound
regions of attraction
sensitivity
control feasibility region
```

### Architectural implication

Promoted witnesses can become reusable quantitative primitives and search heuristics, not merely archived proof certificates.

---

## 12. Current witness-function hypothesis

```text
GLOBAL MATHEMATICAL OBLIGATION
    -> query Theory Profile for applicable WitnessBridge
    -> construct witness synthesis space
    -> coordinate candidate generation and auxiliary invariants
    -> falsify candidates cheaply
    -> reconstruct exact witness where required
    -> independently certify local obligations
    -> apply certified bridge theorem
    -> establish global property
    -> promote reusable witness/derived primitive if valuable
```

This is one of the clearest examples of turning an apparently huge problem into a finite mathematical program search.

---

## 13. New research obligations

1. Generalize the WitnessBridge schema across stability, safety, termination, reachability, liveness, and probabilistic properties.
2. Study certificate formats for exact SOS/nonnegativity witnesses and independent checking.
3. Investigate automated discovery of appropriate witness language/template before coefficient search.
4. Study coupled witness synthesis as an AND/OR/fixed-point campaign.
5. Investigate witness transport through theory morphisms and representation changes.
6. Study quantitative information extractable from a certified witness beyond the base theorem.
7. Investigate compositional witnesses for decomposed systems and assume/guarantee interfaces.
8. Study witness compression/minimization and whether a simpler witness should be preferred for future reuse/checking.
9. Determine how failed witness searches generate useful counterexamples/nogoods without false refutation.
10. Investigate self-expansion where repeated expensive proofs lead the system to synthesize and certify a new reusable witness-generation primitive.
