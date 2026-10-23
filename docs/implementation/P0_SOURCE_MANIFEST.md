# P0 Source and Toolchain Manifest

Status: CANDIDATE IMPLEMENTATION EVIDENCE — NOT P0 PROOF

This file records the repository-level inputs for P0-01. It does not promote P0 and does not replace a clean local build proof.

## Frozen authority input

- architecture checkpoint: `50c0beb021d8bf02d59a177049b9c2cf1783b26a`
- roadmap: `docs/roadmap/2026-08-28-implementation-roadmap.md`
- First Light remains local, ordinary-CPU, model-free, network-free during canonical execution, GPU-free, and Ptah-free.

## Toolchain identity

- Rust: exactly `1.98.0`
- toolchain profile: `minimal`
- required components: `rustfmt`, `clippy`
- Cargo dependency resolution: lockfile version 4, checked into source

`rust-toolchain.toml` is the executable toolchain declaration. `Cargo.lock` is the dependency-resolution artifact. The P0 workspace intentionally begins with no third-party Rust dependencies; later dependencies must be deliberate, lockfile-bound, and reviewed against Formula authority boundaries.

## Source identity rule

The implementation source identity is the exact Git commit containing the workspace plus this manifest. No timestamp, process id, host path, filesystem metadata, random value, or mutable external state participates in Formula source identity.

A build/proof record must bind at minimum:

1. exact Git HEAD;
2. exact frozen architecture checkpoint above;
3. `rustc --version --verbose` output;
4. `cargo --version --verbose` output;
5. SHA-256 of `rust-toolchain.toml`, `Cargo.toml`, and `Cargo.lock`;
6. `cargo metadata --locked --format-version 1` output or its preserved digest;
7. clean/dirty worktree state.

## P0-01 status

Repository declarations for P0-01 are present on the construction branch. P0-01 remains **UNPROVEN** until the declared toolchain is available on the target host and a clean local proof captures the exact metadata above.
