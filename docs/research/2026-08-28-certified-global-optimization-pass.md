# Certified Global Optimization Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Question

Can global optimization be handled in the same architecture as nonlinear roots and exact filtering: fast approximate/heuristic search for good candidates, plus independent mathematical machinery proving bounds and eventual optimality?

Yes.

Optimization naturally supports **two-sided certified progress**:

```
incumbent / feasible value U
certified lower bound L

L <= global optimum <= U
```

The gap `U - L` is then mathematically meaningful progress.

## Exact MIP evidence

SCIP 10 provides a numerically exact solving mode for mixed-integer linear programs using rational, extended-precision, and safe floating-point computation. It can emit proof certificates that are checked by VIPR or a formally verified CakeML checker.

Sources:
- https://www.scipopt.org/doc-10.0.0/html/EXACT.php
- https://www.scipopt.org/doc/html/FAQ.php
- https://github.com/scipopt/vipr

VIPR verifies mixed-integer programming results using exact rational arithmetic. The certificate format supports checking, compression/tightening, and completion of incomplete derivations.

This establishes:

```
industrial branch-and-cut search
        ↓
certificate/proof log
        ↓
exact rational independent verification
```

The search strategy does not need to become part of the trusted core.

## Polynomial/global optimization evidence

Lasserre/SOS hierarchies replace difficult nonconvex polynomial optimization problems by a sequence of semidefinite/conic relaxations. Under appropriate assumptions, the hierarchy converges toward the global optimum, and positivity certificates (sum-of-squares/Positivstellensatz-style) witness lower bounds.

Sources:
- https://arxiv.org/abs/2111.04610
- https://optimization-online.org/2014/11/4680/
- https://arxiv.org/abs/1911.11428

This gives another progress ladder:

```
relaxation level r
      ↓
certified bound L_r
      ↓
stronger level r+1
      ↓
certified bound L_(r+1) >= L_r
```

when the hierarchy/profile guarantees the relevant monotonicity/convergence.

## Architecture-changing conclusion

Optimization should not return a single opaque status such as:

```
OPTIMAL
```

until both sides are established.

The mathematical result object should track independently:

```
feasible witness x
objective value f(x)
certified lower/upper bound
optimality gap
bound certificate
scope/domain
proof of feasibility
proof of optimality, if gap closed
```

## Candidate solver architecture

```
optimization problem
      ↓
structure/decomposition/profile
      ↓
┌────────────────────────────┐
│ candidate/incumbent search │
│ local optimization         │
│ heuristics                 │
│ numerical methods          │
│ models/AI optional         │
└─────────────┬──────────────┘
              │ feasible candidates
              v
          best known U

parallel:

┌────────────────────────────┐
│ bound/certificate engines  │
│ LP/MIP relaxations         │
│ SOS/SDP hierarchy          │
│ interval bounds            │
│ branch-and-bound proofs    │
└─────────────┬──────────────┘
              │ certified bounds
              v
          best proven L

root progress:
    gap = U-L
```

This directly links mathematical semantics to campaign scheduling.

## Certified progress

Optimization gives a domain where progress can be numerically ordered without becoming heuristic:

```
L0 <= L1 <= L2 <= optimum <= U2 <= U1 <= U0
```

when each bound transition is certified.

The scheduler can therefore prioritize work by expected reduction of a proven gap, not merely historical runtime.

## Exactness classes

The project should distinguish:

```
FEASIBLE_CANDIDATE
LOCALLY_OPTIMAL
BEST_KNOWN
BOUND_CERTIFIED
EPSILON_GLOBAL_OPTIMAL(epsilon)
EXACT_GLOBAL_OPTIMAL
INFEASIBLE_CERTIFIED
UNBOUNDED_CERTIFIED
```

These are mathematically different claims.

## Heuristics remain useful

A heuristic that finds a better feasible point can sharply improve `U` even though it has no proof authority.

A relaxation/proof cell can improve `L` independently.

So untrusted and trusted work cooperate cleanly:

```
heuristic discovery improves upper bound
certificate search improves lower bound
meeting bounds proves optimum
```

## Reuse as a primitive

A solved/structured family may later yield a specialized optimization primitive with:

- domain assumptions;
- exact reduction to a known class;
- certified bound method;
- incumbent-generation implementation;
- reconstruction method;
- optimality certificate route;
- native fast realization.

The next problem in the family then starts with a much stronger route.

## Core law

> **Optimization progress is not 'solver confidence'; it is the narrowing of a certified feasible/optimality interval.**

## Open research

1. Exact/certified semidefinite programming and rational recovery of SOS certificates.
2. Automatic selection between branch-and-bound, convexification, SOS, interval methods, dynamic programming, and certified reductions.
3. Certificate-preserving presolve and cutting-plane transformations.
4. How to carry optimality certificates back through chained problem reductions.
5. How to certify approximate optimization for continuous/non-polynomial functions.
6. Whether the search scheduler can use marginal certified gap reduction per compute as a general value-of-computation signal.
7. How discovered symmetries/invariants/decompositions alter bound strength automatically.
