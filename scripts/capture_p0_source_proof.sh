#!/usr/bin/env bash
set -euo pipefail

EXPECTED_ARCH="50c0beb021d8bf02d59a177049b9c2cf1783b26a"
EXPECTED_BRANCH="impl/p0-first-light"
OUT_DIR="${1:-.formula/evidence/p0-01}"

fail() {
  printf 'P0-01 capture refused: %s\n' "$1" >&2
  exit 1
}

command -v git >/dev/null 2>&1 || fail "git is required"
command -v rustc >/dev/null 2>&1 || fail "rustc is required"
command -v cargo >/dev/null 2>&1 || fail "cargo is required"
command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || fail "not inside a Git worktree"
cd "$ROOT"

git rev-parse --verify "$EXPECTED_ARCH^{commit}" >/dev/null 2>&1 || fail "frozen architecture commit unavailable: $EXPECTED_ARCH"
git merge-base --is-ancestor "$EXPECTED_ARCH" HEAD || fail "captured HEAD does not descend from frozen architecture checkpoint $EXPECTED_ARCH"

BRANCH="$(git branch --show-current)"
[ "$BRANCH" = "$EXPECTED_BRANCH" ] || fail "expected branch $EXPECTED_BRANCH, found ${BRANCH:-DETACHED}"

[ -z "$(git status --porcelain)" ] || fail "worktree is not clean"

for path in rust-toolchain.toml Cargo.toml Cargo.lock docs/implementation/P0_SOURCE_MANIFEST.md; do
  [ -f "$path" ] || fail "missing required file: $path"
done

grep -Fq 'channel = "1.98.0"' rust-toolchain.toml || fail "rust-toolchain.toml is not pinned to 1.98.0"

RUSTC_VERSION="$(rustc --version)"
case "$RUSTC_VERSION" in
  'rustc 1.98.0 '*) ;;
  'rustc 1.98.0') ;;
  *) fail "expected rustc 1.98.0, found $RUSTC_VERSION" ;;
esac

[ ! -e "$OUT_DIR" ] || fail "evidence destination already exists: $OUT_DIR"
OUT_PARENT="$(dirname "$OUT_DIR")"
mkdir -p "$OUT_PARENT"
STAGE="$(mktemp -d "$OUT_PARENT/.p0-01.capture.XXXXXX")"
cleanup() {
  [ ! -d "$STAGE" ] || rm -rf "$STAGE"
}
trap cleanup EXIT

HEAD="$(git rev-parse HEAD)"
rustc --version --verbose > "$STAGE/rustc-version.txt"
cargo --version --verbose > "$STAGE/cargo-version.txt"
sha256sum rust-toolchain.toml Cargo.toml Cargo.lock docs/implementation/P0_SOURCE_MANIFEST.md > "$STAGE/manifest-sha256.txt"
cargo metadata --locked --format-version 1 > "$STAGE/cargo-metadata.json"
(
  cd "$STAGE"
  sha256sum cargo-metadata.json > cargo-metadata.sha256
)
printf '%s\n' "$HEAD" > "$STAGE/git-head.txt"
printf '%s\n' "$BRANCH" > "$STAGE/git-branch.txt"
printf '%s\n' "$EXPECTED_ARCH" > "$STAGE/frozen-architecture.txt"
printf 'clean\n' > "$STAGE/worktree-state.txt"

cat > "$STAGE/record.json" <<JSON
{
  "schema": "formula.p0-01.source-proof.v1",
  "status": "CANDIDATE_HOST_EVIDENCE_NOT_AUTHORITY",
  "git_head": "$HEAD",
  "git_branch": "$BRANCH",
  "frozen_architecture": "$EXPECTED_ARCH",
  "worktree_state": "clean",
  "rust_toolchain": "1.98.0",
  "authority_note": "This record captures P0-01 host evidence inputs only. It does not promote P0-01 or P0."
}
JSON

(
  cd "$STAGE"
  sha256sum \
    record.json \
    git-head.txt \
    git-branch.txt \
    frozen-architecture.txt \
    worktree-state.txt \
    rustc-version.txt \
    cargo-version.txt \
    manifest-sha256.txt \
    cargo-metadata.json \
    cargo-metadata.sha256 \
    > evidence-files.sha256
)

mv "$STAGE" "$OUT_DIR"
printf 'Captured candidate P0-01 host evidence for %s at %s\n' "$HEAD" "$OUT_DIR"
