# Certified Escalation Synthesis Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Question

Can the runtime automatically choose and optimize a sequence of approximate, mixed-precision, interval, arbitrary-precision, and exact stages while keeping the mathematical guarantee fixed?

The evidence says yes in important numerical fragments, provided **semantic guarantees and performance policy are separate artifacts**.

## Evidence

### Ziv-style adaptive refinement

Correct rounding of hard transcendental/function evaluations commonly follows a Ziv-style loop:

1. compute an approximation with a proven error enclosure;
2. ask whether every exact value in that enclosure rounds to the same target value;
3. if yes, return that value as correctly rounded;
4. otherwise increase working precision and repeat.

MPFR uses Ziv loops internally for difficult correctly-rounded functions and explicitly increases working precision until the approximate result can be rounded unambiguously.

Sources:
- https://www.mpfr.org/
- https://www.mpfr.org/algorithms.pdf
- https://www.mpfr.org/mpfr-4.2.0/

This establishes a general pattern:

```
approximation + rigorous enclosure
          ↓
rounding/decision cell separated?
      yes       no
       |         |
       v         v
 certified    refine
 answer       precision
```

The result remains exact/correctly rounded even though most work is approximate.

### FPTuner

FPTuner automatically assigns mixed precision (e.g. 32/64/128-bit) to operators while guaranteeing that the final roundoff error remains below a user-specified threshold over the declared input domain.

Source:
- https://github.com/soarlab/FPTuner

This proves that precision selection itself can be a constrained synthesis/optimization problem:

```
minimize execution cost / casts / energy
subject to
proved_error <= required_error
```

### Daisy

Daisy supports sound roundoff-error analysis, mixed-precision assignments, algebraic rewriting, and synthesis of approximate programs under guaranteed error bounds. It can search rewrites because real-valued equivalent expressions can have materially different finite-precision error.

Source:
- https://github.com/malyzajko/daisy/blob/master/doc/documentation.md

The important distinction is:

```
real semantic equivalence
        !=
finite-precision behavioral equivalence
```

Therefore representation/rewrite selection and precision allocation may be optimized jointly while the exact semantic target remains unchanged.

### MPFR

MPFR provides arbitrary precision floating-point operations with correct rounding and explicit rounding modes. It behaves as if the exact mathematical result were computed and then rounded to the requested target precision.

Sources:
- https://www.mpfr.org/
- https://www.mpfr.org/faq.html

This provides a strong high-precision stage before exact symbolic/rational authority when the requested result is a correctly-rounded floating value rather than a symbolic exact object.

## Architecture-changing conclusion

The project should distinguish two objects:

### 1. Authority contract

Immutable for a certified primitive/query class.

Examples:

```
- exact sign
- exact rational result
- enclosure containing true real value
- correctly rounded binary64 value
- absolute error <= 1e-12
- relative error <= 1e-9
- proof of inequality
```

### 2. Escalation policy

A replaceable/optimizable realization strategy.

Example:

```
static filter
 -> f64 interval
 -> f64 expansion
 -> MPFR 128
 -> MPFR 256
 -> exact rational/symbolic
```

The authority contract defines what counts as completion.

The escalation policy defines how cheaply completion is attempted.

Changing the policy must not change the semantic result class.

## Policy synthesis

For a fixed authority contract, candidate stages can be searched using metadata such as:

```
- supported domain
- proof/enclosure strength
- expected decisiveness
- worst-case cost
- measured hardware cost
- memory footprint
- vectorizability
- setup/cast/conversion cost
- refinement reuse
- failure/ambiguity probability
```

A policy optimizer can minimize expected cost while preserving a mandatory exact/foundational fallback when the requested result class requires it.

Conceptually:

```
minimize E[cost(policy, instance-class)]

subject to
forall inputs in domain:
    if policy returns Decisive(v),
    certificate proves required authority contract

and
    fallback/completeness obligations hold where declared
```

## Critical rule

Performance data may change **stage ordering**, but not **stage admissibility**.

A benchmark showing `f64` succeeds on 99.999% of historical cases does not permit `f64` to answer the remaining case without a certificate.

The runtime can learn:

```
try filter A before B on this hardware/problem family
```

but never:

```
trust A because it usually works
```

## Mixed precision as a compilation property

A promoted mathematical primitive may therefore compile into a heterogeneous precision graph:

```
operation 1: f32
operation 2: f64
operation 3: f32
operation 4: interval-f64
operation 5: mpfr-128 only on ambiguity
```

provided the compiler can establish the required end-to-end numerical/error contract.

This is stronger than selecting one numeric type for the whole primitive.

## Relation to primitive promotion

A discovered exact construction can gain multiple realization generations:

```
C184 semantics
  |- exact interpreter
  |- exact rational CPU
  |- filtered f64/exact fallback
  |- mixed-precision realization
  |- SIMD filtered realization
  |- GPU filtered realization
```

All share mathematical semantics.

Each realization has its own realization/error certificate.

## New Theory Profile fields

Potential fields:

```
authority_targets:
  - exact
  - correctly-rounded(binary64)
  - enclosure(abs <= epsilon)

available_filters:
  - id
  - preconditions
  - certificate_family
  - expected_success_class
  - fallback

refinement_structure:
  monotone: true/false
  reusable_intermediate_state: ...

precision_synthesis:
  supported_precisions: ...
  mixed_precision: true/false
  certified_error_model: ...
```

## Broader architectural law

> **The solver may optimize the route to truth; it may not optimize away the definition of truth.**

This law generalizes beyond floating point:

- cheap SAT filter -> SMT -> theorem proof;
- probable-prime test -> primality certificate;
- numerical root estimate -> interval root isolation -> exact algebraic certificate;
- heuristic rewrite -> equality certificate;
- approximate optimization -> dual bound/certificate -> exact rational verification.

So `Certified Escalation` should be considered a generic execution schema, not a numerical special case.

## Open research

1. Formal models for composing heterogeneous certificate-producing filters.
2. Automatic generation of static/semi-static error filters from primitive semantics.
3. Cost-sensitive policy synthesis with deterministic replay.
4. Proof that fallback ladders terminate for declared decidable domains.
5. Hardware-specific retuning without mathematical recertification of the authority contract.
6. Policy learning from prior campaigns while preserving exploration and worst-case safety.
7. Interaction with lazy exact DAGs, out-of-core computation, and cache eviction.
8. Whether an escalation policy itself can be promoted as a certified reusable metaprimitive.
