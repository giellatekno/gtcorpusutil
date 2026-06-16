use std::path::PathBuf;

use crate::AnalysedFilePath;
use crate::KorpMono;
use crate::Module;

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
