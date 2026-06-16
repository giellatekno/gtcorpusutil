use std::path::PathBuf;

use crate::Analysed;
use crate::Corpus;
use crate::CorpusName;

/// Represents the path to an analysed document, that is, a specific xml file.
/// Think of it as storing e.g.
/// `/home/user/gut_root/giellalt/corpus-xxx/analysed/some/path/to/file.xml`.
pub struct AnalysedFilePath {
    /// Path to the directory containing the the corpus directory
    pub(crate) analysed: Analysed,

    /// The relative path to the file, inside the `analysed/` module.
    pub(crate) file: PathBuf,
}

impl AnalysedFilePath {
    /// Construct an [AnalysedFilePath] from a `root` directory path, the name
    /// of the corpus, the section...
    /// Parse a `PathBuf` into an `AnalysedFilePath`, or return an error
    /// if the path does not represent a file or directory in a corpus'
    /// directory's `analysed/` subdirectory.
    pub fn builder() -> AnalysedFilePathBuilder {
        AnalysedFilePathBuilder::default()
    }

    /// Return a newly cloned [PathBuf] to the absolute path to this analysed file.
    pub fn to_path_buf(&self) -> PathBuf {
        let mut p = self.analysed.to_path_buf();
        p.push(&self.file);
        p
    }

    /// Does this file exist? Equivalent to `std::fs::exists(&self.to_path_buf())`.
    pub fn exists(&self) -> bool {
        std::fs::exists(&self.to_path_buf()).unwrap_or(false)
    }

    /// Read the contents of this file.
    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(&self.to_path_buf())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AnalysedFilePathBuildError {
    #[error("missing root")]
    MissingRoot,

    #[error("missing corpus name")]
    MissingCorpusName,

    #[error("missing module")]
    MissingModule,

    #[error("missing file")]
    MissingFile,
}

#[derive(Default)]
pub struct AnalysedFilePathBuilder {
    root: Option<PathBuf>,
    corpus_name: Option<CorpusName>,
    file: Option<PathBuf>,
}

impl AnalysedFilePathBuilder {
    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = Some(root);
        self
    }

    pub fn corpus_name(mut self, corpus_name: CorpusName) -> Self {
        self.corpus_name = Some(corpus_name);
        self
    }

    pub fn file(mut self, file: PathBuf) -> Self {
        self.file = Some(file);
        self
    }

    pub fn build(mut self) -> Result<AnalysedFilePath, AnalysedFilePathBuildError> {
        let root = self.root.take().ok_or(AnalysedFilePathBuildError::MissingRoot)?;
        let corpus_name = self.corpus_name.take().ok_or(AnalysedFilePathBuildError::MissingCorpusName)?;
        let file = self.file.take().ok_or(AnalysedFilePathBuildError::MissingFile)?;
        let analysed = Corpus::builder()
            .root_pathbuf(root)
            .corpus_name(corpus_name)
            .build()
            .unwrap()
            .into_analysed();
        Ok(AnalysedFilePath { analysed, file })
    }
}

/*
#[derive(Debug, thiserror::Error)]
pub enum NotAnalysedFilePathError {
    #[error("no corpus directory in path {0}")]
    NoCorpusDir(PathBuf),

    #[error("no analysed directory in path {0}")]
    NoAnalysedDir(PathBuf),
}

impl TryFrom<PathBuf> for AnalysedFilePath {
    type Error = NotAnalysedFilePathError;

    fn try_from(pathbuf: PathBuf) -> Result<Self, Self::Error> {
        let mut corpus = None;
        let mut saw_corpus_name = false;
        let mut root = PathBuf::new();

        for component in pathbuf.as_path().components() {
            match corpus {
                Some(Ok(corpusname)) if component.as_os_str() == "analysed" => {
                    let corpus = Corpus::builder()
                        .root_pathbuf(root)
                        .corpus_name(corpusname)
                        .build()
                        .unwrap();

                    return Ok(Self {
                        file: pathbuf,
                        analysed: Analysed { corpus },
                    });
                }
                _ => {
                    corpus = Some(CorpusName::from_path_component(&component));
                    if corpus.as_ref().unwrap().is_ok() {
                        saw_corpus_name = true;
                    }
                    root.push(component);
                }
            }
        }

        if !saw_corpus_name {
            Err(NotAnalysedFilePathError::NoCorpusDir(pathbuf))
        } else {
            Err(NotAnalysedFilePathError::NoAnalysedDir(pathbuf))
        }
    }
}
*/
