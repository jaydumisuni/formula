import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

from verify_data_fabric_g0_review import (
    ContractReviewError, extract_review_rows, validate_report
)


FILE = ROOT / "docs/reviews/data-fabric-g0-2170fa4-semantic-preassurance-20261122.json"


def fixture():
    obj = json.loads(FILE.read_text())
    return obj, [(r["input"], r["obligation"]) for r in obj["rows"]]


def test_review_33_rows_all_bound():
    report, rows = fixture()
    result = validate_report(report, rows)
    assert result["rows"] == 33
    assert result["dispositions"]["NEEDS_CLARIFICATION"] == 8
    assert result["dispositions"]["MEASURABLE"] == 25
    assert result["formal_formula_authority_granted"] is False


@pytest.mark.parametrize("mutation,match", [
    (lambda x: x["rows"].pop(), "33 rows"),
    (lambda x: x["rows"][0].update({"disposition": "CERTIFIED"}), "disposition"),
    (lambda x: x["rows"][0].update({"counterexample": ""}), "counterexample"),
    (lambda x: x["rows"][0].update({"evidence": "tests pass"}), "evidence"),
    (lambda x: x["rows"][2].update({"ambiguity": ""}), "ambiguity"),
    (lambda x: x["reviewer"].update({"formal_formula_authority": True}), "self-certify"),
    (lambda x: x["reviewer"].update({"accepted_milestone": True}), "G0 milestone"),
    (lambda x: x["rows"][1].update({"obligation": "unbound row"}), "mismatch"),
])
def test_negative_review_tamper_controls(mutation, match):
    report, rows = fixture()
    mutation(report)
    with pytest.raises(ContractReviewError, match=match):
        validate_report(report, rows)


def test_reject_matrix_with_missing_world():
    report, _ = fixture()
    content = "\n".join(
        f"| {r['input']} | {r['obligation']} | Formula review required | Formula review required | PENDING |"
        for r in report["rows"][:-1]
    )
    with pytest.raises(ContractReviewError, match="33 distinct"):
        extract_review_rows(content)
