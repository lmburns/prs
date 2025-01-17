//! Interface to a password store and its secrets.

use std::{
    ffi::OsString,
    fs,
    path::{self, Path, PathBuf},
};

use anyhow::{ensure, Result};
use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[cfg(all(feature = "tomb", target_os = "linux"))]
use crate::tomb::Tomb;
use crate::{
    crypto::{self, prelude::*},
    sync::Sync,
    vendor::shellexpand,
    Recipients,
};

/// Password store secret file suffix.
pub const SECRET_SUFFIX: &str = ".gpg";

/// Prevent multiple compilations of the same `Regex`.
/// Used for checking for 'sneaky' paths
static SNEAKY_RE: OnceCell<Regex> = OnceCell::new();

/// Represents a password store.
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Store {
    /// Root directory of the password store.
    ///
    /// This path is always absolute.
    pub root: PathBuf,
}

impl std::cmp::PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl std::cmp::Ord for Store {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root.cmp(&other.root)
    }
}

impl std::cmp::PartialOrd for Store {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.root.partial_cmp(&other.root)
    }
}

impl Store {
    /// Open a store at the given path.
    pub fn open<P: AsRef<str>>(root: P) -> Result<Self> {
        let root: PathBuf = shellexpand::full(&root)
            .map_err(Err::ExpandPath)?
            .as_ref()
            .into();

        // Make sure store directory exists
        ensure!(root.is_dir(), Err::NoRootDir(root));

        // TODO: check if .gpg-ids exists? this does not work if this is a tomb

        Ok(Self { root })
    }

    /// Get the recipient keys for this store.
    pub fn recipients(&self) -> Result<Recipients> {
        Recipients::load(self)
    }

    /// Get a sync helper for this store.
    #[must_use]
    pub const fn sync(&self) -> Sync {
        Sync::new(self)
    }

    /// Get a tomb helper for this store.
    #[must_use]
    #[cfg(all(feature = "tomb", target_os = "linux"))]
    pub const fn tomb(&self, quiet: bool, verbose: bool, force: bool) -> Tomb {
        Tomb::new(self, quiet, verbose, force)
    }

    /// Create secret iterator for this store.
    #[must_use]
    pub fn secret_iter(&self) -> SecretIter {
        self.secret_iter_config(SecretIterConfig::default())
    }

    /// Create secret iterator for this store with custom configuration.
    #[must_use]
    pub fn secret_iter_config(&self, config: SecretIterConfig) -> SecretIter {
        SecretIter::new(self.root.clone(), config)
    }

    /// List store password secrets.
    #[must_use]
    pub fn secrets(&self, filter: Option<String>) -> Vec<Secret> {
        self.secret_iter().filter_name(filter).collect()
    }

    /// Try to find matching secret at path.
    #[must_use]
    pub fn find_at(&self, path: &str) -> Option<Secret> {
        // Build path
        let path = self.root.as_path().join(path);
        let path = path.to_str()?;

        // Try path with secret file suffix
        let with_suffix = PathBuf::from(format!("{}{}", path, SECRET_SUFFIX));
        if with_suffix.is_file() {
            return Some(Secret::from(self, with_suffix));
        }

        // Try path without secret file suffix
        let without_suffix = Path::new(path);
        if without_suffix.is_file() {
            return Some(Secret::from(self, without_suffix.to_path_buf()));
        }

        None
    }

    /// Try to find matching secrets for given query.
    ///
    /// If secret is found at exact query path, `FindSecret::Found` is returned.
    /// Otherwise any number of closely matching secrets is returned as
    /// `FindSecret::Many`.
    #[must_use]
    pub fn find(&self, query: Option<String>) -> FindSecret {
        // Try to find exact secret match
        if let Some(query) = &query {
            if let Some(secret) = self.find_at(query) {
                return FindSecret::Exact(secret);
            }
        }

        // Find all closely matching
        FindSecret::Many(self.secrets(query))
    }

    /// Check for 'sneaky' paths found within the password store
    #[allow(clippy::unused_self)]
    pub fn check_sneaky_paths<P: AsRef<Path>>(&self, path: P) -> Result<(), Err> {
        let path = path.as_ref();
        if SNEAKY_RE
            .get_or_init(|| Regex::new("/..$|^../|/../|^..$").unwrap())
            .is_match(&(path.display().to_string()))
        {
            return Err(Err::SneakyPath(path.into()));
        }

        Ok(())
    }

    /// Normalizes a path for a secret in this store.
    ///
    /// - Ensures path is within store.
    /// - If directory is given, name hint is appended.
    /// - Sets correct extension.
    /// - Creates parent directories if non existant (optional).
    pub fn normalize_secret_path<P: AsRef<Path>>(
        &self,
        target: P,
        name_hint: Option<&str>,
        create_dirs: bool,
    ) -> Result<PathBuf> {
        self.check_sneaky_paths(&target)?;

        // Take target as base path
        let mut path = PathBuf::from(target.as_ref());

        // Expand path
        if let Some(path_str) = path.to_str() {
            path = PathBuf::from(
                shellexpand::full(path_str)
                    .map_err(Err::ExpandPath)?
                    .as_ref(),
            );
        }

        let target_is_dir = path.is_dir()
            || target
                .as_ref()
                .to_str()
                .and_then(|s| s.chars().last())
                .map_or(false, path::is_separator);

        // Strip store prefix
        if let Ok(tmp) = path.strip_prefix(&self.root) {
            path = tmp.into();
        }

        // Make relative
        if path.is_absolute() {
            path = PathBuf::from(format!(".{}{}", path::MAIN_SEPARATOR, path.display()));
        }

        // Prefix store root
        path = self.root.as_path().join(path);

        // Add current secret name if target is dir
        if target_is_dir {
            path.push(name_hint.ok_or_else(|| Err::TargetDirWithoutNamehint(path.clone()))?);
        }

        // Add secret extension if non existent
        let ext: OsString = SECRET_SUFFIX.trim_start_matches('.').into();
        if path.extension() != Some(&ext) {
            let mut tmp = path.as_os_str().to_owned();
            tmp.push(SECRET_SUFFIX);
            path = PathBuf::from(tmp);
        }

        // Create parent dir if it doesn't exist
        if create_dirs {
            let parent = path.parent().unwrap();
            if !parent.is_dir() {
                fs::create_dir_all(parent).map_err(Err::CreateDir)?;
            }
        }

        Ok(path)
    }
}

/// Find secret result.
pub enum FindSecret {
    /// Found exact secret match.
    Exact(Secret),

    /// Found any number of non-exact secret matches.
    Many(Vec<Secret>),
}

/// A password store secret.
#[derive(Debug, Clone)]
pub struct Secret {
    /// Display name of the secret, relative path to the password store root.
    pub name: String,

    /// Full path to the password store secret.
    pub path: PathBuf,
}

impl Secret {
    /// Construct secret at given full path from given store.
    #[must_use]
    pub fn from(store: &Store, path: PathBuf) -> Self {
        Self::in_root(&store.root, path)
    }

    /// Construct secret at given path in the given password store root.
    #[must_use]
    pub fn in_root(root: &Path, path: PathBuf) -> Self {
        let name: String = relative_path(root, &path)
            .ok()
            .and_then(Path::to_str)
            .map_or("?", |f| f.trim_end_matches(SECRET_SUFFIX))
            .to_string();
        Self { name, path }
    }

    /// Get relative path to this secret, root must be given.
    pub fn relative_path<'a>(
        &'a self,
        root: &'a Path,
    ) -> Result<&'a Path, std::path::StripPrefixError> {
        relative_path(root, &self.path)
    }

    /// Returns pointed to secret.
    ///
    /// If this secret is an alias, this will return the pointed to secret.
    /// If this secret is not an alias, an error will be returned.
    ///
    /// The pointed to secret may be an alias as well.
    pub fn alias_target(&self, store: &Store) -> Result<Self> {
        // Read alias target path, make absolute, attempt to canonicalize
        let mut path = self.path.parent().unwrap().join(fs::read_link(&self.path)?);
        if let Ok(canonical_path) = path.canonicalize() {
            path = canonical_path;
        }

        Ok(Self::from(store, path))
    }
}

/// Get relative path in given root.
pub fn relative_path<'a>(
    root: &'a Path,
    path: &'a Path,
) -> Result<&'a Path, std::path::StripPrefixError> {
    path.strip_prefix(&root)
}

/// Secret iterator configuration.
///
/// Used to configure what files are found by the secret iterator.
#[derive(Clone, Debug)]
pub struct SecretIterConfig {
    /// Find pure files.
    pub find_files: bool,

    /// Find files that are symlinks.
    ///
    /// Will still find files if they're symlinked to while `find_files` is
    /// `false`.
    pub find_symlink_files: bool,
}

impl Default for SecretIterConfig {
    fn default() -> Self {
        Self {
            find_files:         true,
            find_symlink_files: true,
        }
    }
}

/// Iterator that walks through password store secrets.
///
/// This walks all password store directories, and yields password secrets.
/// Hidden files or directories are skipped.
pub struct SecretIter {
    /// Root of the store to walk.
    root: PathBuf,

    /// Directory walker.
    walker: Box<dyn Iterator<Item = DirEntry>>,
}

impl SecretIter {
    /// Create new store secret iterator at given store root.
    #[must_use]
    pub fn new(root: PathBuf, config: SecretIterConfig) -> Self {
        let walker = WalkDir::new(&root)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !is_hidden_subdir(e))
            .filter_map(Result::ok)
            .filter(is_secret_file)
            .filter(move |entry| filter_by_config(entry, &config));
        Self {
            root,
            walker: Box::new(walker),
        }
    }

    /// Transform into a filtered secret iterator.
    #[must_use]
    pub fn filter_name(self, filter: Option<String>) -> FilterSecretIter<Self> {
        FilterSecretIter::new(self, filter)
    }
}

impl Iterator for SecretIter {
    type Item = Secret;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker
            .next()
            .map(|e| Secret::in_root(&self.root, e.path().into()))
    }
}

/// Check if given WalkDir DirEntry is hidden sub-directory.
fn is_hidden_subdir(entry: &DirEntry) -> bool {
    entry.depth() > 0
        && entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.starts_with('.') || s == "lost+found")
}

/// Check if given WalkDir DirEntry is a secret file.
fn is_secret_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .file_name()
            .to_str()
            .map_or(false, |s| s.ends_with(SECRET_SUFFIX))
}

/// Check if given WalkDir DirEntry passes the configuration.
fn filter_by_config(entry: &DirEntry, config: &SecretIterConfig) -> bool {
    // Optimization, config permutation which includes all files
    if config.find_files && config.find_symlink_files {
        return true;
    }

    // Find symlinks
    if config.find_symlink_files && entry.path_is_symlink() {
        return true;
    }

    // Do not find symlinks
    if !config.find_symlink_files && entry.path_is_symlink() {
        return false;
    }

    // Find files
    if !config.find_files && !entry.path_is_symlink() {
        return false;
    }

    true
}

/// Check whether we can decrypt the first secret in the store.
///
/// If decryption fails, and this returns false, it means we don't own any
/// compatible secret key.
///
/// Returns true if there is no secret.
#[must_use]
pub fn can_decrypt(store: &Store) -> bool {
    // Try all proto's here once we support more
    store
        .secret_iter()
        .next()
        .map_or(false, |secret| {
            crypto::context(&crate::CONFIG)
                .map(|mut context| context.can_decrypt_file(&secret.path).unwrap_or(true))
                .unwrap_or(false)
        })
}

/// Iterator that wraps a `SecretIter` with a filter.
pub struct FilterSecretIter<I>
where
    I: Iterator<Item = Secret>,
{
    inner:  I,
    filter: Option<String>,
}

impl<I> FilterSecretIter<I>
where
    I: Iterator<Item = Secret>,
{
    /// Construct a new filter secret iterator.
    pub fn new(inner: I, filter: Option<String>) -> Self {
        Self { inner, filter }
    }
}

impl<I> Iterator for FilterSecretIter<I>
where
    I: Iterator<Item = Secret>,
{
    type Item = Secret;

    fn next(&mut self) -> Option<Self::Item> {
        // Return all with no filter, or lowercase filter text
        let filter = match &self.filter {
            None => return self.inner.next(),
            Some(filter) => filter.to_lowercase(),
        };

        // Return each secret matching the filter
        for secret in self.inner.by_ref() {
            if secret.name.to_lowercase().contains(&filter) {
                return Some(secret);
            }
        }

        None
    }
}

/// Password store error.
#[derive(Debug, Error)]
pub enum Err {
    #[error("failed to expand store root path")]
    ExpandPath(#[source] shellexpand::LookupError<std::env::VarError>),

    #[error("failed to open password store, not a directory: {0}")]
    NoRootDir(PathBuf),

    #[error("failed to create directory")]
    CreateDir(#[source] std::io::Error),

    #[error("cannot use directory as target without name hint")]
    TargetDirWithoutNamehint(PathBuf),

    #[error("{0:?} contains a sneaky pattern")]
    SneakyPath(PathBuf),
}
