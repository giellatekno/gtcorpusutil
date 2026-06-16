//! A path to an converted file.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use crate::Analysed;
use crate::ConvertedFilePath;
use crate::Corpus;

/// The `converted/` directory in a corpus directory. Used primarily for
/// its `files()` method to iterate over all converted xml files.
#[derive(Clone)]
pub struct Converted {
    /// The corpus this `converted/` directory is inside.
    pub corpus: Corpus,
}

impl From<Corpus> for Converted {
    fn from(corpus: Corpus) -> Self {
        Converted { corpus }
    }
}

impl Converted {
    pub fn to_path_buf(&self) -> PathBuf {
        let mut p = self.corpus.to_path_buf();
        p.push("converted");
        p
    }

    /// Get to a specific file in this `converted/` module.
    pub fn with_file<P: AsRef<Path>>(&self, file: P) -> ConvertedFilePath {
        ConvertedFilePath {
            converted: self.clone(),
            file: file.as_ref().to_path_buf(),
        }
    }

    /// An iterator over all the xml files in this `converted` directory.
    // note: In order to not have to collect in main(), this can consume self
    // instead of take it as a reference, so that the flat_map() call doesn't
    // think that the closure is returning a dangling reference (which it isnt,
    // because it doesn't return any references to self)
    pub fn files(&self) -> impl Iterator<Item = ConvertedFilePath> {
        // anders: sad. has to allocate 2 paths, one to give the root to find_xml_files(),
        // and once to strip that prefix, to retrieve the module and inner file path.
        let converted_path = self.to_path_buf();
        crate::find_xml_files(self.to_path_buf()).filter_map(move |entry| {
            let file = entry.path().strip_prefix(&converted_path)
                .expect("absolute paths from walkdir are subpaths of the input path");
            Some(self.with_file(file))
        })
    }

    pub fn into_analysed(self) -> Analysed {
        Analysed { corpus: self.corpus }
    }

    pub fn to_analysed(&self) -> Analysed {
        Analysed { corpus: self.corpus.clone() }
    }
}

/*
impl TryFrom<PathBuf> for ConvertedFilePath {
    type Error = NotConvertedError;

    fn try_from(pathbuf: PathBuf) -> Result<Self, Self::Error> {
        let mut corpus = None;
        let mut root = PathBuf::new();

        for component in pathbuf.as_path().components() {
            match corpus {
                Some(Ok(corpusname)) if component.as_os_str() == "converted" => {
                    let corpus = Corpus::builder()
                        .root_pathbuf(root)
                        .corpus_name(corpusname)
                        .build()
                        .unwrap();

                    return Ok(Self {
                        file: pathbuf,
                        converted: Converted { corpus },
                    });
                }
                _ => {
                    corpus = Some(CorpusName::from_path_component(&component));
                    root.push(component);
                }
            }
        }

        Err(NotConvertedError)
    }
}
*/
