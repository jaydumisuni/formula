from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import subprocess


class ContractReviewError(ValueError):
    pass


VALID = {"MEASURABLE", "NEEDS_CLARIFICATION", "OUT_OF_SCOPE"}


def extract_review_rows(markdown: str) -> list[tuple[str, str]]:
    rows = []
    for line in markdown.splitlines():
        if line.startswith("| ") and line.endswith(" |") and "| PENDING |" in line:
            cols = [c.strip() for c in line.strip("|").split("|")]
            if len(cols) != 5:
                raise ContractReviewError("malformed G0 review row")
            rows.append((cols[0], cols[1]))
    if len(rows) != 33 or len(set(rows)) != 33:
        raise ContractReviewError("source must have 33 distinct pending rows")
    return rows


def validate_report(report: dict, source_rows: list[tuple[str, str]]) -> dict:
    if report.get("schema") != "ttg.formula.data-fabric-g0-review-candidate.v1":
        raise ContractReviewError("unknown review schema")
    if report.get("status") != "PREASSURANCE__NOT_FORMULA_CERTIFICATION":
        raise ContractReviewError("review cannot claim independent certification")
    owner = report.get("reviewer")
    if not isinstance(owner, dict) or owner.get("formal_formula_authority") is not False:
        raise ContractReviewError("candidate reviewer must not self-certify")
    if owner.get("accepted_milestone") is not False:
        raise ContractReviewError("candidate cannot accept a G0 milestone")
    rows = report.get("rows")
    if not isinstance(rows, list) or len(rows) != 33:
        raise ContractReviewError("all 33 rows required")
    if [(r.get("input"), r.get("obligation")) for r in rows] != source_rows:
        raise ContractReviewError("source row set/order mismatch")
    for row in rows:
        if row.get("disposition") not in VALID:
            raise ContractReviewError("invalid row disposition")
        for key in ("discriminator", "counterexample", "evidence"):
            value = row.get(key)
            if not isinstance(value, str) or len(value.strip()) < 35:
                raise ContractReviewError("missing or insufficient "+key+" for "+str(row.get("obligation")))
        if row["discriminator"].strip() == row["counterexample"].strip():
            raise ContractReviewError("a pass/fail pair cannot use the same observation")
        if row["disposition"] == "NEEDS_CLARIFICATION":
            if not isinstance(row.get("ambiguity"), str) or len(row["ambiguity"].strip()) < 30:
                raise ContractReviewError("unexplained ambiguity")
        elif row.get("ambiguity"):
            raise ContractReviewError("no ambiguous source claim with resolved disposition")
    counts = {c: sum(r["disposition"] == c for r in rows) for c in VALID}
    return {"rows": len(rows), "dispositions": counts, "status": "REVIEW_PREASSURANCE_ONLY",
            "formal_formula_authority_granted": False}


def verify_exact_source(report: dict, source_repo: Path) -> dict:
    source_repo = Path(source_repo).resolve(strict=True)
    def git(*args: str) -> str:
        r = subprocess.run(["git", "-C", str(source_repo), *args],
                           capture_output=True, text=True, timeout=40)
        if r.returncode:
            raise ContractReviewError("source Git query failed")
        return r.stdout.strip()
    if git("rev-parse", "HEAD") != report.get("source_head"):
        raise ContractReviewError("exact source HEAD drift")
    if git("status", "--porcelain=v1", "-uall"):
        raise ContractReviewError("source worktree dirty")
    bindings = report.get("source_bindings")
    required = {"docs/G0_CONSTITUTION.md", "docs/G0_HOSTILE_WORLD_CORPUS.md",
                "docs/BUSINESS_CORE_COMPATIBILITY_SCOPE.md", "docs/DONOR_INTAKE_CONTRACT.md",
                "docs/G0_FORMULA_REVIEW_MATRIX.md"}
    if not isinstance(bindings, dict) or set(bindings) != required:
        raise ContractReviewError("missing bound G0 input")
    for name, digest in bindings.items():
        p = (source_repo / name).resolve(strict=True)
        if not p.is_relative_to(source_repo) or hashlib.sha256(p.read_bytes()).hexdigest() != digest:
            raise ContractReviewError("stale/forged source digest: "+name)
    text = (source_repo / "docs/G0_FORMULA_REVIEW_MATRIX.md").read_text()
    result = validate_report(report, extract_review_rows(text))
    result.update({"exact_source_head": report["source_head"], "source_verified": True,
                   "source_bindings_verified": len(required)})
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description="Verify G0 candidate pre-assurance, never certify.")
    parser.add_argument("report", type=Path)
    parser.add_argument("--source-repo", type=Path, required=True)
    args = parser.parse_args()
    report = json.loads(args.report.read_text())
    print(json.dumps(verify_exact_source(report, args.source_repo), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
