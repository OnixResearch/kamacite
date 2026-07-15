use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(target_os = "linux")]
use rustix::fs::{
    AtFlags, FileType, Mode, OFlags, ResolveFlags, mkdirat, openat2, renameat, statat, unlinkat,
};

const DIRECTORY_CREATE_MODE: u32 = 0o755;
const TEMPORARY_FILE_MODE: u32 = 0o600;
const TEMPORARY_CREATE_ATTEMPTS: usize = 32;
const TEMPORARY_FILE_PREFIX: &str = ".kamacite-root-io";

static TEMPORARY_FILE_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub(crate) struct RootIoError(String);

impl Display for RootIoError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for RootIoError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AdmittedPath {
    path: PathBuf,
    components: Vec<OsString>,
    display: String,
}

impl AdmittedPath {
    fn parent_components(&self) -> &[OsString] {
        let parent_length = self.components.len() - 1;
        &self.components[..parent_length]
    }

    fn file_name(&self) -> &OsStr {
        &self.components[self.components.len() - 1]
    }
}

/// Pure path-policy core. Filesystem resolution remains the shell's responsibility.
fn admit_relative_path(path: &Path) -> Result<AdmittedPath, RootIoError> {
    if path.as_os_str().is_empty() {
        return Err(path_policy_error(path, "empty"));
    }
    if path.is_absolute() {
        return Err(path_policy_error(path, "absolute"));
    }

    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => components.push(value.to_os_string()),
            Component::ParentDir => return Err(path_policy_error(path, "parent-traversal")),
            Component::CurDir => return Err(path_policy_error(path, "current-directory")),
            Component::RootDir => return Err(path_policy_error(path, "root")),
            Component::Prefix(_) => return Err(path_policy_error(path, "platform-prefix")),
        }
    }
    if components.is_empty() {
        return Err(path_policy_error(path, "empty"));
    }

    Ok(AdmittedPath {
        path: path.to_path_buf(),
        components,
        display: path.display().to_string(),
    })
}

fn path_policy_error(path: &Path, class: &str) -> RootIoError {
    RootIoError(format!(
        "root path rejected ({class}): `{}`",
        path.display()
    ))
}

/// Imperative shell owning the single ambient open of a reviewed root.
/// r[impl kamacite.findings.root_io_confinement.single_gateway]
/// r[impl kamacite.findings.root_io_confinement.symlink_safe]
/// r[impl kamacite.findings.root_io_confinement.mutation_safe]
#[derive(Debug)]
pub(crate) struct RootCapability {
    root: File,
}

impl RootCapability {
    pub(crate) fn open(root: &Path) -> Result<Self, RootIoError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = root;
            return Err(RootIoError(
                "root capability unavailable: Linux openat2 confinement is required".to_string(),
            ));
        }

        #[cfg(target_os = "linux")]
        {
            let root_file = File::open(root)
                .map_err(|error| RootIoError(format!("root capability open failed: {error}")))?;
            let metadata = root_file.metadata().map_err(|error| {
                RootIoError(format!("root capability metadata failed: {error}"))
            })?;
            if !metadata.is_dir() {
                return Err(RootIoError(
                    "root capability rejected: selected root is not a directory".to_string(),
                ));
            }
            Ok(Self { root: root_file })
        }
    }

    pub(crate) fn read(&self, path: &Path) -> Result<Vec<u8>, RootIoError> {
        let admitted = admit_relative_path(path)?;
        let mut file = self.open_read_file(&admitted, true)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|error| operation_error("read", &admitted, error))?;
        Ok(bytes)
    }

    pub(crate) fn read_optional(&self, path: &Path) -> Result<Option<Vec<u8>>, RootIoError> {
        let admitted = admit_relative_path(path)?;
        let file = match self.open_read_file_optional(&admitted, true)? {
            Some(file) => file,
            None => return Ok(None),
        };
        let mut file = file;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|error| operation_error("read", &admitted, error))?;
        Ok(Some(bytes))
    }

    pub(crate) fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), RootIoError> {
        self.write_with_pre_commit(path, bytes, || Ok(()))
    }

    fn write_with_pre_commit<F>(
        &self,
        path: &Path,
        bytes: &[u8],
        pre_commit: F,
    ) -> Result<(), RootIoError>
    where
        F: FnOnce() -> Result<(), RootIoError>,
    {
        let admitted = admit_relative_path(path)?;

        #[cfg(not(target_os = "linux"))]
        {
            let _ = (bytes, pre_commit);
            return Err(RootIoError(
                "root capability unavailable: Linux openat2 confinement is required".to_string(),
            ));
        }

        #[cfg(target_os = "linux")]
        {
            let parent = self.open_parent(&admitted, true)?;
            reject_symlink_destination(&parent, admitted.file_name(), &admitted)?;
            let (temporary_name, temporary_fd) = create_temporary_file(&parent, &admitted)?;
            let mut temporary_file = File::from(temporary_fd);
            let write_result = (|| {
                temporary_file
                    .write_all(bytes)
                    .map_err(|error| operation_error("write", &admitted, error))?;
                temporary_file
                    .sync_all()
                    .map_err(|error| operation_error("flush", &admitted, error))?;
                drop(temporary_file);
                pre_commit()?;
                reject_symlink_destination(&parent, admitted.file_name(), &admitted)?;
                renameat(
                    &parent,
                    temporary_name.as_os_str(),
                    &parent,
                    admitted.file_name(),
                )
                .map_err(|error| rustix_operation_error("replace", &admitted, error))?;
                rustix::fs::fsync(&parent)
                    .map_err(|error| rustix_operation_error("directory-flush", &admitted, error))?;
                Ok(())
            })();
            if write_result.is_err() {
                let _ = unlinkat(&parent, temporary_name.as_os_str(), AtFlags::empty());
            }
            write_result
        }
    }

    #[cfg(target_os = "linux")]
    fn open_read_file(
        &self,
        admitted: &AdmittedPath,
        close_on_exec: bool,
    ) -> Result<File, RootIoError> {
        match self.open_read_file_optional(admitted, close_on_exec)? {
            Some(file) => Ok(file),
            None => Err(RootIoError(format!(
                "root I/O read failed (not-found): `{}`",
                admitted.display
            ))),
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn open_read_file(
        &self,
        admitted: &AdmittedPath,
        close_on_exec: bool,
    ) -> Result<File, RootIoError> {
        let _ = (admitted, close_on_exec);
        Err(RootIoError(
            "root capability unavailable: Linux openat2 confinement is required".to_string(),
        ))
    }

    #[cfg(target_os = "linux")]
    fn open_read_file_optional(
        &self,
        admitted: &AdmittedPath,
        close_on_exec: bool,
    ) -> Result<Option<File>, RootIoError> {
        let close_flag = if close_on_exec {
            OFlags::CLOEXEC
        } else {
            OFlags::empty()
        };
        match openat2(
            &self.root,
            &admitted.path,
            OFlags::RDONLY | close_flag,
            Mode::empty(),
            confinement_flags(),
        ) {
            Ok(file) => Ok(Some(File::from(file))),
            Err(error) if error == rustix::io::Errno::NOENT => Ok(None),
            Err(error) => Err(rustix_operation_error("read", admitted, error)),
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn open_read_file_optional(
        &self,
        admitted: &AdmittedPath,
        close_on_exec: bool,
    ) -> Result<Option<File>, RootIoError> {
        let _ = (admitted, close_on_exec);
        Err(RootIoError(
            "root capability unavailable: Linux openat2 confinement is required".to_string(),
        ))
    }

    #[cfg(target_os = "linux")]
    fn open_parent(
        &self,
        admitted: &AdmittedPath,
        create_missing: bool,
    ) -> Result<rustix::fd::OwnedFd, RootIoError> {
        let mut current = rustix::io::dup(&self.root)
            .map_err(|error| rustix_operation_error("root-duplicate", admitted, error))?;
        for component in admitted.parent_components() {
            let opened = open_directory_component(&current, component, admitted);
            current = match opened {
                Ok(directory) => directory,
                Err(error) if error == rustix::io::Errno::NOENT && create_missing => {
                    match mkdirat(
                        &current,
                        component.as_os_str(),
                        Mode::from_raw_mode(DIRECTORY_CREATE_MODE),
                    ) {
                        Ok(()) => {
                            rustix::fs::fsync(&current).map_err(|flush_error| {
                                rustix_operation_error(
                                    "create-directory-flush",
                                    admitted,
                                    flush_error,
                                )
                            })?;
                        }
                        Err(create_error) if create_error == rustix::io::Errno::EXIST => {}
                        Err(create_error) => {
                            return Err(rustix_operation_error(
                                "create-directory",
                                admitted,
                                create_error,
                            ));
                        }
                    }
                    open_directory_component(&current, component, admitted).map_err(
                        |open_error| rustix_operation_error("open-directory", admitted, open_error),
                    )?
                }
                Err(error) => {
                    return Err(rustix_operation_error("open-directory", admitted, error));
                }
            };
        }
        Ok(current)
    }

    #[cfg(test)]
    fn write_with_test_hook<F>(
        &self,
        path: &Path,
        bytes: &[u8],
        pre_commit: F,
    ) -> Result<(), RootIoError>
    where
        F: FnOnce() -> Result<(), RootIoError>,
    {
        self.write_with_pre_commit(path, bytes, pre_commit)
    }
}

#[cfg(target_os = "linux")]
fn confinement_flags() -> ResolveFlags {
    ResolveFlags::BENEATH | ResolveFlags::NO_SYMLINKS | ResolveFlags::NO_MAGICLINKS
}

#[cfg(target_os = "linux")]
fn open_directory_component<Fd: std::os::fd::AsFd>(
    parent: Fd,
    component: &OsStr,
    _admitted: &AdmittedPath,
) -> Result<rustix::fd::OwnedFd, rustix::io::Errno> {
    openat2(
        parent,
        component,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        confinement_flags(),
    )
}

#[cfg(target_os = "linux")]
fn reject_symlink_destination<Fd: std::os::fd::AsFd>(
    parent: Fd,
    file_name: &OsStr,
    admitted: &AdmittedPath,
) -> Result<(), RootIoError> {
    match statat(&parent, file_name, AtFlags::SYMLINK_NOFOLLOW) {
        Ok(stat) if FileType::from_raw_mode(stat.st_mode) == FileType::Symlink => {
            Err(RootIoError(format!(
                "root path rejected (symlink-destination): `{}`",
                admitted.display
            )))
        }
        Ok(_) => Ok(()),
        Err(error) if error == rustix::io::Errno::NOENT => Ok(()),
        Err(error) => Err(rustix_operation_error(
            "inspect-destination",
            admitted,
            error,
        )),
    }
}

#[cfg(target_os = "linux")]
fn create_temporary_file<Fd: std::os::fd::AsFd>(
    parent: Fd,
    admitted: &AdmittedPath,
) -> Result<(OsString, rustix::fd::OwnedFd), RootIoError> {
    for _ in 0..TEMPORARY_CREATE_ATTEMPTS {
        let sequence = TEMPORARY_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = OsString::from(format!(
            "{TEMPORARY_FILE_PREFIX}-{}-{sequence}.tmp",
            std::process::id()
        ));
        match openat2(
            &parent,
            name.as_os_str(),
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::from_raw_mode(TEMPORARY_FILE_MODE),
            confinement_flags(),
        ) {
            Ok(file) => return Ok((name, file)),
            Err(error) if error == rustix::io::Errno::EXIST => {}
            Err(error) => {
                return Err(rustix_operation_error("create-temporary", admitted, error));
            }
        }
    }
    Err(RootIoError(format!(
        "root I/O create-temporary failed (collision-limit): `{}`",
        admitted.display
    )))
}

fn operation_error(operation: &str, admitted: &AdmittedPath, error: std::io::Error) -> RootIoError {
    RootIoError(format!(
        "root I/O {operation} failed for `{}`: {error}",
        admitted.display
    ))
}

#[cfg(target_os = "linux")]
fn rustix_operation_error(
    operation: &str,
    admitted: &AdmittedPath,
    error: rustix::io::Errno,
) -> RootIoError {
    if error == rustix::io::Errno::LOOP || error == rustix::io::Errno::XDEV {
        return RootIoError(format!(
            "root path rejected (symlink-or-escape): `{}`",
            admitted.display
        ));
    }
    if error == rustix::io::Errno::NOSYS {
        return RootIoError(
            "root capability unavailable: Linux openat2 confinement is required".to_string(),
        );
    }
    RootIoError(format!(
        "root I/O {operation} failed for `{}`: {error}",
        admitted.display
    ))
}

// r[verify kamacite.findings.root_io_confinement.single_gateway]
// r[verify kamacite.findings.root_io_confinement.symlink_safe]
// r[verify kamacite.findings.root_io_confinement.mutation_safe]
// r[verify kamacite.findings.root_io_confinement.coverage]
// r[verify kamacite.findings.nickel_sandbox.import_closure]
#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::sync::atomic::{AtomicU64, Ordering};

    const REGULAR_BYTES: &[u8] = b"confined bytes";
    const REPLACEMENT_BYTES: &[u8] = b"replacement bytes";

    static TEST_DIRECTORY_COUNTER: AtomicU64 = AtomicU64::new(1);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn create(label: &str) -> Result<Self, std::io::Error> {
            let sequence = TEST_DIRECTORY_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "kamacite-root-io-{label}-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path)?;
            Ok(Self { path })
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn path_policy_accepts_regular_relative_components() -> Result<(), Box<dyn std::error::Error>> {
        let admitted = admit_relative_path(Path::new("fixtures/input.json"))?;
        assert_eq!(admitted.display, "fixtures/input.json");
        assert_eq!(
            admitted.components,
            vec![OsString::from("fixtures"), OsString::from("input.json")]
        );
        Ok(())
    }

    #[test]
    fn path_policy_rejects_ambient_and_traversal_paths() {
        for (path, class) in [
            ("/tmp/outside", "absolute"),
            ("../outside", "parent-traversal"),
            ("fixtures/../outside", "parent-traversal"),
            ("./fixtures/input", "current-directory"),
            ("", "empty"),
        ] {
            let result = admit_relative_path(Path::new(path));
            assert!(result.is_err());
            let message = result.err().map(|error| error.to_string());
            assert!(message.is_some_and(|value| value.contains(class)));
        }
    }

    #[test]
    fn gateway_reads_creates_and_replaces_regular_files() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = TestDirectory::create("regular")?;
        let capability = RootCapability::open(&root.path)?;
        capability.write(Path::new("nested/artifact.json"), REGULAR_BYTES)?;
        assert_eq!(
            capability.read(Path::new("nested/artifact.json"))?,
            REGULAR_BYTES
        );
        capability.write(Path::new("nested/artifact.json"), REPLACEMENT_BYTES)?;
        assert_eq!(
            capability.read(Path::new("nested/artifact.json"))?,
            REPLACEMENT_BYTES
        );
        Ok(())
    }

    #[test]
    fn gateway_rejects_symlink_sources_destinations_and_parents()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = TestDirectory::create("symlinks")?;
        let outside = TestDirectory::create("outside")?;
        fs::write(outside.path.join("source.txt"), REGULAR_BYTES)?;
        fs::write(outside.path.join("destination.txt"), REGULAR_BYTES)?;
        symlink(
            outside.path.join("source.txt"),
            root.path.join("source-link"),
        )?;
        symlink(
            outside.path.join("destination.txt"),
            root.path.join("destination-link"),
        )?;
        symlink(&outside.path, root.path.join("parent-link"))?;
        let capability = RootCapability::open(&root.path)?;

        let source_result = capability.read(Path::new("source-link"));
        assert!(source_result.is_err());
        assert!(
            source_result
                .err()
                .is_some_and(|error| error.to_string().contains("symlink-or-escape"))
        );
        let destination_result = capability.write(Path::new("destination-link"), REPLACEMENT_BYTES);
        assert!(destination_result.is_err());
        assert_eq!(
            fs::read(outside.path.join("destination.txt"))?,
            REGULAR_BYTES
        );

        let parent_result =
            capability.write(Path::new("parent-link/created.txt"), REPLACEMENT_BYTES);
        assert!(parent_result.is_err());
        assert!(!outside.path.join("created.txt").exists());
        Ok(())
    }

    #[test]
    fn replacement_race_stays_on_the_admitted_parent_handle()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = TestDirectory::create("parent-race")?;
        let outside = TestDirectory::create("parent-race-outside")?;
        fs::create_dir(root.path.join("admitted"))?;
        let displaced = root.path.join("displaced");
        let admitted = root.path.join("admitted");
        let capability = RootCapability::open(&root.path)?;

        capability.write_with_test_hook(
            Path::new("admitted/result.txt"),
            REPLACEMENT_BYTES,
            || {
                fs::rename(&admitted, &displaced).map_err(|error| {
                    RootIoError(format!("test parent replacement failed: {error}"))
                })?;
                symlink(&outside.path, &admitted)
                    .map_err(|error| RootIoError(format!("test symlink failed: {error}")))?;
                Ok(())
            },
        )?;

        assert_eq!(fs::read(displaced.join("result.txt"))?, REPLACEMENT_BYTES);
        assert!(!outside.path.join("result.txt").exists());
        Ok(())
    }

    #[test]
    fn destination_symlink_race_fails_without_modifying_outside_file()
    -> Result<(), Box<dyn std::error::Error>> {
        let root = TestDirectory::create("destination-race")?;
        let outside = TestDirectory::create("destination-race-outside")?;
        fs::create_dir(root.path.join("admitted"))?;
        let outside_file = outside.path.join("outside.txt");
        fs::write(&outside_file, REGULAR_BYTES)?;
        let destination = root.path.join("admitted/result.txt");
        let capability = RootCapability::open(&root.path)?;

        let result = capability.write_with_test_hook(
            Path::new("admitted/result.txt"),
            REPLACEMENT_BYTES,
            || {
                symlink(&outside_file, &destination)
                    .map_err(|error| RootIoError(format!("test symlink failed: {error}")))?;
                Ok(())
            },
        );

        assert!(result.is_err());
        assert_eq!(fs::read(&outside_file)?, REGULAR_BYTES);
        Ok(())
    }
}
