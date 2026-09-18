#!/usr/bin/env python3
"""Static P0 authority-boundary checks.

These checks establish repository invariants only. They do not promote P0 or replace
the roadmap-required clean local build/proof.
"""
from __future__ import annotations

import hashlib
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EXPECTED_WORKSPACE = {
    "crates/formula-core",
    "crates/formula-store",
    "crates/formula-check",
    "crates/formula-engine",
    "crates/formula-packages",
    "crates/formula-realize",
    "crates/formula-first-light",
    "crates/formula-cli",
}
DISCOVERY_CRATES = ("formula-engine", "formula-packages")
FORBIDDEN_CHECK_DEPS = {"formula-engine", "formula-first-light"}
CANONICAL_RUNTIME_ROOTS = ("formula-first-light",)
EXPECTED_LOCK_PACKAGES = {
    "formula-core",
    "formula-store",
    "formula-check",
    "formula-engine",
    "formula-packages",
    "formula-realize",
    "formula-first-light",
    "formula-cli",
}


def load_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def dependency_entries(manifest: Path) -> list[tuple[str, Path | None]]:
    data = load_toml(manifest)
    workspace = load_toml(ROOT / "Cargo.toml").get("workspace", {})
    workspace_dependencies = workspace.get("dependencies") or {}
    entries: list[tuple[str, Path | None]] = []

    def resolved_name(alias: str, spec: object) -> str:
        if isinstance(spec, dict) and spec.get("workspace") is True:
            inherited = workspace_dependencies.get(alias)
            if isinstance(inherited, dict):
                return inherited.get("package", alias)
            return alias
        if isinstance(spec, dict):
            return spec.get("package", alias)
        return alias

    def add_dependencies(table: dict) -> None:
        for section in ("dependencies", "dev-dependencies", "build-dependencies"):
            for alias, spec in (table.get(section) or {}).items():
                name = resolved_name(alias, spec)
                effective = workspace_dependencies.get(alias) if isinstance(spec, dict) and spec.get("workspace") is True else spec
                base = ROOT if isinstance(spec, dict) and spec.get("workspace") is True else manifest.parent
                path = (base / effective["path"] / "Cargo.toml").resolve() if isinstance(effective, dict) and effective.get("path") else None
                entries.append((name, path))

    add_dependencies(data)
    for target in (data.get("target") or {}).values():
        if isinstance(target, dict):
            add_dependencies(target)
    return entries


def dependency_names(crate: str) -> set[str]:
    return {name for name, _ in dependency_entries(ROOT / "crates" / crate / "Cargo.toml")}


def workspace_dependency_closure(crate: str) -> set[str]:
    seen_names: set[str] = set()
    seen_manifests: set[Path] = set()
    pending = [(ROOT / "crates" / crate / "Cargo.toml").resolve()]
    while pending:
        manifest = pending.pop()
        if manifest in seen_manifests:
            continue
        seen_manifests.add(manifest)
        for dependency, dependency_manifest in dependency_entries(manifest):
            seen_names.add(dependency)
            if dependency_manifest is not None and dependency_manifest.is_file():
                pending.append(dependency_manifest)
    return seen_names


def fail(message: str) -> None:
    raise AssertionError(message)


def check_workspace_shape() -> None:
    root = load_toml(ROOT / "Cargo.toml")
    members = set(root["workspace"]["members"])
    if members != EXPECTED_WORKSPACE:
        fail(f"workspace members changed: {sorted(members)}")


def check_checker_isolation() -> None:
    deps = dependency_names("formula-check")
    forbidden = deps & FORBIDDEN_CHECK_DEPS
    if forbidden:
        fail(f"formula-check crosses checker/search boundary: {sorted(forbidden)}")


def check_sealed_fixture_isolation() -> None:
    for crate in DISCOVERY_CRATES:
        deps = workspace_dependency_closure(crate)
        if "formula-first-light" in deps:
            fail(f"{crate} dependency graph reaches sealed formula-first-light crate")
        source_root = ROOT / "crates" / crate / "src"
        for path in source_root.rglob("*.rs"):
            text = path.read_text(encoding="utf-8")
            if "formula_first_light" in text or "formula-first-light" in text:
                fail(f"{path.relative_to(ROOT)} imports/references sealed First-Light implementation")


def check_canonical_runtime_has_no_external_dependencies() -> None:
    lock = load_toml(ROOT / "Cargo.lock")
    packages = {entry["name"] for entry in lock.get("package", [])}
    external = packages - EXPECTED_LOCK_PACKAGES
    if external:
        fail(f"P0 canonical runtime acquired external dependencies: {sorted(external)}")

    runtime_crates: set[str] = set()
    for root in CANONICAL_RUNTIME_ROOTS:
        runtime_crates.add(root)
        runtime_crates.update(workspace_dependency_closure(root))
    for crate in runtime_crates:
        source_root = ROOT / "crates" / crate / "src"
        if not source_root.exists():
            continue
        for path in source_root.rglob("*.rs"):
            compact = "".join(path.read_text(encoding="utf-8").split())
            direct_network = "std::net" in compact
            grouped_network = re.search(r"std::\{[^;]*\bnet(?:\b|::)", compact) is not None
            if direct_network or grouped_network:
                fail(f"P0 canonical runtime contains network source reference: {path.relative_to(ROOT)}")


def check_fixture_identity() -> None:
    fixture_dir = ROOT / "tests/authority-boundary/fixtures"
    fixtures = sorted(fixture_dir.rglob("*.json"))
    if not fixtures:
        fail("P0 fixture set is empty")
    for fixture in fixtures:
        raw = fixture.read_bytes()
        parsed = json.loads(raw)
        canonical = json.dumps(parsed, sort_keys=True, separators=(",", ":")) + "\n"
        if raw != canonical.encode("utf-8"):
            fail(f"P0 fixture bytes are not canonical JSON: {fixture.name}")
        digest_file = fixture.with_suffix(".sha256")
        if not digest_file.is_file():
            fail(f"P0 fixture digest sidecar missing: {fixture.name}")
        expected = digest_file.read_text(encoding="ascii").strip().split()[0]
        actual = hashlib.sha256(raw).hexdigest()
        if actual != expected:
            fail(f"P0 fixture identity changed for {fixture.name}: expected {expected}, got {actual}")


def main() -> int:
    checks = (
        ("P0-02 checker/search isolation", check_checker_isolation),
        ("P0-03 sealed fixture isolation", check_sealed_fixture_isolation),
        ("P0-04 canonical First-Light runtime is network-free", check_canonical_runtime_has_no_external_dependencies),
        ("P0-05 deterministic fixture identity", check_fixture_identity),
        ("P0 workspace shape", check_workspace_shape),
    )
    for label, check in checks:
        check()
        print(f"PASS {label}")
    print("NOTE repository invariants only; P0 remains UNPROVEN pending clean local build/proof")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"FAIL {exc}", file=sys.stderr)
        raise SystemExit(1)
