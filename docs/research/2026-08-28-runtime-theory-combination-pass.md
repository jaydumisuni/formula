# Runtime Theory Combination Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project will contain many specialist mathematical engines/theories. A mixed problem may contain constraints from several of them simultaneously.

The naive architecture:

```
call solver A
call solver B
merge results
```

is mathematically unsafe.

Satisfiability Modulo Theories (SMT) provides a mature narrow-domain study of exactly this runtime-composition problem.

## 1. Individual decidability does not guarantee combined decidability

Two theories can each have a decidable satisfiability problem while their union does not automatically inherit decidability, even under apparently simple signature conditions.

Source:
- https://doi.org/10.3390/math10030461

Therefore:

```
DECIDABLE(T1)
DECIDABLE(T2)
```

does **not** imply:

```
DECIDABLE(T1 ∪ T2)
```

without a theory-combination theorem/contract.

This is the runtime analogue of the package-interference checkpoint.

## 2. Nelson–Oppen combination

The classical Nelson–Oppen method combines decision procedures for quantifier-free theories under key conditions such as:

```
- disjoint signatures except equality;
- stable infiniteness;
- individual decision procedures;
- suitable formula/purification conditions.
```

Sources:
- https://users.aalto.fi/~tjunttil/2020-DP-AUT/notes-smt/combination.html
- https://doi.org/10.3390/math10030461

The mixed formula is purified into theory-specific parts. The theory solvers communicate enough equality information about shared variables/terms to ensure compatible models.

Architecture pattern:

```
mixed problem
      ↓
purification
      ↓
T1 obligations    T2 obligations
      |                |
      └── equality/shared-term exchange ──┘
                  ↓
          consistent arrangement?
                  ↓
          combined SAT/model
          or conflict lemma
```

This is extremely relevant to the project’s Work Cell architecture.

## 3. Equality as a narrow shared interface

The elegance of Nelson–Oppen is that solvers need not understand each other's entire internal theories.

They coordinate through a restricted shared interface—primarily equalities/disequalities over shared variables under the standard method.

This supports a powerful architectural principle:

> **Specialist mathematical engines should exchange the smallest semantic interface required by a proved combination theorem, not expose all internal reasoning to one another.**

This aligns with the interface-extraction and blackboxing checkpoints.

## 4. cvc5 combination architecture

cvc5 uses a Theory Engine/Combination Engine that coordinates multiple theory solvers, propagates or case-splits equalities/disequalities between shared terms, and constructs combined models.

Sources:
- https://cvc5.github.io/papers/2022/BarbosaBBKLMMMN-TACAS22.pdf
- https://cvc5.github.io/docs/cvc5-1.3.0/options.html

cvc5 can maintain distributed equality engines in individual theory solvers or a central equality engine depending on configuration.

This gives us an implementation donor for:

```
local specialist state
      +
central/shared semantic coordination
```

without requiring a monolithic all-math solver.

## 5. Polite theory combination

Stable infiniteness excludes important finite theories such as fixed-width bitvectors.

Polite/strongly-polite combination techniques provide alternative model-theoretic conditions enabling combinations where classic Nelson–Oppen does not apply.

Recent work studies properties including:

- stable infiniteness;
- smoothness;
- finite witnessability;
- strong finite witnessability;
- convexity.

Sources:
- https://link.springer.com/article/10.1007/s10817-025-09746-5
- https://arxiv.org/abs/2505.04870

This is important because the project’s Theory Profile should carry **combination properties**, not merely solver capabilities.

## 6. Combination properties belong in the Theory Profile

Potential fields:

```
combination_properties:
  signature_family
  stably_infinite
  convex
  smooth
  finitely_witnessable
  strongly_finitely_witnessable
  finite_model_property
  shared_sort_constraints
  supported_combination_protocols
```

These properties may themselves require certificates/proofs rather than handwritten labels for high-assurance packages.

## 7. Combination contract as a first-class artifact

A runtime route should require an explicit artifact:

```
CombinationContract(T1,T2,...)
```

containing:

```
preconditions
shared vocabulary
purification/translation rules
information exchanged
conflict semantics
model-combination rule
completeness scope
termination/decidability scope
certificate transport
```

The combined solver is authoritative only inside this contract.

## 8. Lemma exchange

DPLL(T)-style systems do more than independently solve theory fragments.

A theory solver can generate:

```
conflict lemma
propagation lemma
explanation
```

that feeds back into the Boolean/global search.

This maps directly to the project's failure-compilation architecture:

```
local theory conflict
       ↓
compact shared lemma
       ↓
prune global candidate worlds
```

A solver does not need to reveal its entire proof state—only an independently checkable theory lemma/explanation.

## 9. Model combination

For satisfiable problems, local theory models must agree on shared structure.

cvc5's Model Manager combines the local assignments/models once the theory solvers are sufficiently saturated and compatible.

Source:
- https://cvc5.github.io/papers/2022/BarbosaBBKLMMMN-TACAS22.pdf

This is a major lesson for future mathematical campaigns:

> **Local solutions are not automatically a global solution; shared mathematical interfaces must reconcile.**

This applies far beyond SMT.

## 10. Refutational completeness beyond decidability

Research also shows useful combinations where full decidability is lost but refutational completeness can still be retained under conditions.

Source:
- https://members.loria.fr/SMerz/papers/ijcar2012.html

The Theory Profile should therefore distinguish:

```
COMBINED_DECISION_PROCEDURE
COMBINED_SEMI_DECISION / REFUTATION_COMPLETE
HEURISTIC_COOPERATION_ONLY
UNSUPPORTED_COMBINATION
```

rather than one `compatible=true` bit.

## 11. Architecture-changing conclusion

The project should support **runtime mathematical federation**:

```
problem
  ↓
identify theory fragments
  ↓
find certified combination graph/protocol
  ↓
run specialist engines independently
  ↓
exchange only contract-approved semantic facts
  ↓
reconcile conflicts/models/proofs
```

This is much safer and more scalable than constructing one universal solver implementation.

## 12. Interaction with theory morphisms/reductions

If direct combination conditions fail, the engine may search for another route:

```
T2 fragment
   ↓ certified reduction/morphism
T2'
```

where `T1 + T2'` admits a known combination protocol.

So representation/reduction search can make a previously unsupported mixed problem combinable.

This is another reason reductions can unlock entire capabilities.

## 13. Core law

> **Mathematical engines may cooperate only through a combination theorem/contract that states what information must be shared and what global conclusions that cooperation preserves.**

Individual correctness is not enough.

## 14. Open research

1. Proof/certificate formats for theory-combination properties themselves.
2. Combination beyond disjoint signatures and equality-only sharing.
3. Model combination for probabilistic, optimization, algebraic, and numerical packages outside SMT logic.
4. Conflict-lemma transport through the universal certificate envelope.
5. Dynamic selection among Nelson–Oppen, polite, reduction-based, and monolithic specialized routes.
6. Incremental theory combination when one fragment changes.
7. Combination of exact theories with approximate/interval abstractions while preserving soundness.
8. Runtime theory federation over the future semantic hypergraph substrate.
9. Detecting and proving combination incompatibility early so unsupported solver mixes do not consume campaign compute.
