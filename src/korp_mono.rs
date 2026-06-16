//! A path to a korp_mono file.

use std::path::{Path, PathBuf};

use crate::Corpus;
use crate::korp_mono_file_path::KorpMonoFilePath;

/// The `korp_mono/` directory in a corpus directory. Primary use cases are it's
/// `files()` method to iterate over all korp mono xml files.
#[derive(Clone)]
pub struct KorpMono {
    /// The corpus this `korp_mono/` directory resides in.
    pub corpus: Corpus,
}

impl KorpMono {
    pub fn to_path_buf(&self) -> PathBuf {
        let mut p = self.corpus.to_path_buf();
        p.push("korp_mono");
        p
    }

    /// Get to a specific file in this korp_mono section.
    pub fn with_file<P: AsRef<Path>>(&self, file: P) -> KorpMonoFilePath {
        KorpMonoFilePath {
            korp_mono: self.clone(),
            file: file.as_ref().to_path_buf(),
        }
    }

    /// An iterator over all the xml files in this `korp_mono/` directory.
    pub fn files(&self) -> impl Iterator<Item = KorpMonoFilePath> {
        let p = self.to_path_buf();
        crate::find_xml_files(self.to_path_buf()).filter_map(move |entry| {
            let file = entry.path().strip_prefix(&p)
                .expect("absolute path of entry is a subpath of input path");
            Some(self.with_file(file))
        })
    }
}

impl From<Corpus> for KorpMono {
    fn from(corpus: Corpus) -> Self {
        KorpMono { corpus }
    }
}

#[cfg(test)]
mod tests {
}
