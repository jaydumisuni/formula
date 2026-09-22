# Verifier Equivalence Capability Checkpoint

Status: SOURCE PROVED - DOCUMENTATION-HEAD PROOF PENDING

This capability is additive over the finally frozen RSA Public Representation, Modular Number Theory, and P12 authority boundaries.

## Frozen predecessor authority

RSA Public Representation frozen proof boundary:

9f222f0ed40d6c654d282f3d4a337c34d2c62f4a

## Canonical branch

capability/verifier-equivalence

## Source-under-test proof

source head:
20ee736cb1ec3db7222ae41d47f43ecffc42ca61

workflow:
Verifier equivalence capability proof

workflow path:
.github/workflows/verifier-equivalence-proof.yml

workflow blob:
35e2f3ad1d4c407cdc5367c5bde1fef23f84fabd

run:
35696735247

job:
106645152306

conclusion:
success

## Proven Montgomery equivalence surface

For odd modulus N > 1 and radix R = 2^radix_bits with R > N, the checker verifies producer Montgomery multiplication against:

a * b * R^-1 mod N

for 0 <= a,b < N.

The producer output must be canonical decimal and lie in [0,2N). A single conditional subtraction of N must produce the exact mathematical result.

This permits firmware realizations that defer one final subtraction while preserving exact arithmetic as authority.

## Proven Type-1 payload surface

The checker validates exact blocks of the form:

00 01 || FF^PS || 00 || payload

with:

- total width >= 11 bytes;
- PS length >= 8 bytes;
- every PS byte exactly FF;
- payload may be empty;
- every byte constrained;
- canonical lowercase hexadecimal input;
- exact expected-payload equality.

The width-256 controls prove payload lengths 48 and 245 accepted and 246 rejected.

## Integrated RSA public verifier

The checker can independently:

1. compute M = S^e mod N;
2. encode M to the modulus byte width;
3. validate exact Type-1 structure;
4. require exact payload equality.

No private-key operation is introduced.

## Xiaomi ginkgo donor evidence

Recovered donor:
jaydumisuni/Xiaomi-bootloader

donor head:
c41e2a2442240d866757ce923ea287c2f069ac8d

Recovered verifier properties:

- 64 x 32-bit limbs;
- 2048-bit modulus;
- exponent 65537;
- Montgomery low-word inverse condition proved;
- stored helper constant equals R^2 mod N for R = 2^2048;
- fixed exponentiation chain implements public exponent 65537;
- exact Type-1 payload parser with max payload 245 bytes for width 256.

Offline executable-specification simulation:

- 406 full verifier public-operation cases;
- 114 representatives >= N;
- zero mismatches versus exact PowMod;
- 305 direct Montgomery product cases;
- zero mismatches versus a*b*R^-1 mod N after at most one subtraction;
- 52 producer outputs required the final subtraction;
- Type-1 mutation controls rejected bad padding and separator bytes.

Simulation artifact SHA-256:
e265d19c23f37c48874ed98bbdd4dee1644188a78fdc4feb2704f7fa96bfab0a

The donor remains evidence only. It is not Formula authority and no Xiaomi constants are hard-coded into generic checker semantics.

## Source proof gates

Run 35696735247 / job 106645152306 passed:

- exact source identity;
- frozen RSA predecessor authority;
- pinned Rust 1.98.0;
- locked/offline metadata;
- focused verifier-equivalence tests;
- frozen RSA public representation tests;
- frozen modular number theory tests;
- all formula-check targets;
- architecture firewalls;
- full workspace tests;
- full workspace build;
- rustfmt;
- clippy with warnings denied;
- independent deterministic verifier controls;
- formula-check dependency firewall;
- formula-check source authority firewall;
- clean worktree.

## Trust boundary

Exact BigInt/modular/RSA public semantics remain mathematical authority.

Montgomery firmware is an implementation-equivalence target only.

Type-1 parsing is exact structure checking only.

This capability does not generate private keys, signatures, authorization responses, vendor tokens, or device unlock transitions.

## Documentation-head freeze requirement

Only these source-to-docs changes are permitted:

- CURRENT.md
- docs/checkpoints/2026-09-22-verifier-equivalence-capability.md

The permanent workflow blob must remain:

35e2f3ad1d4c407cdc5367c5bde1fef23f84fabd

The unchanged workflow must prove the exact documentation-bearing head before final freeze.

## Freeze state

RSA predecessor: FINALLY FROZEN
verifier-equivalence source proof: PROVED
verifier-equivalence source head: 20ee736cb1ec3db7222ae41d47f43ecffc42ca61
verifier-equivalence source run/job: 35696735247 / 106645152306
verifier-equivalence source conclusion: success
verifier-equivalence documentation proof: PENDING
verifier-equivalence final freeze: PENDING
