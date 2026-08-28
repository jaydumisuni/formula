# Real Quantifier Elimination Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Question

Does the project have an exact decision/normalization route for broad nonlinear polynomial problems over the real numbers, analogous to the Presburger/automatic-structure route for integer arithmetic?

Yes: the first-order theory of real closed fields admits quantifier elimination.

## Tarski-Seidenberg

The theory of real closed fields admits elimination of quantifiers. Semi-algebraic sets are exactly the sets definable by first-order formulas using polynomial equalities/inequalities over an ordered real closed field.

Source:
- https://encyclopediaofmath.org/wiki/Elimination_of_quantifiers

A quantified formula such as:

```
∃x ∀y : polynomial constraints(x,y,p)
```

can in principle be transformed to an equivalent quantifier-free Boolean combination of polynomial sign conditions in the free parameters `p`.

This provides both a decision procedure and a representation-change operation.

## Cylindrical Algebraic Decomposition

CAD is the classical practical algorithmic route for real quantifier elimination. QEPCAD implements partial CAD and returns a quantifier-free solution formula describing the corresponding semialgebraic set.

Sources:
- https://www.usna.edu/Users/cs/wcbrown/qepcad/B/WhatisQEPCAD.html
- https://www.usna.edu/Users/cs/wcbrown/qepcad/B/user/Solution.html

Modern work continues to improve CAD and related approaches such as NLSAT, cylindrical algebraic coverings, non-uniform CAD, virtual term substitution, and hybrid/poly-algorithmic methods.

Sources:
- https://arxiv.org/abs/2508.00505
- https://arxiv.org/abs/2302.06814

## Formal certification evidence

Real-closed-field quantifier elimination has been formalized in Rocq/Coq using algebraic pseudo-remainder methods, including certified real algebraic geometry infrastructure.

Source:
- https://arxiv.org/abs/1201.3731

This matters because a high-performance practical CAD/QE engine can be treated as an untrusted producer while a smaller/formal route establishes the logical equivalence of relevant outputs, or certified fragments can be used directly.

## Architecture-changing conclusion

The Theory Profile should include not only whether a problem is decidable, but **which quantifier-elimination/decision fragment it belongs to**.

Potential profile:

```
logic_fragment:
    real_closed_field_first_order

decidable:
    true

normal_form:
    quantifier_free_semialgebraic

complete_methods:
    - CAD
    - certified_RCF_QE

specialized_methods:
    - NLSAT
    - virtual_term_substitution
    - cylindrical_algebraic_covering

complexity_warning:
    doubly_exponential_general_case
```

## Authority fallback

A problem should not go directly to CAD merely because CAD is complete.

The intended route is:

```
original semialgebraic problem
        ↓
structure profiling
        ↓
symmetry / dimensional reduction / decomposition
        ↓
propagation / numerical candidates / specialized algebra
        ↓
specialized QE/NLSAT if possible
        ↓
CAD / complete RCF decision fallback
```

A complete method provides an authority floor, not a default strategy.

## Representation-change significance

Quantifier elimination itself is a mathematical compression/transformation:

```
formula with hidden/existential variables
        ↓
quantifier elimination
        ↓
formula over observable/free variables only
```

This directly resembles the project's interface extraction/black-boxing goals.

For example, internal variables of a component can sometimes be eliminated to obtain an exact semialgebraic boundary relation.

That connects:

- open-system black-boxing;
- uniform interpolation/forgetting;
- semialgebraic relation extraction;
- mathematical interface synthesis.

## Complexity drives representation search

QEPCAD guidance itself emphasizes:

- fewer variables;
- lower degrees;
- splitting independent pieces;
- eliminating variables early;
- choosing stronger/more structured formula representations where available.

This reinforces the project's central law:

> **Before invoking a complete expensive authority procedure, transform the problem into the smallest structurally equivalent problem possible.**

## Exact-real universe implication

Together with the previous research, the project now has several exact real-number authority routes:

```
polynomial equality/inequality logic
    -> RCF quantifier elimination / CAD

isolated nonlinear roots
    -> alpha theory / interval Newton / Krawczyk

rigorous numerics
    -> interval/ball arithmetic

polynomial nonnegativity/optimization
    -> Positivstellensatz / SOS hierarchies

real algebraic numbers
    -> certified root isolation / algebraic arithmetic
```

These should be interoperable through the certificate envelope rather than forced into one solver.

## Core law

> **Completeness is a property of a mathematical fragment, not a reason to use the complete algorithm first.**

## Open research

1. Practical certificate formats for CAD/QE results and independent checking.
2. NLSAT proof production and interoperability with the general certificate envelope.
3. Automatic decomposition of quantified formulas before CAD.
4. Quantifier-elimination as a general boundary/black-box extraction metaprimitive.
5. Variable-order and equational-constraint selection as search-economy problems.
6. Interaction between RCF-QE, interval methods, SOS certificates, and numerical candidate generation.
7. Whether a reduced quantifier-free result can be automatically compiled into a fast reusable decision primitive.
