use std::path::PathBuf;

use crate::Converted;
use crate::AnalysedFilePath;
use crate::{Category, NotCategoryError};

/// The path to a file in the converted module.
pub struct ConvertedFilePath {
    /// Path to the directory containing the the corpus directory.
    pub converted: Converted,

    /// The relative path to the file, inside the given module.
    pub file: PathBuf,
}

impl ConvertedFilePath {
    /// Parse a `PathBuf` into an `ConvertedFilePath`, or return an error
    /// if the path does not represent a file or directory in a corpus'
    /// directory's `converted/` subdirectory.
    pub fn new(_path: PathBuf) -> Result<Self, NotConvertedError> {
        unimplemented!()
    }

    /// Return the full `PathBuf` to this converted file.
    pub fn to_path_buf(&self) -> PathBuf {
        let mut p = self.converted.to_path_buf();
        p.push(self.file.as_path());
        p
    }

    /// Get the corresponding [AnalysedFilePath] for this [ConvertedFilePath].
    pub fn into_analysed_file_path(self) -> AnalysedFilePath {
        let ConvertedFilePath { converted, file } = self;
        converted.into_analysed().with_file(file)
    }

    /// Does this file exist?
    pub fn exists(&self) -> bool {
        std::fs::exists(&self.file).unwrap_or(false)
    }

    /// Read the file from the file system into a new [`String`]
    pub fn read(&self) -> Result<String, ReadConvertedFileError> {
        use ReadConvertedFileError as Error;
        let file = std::fs::File::open(&self.file).map_err(Error::Open)?;

        // SAFETY: mmap() - so UB if the file is changed underneath us, etc...
        let mmapped = unsafe { memmap2::Mmap::map(&file) }.map_err(Error::Mmap)?;
        Ok(String::from_utf8(mmapped.to_vec())?)
    }

    /// Get the category of this file.
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

    /// Is this file part of the historic part of the corpora?
    pub fn is_hist(&self) -> bool {
        matches!(self.category(), Err(NotCategoryError::Hist))
    }

    // Return the same path, but specifically without a file selected,
    // so only the root
    //pub fn corpus_root(&self) -> ConvertedFilePath {
    //    Self {
    //        file: None,
    //        ..self.clone()
    //    }
    //}
}

#[derive(Debug, thiserror::Error)]
#[error("path doesn't contain '<...>/corpus-xxx[...]/converted/<...>'")]
pub struct NotConvertedError;

#[derive(Debug, thiserror::Error)]
pub enum ReadConvertedFileError {
    #[error("Could not open file: {0}")]
    Open(std::io::Error),

    #[error("Could not mmap() file: {0}")]
    Mmap(std::io::Error),

    #[error("Could not xml parse file: {0}")]
    Parse(quick_xml::DeError),

    #[error("File contents not valid utf-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Could not parse xml of content: {0}")]
    ParseXml(#[from] roxmltree::Error),
}

