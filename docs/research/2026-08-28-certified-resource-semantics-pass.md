# Research Pass — Certified Resource Semantics, Complexity Bounds, and Cost-Aware Mathematical Routing

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates whether time, memory, I/O, and related resource costs can themselves become certified mathematical metadata used by the Problem Compiler and Search Economy when selecting among valid reductions, representations, algorithms, and realizations.

The central finding is:

> **Resource consumption can be given formal semantics and, in important fragments, symbolic bounds can be inferred or mechanically proved. These bounds should be separate from empirical hardware benchmarks: certified cost envelopes describe mathematical/operational guarantees, while benchmarks characterize a particular realization on a particular machine.**

---

## 1. Automatic amortized resource analysis can infer concrete symbolic bounds

Automatic Amortized Resource Analysis (AARA) is a type-based technique for deriving concrete resource bounds using potential functions. Polynomial and exponential variants preserve compositionality and reduce inference to tractable constraint solving for supported fragments.

Sources:

https://pmc.ncbi.nlm.nih.gov/articles/PMC7788609/

https://www.raml.co/

https://arxiv.org/abs/2304.13627

### Architectural implication

A promoted executable construction may optionally carry a certified/inferred resource contract:

```text
resource_bound:
    metric: steps | allocations | stack | custom
    function_of_input_shape: B(n,...)
    bound_type: upper | lower | exact
    proof/analyzer lineage: ...
```

The compiler can use this information before execution.

---

## 2. Resource types make cost compositional

AARA associates potential with values/types so local typing rules prove a global resource bound. Research supports polynomial/exponential bounds and richer recursive data structures.

Sources:

https://edoc.ub.uni-muenchen.de/13955/

https://arxiv.org/abs/2304.13627

### Architectural implication

Mathematical primitive composition can propagate cost information similarly to semantic structure:

```text
C = B o A

semantic composition
    +
resource composition
```

where the resource algebra may include sequential sum, branch maximum/expectation, parallel work/span, or domain-specific rules.

This is more useful than attaching one benchmark number to a primitive.

---

## 3. Asymptotic complexity proofs can be machine checked

Imperative_HOL_Time extends Isabelle's imperative-program reasoning with a running-time model and supports verification of asymptotic complexity, including advanced methods such as amortized analysis and Akra-Bazzi recurrences.

Source:

https://github.com/bzhan/Imperative_HOL_Time

### Architectural implication

The certificate envelope should support claims such as:

```text
time_complexity = O(n log n)
space_complexity = O(n)
```

with theorem/proof lineage rather than treating complexity annotations as comments.

A discovered algorithm can therefore gain a certified complexity profile during primitive promotion.

---

## 4. Space semantics can survive verified compilation

CakeML has a verified space-cost semantics designed to reason about heap/stack usage and proves it sound with respect to its verified compiler and garbage collector.

Sources:

https://cakeml.org/projects.html

https://cakeml.org/oopsla20.pdf

### Architectural implication

The two-proof model can extend to resources:

```text
SEMANTIC/REFERENCE PROGRAM
    resource theorem

VERIFIED/VALIDATED REALIZATION
    theorem that compiled execution respects relevant cost model/bounds
```

This is particularly important when a client imposes a hard memory limit rather than merely preferring a faster implementation.

---

## 5. Resource analysis is itself not universally decidable

Current AARA literature explicitly acknowledges that general resource-bound inference is not computable; automated methods cover useful restricted families and can fail to derive a bound even when one exists.

Sources:

https://arxiv.org/abs/2010.16353

https://www.csd.cmu.edu/academics/doctoral/degrees-conferred/david-m-kahn

### Architectural implication

Resource Profile statuses should mirror mathematical truth discipline:

```text
PROVEN_BOUND
INFERRED_AND_CERTIFIED_BOUND
EMPIRICAL_MODEL
UNKNOWN_BOUND
NO_BOUND_IN_DECLARED_CLASS
```

Failure to infer a polynomial bound does not prove exponential behavior.

---

## 6. Different resources require different algebras

Time, peak RAM, total allocation, stack depth, external I/O, energy, parallel work, and parallel span compose differently.

### Architectural implication

There should be no generic scalar `cost` in the semantic core.

A Cost Profile may be a vector:

```text
work
span
peak_RAM
allocation_bytes
external_IO
certificate_size
verification_work
transform_cost
reconstruction_cost
```

Some components are mathematically bounded; others are empirical realization metrics.

The Search Economy applies a client/hardware utility function later.

---

## 7. Work/span should guide parallel mathematical execution

Parallel algorithms are naturally characterized by total work and critical-path/span rather than one wall-clock number.

### Architectural implication

A decomposition with:

```text
work = W
span = S
```

contains much stronger placement information than `runtime = ?`.

Ptah can later map this to available CPUs/GPUs/nodes, while this project supplies the mathematical/execution dependency structure and certified/estimated work-span properties.

This preserves the Ptah boundary.

---

## 8. Complexity is representation dependent

The same semantic mathematical problem can have radically different complexity under different representations, decompositions, or reductions.

Earlier research showed examples involving treewidth, factorized FAQ queries, modular images, and compact arithmetic circuits.

### Architectural implication

Resource claims belong to:

```text
semantic algorithm / specialization / representation
```

not merely the abstract theorem/problem class.

A representation change should trigger a new cost analysis while preserving the parent mathematical identity.

---

## 9. Certificate cost is part of real problem-solving cost

A solver can return an answer quickly but emit a certificate so enormous that independent verification dominates runtime/storage.

### Architectural implication

Route selection should include:

```text
producer_cost
certificate_generation_cost
certificate_bytes
checker_cost
```

A slightly slower solver with a compact witness may be globally cheaper and more useful for permanent primitive promotion.

This directly connects to the compact-witness research.

---

## 10. Empirical benchmarking remains valuable but is not a theorem

CryptOpt and other optimization systems demonstrate the usefulness of timing candidates on real target hardware.

Certified complexity analysis cannot predict every cache, branch, SIMD, memory-controller, or implementation effect accurately.

### Architectural implication

Primitive realizations should carry both:

```text
CERTIFIED RESOURCE ENVELOPE
    architecture-independent or model-specific bound

BENCHMARK PROFILE
    observed distribution on exact hardware/runtime/compiler
```

The first constrains truth/guarantees.

The second improves practical routing and may become stale when hardware/software changes.

---

## 11. Cost transformations should accompany certified reductions

A CertifiedReduction from problem class `A` to `B` can include a complexity transformation:

```text
size_B <= f(size_A)
parameter_B <= g(parameter_A)
encode_cost <= E(size_A)
reconstruct_cost <= R(...)
```

### Architectural implication

The capability graph can propagate upper bounds along reduction chains and reject mathematically valid routes that violate client resource limits before executing them.

This is especially valuable when multiple routes reach the same target solver class.

---

## 12. Resource contracts can become proof obligations during primitive promotion

A newly discovered algorithm may be correct but unusably expensive.

Primitive promotion can optionally require properties such as:

```text
terminates
peak memory bounded by M(n)
work bounded by W(n)
certificate checker bounded by V(n)
```

for a declared applicability class.

### Architectural implication

The permanent instruction set can distinguish:

```text
mathematically valid primitive
```

from:

```text
production-preferred primitive with certified resource profile
```

without discarding slow but mathematically valuable constructions.

---

## 13. Current certified-resource hypothesis

```text
SEMANTIC / EXECUTABLE CONSTRUCTION
    -> derive/infer resource semantics where supported
    -> independent proof/certification of bounds
    -> attach Cost Profile

REALIZATION
    -> hardware-specific benchmark profile

ROUTE / REDUCTION GRAPH
    -> compose semantic resource bounds
    -> estimate empirical performance
    -> apply client resource policy
    -> select route

EXECUTION
    -> measure actual usage
    -> update ephemeral/empirical profile
    -> never rewrite proved bound from benchmark evidence
```

This gives the mathematical compiler enough information to optimize *which mathematics to execute*, not merely how to compile a chosen algorithm.

---

## 14. New research obligations

1. Study resource-aware type systems for parallel work/span, I/O, and memory rather than only sequential steps.
2. Investigate certificate formats for asymptotic and concrete resource bounds.
3. Study compositional cost algebras over AND/OR/fixed-point/factorized goal graphs.
4. Connect parameterized-complexity/kernel bounds to general Cost Profiles.
5. Investigate automatic cost analysis of generated/synthesized mathematical programs before promotion.
6. Study resource-bound transport through specialization, reduction, and theory morphisms.
7. Investigate cost-aware equality-saturation extraction with certified semantic equivalence.
8. Determine how hardware benchmark profiles are versioned/invalidated separately from mathematics.
9. Study proof/certificate generation cost as a first-class optimization objective.
10. Define First-Light tests where the system must choose between several mathematically valid routes using certified resource constraints.
