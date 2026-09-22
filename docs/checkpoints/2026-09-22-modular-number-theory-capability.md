# Modular Number Theory Capability Checkpoint

Status: SOURCE PROVED - DOCUMENTATION-HEAD PROOF PENDING

This is an additive post-P12 Formula capability. It does not rewrite any frozen P11/P12 authority.

## Frozen predecessor authority

P12 final proof boundary:

75e3cb55c9099200d1b0f36c09b35e23a7b0a482

P12 remains:

BOOTSTRAP_TRUST_REDUCED: FINAL RECOVERY AUTHORITY

The modular capability branch proves that frozen P12 head is an ancestor before any capability gate runs.

## Canonical capability branch

capability/modular-number-theory

## Source-under-test proof

source head:
2d21574ede6f36d9cd15def3fdd8d13290649381

workflow:
Modular number theory capability proof

workflow path:
.github/workflows/modular-number-theory-proof.yml

workflow blob:
e0578cae28057c31c239c09050046b09231b31a0

run:
35694199602

job:
106637380012

conclusion:
success

## Proven capability surface

The independent Formula checker now supports exact arbitrary-precision:

- Mod(value, modulus)
- MulMod(lhs, rhs, modulus)
- PowMod(base, exponent, modulus)
- Gcd(lhs, rhs)
- ExtendedGcd(lhs, rhs)
- ModInverse(value, modulus)

Semantics are generic and vendor-neutral.

Mod, MulMod, and PowMod require positive modulus and use non-negative floor-mod residues.

PowMod requires exponent >= 0.

Gcd is non-negative.

ExtendedGcd accepts any canonical-decimal Bezout witness satisfying both the exact gcd and Bezout identity.

ModInverse requires modulus > 1 and gcd(value, modulus) = 1; accepted inverse is normalized into [0, modulus).

## Trust boundary

External producer results remain untrusted.

formula-check independently recomputes or verifies the operation.

The capability binds operator and exact BigInt operands into Formula structural identity and binds evidence receipts to the operation digest plus producer result text.

Malformed decimal text, invalid modulus, negative exponent, incorrect result, and non-invertible inverse fail closed.

No new dependency class or Cargo.lock change was introduced.

## Source proof gates

Run 35694199602 / job 106637380012 passed every permanent gate:

- exact source identity;
- frozen P12 ancestor authority;
- pinned Rust 1.98.0;
- locked/offline metadata;
- modular number theory focused tests;
- existing P11 exact arithmetic tests;
- all formula-check targets;
- architecture firewalls;
- full workspace tests;
- full workspace build;
- rustfmt;
- clippy with warnings denied;
- independent deterministic arithmetic controls;
- formula-check dependency firewall;
- formula-check source authority firewall;
- clean worktree.

## Independent controls

The permanent workflow independently checks exact arithmetic outside the Rust checker using deterministic Python controls across 32, 64, 128, 256, 512, 1024, and 2048-bit values.

It checks:

- built-in modular exponentiation versus an independent square-and-multiply implementation;
- modular multiplication equivalence;
- gcd and ExtendedGcd/Bezout identity;
- normalized modular inverse whenever the inverse exists.

The focused Rust suite also proves the generic exponent 65537 path, negative floor-mod behavior, zero exponent, modulus one behavior where defined, large modular inverse, negative-input inverse normalization, malformed result rejection, non-invertible failure, and structural-identity binding.

## Explicit exclusions

This capability does not yet define:

- RSA public-operation semantics;
- encoded-message or padding semantics;
- fixed-width integer/byte representation;
- Montgomery implementation equivalence;
- device token formats;
- vendor authorization policy.

Those remain higher layers over the now-proved modular arithmetic substrate.

## Documentation-head freeze requirement

Only these source-to-docs changes are permitted for recursive recovery proof:

- CURRENT.md
- docs/checkpoints/2026-09-22-modular-number-theory-capability.md

The workflow blob must remain:

e0578cae28057c31c239c09050046b09231b31a0

The unchanged permanent workflow must pass on the exact documentation-bearing head before this capability is finally frozen.

## Freeze state

P12 predecessor: FINALLY FROZEN
modular capability source proof: PROVED
modular source head: 2d21574ede6f36d9cd15def3fdd8d13290649381
modular source run/job: 35694199602 / 106637380012
modular source conclusion: success
modular documentation proof: PENDING
modular capability final freeze: PENDING
