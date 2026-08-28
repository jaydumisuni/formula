# D4 — Native Execution Architecture

**Date:** 2026-08-28  
**Status:** FROZEN DESIGN MILESTONE D4  
**Repository name:** temporary only; not product identity  
**Authority:** D1/D1A define semantics and trust; D2 defines Core authority services; D3 defines compiler/campaign semantics.

D4 defines how admitted mathematics and D3 execution obligations become efficient executable realizations while preserving the mathematical semantics and authority contract.

The performance goal is not “always use GPU” or “rewrite all mathematics in one language.” It is:

> **Keep mathematical meaning rich during discovery/compilation, then erase/specialize everything not needed in the hot path and execute through the cheapest realization whose result can satisfy the requested authority.**

---

## 1. Realization is not mathematics

D4 preserves the D1 two-proof model:

```text
P_M: semantic mathematics is valid
P_R: concrete realization implements/refines that mathematics
```

A mathematical Entity/Relation may have many Realizations. Changing implementation does not change semantic identity unless the declared semantics change.

A compiler/optimizer may be untrusted. Its output becomes admitted only through realization validation.

---

## 2. Multi-level mathematical lowering

D4 defines four conceptual lowering levels. Their exact syntax/serialization is not frozen.

### L0 — Semantic Relation IR (SIR)

D1/D2 identities and exact relational semantics:

```text
Entities
Relations
World
parents/structures
observer
exactness/partiality/stochastic class
Judgements
```

SIR is authority-oriented and may be graph/hypergraph structured. It is not designed for direct high-performance execution.

### L1 — Specialized Mathematical Plan IR (MPIR)

D3 fixes enough context to specialize semantics:

```text
query direction
known constants
World assumptions
parent/domain instances
shapes/dimensions
observer
structure witnesses
representation choice
algorithm/reduction route
numeric semantics
```

Generic relation machinery is eliminated where the query fixes a direction.

### L2 — Domain Kernel IR (KIR)

Mathematical operations become implementation-oriented kernels while retaining exact semantic contracts.

Examples:

```text
machine-word modular arithmetic
big-integer/rational kernels
sparse polynomial operations
GF(2) bit operations
BLAS-style kernels
interval/ball kernels
BDD/automata operations
SAT/SMT adapter calls
streaming term transforms
```

KIR operations declare overflow, rounding, exactness, aliasing, domain preconditions, and fallback semantics explicitly.

### L3 — Machine/Backend IR

KIR lowers to one or more targets:

```text
native Rust/C/C++
LLVM/MLIR
domain-native libraries
SIMD intrinsics
Jasmin/verified generator
GPU kernel
specialist process invocation
```

No backend is constitutional.

---

## 3. Lowering proof obligations

Every semantic-changing lowering edge must be classified as one of:

```text
EXACT_EQUIVALENCE
REFINEMENT
OBSERVER_EQUIVALENCE
RIGOROUS_APPROXIMATION
PROBABILISTIC_REALIZATION
CANDIDATE_ONLY
```

An admitted execution path contains enough evidence to compose those relations back to the D1 semantic target.

The optimizer itself need not be verified if each resulting realization is independently translation-validated or exhaustively/certificationally checked for the declared domain.

---

## 4. Specialization law

D4 specializes aggressively on all authority-fixed information that need not survive runtime:

```text
parent/domain
modulus
matrix dimensions
sparsity format
World assumptions
query direction
observer
constant parameters
known structure witnesses
selected representation
precision/error contract
```

This is the primary route from a generic mathematical relation to a small residual program.

Conceptually:

```text
generic semantic relation
    + fixed context
    -> partial evaluation / specialization
    -> residual executable primitive
```

Proof/type/structure metadata may be erased from runtime after validation when it no longer affects computation.

---

## 5. CPU-first execution policy

D4 freezes **CPU-first**, not CPU-only.

The Realization Planner prefers the cheapest admissible route based on:

```text
semantic applicability
proof/checker availability
input size/structure
certified/estimated work
memory traffic
parallelism
latency
hardware availability
fallback cost
```

CPU is the default because large classes of symbolic, exact, irregular, branch-heavy, proof-checking, graph, SAT-style, arbitrary-precision, and small/medium dense computations are well suited to modern CPUs.

GPU is selected only when the workload's mathematical/representation structure provides sufficient regular parallelism and the realization can satisfy the authority contract.

---

## 6. Exact execution does not imply huge exact intermediates

D4 adopts a reusable **cheap-image/reconstruction** execution schema.

For suitable mathematics:

```text
large exact problem
    -> homomorphic/modular/evaluated images
    -> independent machine-word computations
    -> CRT / rational reconstruction / interpolation / lifting
    -> exact candidate
    -> independent verification
```

This applies beyond integers to modular linear algebra, polynomial arithmetic, Gröbner methods, sparse interpolation, and other package-defined image/lift structures.

The Theory Profile declares when image computation, reconstruction uniqueness, and stopping criteria are valid.

Independent images naturally parallelize across CPU cores or future distributed workers.

---

## 7. Filtered exact execution

D4 freezes ambiguity-driven escalation:

```text
cheap approximate/filter stage
        |
        v
can this stage certify a decisive result?
     /       \
   yes       no/ambiguous
    |           |
 return       escalate
 certified     representation/precision
 result         |
                v
             exact authority path
```

A stage may return only when its own error/enclosure/filter proof establishes that the semantic answer is unambiguous.

Examples of possible ladder stages:

```text
hardware float filter
rigorous interval/ball arithmetic
higher precision
modular exact route
arbitrary precision exact arithmetic
symbolic/formal fallback
```

The exact ladder is capability-specific.

`AMBIGUOUS` is an escalation signal, not mathematical failure.

---

## 8. Precision-plan synthesis

The semantic error/authority contract and the performance plan are separate objects.

D4 permits untrusted optimization/search to choose:

- algebraically equivalent rewrites;
- mixed precisions;
- evaluation order;
- interval precision;
- fallback thresholds;
- machine-specific kernels.

The chosen plan is admissible only if its error/rounding/enclosure contract is independently established.

Thus the runtime may continuously retune how a result is obtained without changing what the result means.

---

## 9. Rigorous numerical realization

Real-valued semantics and finite-precision execution are separate.

A numerical Realization declares:

```text
exact semantic target
machine arithmetic model
rounding mode assumptions
input domain
conditioning/preconditions
error/enclosure theorem or certificate
fallback/escalation route
```

Possible authoritative outputs include:

```text
rigorous interval enclosure
certified sign
certified root existence/uniqueness
certified objective gap
correctly rounded value
```

A high-quality floating answer without an admissible semantic error contract remains candidate/empirical output.

---

## 10. Native exact arithmetic policy

D4 prefers specialized arithmetic substrates rather than reimplementing every low-level algorithm.

A package may lower to:

```text
machine-word arithmetic
GMP/MPFR/FLINT-like exact libraries
specialized finite-field kernels
bit-parallel GF(2)
verified/extracted arithmetic
```

The semantic adapter and realization evidence, not library brand, determine authority.

The native hot path should avoid arbitrary-precision allocation where mathematical structure permits machine-word modular/image computation.

---

## 11. Parallel CPU execution

D4 supports parallelism at several mathematical levels:

```text
independent decomposition children
modular/image computations
candidate-space partitions
matrix/tensor blocks
proof/certificate checking partitions where valid
streaming term batches
```

The compiler should expose mathematical independence rather than relying solely on generic thread-level parallelism.

Work stealing/task scheduling may be nondeterministic operationally, but the mathematical result and replay-bound semantic inputs remain deterministic where the capability claims deterministic semantics.

---

## 12. SIMD/vector execution

SIMD lowering is selected when KIR operations admit lane-independent or algebraically valid vectorization.

Reassociation/reordering requires explicit semantic permission. Floating-point expressions cannot be freely reassociated merely because real-number algebra would permit it.

Exact associative operations may permit more aggressive transformation.

Every vectorized realization retains the same realization-equivalence/refinement obligation as scalar code.

---

## 13. Optional GPU execution

GPU is an optional Realization family.

Suitable workloads include:

- dense regular linear algebra;
- large independent modular/image batches;
- massive regular enumeration;
- tensor operations;
- large simulation/search kernels;
- selected proof/synthesis workloads.

Unsuitable or not automatically preferred workloads include highly irregular symbolic manipulation, branch-heavy theorem/search control, small exact problems, or workloads dominated by arbitrary-precision dependencies.

GPU output never gains authority from accelerator execution alone. It returns through the same semantic certificate/realization path.

---

## 14. Incremental mathematical execution

Where a mathematical construction admits an exact/sound change operator, D4 may compile both:

```text
F(x)
```

and:

```text
DeltaF(x, DeltaX)
```

so that small input changes update prior results without complete recomputation.

Incremental realizations must declare:

```text
base-result identity
accepted change semantics
update correctness relation
invalidating change classes
fallback to full recomputation
```

Recursive/fixed-point computations may use mathematically justified incremental maintenance when available.

Caching unchanged arbitrary execution state without a semantic change contract is an optimization, not an incremental mathematical proof.

---

## 15. Out-of-core and streaming execution

D4 classifies operations by locality/streamability.

Suitable symbolic families may execute as:

```text
read chunk/term stream
apply local transformation
emit normalized/sorted runs
external merge/sort/reduce
continue without whole-object residency
```

The semantic identity of the mathematical object is independent of whether its physical realization resides in RAM, memory-mapped storage, or streamed external storage.

Out-of-core plans declare ordering/canonicalization/checkpoint requirements so spilling cannot change mathematics.

The runtime may choose recomputation over retention when recomputation is cheaper than memory pressure, provided semantic identity/evidence remain intact.

---

## 16. Lazy exact DAGs and demand-driven realization

An exact expression/construction may remain as a DAG whose cheap approximate summaries are cached while exact subexpressions are evaluated only when a downstream observer requires them.

This supports:

```text
cheap comparison/filter
    -> decisive: stop
    -> ambiguous: force exact subgraph
```

The DAG's semantic identity and dependency structure remain content-addressed; cached approximations are rebuildable realization state.

---

## 17. Partial, refining, productive, relational, and stochastic execution

D4 preserves the D1 executable semantic classes.

A Realization declares whether it is:

```text
TOTAL
PARTIAL with declared domain/termination semantics
REFINING with an information-order contract
PRODUCTIVE for infinite/coinductive outputs
RELATIONAL/NONDETERMINISTIC
STOCHASTIC with probability-law semantics
APPROXIMATE under rigorous error contract
```

A partial function timing out is not equivalent to mathematical non-existence.

A stochastic semantic object can be exact mathematics even though individual samples are random.

---

## 18. Randomized execution classes

Randomness is typed:

```text
DETERMINISTIC EXACT
LAS VEGAS EXACT
MONTE CARLO / probabilistic certificate
HEURISTIC CANDIDATE
```

Las Vegas algorithms may vary runtime/path while retaining exact results.

Monte Carlo outputs satisfy only authority contracts that explicitly allow their error probability unless upgraded by deterministic/foundational certification.

Reproducible exploratory randomness may derive from stable campaign/realization keys.

---

## 19. Proof erasure and authority caching

Proof may be mandatory for admission without remaining in the runtime hot path.

After a Realization is certified and bound to its exact semantic identity:

```text
proof/certificate -> authority store
stripped residual executable -> runtime
```

Runtime invocation verifies realization/generation identity, not the full proof on every call unless policy requires replay.

This allows proof-rich development with near-native execution.

---

## 20. Realization manifest

Every admitted native realization has an immutable manifest:

```text
RealizationManifest {
    semantic target digest
    universe generation of admission
    specialization digest
    query direction
    World/assumption contract
    input/output representation
    numeric/execution semantics
    source/KIR digest where retained
    compiler/backend identities
    target architecture/features
    binary/artifact digest
    realization certificate/evidence
    fallback/escalation plan
    resource measurements/bounds
}
```

Measured performance is metadata, not mathematical authority.

---

## 21. Realization selection

At runtime the planner chooses among admitted realizations compatible with the current context.

Selection factors:

```text
semantic compatibility
authority compatibility
architecture/features
input size/shape/structure
certified resource bounds
empirical benchmark metadata
warm-cache/startup cost
memory budget
fallback availability
```

If the selected realization encounters a declared ambiguity/precondition boundary, it must escalate/fallback rather than silently return outside contract.

---

## 22. Translation validation and independent checking

Generated/optimized code is never admitted solely because a compiler produced it.

Validation routes may include:

```text
formal compiler proof
translation validation
finite exhaustive equivalence
symbolic equivalence
refinement proof
certificate-producing code generation
diverse independent realization comparison + proof where admissible
```

The route is bound to the Realization manifest.

Search-based/superoptimized assembly is acceptable if the final winner independently satisfies the semantic equivalence contract.

---

## 23. Execution results and authority

Execution may produce:

```text
result value
witness
certificate
certified bound
performance measurement
counterexample
candidate discovery
```

Only admitted evidence can create/upgrade a Judgement through D1/D5 Promotion.

A fast runtime observation does not become durable truth merely because it came from an admitted executable unless the executable's semantic contract makes that observation a valid evidence producer for the target claim.

---

## 24. Ptah boundary

D4 produces serializable resource/work descriptions that Ptah may eventually execute:

```text
kernel/realization identity
exact inputs
CPU/GPU requirements
memory/storage characteristics
checkpoint contract
expected evidence output
```

Ptah is explicitly deferred. Local/native execution must prove D4 first.

Adding Ptah later cannot change mathematical semantics, evidence requirements, or realization identity; it only changes where approved work executes.

---

## 25. D4 proof obligations

```text
D4-P01 every admitted realization binds to exact semantic identity
D4-P02 specialization preserves/refines declared semantics
D4-P03 lowering transformations have explicit equivalence/refinement class
D4-P04 filtered stages return only when decisiveness is certified
D4-P05 ambiguity triggers escalation, never guessed authority
D4-P06 exact-image/reconstruction routes independently verify reconstructed result
D4-P07 numerical realizations bind explicit error/rounding semantics
D4-P08 SIMD/reassociation obeys mathematical/machine arithmetic semantics
D4-P09 GPU output follows the same certification path as CPU output
D4-P10 incremental update paths prove change semantics or fall back
D4-P11 out-of-core execution preserves content/ordering semantics
D4-P12 proof erasure cannot detach binary from its admitted realization identity
D4-P13 untrusted optimizer/compiler cannot self-admit generated code
D4-P14 local CPU path remains viable for canonical First Light
D4-P15 Ptah absence does not prevent consumption of promoted local primitives
```

---

## 26. Deferred from D4

Not frozen:

- final mathematical IR syntax;
- final LLVM/MLIR/Rust/C backend choice;
- permanent arithmetic library set;
- GPU vendor/API;
- distributed execution;
- final auto-tuner;
- hardware custom accelerator;
- final out-of-core storage engine.

D4 freezes the semantic lowering and realization laws, not one backend.

---

## 27. D4 frozen laws

1. **Semantic mathematics and executable realization remain distinct identities.**
2. **Specialize rich mathematical semantics into small residual programs.**
3. **CPU-first means use ordinary hardware aggressively before earning accelerator complexity.**
4. **Exactness may be achieved through cheap images, reconstruction, filtering, and escalation rather than carrying huge exact intermediates everywhere.**
5. **Ambiguity escalates; it never becomes a guessed result.**
6. **Proof may be erased from the hot path after authority is bound to the realization.**
7. **Incremental, streaming, out-of-core, SIMD, and GPU paths are realizations of the same semantics, not separate mathematics.**
8. **Optimizers are replaceable and may be untrusted; admitted outputs require independent realization evidence.**
9. **Resource measurements guide planning but do not alter truth.**
10. **Ptah is an execution substrate integration, not part of mathematical authority.**

D4 is complete when First Light can turn one newly promoted mathematical construction into a stripped native realization, independently validate it, and reuse it without rediscovery.