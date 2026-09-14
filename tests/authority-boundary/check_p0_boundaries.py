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
FIXTURE = ROOT / "tests/authority-boundary/fixtures/p0-identity-v1.json"
FIXTURE_SHA = ROOT / "tests/authority-boundary/fixtures/p0-identity-v1.sha256"


def load_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def dependency_names(crate: str) -> set[str]:
    data = load_toml(ROOT / "crates" / crate / "Cargo.toml")
    workspace = load_toml(ROOT / "Cargo.toml").get("workspace", {})
    workspace_dependencies = workspace.get("dependencies") or {}
    names: set[str] = set()

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
                names.add(resolved_name(alias, spec))

    add_dependencies(data)
    for target in (data.get("target") or {}).values():
        if isinstance(target, dict):
            add_dependencies(target)
    return names


def workspace_dependency_closure(crate: str) -> set[str]:
    seen: set[str] = set()
    pending = [crate]
    while pending:
        current = pending.pop()
        for dependency in dependency_names(current):
            if dependency in seen:
                continue
            seen.add(dependency)
            if (ROOT / "crates" / dependency / "Cargo.toml").exists():
                pending.append(dependency)
    return seen


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
            grouped_network = re.search(r"std::\{[^}]*\bnet(?:\b|::)", compact) is not None
            std_aliases = set(re.findall(r"usestdas([A-Za-z_][A-Za-z0-9_]*)[;{]", compact))
            std_aliases.update(re.findall(r"externcratestdas([A-Za-z_][A-Za-z0-9_]*);", compact))
            aliased_network = any(
                re.search(rf"(?:^|[^A-Za-z0-9_]){re.escape(alias)}::net(?:\b|::)", compact) is not None
                for alias in std_aliases
            )
            if direct_network or grouped_network or aliased_network:
                fail(f"P0 canonical runtime contains network source reference: {path.relative_to(ROOT)}")


def check_fixture_identity() -> None:
    raw = FIXTURE.read_bytes()
    parsed = json.loads(raw)
    canonical = json.dumps(parsed, sort_keys=True, separators=(",", ":")) + "\n"
    if raw != canonical.encode("utf-8"):
        fail("P0 fixture bytes are not canonical JSON")
    expected = FIXTURE_SHA.read_text(encoding="ascii").strip().split()[0]
    actual = hashlib.sha256(raw).hexdigest()
    if actual != expected:
        fail(f"P0 fixture identity changed: expected {expected}, got {actual}")


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
