//! # Overview
//!
//! Library and utilities for working with giellatekno/divvun corpora. This
//! crate complements the python based [corpustools](http://github.com/divvun/corpustools).
//!
//! ## Path structures
//!
//! This crate typifies paths to differet parts of the corpora files. Subdirectories
//! and individual files, are individual types. A diagram may be helpful:
//!
//! ```not_rust
//! |----------------------- AnalysedFilePath --------------------------------|
//! |----------------- Analysed ------------------------|
//! |----------- Corpus ------------------------|
//! |----------- Root ---------------|
//! /home/someuser/gut_repos/giellalt/corpus-sme/analysed/facta/some/x/file.xml
//! ```
//!
//!
//! ## Root
//!
//! [`Root`] is a path to a directory where it is assumed that many `corpus-xxx`
//! directories (repositories cloned from github.com/giellalt/corpus-xxx) are
//! located.
//!
//! If [gut](https://github.com/divvun/gut) is installed on the system, the gut
//! root directory can be retreived using [`Root::from_gut_config()`]. Otherwise,
//! construct it from a [`std::path::PathBuf`] through the `From<PathBuf>` implementation.
//!
//!
//! ## Corpus
//!
//! [`Corpus`] represents a corpus directory, e.g. `corpus-sme`, which
//! typically resides in a parent folder with many corpus directories, but
//! not necessarily. [`Root::corpora()`] returns an iterator over all such
//! corpus directories in a [`Root`] directory. [`Corpus::builder()`] can also
//! be used to construct a [`Corpus`] from a root `PathBuf`, and a custom
//! [`CorpusName`].
//!
//!
//! ## CorpusName
//!
//! [`CorpusName`] represents the name of a corpus directory. Examples of
//! corpus names are `corpus-sme` and `corpus-nob-x-closed`. [`CorpusName`]s
//! are constructed through [FromStr][::std::str::FromStr] implementation, or
//! other constructors.
//!
//! ```
//! let name: CorpusName = "corpus-sme".parse().expect("parses ok");
//! assert_eq!(name.lang, "sme");
//! assert_eq!(name.is_orig, false);
//! assert_eq!(name.visiblity, CorpusNameVisibility::Open);
//! assert_eq!(name.other_x_properties, vec![]);
//!
//! let invalid_name: CorpusName = "invalid-corpus-name".parse();
//! assert!(matches!(invalid_name, Err(NotCorpusNameError::InvalidPrefix)));
//! ```
//!
//!
//! ## Analysed / Converte / KorpMono
//!
//! [`Analysed`], [`Converted`], and [`KorpMono`] corresponds to the different
//! sections of a corpora.
//!
//! ## ...FilePath
//!
//! [`AnalysedFilePath`], [`ConvertedFilePath`], and [`KorpMonoFilePath`] represents
//! paths to individual `.xml` files in the corresponding section.

mod analysed;
mod analysed_document;
mod analysed_file_path;
mod converted;
mod converted_document;
mod converted_file_path;
mod corpus;
mod corpus_name;
mod corpus_stage;
mod korp_mono;
mod korp_mono_file_path;
mod module;
mod remove_ats;
mod root;
pub mod utils;

use std::path::{Path, PathBuf};

// public exports
pub use crate::analysed::{Analysed};
pub use crate::analysed_file_path::AnalysedFilePath;
pub use crate::converted_file_path::ConvertedFilePath;
pub use crate::converted::Converted;
pub use crate::corpus::CorpusBuilder;
pub use crate::converted_document::ConvertedDocument;
pub use crate::corpus::Corpus;
pub use crate::corpus_name::CorpusName;
pub use crate::corpus_name::CorpusNameVisibility;
pub use crate::corpus_name::FromPathComponentError;
pub use crate::corpus_name::NotCorpusNameError;
pub use crate::korp_mono::KorpMono;
pub use crate::korp_mono_file_path::KorpMonoFilePath;
pub use crate::module::InvalidModuleError;
pub use crate::module::Module;
pub use crate::remove_ats::remove_ats;
pub use crate::root::Root;

/// Returns a [`str`] with the prefix and suffix of `substr` removed, or
/// `None` if the string doesn't both start and end with `substr`.
pub fn strip<'a>(string: &'a str, substr: &str) -> Option<&'a str> {
    Some(string.strip_prefix(substr)?.strip_suffix(substr)?)
}

/// Recursively iterate the directory given by `path`, and yield
/// all .xml files as `walkdir::DirEntry`.
pub fn find_xml_files<P: AsRef<std::path::Path>>(path: P) -> impl Iterator<Item = walkdir::DirEntry> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .flat_map(move |maybe_entry| maybe_entry)
        .filter(move |entry| entry.file_type().is_file())
        .filter(move |entry| {
            entry
                .file_name()
                .to_str()
                .expect("all file names are utf-8")
                .ends_with(".xml")
        })
}

pub fn path_rel2abs_with_cwd<P: AsRef<Path>>(path: P) -> Result<PathBuf, std::io::Error> {
    let path = path.as_ref();
    if path.is_absolute() {
        Ok(path.to_owned())
    } else {
        let mut cwd = std::env::current_dir()?;
        cwd.push(path);
        Ok(cwd)
    }
}

pub fn lang_resource_dirs(lang: &str) -> Vec<PathBuf> {
    let mut out = vec![];
    if let Some(mut home) = std::env::home_dir() {
        home.push(".local/share/giella/");
        home.push(lang);
        out.push(home);
    }
    out.push(PathBuf::from(format!("/usr/local/share/giella/{lang}")));
    out.push(PathBuf::from(format!("/usr/share/giella/{lang}")));
    out
}

/// Find a language resource file (e.g. .hfstol, .cg3, etc) for language `lang`, in the
/// common paths, i.e. in `(~/.local | /usr/ | /usr/local/)/share/giella/{lang}/{file}`.
/// Will print warnings to stderr when an error occured trying to read a file that
/// exists.
pub fn find_lang_resource(lang: &str, file: &str) -> Option<PathBuf> {
    lang_resource_dirs(lang)
        .into_iter()
        .map(|mut dir| {
            dir.push(file);
            dir
        })
        .filter_map(|path| match path.try_exists() {
            Ok(true) => Some(path),
            Ok(false) => None,
            Err(e) => {
                eprintln!("warn: Error reading '{}': {e}", path.display());
                None
            }
        })
        .next()
}
