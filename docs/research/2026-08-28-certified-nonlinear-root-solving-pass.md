# Certified Nonlinear Root-Solving Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Question

Can the project use extremely fast numerical/nonlinear search to locate candidate solutions while preserving a strict deterministic truth boundary?

Yes. Numerical algebraic geometry and interval methods provide direct examples of:

```
approximate candidate
      ↓
finite mathematical certificate/test
      ↓
proved root / unique root / no root
```

The search engine and the authority mechanism can therefore remain separate.

## Smale alpha theory / alphaCertified

Smale's alpha theory gives computable criteria under which an approximate point is guaranteed to lie in the quadratic-convergence basin of a unique exact solution of a polynomial system.

`alphaCertified` implements this approach using exact rational arithmetic and arbitrary-precision floating-point arithmetic to certify candidate solutions produced numerically. It can also certify whether a candidate corresponds to a real solution for real systems.

Source:
- https://arxiv.org/abs/1011.1091

Architecture pattern:

```
homotopy/Newton/numerical solver
        ↓
approximate zero z
        ↓
alpha-theory bounds
        ↓
CERTIFIED: z represents exact solution ξ
```

The numerical solver does not become trusted merely because it converged.

## Krawczyk and interval certification

Krawczyk operators and interval-Newton methods use interval enclosures of the nonlinear system/Jacobian to establish rigorous statements over whole regions.

A sufficiently strong interval inclusion can prove:

- existence of a solution;
- uniqueness of a solution in the box;
- exclusion/no solution in a box;
- progressively tighter enclosures.

Sources:
- https://arxiv.org/abs/2011.05000
- https://interval.louisiana.edu/GLOBSOL/whatisop/node9.html
- https://interval.louisiana.edu/GLOBSOL/whatisop/node10.html

`HomotopyContinuation.jl` added interval/Krawczyk-based certification precisely so approximate roots found numerically can be upgraded into mathematically certified roots.

## Certified homotopy tracking

Certified homotopy-continuation algorithms can rigorously track a solution path from a start system to a target system while retaining correctness guarantees.

Source:
- https://arxiv.org/abs/0912.0920

This matters because the expensive discovery path can remain numerical/continuation-based while proof obligations are attached to each accepted path/result.

## Architecture-changing conclusion

The project should support an explicit distinction between:

```
CANDIDATE POINT
APPROXIMATE ROOT
CERTIFIED ROOT ENCLOSURE
CERTIFIED UNIQUE ROOT
CERTIFIED ROOT FAMILY / COMPONENT
```

A floating-point tuple is never promoted merely because residuals are small.

## General nonlinear solving pipeline

Potential generic pipeline:

```
problem F(x)=0
       ↓
cheap representation / numerical preconditioner
       ↓
parallel numerical search
       ↓
candidate roots
       ↓
cluster / deduplicate candidates
       ↓
interval / alpha / symbolic certificate routing
       ↓
  ┌───────────────┬───────────────┐
  ↓               ↓               ↓
certified       rejected       unresolved
root            candidate      region
  │                               │
  └───────────── refine/search ────┘
```

This fits the broader certified-escalation architecture.

## Global-versus-local distinction

Local certification of found roots does not automatically prove that **all** roots have been found.

The system must track separately:

```
root R exists
root R is unique in box B
no other roots in region D
all roots in declared domain have been enumerated
```

These are different mathematical claims and require different certificates.

## Candidate work-cell strategy

A nonlinear campaign can use heterogeneous cells:

```
- homotopy continuation cells
- Newton/Quasi-Newton cells
- interval branch-and-prune cells
- algebraic decomposition cells
- Gröbner/resultant cells
- alpha-certificate cells
- Krawczyk/interval-Newton cells
```

Numerical cells maximize discovery speed.

Certificate cells provide authority.

## Ordinary-hardware implication

Most candidate localization can use optimized floating-point linear algebra.

Expensive exact/interval work is focused only on:

- promising candidates;
- singular/ill-conditioned cases;
- unresolved boxes;
- completeness proofs.

Thus exact nonlinear mathematics does not require every exploratory iteration to use exact arithmetic.

## Relation to representation search

A failure to certify may itself trigger representation changes:

```
ill-conditioned coordinates
    ↓
rescale / nondimensionalize
    ↓
change basis
    ↓
precondition
    ↓
algebraic deflation for singular root
    ↓
retry certificate
```

So certification failure becomes search information, not merely an error.

## Core law

> **Numerical convergence proposes mathematical structure; finite certification establishes what structure actually exists.**

## Open research

1. Certification of singular and positive-dimensional solution sets.
2. Completeness certificates for enumerating all roots in a declared domain.
3. Automatic choice between alpha-theory, Krawczyk, interval Newton, symbolic elimination, and exact algebraic methods.
4. Deflation/preconditioning as automatically discovered certified transformations.
5. Efficient root identity across multiple numerical/certified representations.
6. How nonlinear candidate search feeds the general work-cell search economy and certified-progress metrics.
