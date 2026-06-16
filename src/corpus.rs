use std::path::{Path, PathBuf};

use crate::analysed::Analysed;
use crate::converted::Converted;
use crate::corpus_name::CorpusName;
use crate::korp_mono::KorpMono;
use crate::root::Root;

/// A path to a corpus directory.
#[derive(Clone)]
pub struct Corpus {
    /// Where this corpus directory is located. Essentially just the parent
    /// of this `corpus-xxx` directory.
    pub root: Root,

    /// The corpus name itself.
    pub corpus_name: CorpusName,
}

impl Corpus {
    pub fn builder() -> CorpusBuilder {
        CorpusBuilder::default()
    }

    pub fn to_path_buf(&self) -> PathBuf {
        let mut buf = self.root.to_path_buf();
        buf.push(self.corpus_name.to_string());
        buf
    }

    pub fn into_analysed(self) -> Analysed {
        Analysed::from(self)
    }

    pub fn into_converted(self) -> Converted {
        Converted::from(self)
    }

    /// Get a [KorpMono] from this [Corpus].
    pub fn to_korp_mono(&self) -> KorpMono {
        KorpMono {
            corpus: self.clone(),
        }
    }

    /// Turn this [Corpus] into a [KorpMono].
    pub fn into_korp_mono(self) -> KorpMono {
        KorpMono::from(self)
    }
}

#[derive(Default)]
pub struct CorpusBuilder {
    root: Option<PathBuf>,
    corpus_name: Option<CorpusName>,
}

impl CorpusBuilder {
    pub fn build(self) -> Result<Corpus, String> {
        let root = self.root.ok_or("root not set")?;
        let root: Root = Root::new(root);
        let corpus_name = self.corpus_name.ok_or("name not set")?;
        Ok(Corpus { root, corpus_name })
    }

    /// Set the root from cloning the given path reference.
    pub fn root_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.root = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the root by moving the given [PathBuf] into this.
    pub fn root_pathbuf(mut self, pathbuf: PathBuf) -> Self {
        self.root = Some(pathbuf);
        self
    }

    /// Set the root by giving a [Root].
    pub fn root_corpus_root(mut self, root: Root) -> Self {
        self.root = Some(root.into_path_buf());
        self
    }

    pub fn corpus_name(mut self, name: CorpusName) -> Self {
        self.corpus_name = Some(name);
        self
    }
}

#[derive(Debug)]
pub struct NotCorpusPathError;

impl TryFrom<PathBuf> for Corpus {
    type Error = NotCorpusPathError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut root = PathBuf::new();

        for component in value.components() {
            let corpus_name = CorpusName::from_path_component(&component);
            if let Ok(corpus_name) = corpus_name {
                let corpus_path = Corpus::builder()
                    .root_pathbuf(root)
                    .corpus_name(corpus_name)
                    .build()
                    .expect("specified both root and name");
                return Ok(corpus_path);
            };
            root.push(component);
        }

        Err(NotCorpusPathError)
    }
}
