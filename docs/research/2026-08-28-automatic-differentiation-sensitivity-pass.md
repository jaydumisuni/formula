# Research Pass — Automatic Differentiation, Sensitivity, and Derived Mathematical Programs

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates automatic differentiation (AD) as a mathematical metaprimitive for a system whose formulas may naturally be executable programs.

The central finding is:

> **For constructions with the required differentiable semantics, differentiation can be a structure-preserving program transformation that automatically creates new executable mathematics: derivatives, Jacobian actions, adjoints, gradients, and sensitivity programs.**

This is substantially broader than a machine-learning feature.

---

## 1. AD transforms programs into derivative programs

Automatic differentiation applies the chain rule through a program's computational structure rather than using numerical finite differences or purely symbolic expression manipulation.

Enzyme performs AD at LLVM/MLIR level and can differentiate code originating from multiple source languages.

Sources:

https://github.com/EnzymeAD/Enzyme

https://enzyme.mit.edu/

https://proceedings.neurips.cc/paper_files/paper/2020/hash/9332c513ef44b682e9347822c2e457ac-Abstract.html

### Architectural implication

A certified mathematical construction `C` may support metaprimitives such as:

```text
differentiate_forward(C)
differentiate_reverse(C)
jacobian_action(C)
adjoint(C)
```

which produce new constructions with lineage back to `C`.

---

## 2. AD has semantic correctness theories, not only implementation folklore

Research gives denotational/categorical correctness arguments for forward and reverse AD transformations, including higher-order functional languages and recursion-related settings.

Sources:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7984537/

https://pmc.ncbi.nlm.nih.gov/articles/PMC7788619/

https://research-portal.uu.nl/en/publications/automatic-differentiation-for-ml-family-languages-correctness-via-2/

### Architectural implication

The project should treat AD legality as a Theory Profile/property question:

```text
differentiability_semantics: established / partial / unavailable
AD_transformation_family: ...
proof_route: ...
```

A generated derivative program becomes mathematical authority only under the applicable semantic theorem/certificate.

---

## 3. Differentiation is a primitive-growth mechanism

Given:

```text
semantic construction C
```

AD can automatically derive:

```text
C'
Jacobian(C)
vector-Jacobian product
Jacobian-vector product
```

without discovering each derivative independently.

### Architectural implication

Some primitive families should be **closed under certified metatransformations**.

Promoting one new differentiable construction can therefore automatically unlock an entire derivative/sensitivity capability family.

This is multiplicative primitive growth.

---

## 4. Forward versus reverse mode is an execution-plan choice

Forward- and reverse-mode AD compute related mathematical derivatives but have different cost profiles depending on input/output dimensions and program structure.

Enzyme demonstrates high-performance reverse-mode generation even for optimized GPU/LLVM code.

Source:

https://github.com/EnzymeAD/Enzyme

### Architectural implication

The derivative's semantic identity should remain separate from realization strategy:

```text
DERIVATIVE SEMANTICS
    -> forward realization
    -> reverse/adjoint realization
    -> sparse realization
    -> mixed mode
```

The compiler/search economy chooses the cheapest valid route.

---

## 5. Fixed-point and implicit constructions can be differentiated without naïve unrolling

Many solvers/optimization routines define their result implicitly through a fixed-point equation. Implicit differentiation and fixed-point AD can derive sensitivities of the solution map without retaining every iteration.

Sources:

https://implicit-layers-tutorial.org/implicit_functions/

https://arxiv.org/abs/2208.03107

### Architectural implication

The project's fixed-point semantics can connect directly to sensitivity analysis:

```text
z = F(z, θ)
```

may yield a certified/derived sensitivity relation for:

```text
dz/dθ
```

without treating the iterative solver trace as the mathematical definition.

This is important for optimization, inverse problems, control, numerical solvers, and Wolf-Coin-style quantitative systems.

---

## 6. Sensitivity can guide inversion and representation search

Derivatives/Jacobians expose local structure:

```text
sensitivity
rank
null directions
conditioning
local invertibility
```

### Architectural implication

A Work Cell can use derived sensitivity information to decide:

```text
which variable to solve for
which dimensions can be eliminated
where inversion is ill-conditioned
which representation/basis may be useful
where branch boundaries occur
```

AD therefore feeds the search compiler, not merely numerical optimization.

---

## 7. Nondifferentiability must remain explicit

AD is not universally valid across arbitrary branching, discontinuity, integer operations, max/min boundaries, or nonsmooth structures.

Research on machine-representable neural-network parameters explicitly studies points where differentiability fails and generalized derivative behavior.

Source:

https://proceedings.mlr.press/v202/lee23p.html

### Architectural implication

The derivative result taxonomy should distinguish:

```text
CLASSICAL_DERIVATIVE
PIECEWISE_DERIVATIVE
SUBDERIVATIVE / GENERALIZED_DERIVATIVE
UNDEFINED
UNKNOWN
```

The engine must never silently turn an AD-produced number at a nonsmooth point into a classical derivative claim.

---

## 8. AD over machine realizations and AD over exact semantics are different claims

Differentiating optimized finite-precision code can produce a useful machine gradient, while differentiating the ideal real-valued construction establishes a different mathematical object.

### Architectural implication

The project should track:

```text
semantic_derivative_digest
machine_derivative_realization_digest
finite_precision_error_contract
```

separately.

This mirrors the numerical-realization separation discovered in the previous pass.

---

## 9. Derivative structures can themselves be compressed and specialized

Jacobians are frequently sparse/structured. A generic dense derivative representation can be catastrophically wasteful.

### Architectural implication

Structure discovery should search derivative constructions for:

```text
sparsity
block structure
symmetry
low rank
Kronecker/tensor structure
repeated subexpressions
```

and compile specialized derivative programs.

Again, mathematical structure drives native performance before GPU power is considered.

---

## 10. AD can participate in primitive certification

For differentiable relations, derivatives can provide independent mathematical checks such as sensitivity bounds, monotonicity regions, or local invertibility conditions.

They do not by themselves prove the original theorem, but they can generate useful proof/search obligations.

### Architectural implication

AD belongs in the **metaprimitive/search fabric**, with derived artifacts entering the same falsification/certification pipeline as other constructions.

---

## 11. Current differentiation hypothesis

```text
CERTIFIED CONSTRUCTION C
    -> Theory Profile establishes differentiability fragment
    -> derive semantic derivative construction D(C)
    -> certify/justify AD transform under fragment theorem
    -> analyze derivative structure
    -> specialize forward/reverse/sparse realization
    -> numerical realization certification
    -> expose D(C) as reusable derived primitive
```

A single mathematical discovery can therefore unlock many additional problem-solving operations automatically.

---

## 12. New research obligations

1. Study AD over relational rather than purely functional semantics.
2. Investigate differentiating implicit constraints and solution sets, not only functions.
3. Study sparse Jacobian/Hessian discovery and automatic specialized realization.
4. Investigate certified AD through branches, piecewise functions, and nonsmooth mathematics.
5. Study generalized derivatives/subdifferentials as first-class mathematical structures.
6. Connect conditioning analysis to AD-derived sensitivity measures.
7. Investigate derivative transport through theory morphisms and representation changes.
8. Study differentiation of probabilistic/stochastic constructions and expectation operators.
9. Determine how derivative programs interact with exact real / interval enclosures.
10. Investigate whether repeated use of generic AD can be distilled into domain-specific closed-form derivative primitives.
