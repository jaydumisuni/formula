//! Content-addressed immutable blob storage for Formula P1.
//!
//! This slice stores exact bytes under their SHA-256 digest and verifies bytes
//! again on read. It does not publish generations or grant mathematical authority.

use formula_core::{ArtifactDigest, UniverseGeneration};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct BlobStore {
    root: PathBuf,
}

impl BlobStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Store exact bytes immutably and return their content digest.
    ///
    /// An existing digest path is accepted only when its bytes still hash to
    /// the same digest; mutation/corruption therefore fails closed.
    pub fn put(&self, bytes: &[u8]) -> io::Result<ArtifactDigest> {
        let digest = ArtifactDigest::sha256(bytes);
        let path = self.blob_path(digest);
        self.ensure_blob_directories(digest)?;

        match fs::symlink_metadata(&path) {
            Ok(_) => {
                self.verify_existing(&path, digest)?;
                return Ok(digest);
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }

        // Interrupted writes can leave temporary blobs behind. Choose an unused
        // slot without overwriting or deleting an earlier writer's evidence.
        let (temp, mut file) = {
            let mut opened = None;
            for suffix in 0..32 {
                let temp = path.with_extension(format!("tmp-{}-{suffix}", std::process::id()));
                match OpenOptions::new().write(true).create_new(true).open(&temp) {
                    Ok(file) => {
                        opened = Some((temp, file));
                        break;
                    }
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => return Err(error),
                }
            }
            opened.ok_or_else(|| {
                io::Error::new(io::ErrorKind::AlreadyExists, "blob temp slots exhausted")
            })?
        };
        let installation = (|| -> io::Result<bool> {
            file.write_all(bytes)?;
            file.sync_all()?;
            match Self::install_no_replace(&temp, &path) {
                Ok(()) => Ok(true),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    self.verify_existing(&path, digest)?;
                    Ok(false)
                }
                Err(error) => Err(error),
            }
        })();
        match installation {
            Ok(true) => {}
            Ok(false) => fs::remove_file(&temp)?,
            Err(error) => {
                let _ = fs::remove_file(&temp);
                return Err(error);
            }
        }
        fs::File::open(path.parent().expect("blob path has parent"))?.sync_all()?;
        Ok(digest)
    }

    // Linux renameat2 installs the fully synced temp file atomically, without
    // replacing a destination created by a concurrent writer. A hard-link
    // fallback is deliberately excluded: it would transiently violate the
    // single-link invariant enforced by read_verified().
    #[cfg(target_os = "linux")]
    fn install_no_replace(temp: &Path, path: &Path) -> io::Result<()> {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;

        let source = CString::new(temp.as_os_str().as_bytes())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "NUL in blob temp path"))?;
        let target = CString::new(path.as_os_str().as_bytes())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "NUL in blob target path"))?;
        // SAFETY: both CStrings are NUL-terminated and remain live for this call.
        let result = unsafe {
            libc::renameat2(
                libc::AT_FDCWD,
                source.as_ptr(),
                libc::AT_FDCWD,
                target.as_ptr(),
                libc::RENAME_NOREPLACE,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn install_no_replace(_temp: &Path, _path: &Path) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "atomic no-replace blob publication requires a qualified platform backend",
        ))
    }

    /// Persist one canonical universe-generation manifest as an immutable blob.
    ///
    /// This records manifest bytes only; it does not activate or publish the generation.
    pub fn put_generation_manifest(
        &self,
        generation: &UniverseGeneration,
    ) -> io::Result<ArtifactDigest> {
        self.put(&generation.canonical_bytes())
    }

    /// Read exact bytes only when their content still matches the requested digest.
    pub fn get(&self, digest: ArtifactDigest) -> io::Result<Vec<u8>> {
        let path = self.blob_path(digest);
        self.check_blob_directories(digest)?;
        self.read_verified(&path, digest)
    }

    #[must_use]
    pub fn blob_path(&self, digest: ArtifactDigest) -> PathBuf {
        let hex = digest.to_hex();
        self.root.join("blobs").join(&hex[..2]).join(hex)
    }

    /// Return whether an exact immutable blob is present and still valid.
    pub fn contains(&self, digest: ArtifactDigest) -> io::Result<bool> {
        let path = self.blob_path(digest);
        match self.check_blob_directories(digest) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error),
        }
        match fs::symlink_metadata(&path) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
            Ok(_) => {
                self.verify_existing(&path, digest)?;
                Ok(true)
            }
        }
    }

    fn require_real_directory(path: &Path) -> io::Result<()> {
        if !fs::symlink_metadata(path)?.file_type().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "immutable blob ancestor must be a real directory",
            ));
        }
        Ok(())
    }

    fn check_blob_directories(&self, digest: ArtifactDigest) -> io::Result<()> {
        let blobs = self.root.join("blobs");
        let hex = digest.to_hex();
        let shard = blobs.join(&hex[..2]);
        for directory in [&self.root, &blobs, &shard] {
            Self::require_real_directory(directory)?;
        }
        Ok(())
    }

    fn ensure_blob_directories(&self, digest: ArtifactDigest) -> io::Result<()> {
        fs::create_dir_all(&self.root)?;
        Self::require_real_directory(&self.root)?;
        let blobs = self.root.join("blobs");
        fs::create_dir_all(&blobs)?;
        Self::require_real_directory(&blobs)?;
        let hex = digest.to_hex();
        let shard = blobs.join(&hex[..2]);
        fs::create_dir_all(&shard)?;
        Self::require_real_directory(&shard)
    }

    fn verify_existing(&self, path: &Path, expected: ArtifactDigest) -> io::Result<()> {
        self.read_verified(path, expected).map(|_| ())
    }

    fn read_verified(&self, path: &Path, expected: ArtifactDigest) -> io::Result<Vec<u8>> {
        // Bind validation and reading to one opened inode. A pathname can be
        // swapped between symlink_metadata() and fs::read(); O_NOFOLLOW also
        // prevents a replacement symlink from redirecting the open itself.
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            // Nonblocking open prevents a swapped-in FIFO from hanging a reader.
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let mut file = options.open(path).map_err(|error| {
            #[cfg(unix)]
            if error.raw_os_error() == Some(libc::ELOOP) {
                return io::Error::new(
                    io::ErrorKind::InvalidData,
                    "immutable blob leaf must not be a symlink",
                );
            }
            error
        })?;
        let metadata = file.metadata()?;
        if !metadata.file_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "immutable blob leaf must be a regular file",
            ));
        }
        #[cfg(unix)]
        if std::os::unix::fs::MetadataExt::nlink(&metadata) != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "immutable blob leaf must not have hardlink aliases",
            ));
        }
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        if ArtifactDigest::sha256(&bytes) != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "existing blob digest mismatch",
            ));
        }
        Ok(bytes)
    }
}

/// Local generation index with an atomically replaced active-generation pointer.
///
/// Publication requires the manifest blob to exist and verify first. Historical
/// generation roots are append-only files; the active pointer is replaced only
/// after that durable history entry is synced.
#[derive(Debug)]
pub struct GenerationIndex {
    root: PathBuf,
}

impl GenerationIndex {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn require_real_root(&self) -> io::Result<()> {
        if !fs::symlink_metadata(&self.root)?.file_type().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generation index root must be a real directory",
            ));
        }
        Ok(())
    }

    /// Read an index leaf from the same inode whose type and link count were checked.
    /// A concurrent pathname replacement must not redirect a validated read.
    fn read_index_leaf(path: &Path, description: &str) -> io::Result<String> {
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let mut file = options.open(path).map_err(|error| {
            #[cfg(unix)]
            if error.raw_os_error() == Some(libc::ELOOP) {
                return io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{description} must not be a symlink"),
                );
            }
            error
        })?;
        let metadata = file.metadata()?;
        if !metadata.file_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{description} must be a regular file"),
            ));
        }
        #[cfg(unix)]
        if std::os::unix::fs::MetadataExt::nlink(&metadata) != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{description} must not have hardlink aliases"),
            ));
        }
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn publish(
        &self,
        store: &BlobStore,
        generation: u64,
        manifest: ArtifactDigest,
    ) -> io::Result<()> {
        fs::create_dir_all(&self.root)?;
        self.require_real_root()?;
        if !store.contains(manifest)? {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "generation manifest blob missing",
            ));
        }
        let active = self.root.join("active-generation");
        let active_present = match fs::symlink_metadata(&active) {
            Ok(_) => true,
            Err(error) if error.kind() == io::ErrorKind::NotFound => false,
            Err(error) => return Err(error),
        };
        if active_present {
            if !fs::symlink_metadata(&active)?.file_type().is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "active generation pointer must be a regular file",
                ));
            }
            let (current, _) = self.active_generation_pointer()?;
            if generation < current {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "generation publication cannot roll back active generation",
                ));
            }
        }
        let generations = self.root.join("generations");
        fs::create_dir_all(&generations)?;
        if !fs::symlink_metadata(&generations)?.file_type().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generation history directory must be a real directory",
            ));
        }
        // The active pointer may be absent after an interrupted publication.
        // Durable history is still a high-water authority boundary: never
        // activate an older generation when a newer history entry exists.
        for entry in fs::read_dir(&generations)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(number) = name.strip_suffix(".manifest") {
                let recorded = number.parse::<u64>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid generation history name",
                    )
                })?;
                if recorded.to_string() != number {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "non-canonical generation history name",
                    ));
                }
                if recorded > generation {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "generation publication cannot roll back durable history",
                    ));
                }
                // Even an older immutable history leaf is part of the durable
                // generation chain; corruption must block new publication.
                if recorded < generation {
                    let historical = self.historical_manifest(recorded)?;
                    if !store.contains(historical)? {
                        return Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "historical generation manifest blob missing",
                        ));
                    }
                }
            }
        }
        let history = generations.join(format!("{generation}.manifest"));
        let manifest_hex = manifest.to_hex();
        let history_present = match fs::symlink_metadata(&history) {
            Ok(metadata) => {
                if !metadata.file_type().is_file() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "generation history entry must be a regular file",
                    ));
                }
                #[cfg(unix)]
                if std::os::unix::fs::MetadataExt::nlink(&metadata) != 1 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "generation history entry must not have hardlink aliases",
                    ));
                }
                true
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => false,
            Err(error) => return Err(error),
        };
        if history_present {
            if Self::read_index_leaf(&history, "generation history entry")? != manifest_hex {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "generation already bound to another manifest",
                ));
            }
        } else {
            // A history entry must never become visible before its complete
            // digest is durable. Retain interrupted temp slots for inspection.
            let (temp, mut file) = {
                let mut opened = None;
                for suffix in 0..32 {
                    let temp = generations.join(format!(
                        "{generation}.manifest.tmp-{}-{suffix}",
                        std::process::id()
                    ));
                    match OpenOptions::new().write(true).create_new(true).open(&temp) {
                        Ok(file) => {
                            opened = Some((temp, file));
                            break;
                        }
                        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                        Err(error) => return Err(error),
                    }
                }
                opened.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::AlreadyExists, "history temp slots exhausted")
                })?
            };
            file.write_all(manifest_hex.as_bytes())?;
            file.sync_all()?;
            match BlobStore::install_no_replace(&temp, &history) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    // A competing publisher won the race. Only the same exact
                    // canonical digest is acceptable for this generation.
                    if self.historical_manifest(generation)? != manifest {
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            "generation already bound to another manifest",
                        ));
                    }
                    fs::remove_file(&temp)?;
                }
                Err(error) => return Err(error),
            }
            fs::File::open(&generations)?.sync_all()?;
        }

        // An interrupted publication can leave its temporary pointer behind.
        // Never overwrite it or let it permanently prevent a later publication.
        let (temp, mut file) = {
            let mut opened = None;
            for suffix in 0..32 {
                let temp = self.root.join(format!(
                    "active-generation.tmp-{}-{suffix}",
                    std::process::id()
                ));
                match OpenOptions::new().write(true).create_new(true).open(&temp) {
                    Ok(file) => {
                        opened = Some((temp, file));
                        break;
                    }
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => return Err(error),
                }
            }
            opened.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "generation pointer temp slots exhausted",
                )
            })?
        };
        if let Err(error) = (|| -> io::Result<()> {
            write!(file, "{generation}\n{manifest_hex}")?;
            file.sync_all()?;
            fs::rename(&temp, &active)?;
            fs::File::open(&self.root)?.sync_all()?;
            Ok(())
        })() {
            let _ = fs::remove_file(&temp);
            return Err(error);
        }
        Ok(())
    }

    pub fn historical_manifest(&self, generation: u64) -> io::Result<ArtifactDigest> {
        self.require_real_root()?;
        let generations = self.root.join("generations");
        if !fs::symlink_metadata(&generations)?.file_type().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generation history directory must be a real directory",
            ));
        }
        let history = self
            .root
            .join("generations")
            .join(format!("{generation}.manifest"));
        let metadata = fs::symlink_metadata(&history)?;
        if !metadata.file_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generation history entry must be a regular file",
            ));
        }
        #[cfg(unix)]
        if std::os::unix::fs::MetadataExt::nlink(&metadata) != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "generation history entry must not have hardlink aliases",
            ));
        }
        let hex = Self::read_index_leaf(&history, "generation history entry")?;
        let digest = ArtifactDigest::from_hex(hex.trim()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid historical manifest digest",
            )
        })?;
        if hex != digest.to_hex() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "non-canonical historical manifest digest",
            ));
        }
        Ok(digest)
    }

    /// Recover the exact active generation and manifest digest from durable state.
    ///
    /// Replay fails closed on malformed or non-canonical pointer contents instead
    /// of guessing a generation or accepting an ambiguous digest identity.
    fn active_generation_pointer(&self) -> io::Result<(u64, ArtifactDigest)> {
        self.require_real_root()?;
        let active = self.root.join("active-generation");
        let metadata = fs::symlink_metadata(&active)?;
        if !metadata.file_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "active generation pointer must be a regular file",
            ));
        }
        #[cfg(unix)]
        if std::os::unix::fs::MetadataExt::nlink(&metadata) != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "active generation pointer must not have hardlink aliases",
            ));
        }
        let pointer = Self::read_index_leaf(&active, "active generation pointer")?;
        let mut lines = pointer.lines();
        let generation = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "active generation missing"))?
            .parse::<u64>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid active generation"))?;
        let manifest = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "active manifest missing"))?;
        if lines.next().is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "active generation pointer has trailing data",
            ));
        }
        let digest = ArtifactDigest::from_hex(manifest).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "invalid active manifest digest")
        })?;
        if pointer != format!("{generation}\n{}", digest.to_hex()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "non-canonical active generation pointer",
            ));
        }
        if self.historical_manifest(generation)? != digest {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "active manifest differs from generation history",
            ));
        }
        Ok((generation, digest))
    }

    /// Replay must reject an active pointer older than durable history.
    pub fn active_generation(&self) -> io::Result<(u64, ArtifactDigest)> {
        let (generation, digest) = self.active_generation_pointer()?;
        for entry in fs::read_dir(self.root.join("generations"))? {
            let name = entry?.file_name();
            let name = name.to_string_lossy();
            if let Some(number) = name.strip_suffix(".manifest") {
                let recorded = number.parse::<u64>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid generation history name",
                    )
                })?;
                if recorded.to_string() != number || recorded > generation {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "active generation trails or conflicts with durable history",
                    ));
                }
                // A valid active pointer cannot mask corruption in older
                // append-only history entries.
                self.historical_manifest(recorded)?;
            }
        }
        Ok((generation, digest))
    }

    /// Replay an active generation only after verifying every referenced
    /// immutable manifest blob, including the historical chain.
    pub fn verified_active_generation(
        &self,
        store: &BlobStore,
    ) -> io::Result<(u64, ArtifactDigest)> {
        let active = self.active_generation()?;
        for entry in fs::read_dir(self.root.join("generations"))? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(number) = name.strip_suffix(".manifest") {
                let generation = number.parse::<u64>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid generation history name",
                    )
                })?;
                let digest = self.historical_manifest(generation)?;
                if !store.contains(digest)? {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "historical generation manifest blob missing",
                    ));
                }
            }
        }
        if !store.contains(active.1)? {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "active generation manifest blob missing",
            ));
        }
        Ok(active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("formula-store-{name}-{}", std::process::id()))
    }

    #[test]
    fn put_and_get_are_content_addressed() {
        let root = test_root("roundtrip");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let digest = store.put(b"canonical artifact").unwrap();
        assert_eq!(digest, ArtifactDigest::sha256(b"canonical artifact"));
        assert_eq!(store.get(digest).unwrap(), b"canonical artifact");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generation_manifest_roundtrips_as_canonical_immutable_bytes() {
        let root = test_root("generation-manifest");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let generation = UniverseGeneration::new(
            7,
            Some(ArtifactDigest::sha256(b"parent")),
            ArtifactDigest::sha256(b"world"),
            ArtifactDigest::sha256(b"authority"),
            vec![ArtifactDigest::sha256(b"evidence")],
            vec![ArtifactDigest::sha256(b"realization")],
        );
        let expected = generation.canonical_bytes();
        let digest = store.put_generation_manifest(&generation).unwrap();
        assert_eq!(digest, ArtifactDigest::sha256(&expected));
        assert_eq!(store.get(digest).unwrap(), expected);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn duplicate_put_reuses_same_immutable_blob() {
        let root = test_root("duplicate");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let first = store.put(b"same bytes").unwrap();
        let second = store.put(b"same bytes").unwrap();
        assert_eq!(first, second);
        assert_eq!(store.get(first).unwrap(), b"same bytes");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn contains_distinguishes_missing_valid_and_corrupt_blobs() {
        let root = test_root("contains");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let missing = ArtifactDigest::sha256(b"missing");
        assert!(!store.contains(missing).unwrap());
        let digest = store.put(b"present").unwrap();
        assert!(store.contains(digest).unwrap());
        fs::write(store.blob_path(digest), b"corrupt").unwrap();
        assert_eq!(
            store.contains(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generation_publication_requires_present_manifest_and_preserves_history() {
        let root = test_root("generation-index");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let missing = ArtifactDigest::sha256(b"missing");
        assert_eq!(
            index.publish(&store, 1, missing).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
        assert!(!root.join("index/active-generation").exists());

        let manifest = store.put(b"generation-one").unwrap();
        index.publish(&store, 1, manifest).unwrap();
        assert_eq!(index.historical_manifest(1).unwrap(), manifest);
        assert_eq!(
            fs::read_to_string(root.join("index/active-generation")).unwrap(),
            format!("1\n{}", manifest.to_hex())
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn active_generation_replays_exact_published_pointer() {
        let root = test_root("active-replay");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let manifest = store.put(b"generation-seven").unwrap();
        index.publish(&store, 7, manifest).unwrap();

        assert_eq!(index.active_generation().unwrap(), (7, manifest));

        fs::write(
            root.join("index/active-generation"),
            format!("7\n{}\ntrailing", manifest.to_hex()),
        )
        .unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(
            root.join("index/active-generation"),
            format!("07\n{}", manifest.to_hex()),
        )
        .unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        let unrelated = store.put(b"unrelated-generation").unwrap();
        fs::write(
            root.join("index/active-generation"),
            format!("7\n{}", unrelated.to_hex()),
        )
        .unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        fs::write(
            root.join("index/active-generation"),
            format!("7\n{}", manifest.to_hex()),
        )
        .unwrap();
        fs::write(
            root.join("index/generations/7.manifest"),
            format!("{}\n", manifest.to_hex()),
        )
        .unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generation_history_cannot_be_rebound() {
        let root = test_root("generation-rebind");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let first = store.put(b"first").unwrap();
        let second = store.put(b"second").unwrap();
        index.publish(&store, 4, first).unwrap();
        assert_eq!(
            index.publish(&store, 4, second).unwrap_err().kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(index.historical_manifest(4).unwrap(), first);
        assert_eq!(index.active_generation().unwrap(), (4, first));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn missing_active_pointer_cannot_roll_back_persisted_generation_history() {
        let root = test_root("history-high-water");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let older = store.put(b"generation-eight").unwrap();
        let newer = store.put(b"generation-nine").unwrap();
        index.publish(&store, 9, newer).unwrap();
        fs::remove_file(root.join("index/active-generation")).unwrap();

        assert_eq!(
            index.publish(&store, 8, older).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(index.historical_manifest(9).unwrap(), newer);
        assert!(!root.join("index/generations/8.manifest").exists());
        assert!(!root.join("index/active-generation").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn stale_active_pointer_cannot_hide_newer_generation_history() {
        let root = test_root("stale-active-high-water");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let older = store.put(b"generation-eight").unwrap();
        let newer = store.put(b"generation-nine").unwrap();
        index.publish(&store, 8, older).unwrap();
        index.publish(&store, 9, newer).unwrap();
        fs::write(
            root.join("index/active-generation"),
            format!("8\n{}", older.to_hex()),
        )
        .unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        index
            .publish(&store, 10, store.put(b"generation-ten").unwrap())
            .unwrap();
        assert_eq!(index.active_generation().unwrap().0, 10);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_blob_leaf_cannot_impersonate_matching_content() {
        use std::os::unix::fs::symlink;

        let root = test_root("symlink-leaf");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let outside = root.join("outside");
        let bytes = b"matching-external-content";
        fs::write(&outside, bytes).unwrap();
        let store = BlobStore::new(root.join("store"));
        let digest = ArtifactDigest::sha256(bytes);
        let leaf = store.blob_path(digest);
        fs::create_dir_all(leaf.parent().unwrap()).unwrap();
        symlink(&outside, &leaf).unwrap();

        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.contains(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.put(bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(fs::read(&outside).unwrap(), bytes);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn hardlinked_blob_leaf_cannot_impersonate_immutable_content() {
        let root = test_root("hardlink-leaf");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let outside = root.join("outside");
        let bytes = b"matching-hardlinked-content";
        fs::write(&outside, bytes).unwrap();
        let store = BlobStore::new(root.join("store"));
        let digest = ArtifactDigest::sha256(bytes);
        let leaf = store.blob_path(digest);
        fs::create_dir_all(leaf.parent().unwrap()).unwrap();
        fs::hard_link(&outside, &leaf).unwrap();

        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.contains(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.put(bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(fs::read(&outside).unwrap(), bytes);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn directory_blob_leaf_is_rejected_without_modification() {
        let root = test_root("directory-leaf");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let bytes = b"directory-cannot-be-a-blob";
        let digest = ArtifactDigest::sha256(bytes);
        let leaf = store.blob_path(digest);
        fs::create_dir_all(&leaf).unwrap();

        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.contains(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.put(bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(leaf.is_dir());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_blob_directory_cannot_redirect_immutable_storage() {
        use std::os::unix::fs::symlink;

        let root = test_root("symlink-blob-directory");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("outside")).unwrap();
        fs::create_dir_all(root.join("store")).unwrap();
        symlink(root.join("outside"), root.join("store/blobs")).unwrap();

        let store = BlobStore::new(root.join("store"));
        let bytes = b"no-external-blob-write";
        let digest = ArtifactDigest::sha256(bytes);
        assert_eq!(
            store.put(bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("outside").join(&digest.to_hex()[..2]).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_blob_shard_cannot_redirect_reads_or_writes() {
        use std::os::unix::fs::symlink;

        let root = test_root("symlink-blob-shard");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("outside")).unwrap();
        fs::create_dir_all(root.join("store/blobs")).unwrap();

        let bytes = b"no-external-shard-write";
        let digest = ArtifactDigest::sha256(bytes);
        symlink(
            root.join("outside"),
            root.join("store/blobs").join(&digest.to_hex()[..2]),
        )
        .unwrap();

        let store = BlobStore::new(root.join("store"));
        assert_eq!(
            store.put(bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.contains(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("outside").join(digest.to_hex()).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn mutated_blob_is_rejected_on_read_and_reput() {
        let root = test_root("mutation");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let digest = store.put(b"original").unwrap();
        fs::write(store.blob_path(digest), b"tampered").unwrap();
        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            store.put(b"original").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }
}
