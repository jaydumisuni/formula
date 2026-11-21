"""Executable negative-branch guard regression for P0-01 candidate capture."""
import pathlib
import subprocess
import tempfile
import unittest
import uuid


ROOT = pathlib.Path(__file__).resolve().parents[2]


class CaptureBranchRejection(unittest.TestCase):
    def test_named_unauthorized_branch_refuses_capture_before_writing_evidence(self):
        branch = f"supervisor/p0-unauthorized-negative-fixture-{uuid.uuid4().hex}"
        with tempfile.TemporaryDirectory(prefix="formula-p0-named-branch-") as temp:
            checkout = pathlib.Path(temp) / "checkout"
            output = pathlib.Path(temp) / "evidence"
            subprocess.run(
                ["git", "-C", str(ROOT), "worktree", "add", "-b", branch, str(checkout), "HEAD"],
                check=True, capture_output=True, text=True,
            )
            try:
                result = subprocess.run(
                    ["bash", str(checkout / "scripts/capture_p0_source_proof.sh"), str(output)],
                    cwd=checkout, capture_output=True, text=True, timeout=30,
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(f"branch not admitted for P0-01 capture: {branch}", result.stderr)
                self.assertFalse(output.exists(), "refused capture must not publish evidence")
            finally:
                subprocess.run(
                    ["git", "-C", str(ROOT), "worktree", "remove", "--force", str(checkout)],
                    check=True, capture_output=True, text=True,
                )
                # Delete only the unique ref created by this test, and only if
                # it still points at the checkout's original commit.
                ref = f"refs/heads/{branch}"
                actual = subprocess.run(
                    ["git", "-C", str(ROOT), "rev-parse", "--verify", ref],
                    check=True, capture_output=True, text=True,
                ).stdout.strip()
                expected = subprocess.run(
                    ["git", "-C", str(ROOT), "rev-parse", "HEAD"],
                    check=True, capture_output=True, text=True,
                ).stdout.strip()
                if actual != expected:
                    raise AssertionError("temporary fixture branch changed; preserving ref")
                subprocess.run(
                    ["git", "-C", str(ROOT), "branch", "-d", branch],
                    check=True, capture_output=True, text=True,
                )

    def test_detached_head_refuses_capture_before_writing_evidence(self):
        with tempfile.TemporaryDirectory(prefix="formula-p0-branch-") as temp:
            checkout = pathlib.Path(temp) / "checkout"
            output = pathlib.Path(temp) / "evidence"
            subprocess.run(
                ["git", "-C", str(ROOT), "worktree", "add", "--detach", str(checkout), "HEAD"],
                check=True, capture_output=True, text=True,
            )
            try:
                result = subprocess.run(
                    ["bash", str(checkout / "scripts/capture_p0_source_proof.sh"), str(output)],
                    cwd=checkout, capture_output=True, text=True, timeout=30,
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("branch not admitted for P0-01 capture: DETACHED", result.stderr)
                self.assertFalse(output.exists(), "refused capture must not publish evidence")
            finally:
                subprocess.run(
                    ["git", "-C", str(ROOT), "worktree", "remove", "--force", str(checkout)],
                    check=True, capture_output=True, text=True,
                )


if __name__ == "__main__":
    unittest.main()
