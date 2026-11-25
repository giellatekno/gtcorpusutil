use std::path::PathBuf;

use crate::AnalysedFilePath;
use crate::KorpMono;
use crate::Category;
use crate::NotCategoryError;

/// The path to a korp mono xml file.
pub struct KorpMonoFilePath {
    /// The KorpMono section of the corpus the file resides in.
    pub korp_mono: KorpMono,

    /// The relative path to the file, inside the given module.
    pub file: PathBuf,
}

impl KorpMonoFilePath {
    /// Return a new [PathBuf] to the full, absolute path of this korp_mono file.
    pub fn to_path_buf(&self) -> PathBuf {
        let mut p = self.korp_mono.to_path_buf();
        p.push(self.file.clone());
        p
    }

    /// Read the file contents into a new [String]. Equivalent to
    /// `std::fs::read_to_string(&self.to_path_buf())`.
    pub fn read_to_string(&self) -> std::io::Result<String> {
        std::fs::read_to_string(self.to_path_buf())
    }

    /// Get the category of this file. If this file resides in a directory that is
    /// not a known category, [NotCategoryError] will be returned.
    pub fn category(&self) -> Result<Category, NotCategoryError> {
        let mut components = self.file.components();
        let c = components.next().expect("path is not empty");
        let s = c.as_os_str().to_str().expect("file paths are valid utf-8");
        match Category::from_path_str(s) {
            Ok(category) => Ok(category),
            Err(NotCategoryError::Hist) => {
                let c = components.next().unwrap();
                let s = c.as_os_str().to_str().expect("file paths are valid utf-8");
                Category::from_path_str(s)
            }
            Err(other) => Err(other),
        }
    }
}

impl From<AnalysedFilePath> for KorpMonoFilePath {
    /// Turn an [AnalysedFilePath] into the corresponding [KorpMonoFilePath]. In terms
    /// of the actual path to the file, it replaces `analysed` with `korp_mono`, i.e.
    /// it goes from `/some/directory/root/corpus-xxx/analysed/whatever/file.xml`
    /// to `/some/directory/root/corpus-xxx/korp_mono/whatever/file.xml`.
    fn from(v: AnalysedFilePath) -> KorpMonoFilePath {
        let AnalysedFilePath { analysed, file } = v;
        analysed.into_korp_mono().with_file(file)
    }
}
