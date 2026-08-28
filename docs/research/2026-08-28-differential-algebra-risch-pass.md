# Differential Algebra, Risch Integration, and Liouvillian Solvability Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

A general mathematical discovery system must know when a target representation does **not** exist inside a declared class.

Otherwise formula search can continue indefinitely looking for:

- an elementary antiderivative that does not exist;
- a Liouvillian differential-equation solution that does not exist;
- a closed form in a restricted language that mathematics rules out.

Differential algebra provides exact structural algorithms for important versions of this question.

## 1. Risch/Bronstein integration as a decision problem

Risch-style symbolic integration does not merely search a table of antiderivatives.

It translates elementary functions into towers of differential-field extensions and uses Liouville/Risch structure theorems to decide whether an elementary antiderivative exists in the declared class.

Bronstein's work describes a decision procedure that can:

```
input elementary function f
       ↓
construct differential-field representation
       ↓
perform Hermite/reduction/structure analysis
       ↓
   ┌───────────────┬─────────────────────┐
   ↓               ↓
find g          prove no elementary g
with g' = f     exists in target class
```

Sources:
- https://www.sciencedirect.com/science/article/pii/S0747717108800272
- https://link.springer.com/book/10.1007/978-3-662-03386-9

This is a major architectural precedent for **negative formula discovery**.

## 2. Differential-field representation

The algorithm constructs a tower whose generators represent operations such as:

```
algebraic extensions
logarithms
exponentials
```

and turns integration into algebraic/differential equations inside that tower.

This parallels the discrete difference-ring summation pass:

```
discrete sums
 -> difference rings
 -> difference equations

continuous elementary integrals
 -> differential fields
 -> differential equations
```

These should eventually be viewed as sibling Theory Profile families.

## 3. Liouville-type structure theorem

Liouville's theorem constrains the form an elementary antiderivative can have. Roughly, if an elementary antiderivative exists, it can be represented by a derivative term plus a finite combination of logarithmic derivatives under the relevant differential-field conditions.

This converts the open-ended search:

```
try arbitrary elementary expression
```

into a finite/algebraically constrained structural problem.

That is exactly the project's central objective: **discover stronger mathematics first so less search is necessary**.

## 4. Risch differential equations

Implementations such as FriCAS expose explicit routines for Risch differential equations and parameterized variants.

Sources:
- https://fricas.org/api/ElementaryIntegration.html
- https://fricas.org/api/ElementaryRischDEX2.html

This is important because integration, algebraic dependence, recurrence/ODE solution, and parametric problems can share lower-level differential-field equation solvers rather than being unrelated features.

## 5. Practical implementation warning

The mathematical algorithm/theory may be complete under assumptions while real CAS implementations remain incomplete.

FriCAS documents significant implementation gaps and assumptions around:

- transcendental constants / constant-field decisions;
- algebraic extensions;
- portions of the Risch preparation stage.

Source:
- https://wiki.fricas.org/RischImplementationStatus

Therefore the project must distinguish:

```
THEORY_COMPLETE_FOR_CLASS
```

from

```
BACKEND_IMPLEMENTATION_COMPLETE_FOR_CLASS
```

A backend returning `failed/unimplemented` must never be interpreted as a mathematical proof of non-existence unless the backend certifies that the complete decision branch applied.

## 6. Kovacic algorithm: ODE solvability with a negative answer

Kovacic's algorithm decides Liouvillian solvability for second-order homogeneous linear ODEs with rational-function coefficients.

The original paper explicitly states that if the algorithm finds no closed-form/Liouvillian solution, then no such solution exists in the target class.

Source:
- https://www.sciencedirect.com/science/article/pii/S0747717186800104

Conceptually:

```
y'' + a(x)y' + b(x)y = 0
       ↓
differential-Galois/structural classification
       ↓
Liouvillian solution found
       OR
certified no Liouvillian solution in class
```

This is an ideal Theory Profile operation.

## 7. Differential Galois perspective

Kovacic-style solvability is tied to structure of the differential Galois group. Structural properties of the group determine whether Liouvillian solutions exist.

Source:
- https://link.springer.com/article/10.1007/s40863-023-00359-7

This reinforces the project's general architecture:

```
problem
  ↓
discover/classify hidden mathematical structure
  ↓
structure theorem
  ↓
whole search class becomes solvable or impossible
```

The system should actively seek such structural classifiers because one proof can eliminate enormous candidate spaces.

## 8. Formula-language escalation

A result like:

```
NO_ELEMENTARY_ANTIDERIVATIVE
```

must not mean:

```
NO MATHEMATICAL REPRESENTATION EXISTS
```

The system may escalate representation class:

```
elementary functions
       ↓ impossible
special functions
       ↓
holonomic representation
       ↓
integral representation
       ↓
ODE/annihilator representation
       ↓
rigorous numerical evaluator
```

This is a direct application of Certified Escalation, but over **representation languages** rather than numeric precision.

## 9. Proposed target-language search contract

A formula-discovery query should carry a requested representation class:

```
FindRepresentation {
    object,
    target_language,
    allowed_extensions,
    exactness,
    complexity_budget
}
```

Possible results:

```
FOUND(representation, certificate)
NO_REPRESENTATION_IN_CLASS(certificate/theorem-scope)
UNKNOWN_IMPLEMENTATION_GAP
UNKNOWN_BUDGET
UNDECIDABLE_GENERAL_CLASS
```

This is much more precise than returning an unevaluated integral.

## 10. Search-space pruning

If a complete class decision says:

```
no elementary antiderivative
```

then every future Work Cell restricted to elementary expressions can be closed immediately.

That negative result becomes durable search knowledge.

Likewise a Kovacic result can rule out the entire Liouvillian solution grammar for the ODE.

This is a very high-value compact witness even if the final useful representation must live in a richer class.

## 11. Continuous/discrete unification hypothesis

The difference-ring and differential-field passes suggest a broader structural abstraction:

```
object + operator σ/D
       ↓
operator field/ring
       ↓
solve coboundary / linear operator equation
       ↓
new formula / telescoper / impossibility
```

where:

- `σ` may be a shift/difference operator;
- `D` may be a derivation;
- more general Ore operators may combine them.

This strongly supports keeping Ore/difference/differential operator semantics connected in the future mathematical universe.

## 12. Core law

> **A complete negative result about a representation class is a successful mathematical solution, not a failed search.**

The project should use such results to widen representation deliberately rather than burn compute on impossible grammars.

## 13. Open research

1. Differential Galois theory beyond second-order Kovacic-class equations.
2. Complete/partial solvability algorithms for higher-order linear ODEs.
3. Differentially algebraic functions beyond elementary/Liouvillian classes.
4. Certificate formats/checkers for Risch non-integrability and Kovacic non-Liouvillian results.
5. Unified Ore-operator layer covering differential and difference operators.
6. Automatic representation-language escalation after a certified non-existence result.
7. Special-function introduction as a primitive-formation mechanism: if a recurring non-elementary object appears, when should it become a named/certified new primitive?
8. Independent reimplementation/licensing study of Risch/Bronstein components versus using FriCAS/SymbolicIntegration backends.
9. Search for algebraic dependencies/constant-field structure as a standalone metaprimitive.
