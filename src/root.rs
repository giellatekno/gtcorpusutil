use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::corpus::Corpus;
use crate::utils::{FindGutRootError, find_gut_root};

/// Just a [PathBuf], but with the added convention that it is a directory where many
/// corpora (i.e. `corpus-xxx[...]`) directories are stored.
///
/// A very common example is `/home/user/gut_root/giellalt/`, since using `gut` is common,
/// and all giellalt corpora resides as `https://github.com/giellalt/corpus-xxx`.
#[derive(Clone)]
pub struct Root(PathBuf);

impl Root {
    /// Find the [Root] from the `root` property defined in the `gut` config (which
    /// is at `$HOME/.config/gut/app.toml`).
    pub fn from_gut_config() -> Result<Self, FindGutRootError> {
        Ok(Self(find_gut_root()?))
    }

    /// Create a new [Root]. This is an infallible operations, as nothing is checked,
    /// it just moves the given [PathBuf] into this new structure.
    pub fn new(pathbuf: PathBuf) -> Self {
        Self(pathbuf)
    }

    /// Scan the directory for corpus directories, and return an Iterator
    /// over all of them.
    pub fn corpora(&self) -> impl Iterator<Item = Corpus> {
        WalkDir::new(&self.0)
            .max_depth(1)
            .into_iter()
            .flatten()  // ignore errors
            .filter(|entry| entry.file_type().is_dir())
            .filter_map(|entry| Corpus::try_from(entry.into_path()).ok())
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn as_pathbuf(&self) -> &PathBuf {
        &self.0
    }

    /// Return the inner path as a newly cloned `PathBuf`.
    pub fn to_path_buf(&self) -> PathBuf {
        self.0.clone()
    }

    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }

    pub fn into_inner(self) -> PathBuf {
        self.into_path_buf()
    }
}

impl From<PathBuf> for Root {
    fn from(path_buf: PathBuf) -> Root {
        Root(path_buf)
    }
}

impl AsRef<Path> for Root {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
