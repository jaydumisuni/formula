# RSA Public Representation Capability Checkpoint

Status: FINALLY FROZEN

This capability is additive over the finally frozen Modular Number Theory capability and frozen P12 authority.

## Frozen predecessor authority

Modular Number Theory frozen proof boundary:

79755993f0e2a7c5d369d90c3a369433ee22c4b9

P12 remains finally frozen beneath it.

## Canonical branch

capability/rsa-public-representation

## Source-under-test proof

source head:
b379c3c340881b3a6ae4b196ff76c2a6b2f5135a

workflow:
RSA public representation capability proof

workflow path:
.github/workflows/rsa-public-representation-proof.yml

workflow blob:
e5e1cac03e69cc178e0c148410ecacbcbe74894b

run:
35695302121

job:
106640751390

conclusion:
success

## Proven capability surface

Fixed-width unsigned big-endian representation:

- EncodeUnsignedBe(value, width_bytes)
- DecodeUnsignedBe(hex, width_bytes)

Exact representation rules:

- width_bytes >= 1;
- values are unsigned;
- output is exactly width_bytes bytes;
- output hex is exactly 2 * width_bytes lowercase hexadecimal characters;
- leading zero bytes are preserved;
- overflow, negative values, malformed hex, uppercase hex, and width mismatch fail closed.

Generic RSA public operation:

M = S^e mod N

with:

- N > 1;
- e > 0;
- 0 <= S < N.

The checker independently validates:

- canonical decimal M;
- exact fixed-width big-endian hex for M;
- output width k = ceil(bit_length(N) / 8).

The focused source proof includes an exact 2048-bit e=65537 vector and a leading-zero modulus-width control.

## Trust boundary

External producers remain untrusted.

formula-check independently computes the public operation and representation.

Structural identity binds S, e, and N for RSA, and value/hex plus explicit byte width for representation operations.

Evidence identity binds operation digest plus producer outputs.

No private-key operation is introduced.

## Source proof gates

Run 35695302121 / job 106640751390 passed every permanent gate:

- exact source identity;
- frozen modular predecessor authority;
- pinned Rust 1.98.0;
- locked/offline metadata;
- focused RSA public representation tests;
- frozen modular number theory tests;
- all formula-check targets;
- architecture firewalls;
- full workspace tests;
- full workspace build;
- rustfmt;
- clippy with warnings denied;
- independent deterministic representation and RSA controls;
- formula-check dependency firewall;
- formula-check source authority firewall;
- clean worktree.

## Independent controls

The permanent workflow independently tests fixed-width integer/byte round trips across 1, 2, 4, 8, 16, 32, 64, 128, and 256-byte widths.

It independently tests RSA public operations across 64, 128, 256, 512, 1024, and 2048-bit moduli using both exponent 65537 and other positive odd exponents.

Python built-in modular exponentiation is cross-checked against an independent square-and-multiply implementation.

## Explicit exclusions

This capability does not define:

- private-key operations;
- signature generation;
- PKCS#1 v1.5 padding;
- PSS;
- OAEP;
- encoded-message parsing;
- vendor token formats;
- bootloader policy;
- Montgomery implementation equivalence.

Those remain separate additive layers.

## Final documentation proof

The unchanged permanent workflow proved exact documentation-bearing head:

9f222f0ed40d6c654d282f3d4a337c34d2c62f4a

Run:
35695582112

Job:
106641610762

Conclusion:
success

The source-to-docs delta contained only CURRENT.md and this checkpoint. The permanent workflow blob remained:

e5e1cac03e69cc178e0c148410ecacbcbe74894b

This exact head is the finally frozen RSA Public Representation capability boundary. Later metadata-only recovery commits do not move the proved boundary.

## Freeze state

modular predecessor: FINALLY FROZEN
RSA/representation source proof: PROVED
RSA/representation source head: b379c3c340881b3a6ae4b196ff76c2a6b2f5135a
RSA/representation source run/job: 35695302121 / 106640751390
RSA/representation source conclusion: success
RSA/representation documentation proof: PROVED
RSA/representation frozen docs head: 9f222f0ed40d6c654d282f3d4a337c34d2c62f4a
RSA/representation docs run/job: 35695582112 / 106641610762
RSA/representation docs conclusion: success
RSA/representation final freeze: FINALLY FROZEN
