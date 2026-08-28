# Verifiable Computation and Interactive Proofs Research Pass

Date: 2026-08-28
Status: research checkpoint
Repository name: temporary only; not product identity

## Why this pass exists

The cross-domain certificate atlas showed an important gap: some expensive exact computations do not naturally produce a tiny deterministic domain-native witness. Exact counting and large generic arithmetic-circuit evaluations are examples.

Interactive proof and verifiable-computation systems provide another route:

```
large computation
      ↓
prover executes / arithmetizes computation
      ↓
interactive probabilistic proof
      ↓
weak verifier performs much less work
```

This is a distinct authority class from formal proof, exact deterministic certificates, and cryptographic SNARK/STARK arguments.

## 1. Sum-check

The sum-check protocol verifies a claim of the form

```
H = Σ_{x ∈ {0,1}^v} g(x)
```

without requiring the verifier to evaluate `g` at all `2^v` Boolean points.

Round structure:

1. prover sends a low-degree univariate polynomial representing the remaining partial sum;
2. verifier checks consistency with the prior claim;
3. verifier samples a fresh random field challenge;
4. the claim is reduced to one fewer variable;
5. after the final round the verifier checks one evaluation of `g` at a random point.

Source:
- https://microsoft.github.io/vega-prover/appendix/sumcheck-primer.html

For degree bounds `d_i`, the soundness error is controlled by Schwartz-Zippel and is roughly:

```
(sum_i d_i) / |F|
```

for the standard protocol assumptions.

Justin Thaler's notes emphasize that verifier work becomes approximately:

```
O(v + cost(one evaluation of g))
```

instead of `2^v` evaluations, while the honest prover can often remain within a small factor of the work required to compute the sum itself.

Source:
- https://zkproof.org/2020/03/16/sum-checkprotocol/

## 2. Exact counting / #SAT relevance

Arithmetization plus sum-check gives interactive proofs for #SAT and related counting claims.

This is highly relevant to the certificate-atlas gap:

```
claim: exactly N assignments satisfy φ
```

has no obvious tiny NP-style deterministic witness comparable to one SAT assignment.

But an interactive verifier can probabilistically check the claimed exact count much more cheaply than enumerating all assignments.

Thaler's probabilistic-proof course explicitly covers:

- #SAT;
- triangle counting;
- matrix multiplication;
- GKR circuit verification.

Source:
- https://people.cs.georgetown.edu/jthaler/COSC544.html

## 3. GKR for arithmetic-circuit computation

The Goldwasser-Kalai-Rothblum protocol verifies evaluation of layered arithmetic circuits over finite fields.

For a circuit of:

```
size S
 depth d
 input size n
```

Thaler's notes give verifier cost roughly:

```
O(n + d * polylog(S))
```

with communication/round complexity scaling similarly in depth/polylog size, and modern/specialized prover implementations can reach `O(S)` for suitable regular layered circuits.

Sources:
- https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf
- https://people.cs.georgetown.edu/jthaler/COSC544/Lecture9.pdf
- https://people.cs.georgetown.edu/jthaler/TimeOptimalIPs.html

This yields a new verification route for promoted or distributed computation:

```
semantic construction
      ↓
arithmetic-circuit realization
      ↓
large worker/prover evaluates circuit
      ↓
GKR transcript
      ↓
small verifier
```

The verifier does not need to repeat all `S` gate evaluations.

## 4. Formal verification of sum-check soundness

A 2024 Isabelle/HOL development formally verifies a generalized sum-check protocol. It formalizes public-coin interactive proofs, proves generalized completeness/soundness axioms, and instantiates them for multivariate polynomials.

Source:
- https://arxiv.org/abs/2402.06093

This is particularly valuable to the project because it demonstrates that the *verification protocol's own soundness argument* can itself be placed under a formal proof authority.

So the stack can become:

```
very large computation
      ↓
probabilistic interactive certificate
      ↓
small protocol verifier
      ↓
formal theorem establishing verifier/protocol soundness
```

## 5. Replayability versus challenge freshness

Interactive statistical soundness relies on verifier challenges being sampled at the correct protocol point **after the prover has committed to the preceding message**.

In sum-check:

```
prover sends g_i
      ↓
verifier checks consistency
      ↓
verifier samples fresh r_i
```

A prover that knew all challenges before choosing its messages could tailor false polynomials to those challenges, invalidating the standard soundness argument.

Therefore:

> deterministic replay is safe; deterministic preannouncement of verifier randomness is not generally equivalent.

The project can record after execution:

```
- statement digest
- prover messages
- verifier challenges
- verifier RNG source/commitment metadata
- field/modulus
- protocol version
- final verdict
```

and replay the transcript deterministically afterward.

But a reusable fixed challenge schedule must not be substituted for fresh protocol randomness unless a separate theorem establishes its soundness.

## 6. Information-theoretic interactive proof authority class

Interactive proofs such as sum-check/GKR should have an authority class distinct from ordinary formal proof.

Potential class:

```
INTERACTIVE_STATISTICAL_CERTIFICATE
```

Metadata must include:

```
statement
protocol
field / domain
soundness_error_bound
challenge generation method
number of repetitions / amplification
transcript digest
verifier implementation/checker
formal protocol theorem, if available
```

A verified transcript establishes something like:

```
accepts with declared statistical soundness error ε
```

not:

```
FORMAL_KERNEL_PROOF with zero protocol error
```

Those must never be collapsed.

## 7. Amplification

Statistical soundness can often be made arbitrarily small through:

- larger fields;
- repeated independent challenges/protocol repetitions;
- batched checks where the relevant theorem supports them.

So the project may choose a target:

```
ε <= 2^-128
```

or another profile-specific bound.

That is still probabilistic/statistical authority, however small the value becomes.

It does not become logical certainty merely because the error is astronomically small.

## 8. Cryptographic arguments are a different trust class

STARK/SNARK and Fiat-Shamir-derived noninteractive systems are extremely useful, but their soundness is **computational** rather than purely information-theoretic in the ordinary deployed setting.

The 2018 STARK work emphasizes:

- transparency/no trusted setup;
- sublinear verification;
- scalable proving;
- hash-based/post-quantum-oriented assumptions.

Source:
- https://eprint.iacr.org/2018/046.pdf

Fiat-Shamir transforms public-coin interactive protocols into noninteractive arguments/proofs in the random-oracle model under protocol-specific conditions. Modern work stresses that soundness after Fiat-Shamir is not automatic for every multi-round protocol and must be established under explicit assumptions.

Sources:
- https://eprint.iacr.org/2021/1377.pdf
- https://eprint.iacr.org/2023/1256

Therefore the project must distinguish:

```
INTERACTIVE_STATISTICAL_PROOF

from

CRYPTOGRAPHIC_ARGUMENT
```

The latter needs metadata such as:

```
security_assumptions
hash/commitment scheme
security level
random-oracle / Fiat-Shamir model
trusted setup status
protocol version
```

`transparent` means no trusted setup; it does **not** mean assumption-free mathematical proof.

## 9. Suggested certificate classes

Potential hierarchy:

```
FORMAL_KERNEL_PROOF
EXACT_DETERMINISTIC_CERTIFICATE
RIGOROUS_ENCLOSURE_CERTIFICATE
INTERACTIVE_STATISTICAL_CERTIFICATE(ε)
PROBABILISTIC_CHECK(ε)
CRYPTOGRAPHIC_ARGUMENT(security_assumptions, bits)
EMPIRICAL_EVIDENCE
```

These classes are incomparable in some dimensions and should not be encoded as one linear `confidence` score.

## 10. Distributed/Ptah relevance later

This is highly relevant to future Ptah-scale computation, without requiring Ptah changes now.

A remote/untrusted/high-performance Work Cell could eventually return:

```
result
+ native mathematical certificate, if available
```

or, when native certification is unavailable/too expensive:

```
result
+ interactive proof transcript / proof artifact
```

The local mathematical authority can verify the computation without rerunning the full workload.

This can apply to:

- GPU arithmetic circuits;
- distributed matrix/tensor computations;
- huge counting jobs;
- outsourced search/aggregation;
- large parallel Work Cell results.

## 11. Prover overhead matters

Verifiable computation is not free.

General-purpose proof systems can impose significant prover overhead. Thaler's work identifies prover overhead as a central bottleneck, although specialized GKR implementations for regular circuits can approach linear work in circuit size.

Source:
- https://people.cs.georgetown.edu/jthaler/TimeOptimalIPs.html

Therefore the Certificate Router should prefer:

```
1. tiny native deterministic witness
2. cheap independent exact checker
3. domain-specific certificate
4. interactive/verifiable computation
5. full recomputation / foundational proof
```

only according to actual cost and required assurance.

Interactive proof machinery should not replace simpler certificates where they exist.

## 12. Circuitization cost is part of the route

Although any finite computation can in principle be represented as an arithmetic circuit, a poor circuit representation may be too deep/large for GKR to save work.

Thaler's GKR notes explicitly warn that circuit size/depth can eliminate the verifier advantage.

Therefore the route cost must include:

```
original computation
 -> arithmetization / circuitization cost
 -> circuit size/depth
 -> prover cost
 -> transcript/communication
 -> verifier cost
```

This links verifiable computation back to representation search: the project may need to discover a proof-friendly arithmetic representation before GKR becomes worthwhile.

## 13. Core law

> **When a computation lacks a compact native mathematical witness, the project may certify the computation itself—but the protocol's soundness class and assumptions must remain explicit.**

## 14. Architecture consequence: proof-carrying Work Cells

A future Work Cell output contract can become:

```
CellResult {
    result_artifact,
    semantic_claim,
    certificate_envelope {
        family,
        assurance_class,
        soundness_error_or_assumptions,
        transcript_or_proof,
        checker,
        replay_metadata
    }
}
```

This lets cheap/local/domain-native certificates and heavyweight generic verifiable-computation proofs coexist under one envelope.

## 15. Open research

1. Exact cost thresholds for when GKR/sum-check beats independent recomputation.
2. Circuit representations/IRs optimized simultaneously for execution and proof generation.
3. Proof generation for GPU/SIMD/native mathematical kernels without forcing all computation through a field circuit when a better domain-native protocol exists.
4. Noninteractive transforms with precisely tracked cryptographic assumptions.
5. Transcript aggregation/recursive verification without collapsing assurance classes.
6. Privacy/zero-knowledge as an optional later property distinct from correctness verification.
7. Interactive proof routes for exact counting/model counting and tensor/network contractions.
8. Formally verified GKR/sum-check verifier implementations suitable for a small trusted runtime.
9. Proof-carrying distributed Work Cells and how challenge generation works across asynchronous execution.
10. Whether proof-oriented compilation can become another realization target beside CPU/GPU speed targets.
