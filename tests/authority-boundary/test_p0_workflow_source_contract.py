"""Regression: P0 candidate CI must check the current source capture branch contract."""

import subprocess
import textwrap
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github" / "workflows" / "p0-boundaries.yml"
CAPTURE = ROOT / "scripts" / "capture_p0_source_proof.sh"


class P0WorkflowSourceContractTests(unittest.TestCase):
    def test_workflow_branch_markers_match_capture_script(self):
        workflow = WORKFLOW.read_text(encoding="utf-8")
        capture = CAPTURE.read_text(encoding="utf-8")
        self.assertIn('ALLOWED_BRANCHES=(', capture)
        self.assertIn('"impl/p0-first-light"', capture)
        self.assertIn('"c3/p1-continuation-20261006"', capture)
        self.assertIn('BRANCH_ALLOWED=false', capture)
        self.assertIn('[ "$BRANCH_ALLOWED" = true ]', capture)
        self.assertNotIn("grep -Fq 'EXPECTED_BRANCH=", workflow)

    def test_workflow_safety_marker_step_runs(self):
        workflow = WORKFLOW.read_text(encoding="utf-8")
        name = "      - name: Verify P0 host-capture procedure syntax and safety markers"
        self.assertEqual(workflow.count(name), 1)
        body_start = workflow.index("        run: |\n", workflow.index(name)) + len("        run: |\n")
        body_end = workflow.index("      - name:", body_start)
        shell = textwrap.dedent(workflow[body_start:body_end])
        self.assertIn("ALLOWED_BRANCHES", shell)
        result = subprocess.run(
            ["bash", "-c", "set -euo pipefail\n" + shell],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=20,
            check=False,
        )
        self.assertEqual(
            result.returncode, 0,
            f"workflow branch-contract assertions failed: {result.stderr}",
        )


if __name__ == "__main__":
    unittest.main()
