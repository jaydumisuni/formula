import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_p0_boundaries.py")
spec = importlib.util.spec_from_file_location("p0_boundaries", SCRIPT)
p0 = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(p0)


class NetworkFreeRuntimeTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.previous_root = p0.ROOT
        p0.ROOT = self.root
        (self.root / "Cargo.toml").write_text(
            "[workspace]\nmembers = []\n",
            encoding="utf-8",
        )
        crate = self.root / "crates" / "formula-first-light"
        (crate / "src").mkdir(parents=True)
        (crate / "Cargo.toml").write_text(
            "[package]\nname = \"formula-first-light\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
            encoding="utf-8",
        )
        (self.root / "Cargo.lock").write_text(
            "version = 4\n\n[[package]]\nname = \"formula-first-light\"\nversion = \"0.0.0\"\n",
            encoding="utf-8",
        )
        self.source = crate / "src" / "lib.rs"

    def tearDown(self):
        p0.ROOT = self.previous_root
        self.tmp.cleanup()

    def _add_unrelated_formula_core_sha2(self):
        crate = self.root / "crates" / "formula-core"
        (crate / "src").mkdir(parents=True)
        (crate / "Cargo.toml").write_text(
            "[package]\nname = \"formula-core\"\nversion = \"0.0.0\"\nedition = \"2021\"\n"
            "[dependencies]\nsha2 = \"0.10.9\"\n",
            encoding="utf-8",
        )
        (crate / "src" / "lib.rs").write_text("pub fn digest() {}\n", encoding="utf-8")

    def test_external_dependency_outside_first_light_runtime_closure_is_allowed(self):
        self._add_unrelated_formula_core_sha2()
        self.source.write_text("pub fn run() {}\n", encoding="utf-8")
        p0.check_canonical_runtime_has_no_external_dependencies()

    def test_external_dependency_reachable_from_first_light_runtime_is_rejected(self):
        self._add_unrelated_formula_core_sha2()
        crate = self.root / "crates" / "formula-first-light"
        (crate / "Cargo.toml").write_text(
            "[package]\nname = \"formula-first-light\"\nversion = \"0.0.0\"\nedition = \"2021\"\n"
            "[dependencies]\nformula-core = { path = \"../formula-core\" }\n",
            encoding="utf-8",
        )
        self.source.write_text("pub fn run() {}\n", encoding="utf-8")
        with self.assertRaisesRegex(AssertionError, "external dependencies"):
            p0.check_canonical_runtime_has_no_external_dependencies()

    def test_rejects_std_network_use_in_first_light_runtime(self):
        self.source.write_text(
            "use std::net::TcpStream;\npub fn connect() { let _ = TcpStream::connect(\"127.0.0.1:9\"); }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "network"):
            p0.check_canonical_runtime_has_no_external_dependencies()

    def test_rejects_network_use_in_nonleading_grouped_std_import(self):
        self.source.write_text(
            "use std::{io, net::TcpStream};\npub fn connect() { let _ = TcpStream::connect(\"127.0.0.1:9\"); let _ = io::empty(); }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "network"):
            p0.check_canonical_runtime_has_no_external_dependencies()


    def test_rejects_network_use_after_nested_grouped_std_import(self):
        self.source.write_text(
            "use std::{io::{self}, net::TcpStream};\npub fn connect() { let _ = TcpStream::connect(\"127.0.0.1:9\"); let _ = io::empty(); }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "network"):
            p0.check_canonical_runtime_has_no_external_dependencies()

    def test_unrelated_net_modules_do_not_trip_std_network_guard(self):
        self.source.write_text(
            "use core::net::IpAddr;\nuse crate::net::Thing;\nuse my::net::Other;\n",
            encoding="utf-8",
        )
        p0.check_canonical_runtime_has_no_external_dependencies()


if __name__ == "__main__":
    unittest.main()
