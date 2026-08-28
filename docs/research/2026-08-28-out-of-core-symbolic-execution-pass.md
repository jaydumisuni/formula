# Research Pass — Out-of-Core Symbolic Execution, Streaming Mathematics, and RAM-Independent Scale

**Date:** 2026-08-28  
**Status:** RESEARCH supplement

This pass investigates how the unnamed mathematical project can keep ordinary RAM from becoming the primary capability ceiling for large symbolic/graph mathematical workloads.

The central finding is:

> **Large mathematical cardinality/representation size does not imply the entire active object must reside in RAM. FORM demonstrates a mature architecture where local term transformations plus staged external sorting allow algebraic expressions with enormous term counts to be processed primarily under disk-space rather than RAM limits. External-memory graph theory provides analogous algorithmic models.**

---

## 1. FORM deliberately handles expressions larger than main memory

FORM is designed for extremely large symbolic expressions and uses a locality principle: many transformations operate independently on individual terms. This enables disk-backed processing and parallelization.

Sources:

https://www.nikhef.nl/~form/aboutform/aboutform.html

https://www.nikhef.nl/~form/oldsite/maindir/oldversions/FORMdistribution/publications/external/extform.pdf

A January 2026 Nikhef update describes current FORM calculations manipulating expressions with billions of terms beyond ordinary in-memory CAS limits.

Source:

https://www.nikhef.nl/en/news/groundbreaking-computer-tool-form-for-theorists-further-updated/

### Architectural implication

The project should classify mathematical transformations by locality:

```text
term-local / edge-local / factor-local
block-local
global-reduction
random-access-global
```

Local transforms can stream through objects too large for RAM.

---

## 2. External sorting is a first-class mathematical runtime primitive

FORM accumulates manageable term patches in memory, sorts/combines them, spills sorted patches to disk, and later merges those runs. Its manual describes explicit small/large buffers, sort files, and disk-to-disk final merge stages.

Source:

https://www.nikhef.nl/~form/maindir/documentation/reference/html/manual.html

### Architectural implication

The runtime should own optimized primitives such as:

```text
external_sort
external_merge
stream_reduce
spillable_hash_partition
```

because canonicalization/combine-like operations appear repeatedly across algebra, graph, proof, and candidate-space workloads.

These should be infrastructure facilities, not reimplemented separately by every mathematical engine.

---

## 3. A bad intermediate can dominate memory more than the final result

FORM's sorting architecture aggressively combines/cancels terms early so fewer terms reach large disk buffers/files.

Source:

https://www.nikhef.nl/~form/maindir/documentation/reference/html/manual.html

FAQ/worst-case-optimal join research from earlier passes independently gave the same lesson.

### Architectural implication

The compiler should optimize **peak intermediate representation size**, not merely final output size or arithmetic count.

Possible cost dimensions:

```text
peak_RAM
peak_external_bytes
sequential_IO
random_IO
materialized_intermediate_size
recomputation_cost
```

This can influence representation and elimination order.

---

## 4. External-memory algorithms have their own complexity model

External-memory graph research measures cost in block transfers/scans/sorts rather than pretending disk is slow RAM. There are I/O-efficient algorithms for large graph problems whose data exceed internal memory.

Sources:

https://epubs.siam.org/doi/10.1137/1.9781611973105.65

https://epubs.siam.org/doi/abs/10.1137/1.9781611972870.1

### Architectural implication

Theory/Execution Profiles should optionally expose:

```text
RAM complexity
I/O complexity
streaming passes
random-access requirements
external representation compatibility
```

A mathematically superior algorithm in RAM complexity may be poor on a constrained workspace if it creates pathological random I/O.

---

## 5. Some external algorithms beat in-memory algorithms even when data fit

External-memory algorithm engineering can outperform conventional internal-memory implementations because it improves locality/layout and avoids cache/pathological access patterns.

Example:

https://epubs.siam.org/doi/10.1137/1.9781611974768.5

### Architectural implication

Out-of-core capability is not merely an emergency fallback.

A block/sequential representation may be the preferred realization on SSD/NVMe even before RAM is exhausted.

The execution planner should benchmark/profile rather than hardcode:

```text
if size < RAM -> in-memory
else -> disk
```

---

## 6. Read-only external structures can be shared safely

The read-only semi-external model studies graph algorithms where a large immutable external graph can be shared across processors/algorithms without cache-coherence/write synchronization problems and can remain compressed.

Source:

https://epubs.siam.org/doi/10.1137/1.9781611976489.6

### Architectural implication

The project's immutable/canonical mathematical objects are naturally suited to:

```text
shared read-only external store
    + small per-campaign mutable overlays
```

This matches the broader architecture where permanent mathematical truth is immutable while candidate/search state is rebuildable.

---

## 7. Structural sharing reduces memory before spilling

Hash-consing canonicalizes identical symbolic subexpressions so repeated structure is stored once. Recent JuliaSymbolics work reports reductions in memory and speedups across simplification, code generation, compilation, and evaluation.

Source:

https://arxiv.org/abs/2509.20534

### Architectural implication

Memory strategy should be layered:

```text
semantic sharing / hash-consing
    -> equivalence sharing / e-graph where justified
    -> compressed/succinct representation
    -> streaming / spill
    -> recomputation where cheaper than storage
```

Spilling duplicate structure is wasteful; representation compression comes first.

---

## 8. Exact equivalence reductions can also shrink external structures

External-memory algorithms exist for bisimulation reduction of massive graphs, computing structural equivalence classes while data remain disk based.

Source:

https://arxiv.org/abs/1210.0748

### Architectural implication

Equivalence/canonicalization need not become unavailable once the mathematical universe exceeds RAM.

Some quotienting operations can themselves run out-of-core and make subsequent representations smaller.

---

## 9. Search-state spillability differs from mathematical-object spillability

Some structures have strong sequential/block behavior; others require highly irregular pointer chasing or mutable equality closure and may perform poorly on disk.

### Architectural implication

Every representation/backend should advertise an execution-storage profile:

```text
streamable
spillable
partitionable
random_access_intensity
mutable_locality
rebuild_cost
checkpoint_cost
```

The compiler may change mathematical representation before spill rather than naively serialize the current in-memory structure.

---

## 10. Recompute versus store should be a cost decision

For cheap deterministic derived artifacts, recomputation can consume less I/O/storage than persisting every intermediate.

### Architectural implication

Because semantic inputs and transformations are content-addressed/replayable, the runtime can choose:

```text
cache
spill
recompute
```

based on cost while retaining the same mathematical identity.

This reinforces the distinction between mathematical artifact identity and operational cache state.

---

## 11. Current out-of-core hypothesis

```text
MATHEMATICAL OBJECT
    -> structural sharing / canonical compact representation
    -> classify operations by locality and access pattern

EXECUTION PLANNER
    -> in-memory / streaming / semi-external / external
    -> partition + block layout
    -> external sort/merge/reduce where needed
    -> overlap I/O and compute

SEMANTIC RESULT
    -> identical regardless of memory placement
```

The architecture should make RAM an optimization tier, not the definition of what mathematics can exist.

---

## 12. New research obligations

1. Study FORM/TFORM/ParFORM locality and parallel execution in enough detail to extract reusable operator classes.
2. Investigate out-of-core e-graph/equality structures and whether rebuildable partitions are viable.
3. Study external-memory Gröbner/polynomial/BDD/automata algorithms.
4. Define a storage/locality profile for every promoted representation and primitive.
5. Investigate cost-based cache/spill/recompute policy tied to content-addressed semantic artifacts.
6. Study NVMe/mmap/io_uring/direct-I/O style implementation choices later, without baking hardware into semantics.
7. Investigate compressed immutable mathematical stores plus small campaign overlays.
8. Study checkpointing long-running exact computations without serializing unnecessary ephemeral heuristics.
9. Connect decomposition/separators to external-memory partition layout.
10. Define First-Light stress tests where the represented mathematics intentionally exceeds RAM while remaining exactly solvable.
