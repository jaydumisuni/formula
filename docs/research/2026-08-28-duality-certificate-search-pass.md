# Duality and Certificate Search Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this matters

Many problems are easier to certify, bound, or reason about in a **dual** representation rather than in the original/primal representation.

The project should therefore treat `construct_dual` as a potential metaprimitive family, with explicit structural preconditions and result-transport contracts.

## Optimization evidence

Conic optimization has a mechanically defined primal/dual relationship. Current `Dualization.jl` can automatically formulate the dual of MathOptInterface/JuMP conic models and can wrap solvers so the dual problem is solved instead of the primal.

Sources:
- https://jump.dev/Dualization.jl/dev/
- https://jump.dev/JuMP.jl/stable/tutorials/conic/dualization/
- https://jump.dev/MathOptInterface.jl/stable/MathOptInterface.pdf

MathOptInterface explicitly models primal-dual pairs and dual certificates.

Source:
- https://jump.dev/MathOptInterface.jl/stable/manual/solutions/

## Disciplined convex structure

CVX/DCP systems show that convexity can be established compositionally from a restricted algebra of known convex/concave/affine atoms. Problems satisfying the rules can be automatically converted to solver forms and carry dual variables/certificates.

Sources:
- https://cvxr.com/cvx/doc/intro.html
- https://cvxr.com/cvx/doc/dcp.html
- https://web.cvxr.com/cvx/doc/basics.html

DCP is intentionally sufficient but not complete: a problem may be convex yet fail the rules until represented differently.

This reinforces representation search:

```
problem not recognized as dualizable/convex
        ↓
rewrite/change representation
        ↓
DCP-recognizable form
        ↓
automatic primal/dual construction
```

## Duality as certificate generation

Examples already present in the broader research:

```
LP/MIP primal feasibility/optimization
    ↔ dual bounds / Farkas certificates

convex optimization
    ↔ conic dual variables / optimality gap

SOS/polynomial optimization
    ↔ positivity certificates / moment duals
```

The dual side often contains exactly the finite witness required to certify impossibility or optimality on the primal side.

## Architecture-changing conclusion

The project should not model a dual as merely another problem instance.

It should model a certified relationship:

```
Dualization D:
    ProblemClass P
      -> ProblemClass P*

with contracts:
    feasibility correspondence
    objective bound direction
    witness reconstruction
    strong-duality conditions
    gap semantics
    infeasibility certificate mapping
```

Without those contracts, a generated 'dual' has no general truth authority.

## Weak versus strong duality

This distinction is fundamental.

A dual feasible point may always provide a valid bound under weak-duality conditions, while equality of primal and dual optima may require stronger assumptions/constraint qualifications.

Therefore the Theory Profile must record separately:

```
weak_duality: proven
strong_duality: conditional(condition_id)
attainment: ...
constraint_qualification: ...
```

The solver must not convert a dual bound into an optimality proof unless the required strong-duality/gap conditions are established.

## Search-economy implication

The scheduler can launch primal and dual campaigns concurrently:

```
primal search
    -> improves feasible incumbent U

dual search
    -> improves certified bound L

U-L
    -> certified progress
```

Whichever side is cheaper for the current structure can receive more compute.

This also makes duality a representation-search target rather than merely a solver option.

## General metaprimitive shape

Potential semantic operation:

```
dualize(X, theory/profile)
    -> {
         dual_object,
         transport_contract,
         assumptions,
         certificate_routes
       }
```

A promoted dualization rule itself needs certification.

## Cross-domain caution

There is no single universal notion of duality.

Examples include:

- vector-space duals;
- convex/conic optimization duals;
- Pontryagin/Fourier character duality;
- planar graph duality;
- categorical opposite/dual constructions;
- algebraic dual modules/spaces.

These have different laws and transport properties.

The project should therefore treat `DUALITY` as a metaprimitive **family indexed by mathematical structure**, not one magic operator.

## Core law

> **Search both the object and the certified dual viewpoint when the Theory Profile proves what information dualization preserves.**

## Open research

1. Automatic discovery that a problem admits a useful dual representation.
2. Certified dualization transformations beyond convex/conic optimization.
3. How duals compose with reductions and theory morphisms.
4. Automatic recovery of primal witnesses from dual/certificate results.
5. Duality-gap progress as a work-cell scheduling signal.
6. Whether theory-level dualities can transport entire primitive libraries similarly to theory morphisms.
7. How dual representations interact with black-boxing and boundary semantics.
