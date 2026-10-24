//! Content-addressed immutable blob storage for Formula P1.
//!
//! This slice stores exact bytes under their SHA-256 digest and verifies bytes
//! again on read. It does not publish generations or grant mathematical authority.

use formula_core::{ArtifactDigest, UniverseGeneration};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
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
                let mut options = OpenOptions::new();
                options.write(true).create_new(true);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.mode(0o600);
                }
                match options.open(&temp) {
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
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.file_type().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "immutable authority directory must be a real directory",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            if !Self::authority_directory_custody_permitted(
                metadata.permissions().mode(),
                metadata.uid(),
                unsafe { libc::geteuid() },
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "immutable authority directory must have trusted ownership and private writes",
                ));
            }
        }
        Ok(())
    }

    // Unlike a sticky shared ancestor, an authority directory itself must
    // never be shared-writable. Its owner must be root or the current user:
    // a foreign owner can replace its children even with mode 0755.
    #[cfg(unix)]
    fn authority_directory_custody_permitted(
        mode: u32,
        owner_uid: u32,
        effective_uid: u32,
    ) -> bool {
        (owner_uid == 0 || owner_uid == effective_uid) && mode & 0o022 == 0
    }

    // Blob and generation-index leaves can be edited by their owner even
    // with mode 0600. Only this process or root may own immutable authority.
    #[cfg(unix)]
    fn authority_leaf_custody_permitted(mode: u32, owner_uid: u32, effective_uid: u32) -> bool {
        Self::authority_directory_custody_permitted(mode, owner_uid, effective_uid)
    }

    // Every authority ancestor must be owned by root or by this process:
    // a foreign owner can replace descendants even when its directory is 0755.
    // A shared-writable trusted ancestor is safe only with sticky protection.
    #[cfg(unix)]
    fn ancestor_custody_permitted(mode: u32, owner_uid: u32, effective_uid: u32) -> bool {
        (owner_uid == 0 || owner_uid == effective_uid) && (mode & 0o022 == 0 || mode & 0o1000 != 0)
    }

    // A real store root is insufficient if a parent redirects it through a
    // symlink. Check existing ancestors before creating any directories.
    fn require_real_ancestors(path: &Path) -> io::Result<()> {
        // Relative roots change meaning with process cwd. A lexical ".."
        // component can traverse a different directory than the one whose
        // custody the caller intended to verify. Reject both before any I/O.
        if !path.is_absolute()
            || path.components().any(|part| {
                matches!(
                    part,
                    std::path::Component::ParentDir | std::path::Component::CurDir
                )
            })
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "immutable authority root requires an absolute path without traversal",
            ));
        }
        for ancestor in path.ancestors() {
            match fs::symlink_metadata(ancestor) {
                Ok(metadata) if metadata.file_type().is_dir() => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::{MetadataExt, PermissionsExt};
                        let mode = metadata.permissions().mode();
                        // A foreign-owned sticky ancestor is not protected:
                        // its owner can replace a private authority child.
                        if !Self::ancestor_custody_permitted(mode, metadata.uid(), unsafe {
                            libc::geteuid()
                        }) {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "immutable authority ancestor lacks trusted sticky ownership",
                            ));
                        }
                    }
                }
                Ok(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "immutable blob ancestor must be a real directory",
                    ));
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    fn check_blob_directories(&self, digest: ArtifactDigest) -> io::Result<()> {
        Self::require_real_ancestors(&self.root)?;
        let blobs = self.root.join("blobs");
        let hex = digest.to_hex();
        let shard = blobs.join(&hex[..2]);
        for directory in [&self.root, &blobs, &shard] {
            Self::require_real_directory(directory)?;
        }
        Ok(())
    }

    // A synced leaf is not crash-durable if an ancestor directory entry was
    // just created but never synced. On first creation, sync the root and its
    // ancestors from child to parent so each newly installed name is durable.
    // Bind every durability barrier to a real directory inode, including
    // ordinary parent-entry and generation-pointer fsyncs. Pathname preflight
    // alone cannot prevent a symlink replacement before a later open().
    fn sync_real_directory(path: &Path) -> io::Result<()> {
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC);
        }
        let directory = options.open(path)?;
        #[cfg(not(unix))]
        if !directory.metadata()?.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "durability barrier target must be a directory",
            ));
        }
        directory.sync_all()
    }

    fn sync_directory_chain(path: &Path) -> io::Result<()> {
        for directory in path.ancestors() {
            Self::sync_real_directory(directory)?;
        }
        Ok(())
    }

    fn directory_missing(path: &Path) -> io::Result<bool> {
        match fs::symlink_metadata(path) {
            Ok(_) => Ok(false),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(true),
            Err(error) => Err(error),
        }
    }

    fn ensure_blob_directories(&self, digest: ArtifactDigest) -> io::Result<()> {
        Self::require_real_ancestors(&self.root)?;
        let new_root = Self::directory_missing(&self.root)?;
        fs::create_dir_all(&self.root)?;
        Self::require_real_ancestors(&self.root)?;
        Self::require_real_directory(&self.root)?;
        if new_root {
            Self::sync_directory_chain(&self.root)?;
        }
        let blobs = self.root.join("blobs");
        let new_blobs = Self::directory_missing(&blobs)?;
        fs::create_dir_all(&blobs)?;
        Self::require_real_directory(&blobs)?;
        if new_blobs {
            BlobStore::sync_real_directory(&self.root)?;
        }
        let hex = digest.to_hex();
        let shard = blobs.join(&hex[..2]);
        let new_shard = Self::directory_missing(&shard)?;
        fs::create_dir_all(&shard)?;
        Self::require_real_directory(&shard)?;
        if new_shard {
            BlobStore::sync_real_directory(&blobs)?;
        }
        Ok(())
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
        Self::read_verified_opened(&mut file, path, expected)
    }

    // Validate hostile/corrupt blob contents without first allocating a Vec
    // sized by an untrusted file. Valid blobs still return exact bytes.
    fn digest_reader(reader: &mut impl Read) -> io::Result<ArtifactDigest> {
        let mut hasher = Sha256::new();
        let mut chunk = [0_u8; 64 * 1024];
        loop {
            let count = match reader.read(&mut chunk) {
                Ok(count) => count,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            };
            if count == 0 {
                break;
            }
            hasher.update(&chunk[..count]);
        }
        Ok(ArtifactDigest::from_bytes(hasher.finalize().into()))
    }

    // Keep the opened inode and its pathname bound through verification. A
    // digest match alone does not prove that the name still resolves to it.
    fn read_verified_opened(
        file: &mut fs::File,
        path: &Path,
        expected: ArtifactDigest,
    ) -> io::Result<Vec<u8>> {
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
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            if !Self::authority_leaf_custody_permitted(
                metadata.permissions().mode(),
                metadata.uid(),
                unsafe { libc::geteuid() },
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "immutable blob leaf lacks trusted ownership or private writes",
                ));
            }
        }
        // A corrupt sparse/oversized blob must not force a correspondingly
        // large allocation merely because its filename claims a known digest.
        if Self::digest_reader(file)? != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "existing blob digest mismatch",
            ));
        }
        let preflight_metadata = file.metadata()?;
        if preflight_metadata.len() != metadata.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "immutable blob leaf changed during digest preflight",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let named = fs::symlink_metadata(path)?;
            if !GenerationIndex::index_leaf_metadata_stable(&metadata, &preflight_metadata)
                || named.dev() != metadata.dev()
                || named.ino() != metadata.ino()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "immutable blob leaf changed during digest preflight",
                ));
            }
        }
        file.seek(SeekFrom::Start(0))?;
        let mut bytes = Vec::new();
        (&mut *file)
            .take(metadata.len().saturating_add(1))
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 != metadata.len() || ArtifactDigest::sha256(&bytes) != expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "existing blob digest mismatch",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let after = file.metadata()?;
            let named = fs::symlink_metadata(path)?;
            if !GenerationIndex::index_leaf_metadata_stable(&metadata, &after)
                || named.dev() != metadata.dev()
                || named.ino() != metadata.ino()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "immutable blob leaf changed during verified read",
                ));
            }
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
        BlobStore::require_real_ancestors(&self.root)?;
        BlobStore::require_real_directory(&self.root)
    }

    /// Reject malformed or newer history entries during the second verified scan.
    fn verified_history_entry_number(
        name: &str,
        active_generation: u64,
    ) -> io::Result<Option<u64>> {
        let Some(number) = name.strip_suffix(".manifest") else {
            return Ok(None);
        };
        let recorded = number.parse::<u64>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid generation history name",
            )
        })?;
        if recorded.to_string() != number || recorded > active_generation {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "verified generation history is noncanonical or newer than active",
            ));
        }
        Ok(Some(recorded))
    }

    #[cfg(unix)]
    fn index_leaf_metadata_stable(before: &fs::Metadata, after: &fs::Metadata) -> bool {
        use std::os::unix::fs::MetadataExt;
        before.is_file()
            && after.is_file()
            && before.nlink() == 1
            && after.nlink() == 1
            && before.dev() == after.dev()
            && before.ino() == after.ino()
            && before.len() == after.len()
            && before.mtime() == after.mtime()
            && before.mtime_nsec() == after.mtime_nsec()
            && before.ctime() == after.ctime()
            && before.ctime_nsec() == after.ctime_nsec()
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
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            if !BlobStore::authority_leaf_custody_permitted(
                metadata.permissions().mode(),
                metadata.uid(),
                unsafe { libc::geteuid() },
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{description} lacks trusted ownership or private writes"),
                ));
            }
        }
        // Generation history leaves contain one digest; the active pointer adds
        // only a decimal generation and separator. Bound both metadata and
        // actual bytes read so concurrent growth cannot exhaust memory.
        const MAX_INDEX_LEAF_BYTES: u64 = 128;
        if metadata.len() > MAX_INDEX_LEAF_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{description} exceeds the maximum index leaf size"),
            ));
        }
        let mut content = String::new();
        (&mut file)
            .take(MAX_INDEX_LEAF_BYTES + 1)
            .read_to_string(&mut content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let after = file.metadata()?;
            let named = fs::symlink_metadata(path)?;
            if !Self::index_leaf_metadata_stable(&metadata, &after)
                || named.dev() != metadata.dev()
                || named.ino() != metadata.ino()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{description} changed during verified read"),
                ));
            }
        }
        if content.len() as u64 > MAX_INDEX_LEAF_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{description} exceeds the maximum index leaf size"),
            ));
        }
        Ok(content)
    }

    pub fn publish(
        &self,
        store: &BlobStore,
        generation: u64,
        manifest: ArtifactDigest,
    ) -> io::Result<()> {
        BlobStore::require_real_ancestors(&self.root)?;
        let new_root = BlobStore::directory_missing(&self.root)?;
        fs::create_dir_all(&self.root)?;
        self.require_real_root()?;
        if new_root {
            BlobStore::sync_directory_chain(&self.root)?;
        }
        // Hold the index directory inode's advisory lock across the complete
        // high-water scan, immutable history installation, and active-pointer
        // rename/fsync. A pathname lock file would itself be replaceable.
        #[cfg(unix)]
        let _publication_lock = {
            use std::os::fd::AsRawFd;
            use std::os::unix::fs::OpenOptionsExt;
            let directory = OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
                .open(&self.root)?;
            if unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_EX) } != 0 {
                return Err(io::Error::last_os_error());
            }
            directory
        };
        #[cfg(not(unix))]
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "generation publication requires an exclusive directory lock",
        ));
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
        let new_generations = BlobStore::directory_missing(&generations)?;
        fs::create_dir_all(&generations)?;
        BlobStore::require_real_directory(&generations)?;
        if new_generations {
            BlobStore::sync_real_directory(&self.root)?;
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
                    let open_result = {
                        let mut opts = OpenOptions::new();
                        opts.write(true).create_new(true);
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::OpenOptionsExt;
                            opts.mode(0o600);
                        }
                        opts.open(&temp)
                    };
                    match open_result {
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
            BlobStore::sync_real_directory(&generations)?;
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
                let open_result = {
                    let mut opts = OpenOptions::new();
                    opts.write(true).create_new(true);
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::OpenOptionsExt;
                        opts.mode(0o600);
                    }
                    opts.open(&temp)
                };
                match open_result {
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
            BlobStore::sync_real_directory(&self.root)?;
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
        BlobStore::require_real_directory(&generations)?;
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
            if let Some(generation) = Self::verified_history_entry_number(&name, active.0)? {
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

    #[cfg(unix)]
    #[test]
    fn directory_sync_rejects_redirected_and_nondirectory_components() {
        use std::os::unix::fs::symlink;

        let root = test_root("directory-sync-custody");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("real")).unwrap();
        symlink(root.join("real"), root.join("alias")).unwrap();
        fs::write(root.join("regular"), b"not a directory").unwrap();

        // The directory durability barrier must never follow a replacement
        // symlink or silently fsync an attacker-supplied regular file.
        assert!(BlobStore::sync_directory_chain(&root.join("alias")).is_err());
        assert!(BlobStore::sync_directory_chain(&root.join("regular")).is_err());
        assert!(BlobStore::sync_directory_chain(&root.join("real")).is_ok());
        // Every single-directory fsync used during publication must enforce
        // the same no-follow, directory-only rule as the ancestor barrier.
        assert!(BlobStore::sync_real_directory(&root.join("alias")).is_err());
        assert!(BlobStore::sync_real_directory(&root.join("regular")).is_err());
        assert!(BlobStore::sync_real_directory(&root.join("real")).is_ok());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn verified_history_rescan_rejects_noncanonical_or_newer_entries() {
        assert_eq!(
            GenerationIndex::verified_history_entry_number("1.manifest", 1).unwrap(),
            Some(1)
        );
        assert_eq!(
            GenerationIndex::verified_history_entry_number("stale.tmp", 1).unwrap(),
            None
        );
        for invalid in [
            "01.manifest",
            "2.manifest",
            "-1.manifest",
            "18446744073709551616.manifest",
        ] {
            assert_eq!(
                GenerationIndex::verified_history_entry_number(invalid, 1)
                    .unwrap_err()
                    .kind(),
                io::ErrorKind::InvalidData
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn index_leaf_metadata_guard_detects_in_place_and_replaced_inode() {
        use std::os::unix::fs::MetadataExt;
        let root = test_root("index-leaf-metadata");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let leaf = root.join("active-generation");
        fs::write(&leaf, b"first").unwrap();
        let before = fs::metadata(&leaf).unwrap();
        assert!(GenerationIndex::index_leaf_metadata_stable(
            &before,
            &fs::metadata(&leaf).unwrap()
        ));

        fs::write(&leaf, b"second-longer").unwrap();
        let changed = fs::metadata(&leaf).unwrap();
        assert_eq!(before.ino(), changed.ino());
        assert!(!GenerationIndex::index_leaf_metadata_stable(
            &before, &changed
        ));

        let replacement = root.join("replacement");
        fs::write(&replacement, b"second-longer").unwrap();
        fs::rename(&replacement, &leaf).unwrap();
        let replaced = fs::metadata(&leaf).unwrap();
        assert_ne!(changed.ino(), replaced.ino());
        assert!(!GenerationIndex::index_leaf_metadata_stable(
            &changed, &replaced
        ));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn index_leaf_read_is_bounded() {
        let root = test_root("index-leaf-bound");
        fs::create_dir_all(&root).unwrap();
        let leaf = root.join("oversized.manifest");
        fs::write(&leaf, vec![b'a'; 129]).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&leaf, fs::Permissions::from_mode(0o600)).unwrap();
        }
        assert_eq!(
            GenerationIndex::read_index_leaf(&leaf, "test index leaf")
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(&leaf, vec![b'a'; 128]).unwrap();
        assert_eq!(
            GenerationIndex::read_index_leaf(&leaf, "test index leaf")
                .unwrap()
                .len(),
            128
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn streaming_digest_preflight_reads_bounded_chunks() {
        struct BoundedReader {
            remaining: usize,
        }
        impl Read for BoundedReader {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                assert!(buffer.len() <= 64 * 1024);
                let count = self.remaining.min(buffer.len());
                buffer[..count].fill(0x5a);
                self.remaining -= count;
                Ok(count)
            }
        }
        let count = 1024 * 1024 + 17;
        let digest = BlobStore::digest_reader(&mut BoundedReader { remaining: count }).unwrap();
        assert_eq!(digest, ArtifactDigest::sha256(&vec![0x5a; count]));
    }

    #[test]
    fn streaming_digest_preflight_retries_interrupted_reads() {
        struct InterruptedOnce {
            interrupted: bool,
            offset: usize,
        }
        impl Read for InterruptedOnce {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                if !self.interrupted {
                    self.interrupted = true;
                    return Err(io::Error::from(io::ErrorKind::Interrupted));
                }
                let content = b"interrupted stream resumes";
                let count = (content.len() - self.offset).min(buffer.len());
                buffer[..count].copy_from_slice(&content[self.offset..self.offset + count]);
                self.offset += count;
                Ok(count)
            }
        }
        let digest = BlobStore::digest_reader(&mut InterruptedOnce {
            interrupted: false,
            offset: 0,
        })
        .unwrap();
        assert_eq!(
            digest,
            ArtifactDigest::sha256(b"interrupted stream resumes")
        );
    }

    #[test]
    fn streaming_digest_preflight_propagates_noninterrupted_errors() {
        struct DeniedReader;
        impl Read for DeniedReader {
            fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::from(io::ErrorKind::PermissionDenied))
            }
        }
        assert_eq!(
            BlobStore::digest_reader(&mut DeniedReader)
                .unwrap_err()
                .kind(),
            io::ErrorKind::PermissionDenied
        );
    }

    #[cfg(unix)]
    #[test]
    fn replaced_blob_pathname_cannot_validate_old_open_inode() {
        let root = test_root("blob-replaced-during-read");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(&root);
        let digest = store.put(b"original immutable bytes").unwrap();
        let path = store.blob_path(digest);
        let mut opened = OpenOptions::new().read(true).open(&path).unwrap();

        // Even matching bytes at a new pathname inode cannot make the old
        // opened inode authoritative for the current digest path.
        let replacement = root.join("replacement");
        fs::write(&replacement, b"original immutable bytes").unwrap();
        fs::rename(&replacement, &path).unwrap();
        assert_eq!(
            BlobStore::read_verified_opened(&mut opened, &path, digest)
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn foreign_owned_sticky_ancestor_cannot_claim_authority() {
        let current = unsafe { libc::geteuid() };
        let foreign = if current == u32::MAX { 1 } else { current + 1 };
        assert!(!BlobStore::ancestor_custody_permitted(
            0o1777, foreign, current
        ));
        assert!(!BlobStore::ancestor_custody_permitted(
            0o0777, current, current
        ));
        assert!(BlobStore::ancestor_custody_permitted(
            0o1777, current, current
        ));
        assert!(BlobStore::ancestor_custody_permitted(0o1777, 0, current));
        // Even a non-writable-by-others ancestor is replaceable by its
        // foreign owner, who can rename its authority descendants.
        assert!(!BlobStore::ancestor_custody_permitted(
            0o0755, foreign, current
        ));
        assert!(BlobStore::ancestor_custody_permitted(
            0o0755, current, current
        ));
    }

    #[cfg(unix)]
    #[test]
    fn immutable_authority_leaf_requires_trusted_owner_and_private_writes() {
        let current = unsafe { libc::geteuid() };
        let foreign = if current == u32::MAX { 1 } else { current + 1 };
        assert!(!BlobStore::authority_leaf_custody_permitted(
            0o600, foreign, current
        ));
        assert!(!BlobStore::authority_leaf_custody_permitted(
            0o644, foreign, current
        ));
        assert!(BlobStore::authority_leaf_custody_permitted(
            0o600, current, current
        ));
        assert!(BlobStore::authority_leaf_custody_permitted(
            0o644, 0, current
        ));
        assert!(!BlobStore::authority_leaf_custody_permitted(
            0o622, current, current
        ));
        assert!(!BlobStore::authority_leaf_custody_permitted(
            0o666, current, current
        ));
    }

    #[cfg(unix)]
    #[test]
    fn foreign_owned_private_authority_directories_are_rejected() {
        let current = unsafe { libc::geteuid() };
        let foreign = if current == u32::MAX { 1 } else { current + 1 };
        // A foreign owner can replace children of a 0755 directory despite
        // the absence of group/world write permission.
        assert!(!BlobStore::authority_directory_custody_permitted(
            0o755, foreign, current
        ));
        assert!(BlobStore::authority_directory_custody_permitted(
            0o755, current, current
        ));
        assert!(BlobStore::authority_directory_custody_permitted(
            0o755, 0, current
        ));
        assert!(!BlobStore::authority_directory_custody_permitted(
            0o777, current, current
        ));
        assert!(!BlobStore::authority_directory_custody_permitted(
            0o1777, current, current
        ));
    }

    #[cfg(unix)]
    #[test]
    fn nonsticky_shared_writable_ancestor_cannot_host_authority() {
        use std::os::unix::fs::PermissionsExt;

        let root = test_root("writable-parent-ancestor");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o777)).unwrap();

        let store = BlobStore::new(root.join("store"));
        assert_eq!(
            store.put(b"unsafe-parent").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("store").exists());

        // A sticky shared directory, such as /tmp, protects existing
        // owned entries and remains supported.
        fs::set_permissions(&root, fs::Permissions::from_mode(0o1777)).unwrap();
        let digest = store.put(b"sticky-parent-ok").unwrap();
        let index = GenerationIndex::new(root.join("index"));
        index.publish(&store, 0, digest).unwrap();

        // Both existing blob and generation authority must also fail closed
        // if their ancestor becomes replaceable after initial creation.
        fs::set_permissions(&root, fs::Permissions::from_mode(0o777)).unwrap();
        assert_eq!(
            store.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 1, digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        fs::set_permissions(&root, fs::Permissions::from_mode(0o1777)).unwrap();
        assert_eq!(store.get(digest).unwrap(), b"sticky-parent-ok");
        assert_eq!(index.active_generation().unwrap(), (0, digest));
        fs::set_permissions(&root, fs::Permissions::from_mode(0o755)).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn shared_writable_blob_shard_cannot_claim_immutable_authority() {
        use std::os::unix::fs::PermissionsExt;

        let root = test_root("writable-blob-shard");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let bytes = b"shard-directory-authority";
        let digest = store.put(bytes).unwrap();
        let shard = store.blob_path(digest).parent().unwrap().to_path_buf();
        fs::set_permissions(&shard, fs::Permissions::from_mode(0o777)).unwrap();
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
        fs::set_permissions(&shard, fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(store.get(digest).unwrap(), bytes);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn shared_writable_index_directories_cannot_claim_generation_authority() {
        use std::os::unix::fs::PermissionsExt;

        let root = test_root("writable-index-directories");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let manifest = store.put(b"index-directory-authority").unwrap();
        index.publish(&store, 0, manifest).unwrap();

        let history_dir = root.join("index/generations");
        fs::set_permissions(&history_dir, fs::Permissions::from_mode(0o777)).unwrap();
        assert_eq!(
            index.historical_manifest(0).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 1, manifest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::set_permissions(&history_dir, fs::Permissions::from_mode(0o755)).unwrap();

        let index_root = root.join("index");
        fs::set_permissions(&index_root, fs::Permissions::from_mode(0o777)).unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            index.publish(&store, 1, manifest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::set_permissions(&index_root, fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(index.active_generation().unwrap(), (0, manifest));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn shared_writable_blob_leaf_cannot_claim_immutable_authority() {
        use std::os::unix::fs::PermissionsExt;

        let root = test_root("writable-blob-leaf");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let bytes = b"immutable-blob-permissions";
        let digest = store.put(bytes).unwrap();
        let leaf = store.blob_path(digest);
        let original = fs::metadata(&leaf).unwrap().permissions();
        assert_eq!(original.mode() & 0o077, 0);

        fs::set_permissions(&leaf, fs::Permissions::from_mode(0o666)).unwrap();
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

        fs::set_permissions(&leaf, original).unwrap();
        assert_eq!(store.get(digest).unwrap(), bytes);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn shared_writable_index_leaves_cannot_claim_generation_authority() {
        use std::os::unix::fs::PermissionsExt;

        let root = test_root("writable-index-leaves");
        let _ = fs::remove_dir_all(&root);
        let store = BlobStore::new(root.join("store"));
        let index = GenerationIndex::new(root.join("index"));
        let manifest = store.put(b"writable-index-evidence").unwrap();
        index.publish(&store, 0, manifest).unwrap();

        let active = root.join("index/active-generation");
        let history = root.join("index/generations/0.manifest");
        fs::set_permissions(&active, fs::Permissions::from_mode(0o666)).unwrap();
        assert_eq!(
            index.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::set_permissions(&active, fs::Permissions::from_mode(0o600)).unwrap();
        fs::set_permissions(&history, fs::Permissions::from_mode(0o666)).unwrap();
        assert_eq!(
            index.historical_manifest(0).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn authority_public_entrypoints_reject_noncanonical_roots() {
        let relative = PathBuf::from(format!(
            ".formula-relative-authority-{}",
            std::process::id()
        ));
        assert!(!relative.exists());
        let store = BlobStore::new(&relative);
        assert_eq!(
            store.put(b"no-relative-authority").unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert!(
            !relative.exists(),
            "rejected store root must not be created"
        );

        let index = GenerationIndex::new(&relative);
        assert_eq!(
            index
                .publish(&store, 1, ArtifactDigest::sha256(b"no-relative-authority"))
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
        assert!(
            !relative.exists(),
            "rejected index root must not be created"
        );

        let parent = test_root("public-parent-traversal")
            .join("..")
            .join("authority");
        assert_eq!(
            BlobStore::new(&parent)
                .put(b"no-parent-traversal")
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
    }

    #[test]
    fn authority_paths_reject_relative_and_parent_traversal() {
        // Authority custody cannot depend on a process's mutable current
        // directory or on lexical parent traversal through a different inode.
        let relative = Path::new("relative/authority");
        assert_eq!(
            BlobStore::require_real_ancestors(relative)
                .expect_err("relative authority root must be rejected")
                .kind(),
            io::ErrorKind::InvalidInput
        );
        let parent = test_root("lexical-parent").join("..").join("authority");
        assert_eq!(
            BlobStore::require_real_ancestors(&parent)
                .expect_err("parent traversal must be rejected")
                .kind(),
            io::ErrorKind::InvalidInput
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_store_parent_cannot_redirect_immutable_blob_io() {
        use std::os::unix::fs::symlink;

        let root = test_root("symlink-parent");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("outside")).unwrap();
        symlink(root.join("outside"), root.join("alias")).unwrap();

        let redirected = BlobStore::new(root.join("alias/store"));
        assert_eq!(
            redirected.put(b"must-not-redirect").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("outside/store").exists());

        let legitimate = BlobStore::new(root.join("outside/store"));
        let digest = legitimate.put(b"existing").unwrap();
        assert_eq!(
            redirected.get(digest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_generation_index_parent_cannot_redirect_publication_or_replay() {
        use std::os::unix::fs::symlink;

        let root = test_root("symlink-index-parent");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("outside")).unwrap();
        symlink(root.join("outside"), root.join("alias")).unwrap();

        let store = BlobStore::new(root.join("store"));
        let manifest = store.put(b"index-parent-manifest").unwrap();
        let redirected = GenerationIndex::new(root.join("alias/index"));
        assert_eq!(
            redirected.publish(&store, 1, manifest).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(!root.join("outside/index").exists());

        let legitimate = GenerationIndex::new(root.join("outside/index"));
        legitimate.publish(&store, 1, manifest).unwrap();
        assert_eq!(
            redirected.active_generation().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
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
