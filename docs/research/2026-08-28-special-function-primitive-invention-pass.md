# Special-Function and New-Primitive Invention Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The project may repeatedly encounter a mathematically meaningful object that:

- has no representation in the current preferred vocabulary;
- occurs across many solved problems;
- has a compact certified recurrence, differential equation, integral representation, transform, or relation space;
- has efficient evaluation algorithms once treated as its own object.

At that point the right action may not be to keep expanding the object's long definition.

The project may need to **invent and promote a new mathematical primitive**.

This pass asks what information is required for such a primitive to have an unambiguous mathematical identity rather than being merely a shorthand name.

## 1. Existing special functions are precedent for primitive invention

Historically, functions such as:

- Bessel functions;
- Airy functions;
- Gamma/zeta functions;
- hypergeometric functions;
- elliptic functions;

became standard mathematical objects because recurring mathematical structures made them worth treating as primitives with rich independent theory and algorithms.

For the project, the analogous process can be automated:

```
recurring certified mathematical object
        ↓
compress semantic characterization
        ↓
prove uniqueness/well-definedness
        ↓
assign structural primitive identity
        ↓
compile evaluators/transforms/derivatives/etc.
        ↓
new primitive available to future search
```

The novelty is that promotion need not wait for a human to give the object a conventional notation or name.

## 2. Differential equation alone is generally not enough

DLMF's Bessel-function definitions illustrate the issue clearly.

Bessel's differential equation has a multi-dimensional solution space. `J_ν`, `Y_ν`, and the Hankel functions are distinguished through specific series/normalization/branch/asymptotic conventions.

Source:
- https://dlmf.nist.gov/10.2

For non-integer order, branch points and branch cuts are part of the function's identity. DLMF explicitly specifies principal branches and cuts.

Therefore:

```
L(f)=0
```

usually defines a **solution space**, not one unique primitive.

A primitive must include enough selectors to identify exactly one semantic object (or deliberately define the solution space itself as the object).

## 3. Airy functions show operator + standard-solution selection

Airy functions are standard solutions of Airy's differential equation, but the equation alone again does not distinguish `Ai` from `Bi` or other linear combinations.

Source:
- https://dlmf.nist.gov/9.2

DLMF computation methods can integrate the defining ODE using appropriate initial values and stability-aware paths.

Source:
- https://dlmf.nist.gov/9.17

This supports a primitive package of the form:

```
defining operator / relation
+ initial / boundary / asymptotic conditions
+ analytic-domain semantics
+ evaluation strategies
```

rather than one enormous closed-form expression.

## 4. FunGrim: symbols require globally fixed semantics

FunGrim's design requires mathematical symbols/operators to have an unambiguous interpretation independent of context. Its semantic rules explicitly fix branch cuts/principal values for multivalued complex functions and require assumptions for rewrites.

Source:
- https://pmc.ncbi.nlm.nih.gov/articles/PMC7340917/

Current FunGrim data uses permanent symbol/entry IDs and represents formulas semantically with assumptions.

Source:
- https://fungrim.org/

This gives a strong donor rule:

> **A promoted primitive's internal identity must be semantic and permanent; human notation/name is metadata.**

The project can therefore invent primitive `P-<digest>` long before a person chooses to call it anything.

## 5. Primitive semantic selector

A candidate new function primitive may require some combination of:

```
parameter space
input domain / Riemann surface / branch domain
codomain
annihilating operator or defining relation
normalization
initial values
boundary values
asymptotic behavior
branch cuts / monodromy convention
singularity behavior
analytic continuation rule
recurrence relations
symmetry/functional equations
```

The exact selector depends on the structure class.

Potential semantic object:

```
PrimitiveDefinition {
    structural_id,
    structure_class,
    parameter_domains,
    relation_space,
    selector_conditions,
    uniqueness_certificate,
    branch_and_singularity_semantics,
    dependency_primitives,
    foundation/theory,
    authority_certificate
}
```

The primitive is admitted only when these data determine the declared object unambiguously in its Theory Profile.

## 6. Primitive is not tied to one executable formula

Once semantic identity is fixed, there may be many realizations:

```
new primitive P
   |- recurrence evaluator
   |- ODE integrator
   |- power series
   |- asymptotic expansion
   |- continued fraction
   |- integral transform
   |- hypergeometric/Meijer-G reduction
   |- rigorous ball-arithmetic evaluator
   |- CPU SIMD kernel
   |- GPU/batched kernel
```

Each is a realization of the same mathematical primitive under its declared domain/accuracy contract.

This is identical to the project's mathematical-identity versus realization-identity constitution.

## 7. Representation in broader universal families

The Meijer G-function is a useful precedent for using a broad function family as a common representation. DLMF states that generalized hypergeometric functions and many standard special functions (including Airy/Bessel/Legendre-related cases) can be represented as special cases of Meijer G.

Source:
- https://dlmf.nist.gov/16.18

This creates two possible outcomes when a recurring object appears:

### Existing-family absorption

```
new recurring object
      ↓
prove it is instance/reduction of existing general primitive family G
      ↓
no new fundamental primitive needed
```

### Genuine primitive promotion

```
no useful existing-family representation
or representation destroys critical structure/performance
      ↓
promote new semantic primitive P
```

The project should search existing theory morphisms/reductions before creating new vocabulary.

## 8. General primitive-promotion criteria

A candidate object should not become a primitive simply because it appears twice.

Potential criteria:

### Semantic maturity
- exact definition/selector established;
- domain/branch semantics established;
- uniqueness/well-definedness certified;
- dependencies explicit.

### Reuse
- appears across multiple independent problems;
- enables significant compression;
- exposes useful closure/transformation laws;
- reduces future search cost.

### Executability
- at least one certified evaluator/solver exists;
- result can be independently checked or rigorously enclosed;
- computational complexity is characterized enough for routing.

### Stability
- identity is not tied to an accidental implementation;
- changing realization does not redefine the primitive.

### Novelty relative to current vocabulary
- no existing primitive/reduction provides the same semantics at acceptable structural cost;
- or a new abstraction genuinely compresses an existing recurring family.

Promotion itself can therefore be a certifiable optimization decision under a declared primitive vocabulary/cost model.

## 9. Primitive creation is mathematical compression

Suppose certified constructions:

```
C1, C2, ... C1000
```

all repeatedly contain an object with a common semantic characterization `P`.

Before promotion:

```
C_i = long expression containing definition D
```

After promotion:

```
P := certified semantic object defined by D/relations
C_i = compact expression using P
```

This reduces:

- representation size;
- search grammar complexity;
- theorem/rewrite duplication;
- repeated proof/evaluation work.

But unlike a mere macro, `P` gains independent mathematical properties and implementations.

## 10. Theory growth around a new primitive

Once admitted, work cells can explore the primitive itself:

```
derivative / recurrence
integral transform
zeros / poles
symmetries
functional equations
asymptotics
special values
composition laws
inverses
connections to existing primitives
```

Each certified discovery expands the capability graph around `P`.

This is how the system can create a mathematical object first and then progressively build its theory.

## 11. Human naming is deliberately late

The internal primitive can use a structural/content identity:

```
primitive://<semantic-digest>
```

or an opaque stable generated ID.

Human-facing notation can be introduced later without changing mathematical identity.

This directly supports the user's decision not to prematurely name the project itself: names are projections for people, not mathematical semantics.

## 12. Rigorous numerical realization

Arb/FLINT provides precedent for high-performance rigorous evaluation of many special functions using arbitrary-precision ball arithmetic.

Source:
- https://www.arblib.org/

A new primitive could similarly compile a rigorous numerical evaluator even if no simple closed form exists.

Thus:

```
no elementary formula
```

does not block executable primitive promotion.

A relation/ODE/integral definition plus certified evaluator can be fully legitimate mathematics.

## 13. Candidate versus standard primitive

The system should preserve stages:

```
CANDIDATE_OBJECT
      ↓
CERTIFIED_DEFINED_OBJECT
      ↓
LOCAL_PRIMITIVE (used inside one campaign/theory)
      ↓
PROMOTED_PRIMITIVE (general reusable vocabulary)
      ↓
FOUNDATIONAL/CORE_PRIMITIVE (rare; very mature)
```

Promotion is therefore reversible at the *vocabulary preference* level without deleting the underlying mathematics.

The semantic object remains preserved even if a later broader primitive makes the shorthand obsolete.

## 14. Primitive deprecation without mathematical deletion

If later the system discovers:

```
P = special_case(Q)
```

then `P` need not disappear.

It can become:

```
semantic alias / specialized view / optimized realization
```

while its exact historical identity and results remain valid.

This avoids rewriting the mathematical ledger every time abstraction improves.

## 15. Strong architectural conclusion

The self-expanding primitive set should grow not only by discovering faster algorithms, but by discovering **new semantic mathematical objects worth naming internally**.

The loop becomes:

```
problem campaigns
      ↓
recurring nontrivial object/relation
      ↓
prove semantic selector/uniqueness
      ↓
primitive promotion
      ↓
compile evaluator(s)
      ↓
build theory around primitive
      ↓
primitive becomes vocabulary for later discovery
```

This is one of the strongest mechanisms for turning solved mathematics into genuinely expanded problem-solving capability.

## 16. Core law

> **A new primitive is justified when a certified semantic object has become more useful as a reusable mathematical atom than as a repeatedly expanded construction.**

Its human name is optional.

Its semantics are not.

## 17. Open research

1. Formal primitive-selection/uniqueness contracts for functions, sequences, transforms, relations, operators, and state systems.
2. Branch/analytic-continuation semantics for automatically invented complex functions.
3. Automatic detection that recurring constructions warrant primitive promotion.
4. Search for absorption into existing general families (hypergeometric/Meijer-G/Ore/difference-algebra classes) before vocabulary growth.
5. Automatic creation of rigorous numerical evaluators from ODE/recurrence/integral definitions.
6. Primitive-theory bootstrapping: automatically derive first identities, derivatives, transforms, asymptotics, zeros, and special values after promotion.
7. Stable semantic ID/alias system when a later abstraction subsumes an earlier primitive.
8. How proof repair works when primitive definitions are reorganized but remain equivalent.
9. Criteria for promotion from local/campaign primitive to universal reusable primitive.
10. Whether contextual minimal-state or algebraic-compression metrics can quantify when primitive introduction genuinely reduces future mathematical search.
