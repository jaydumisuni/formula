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

    def test_rejects_std_network_use_in_first_light_runtime(self):
        self.source.write_text(
            "use std::net::TcpStream;\npub fn connect() { let _ = TcpStream::connect(\"127.0.0.1:9\"); }\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(AssertionError, "network"):
            p0.check_canonical_runtime_has_no_external_dependencies()


if __name__ == "__main__":
    unittest.main()
