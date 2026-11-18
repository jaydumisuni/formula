//! Test-only private fixture defaults; negative cases explicitly loosen modes.
use std::path::PathBuf;

pub fn private_temp_dir() -> PathBuf {
    #[cfg(unix)]
    {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| unsafe {
            libc::umask(0o077);
        });
    }
    std::env::temp_dir()
}
