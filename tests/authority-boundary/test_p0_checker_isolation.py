import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_p0_boundaries.py")
spec = importlib.util.spec_from_file_location("p0_boundaries", SCRIPT)
p0 = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(p0)


class CheckerIsolationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        crate = self.root / "crates" / "formula-check"
        crate.mkdir(parents=True)
        self.manifest = crate / "Cargo.toml"
        self.previous_root = p0.ROOT
        p0.ROOT = self.root
        (self.root / "Cargo.toml").write_text(
            "[workspace]\nmembers = []\n",
            encoding="utf-8",
        )

    def tearDown(self):
        p0.ROOT = self.previous_root
        self.tmp.cleanup()

    def test_rejects_aliased_formula_engine_dependency(self):
        self.manifest.write_text(
            "[dependencies]\nsearch-backend = { package = \"formula-engine\", path = \"../formula-engine\" }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-engine"):
            p0.check_checker_isolation()

    def test_rejects_workspace_inherited_aliased_formula_engine_dependency(self):
        (self.root / "Cargo.toml").write_text(
            "[workspace]\nmembers = [\"crates/formula-check\"]\n\n[workspace.dependencies]\nsearch-backend = { package = \"formula-engine\", path = \"crates/formula-engine\" }\n",
            encoding="utf-8",
        )
        self.manifest.write_text(
            "[dependencies]\nsearch-backend = { workspace = true }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-engine"):
            p0.check_checker_isolation()

    def test_rejects_target_specific_formula_engine_dependency(self):
        self.manifest.write_text(
            "[target.\"cfg(unix)\".dependencies]\nformula-engine = { path = \"../formula-engine\" }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-engine"):
            p0.check_checker_isolation()


    def test_rejects_transitive_formula_engine_through_core(self):
        core = self.root / "crates" / "formula-core"
        core.mkdir(parents=True)
        (core / "Cargo.toml").write_text(
            '[dependencies]\\nformula-engine = { path = "../formula-engine" }\\n',
            encoding="utf-8",
        )
        self.manifest.write_text(
            '[dependencies]\\nformula-core = { path = "../formula-core" }\\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-engine"):
            p0.check_checker_isolation()

    def test_rejects_transitive_first_light_through_target_specific_core(self):
        core = self.root / "crates" / "formula-core"
        core.mkdir(parents=True)
        (core / "Cargo.toml").write_text(
            '[target."cfg(unix)".dependencies]\\n'
            'formula-first-light = { path = "../formula-first-light" }\\n',
            encoding="utf-8",
        )
        self.manifest.write_text(
            '[dependencies]\\nformula-core = { path = "../formula-core" }\\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "formula-first-light"):
            p0.check_checker_isolation()

    def test_unrelated_search_crate_outside_checker_closure_is_allowed(self):
        core = self.root / "crates" / "formula-core"
        core.mkdir(parents=True)
        (core / "Cargo.toml").write_text(
            '[package]\\nname = "formula-core"\\nversion = "0.0.0"\\n',
            encoding="utf-8",
        )
        engine = self.root / "crates" / "formula-engine"
        engine.mkdir(parents=True)
        (engine / "Cargo.toml").write_text(
            '[package]\\nname = "formula-engine"\\nversion = "0.0.0"\\n',
            encoding="utf-8",
        )
        self.manifest.write_text(
            '[dependencies]\\nformula-core = { path = "../formula-core" }\\n',
            encoding="utf-8",
        )
        p0.check_checker_isolation()


if __name__ == "__main__":
    unittest.main()
