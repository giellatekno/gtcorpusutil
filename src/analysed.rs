#![allow(dead_code)]

use std::path::Path;
use std::path::PathBuf;

use crate::utils::is_corpusdir_name;
use crate::AnalysedFilePath;
use crate::Corpus;
use crate::KorpMono;
use crate::Module;

/// The `analysed/` subdirectory of a corpus directory. Used primarily for
/// its `files()` method to iterate over all analysed xml files.
#[derive(Clone)]
pub struct Analysed {
    /// The corpus this `analysed/`directory is inside.
    pub(crate) corpus: Corpus,
}

impl From<Corpus> for Analysed {
    fn from(corpus: Corpus) -> Self {
        Analysed { corpus }
    }
}

impl Analysed {
    /// Get a [PathBuf] to this `Analysed` directory.
    pub fn to_path_buf(&self) -> PathBuf {
        let mut p = self.corpus.to_path_buf();
        p.push("analysed");
        p
    }

    /// Turn this [Analysed] into a [KorpMono].
    pub fn into_korp_mono(self) -> KorpMono {
        let Self { corpus } = self;
        KorpMono { corpus }
    }

    /// Get to a specific file in this analysed module.
    pub fn with_file<P: AsRef<Path>>(&self, file: P) -> AnalysedFilePath {
        AnalysedFilePath {
            analysed: self.clone(),
            file: file.as_ref().to_path_buf(),
        }
    }

    /// An iterator over all the xml files in this `analysed` directory.
    pub fn files(&self) -> impl Iterator<Item = AnalysedFilePath> {
        let p = self.to_path_buf();
        crate::find_xml_files(self.to_path_buf()).filter_map(move |entry| {
            let file = entry.path().strip_prefix(&p)
                .expect("absolute path of entry is a subpath of input path");
            Some(self.with_file(file))
        })
    }
}


/// Is this path inside the analysed/ directory of a corpus-xxx folder?
/// We scan from the start of the path, and if we find 'corpus-xxx/analysed',
/// then yes. Else no.
pub fn is_analysed_corpus_dir<P: AsRef<Path>>(path: P) -> bool {
    let mut prev = false;
    for component in path.as_ref().components() {
        if prev {
            if component.as_os_str() == "analysed" {
                return true;
            }
        } else {
            prev = is_corpusdir_name(&component);
        }
    }
    false
}

/*
#[cfg(test)]
mod tests {
    use super::AnalysedFilePath;
    use std::path::PathBuf;

    fn assert_valid(s: &str) {
        assert!(matches!(
            AnalysedFilePath::try_from(PathBuf::from(s)),
            Ok(_)
        ));
    }

    fn assert_invalid(s: &str) {
        assert!(matches!(
            AnalysedFilePath::try_from(PathBuf::from(s)),
            Err(_)
        ));
    }

    #[test]
    fn handles_common_setup() {
        assert_valid("/home/username/gutrepos/giellalt/corpus-abc/analysed/yeah");
    }

    #[test]
    fn handles_common_setup_mac() {
        assert_valid("/Users/username/gutrepos/giellalt/corpus-abc/analysed/yeah");
    }

    #[test]
    fn invalid_dir() {
        assert_invalid("/this/is/not/a/corpus/dir");
    }

    #[test]
    fn only_corpus_xx_but_not_analysed_dir() {
        assert_invalid("/contains-only/corpus-xxx");
    }

    #[test]
    fn analysed_file_path_from_parts() {
        let a = AnalysedFilePath::from_parts("/some/dir/root", "corpus-xxx", "path/to/file.xml")
            .unwrap();
        assert_eq!(
            a.as_path(),
            "/some/dir/root/corpus-xxx/analysed/path/to/file.xml"
        );
    }

    #[test]
    fn handles_long_dir_name() {
        assert_invalid("/superduperlongdirectorycorpus/dir");
    }

    #[cfg(unix)]
    #[test]
    fn handles_nonutf8_component() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        // BYTES is: "/r<INVALID_BYTE>t/corpus-nob/analysed/somefile"
        // 0x80 is 0b10000000, which would be a continuation byte, but the byte
        // before it is not a continuation byte, nor the first byte of a multi
        // byte sequence, so BYTES won't be valid utf-8.
        // It doesn't contain any \0 bytes though, so it's still a valid path
        // component on unix
        static BYTES: &[u8] = &[
            b'/', b'r', 0x80, b't', b'/', b'c', b'o', b'r', b'p', b'u', b's', b'-', b'n', b'o',
            b'b', b'/', b'a', b'n', b'a', b'l', b'y', b's', b'e', b'd', b'/', b's', b'o', b'm',
            b'e', b'f', b'i', b'l', b'e',
        ];

        // anders: the compiler complains about statically knowing that the
        // BYTES is not a valid utf-8 str, and therefore fails to compile.
        // Therefore, we make a vec here, and take a str from it, to force
        // that it should happen at runtime.
        let v = Vec::from(BYTES);

        // assert that the bytes actually are not valid utf-8
        let Err(_) = std::str::from_utf8(&v) else {
            unreachable!("unexpectedly valid utf-8");
        };
        let s = OsStr::from_bytes(BYTES);
        let path = PathBuf::from(s);

        assert!(matches!(AnalysedFilePath::try_from(path), Ok(_)));
    }

    #[test]
    fn handles_stupidly_long_dir_name() {
        assert_invalid(
            "/superduperlongultralongextremelytedioustothepointofbeingbasicallyrediculousofaninsanedirectorynamethatactuallyisntcalledwhatyouthinkitiscalledbutforsomereasonyouhaveactuallyreadouttheentirethingthatiwroteheresoiguesstheonlythingtodoistocongratulateyouonthatandsaygoodjobdirectory/somesubdir",
        );
    }

    /*
    mod analysed_corpus_dir {
        use super::super::is_analysed_corpus_dir;
        use std::path::Path;

        fn assert_valid(s: &str) {
            assert!(is_analysed_corpus_dir(Path::new(s)));
        }

        fn assert_invalid(s: &str) {
            assert!(!is_analysed_corpus_dir(Path::new(s)));
        }

        #[test]
        fn test1() {
            assert_valid("/home/username/gutroot/corpus-xxx/analysed");
        }

        #[test]
        fn test2() {
            assert_invalid("/home/username/gutroot/corpus-xxx/other-invalid/analysed");
        }
    }

    mod test_corpus_dir {
        use crate::corpus_dir::{CorpusDir, CorpusDirVisibility};
        use std::path::Path;

        fn expect_eq(path: &str, expected: Result<CorpusDir, NotCorpusDir>) {
            let path = Path::new(path);
            let mut components = path.components();
            components.next().expect("first path component is /");
            components.next().expect("second compoentn is giellalt");
            let Some(actual) = components.next() else {
                panic!("error in test: third component is ..");
            };

            let actual = corpus_dir(&actual);
            match (expected, actual) {
                (Ok(a), Ok(b)) if a == b => {}
                (Err(a), Err(b)) if a == b => {}
                (a, b) => panic!("expectation failed: expected {a:?}, found {b:?}"),
            }
        }

        #[test]
        fn corpus_dir_not_normal_dir() {
            expect_eq("/giellalt/..", Err(NotCorpusDir::NotNormalDir));
        }

        #[test]
        fn corpus_dir_doesnt_start_with_corpus() {
            expect_eq("/giellalt/dict-xxx-yyy", Err(NotCorpusDir::DoesntStartWithCorpus));
        }

        #[test]
        fn corpus_dir_doesnt_start_with_corpus2() {
            expect_eq("/giellalt/corpu", Err(NotCorpusDir::DoesntStartWithCorpus));
        }

        #[test]
        fn corpus_dir_no_lang() {
            expect_eq("/giellalt/corpus-bleh", Err(NotCorpusDir::NotALang));
        }

        #[test]
        fn corpus_dir_plain() {
            expect_eq("/giellalt/corpus-sme", Ok(CorpusDir {
                lang: "sme".to_string(),
                is_orig: false,
                visiblity: CorpusDirVisibility::Open,
                other_x_properties: vec![]
            }));
        }

        #[test]
        fn corpus_dir_double_lang() {
            expect_eq("/giellalt/corpus-rmy-rka", Ok(CorpusDir {
                lang: "rmy-rka".to_string(),
                is_orig: false,
                visiblity: CorpusDirVisibility::Open,
                other_x_properties: vec![]
            }));
        }

        #[test]
        fn corpus_dir_orig() {
            expect_eq("/giellalt/corpus-sma-orig", Ok(CorpusDir {
                lang: "sma".to_string(),
                is_orig: true,
                visiblity: CorpusDirVisibility::Open,
                other_x_properties: vec![]
            }));
        }

        #[test]
        fn corpus_dir_closed() {
            expect_eq("/giellalt/corpus-sma-x-closed", Ok(CorpusDir {
                lang: "sma".to_string(),
                is_orig: false,
                visiblity: CorpusDirVisibility::Closed,
                other_x_properties: vec![]
            }));
        }

        #[test]
        fn corpus_dir_orig_closed() {
            expect_eq("/giellalt/corpus-sma-orig-x-closed", Ok(CorpusDir {
                lang: "sma".to_string(),
                is_orig: true,
                visiblity: CorpusDirVisibility::Closed,
                other_x_properties: vec![]
            }));
        }

        #[test]
        fn corpus_dir_sensitive() {
            expect_eq("/giellalt/corpus-smn-x-sensitive", Ok(CorpusDir {
                lang: "smn".to_string(),
                is_orig: false,
                visiblity: CorpusDirVisibility::Sensitive,
                other_x_properties: vec![]
            }));
        }

        #[test]
        fn corpus_dir_orig_sensitive() {
            expect_eq("/giellalt/corpus-smn-orig-x-sensitive", Ok(CorpusDir {
                lang: "smn".to_string(),
                is_orig: true,
                visiblity: CorpusDirVisibility::Sensitive,
                other_x_properties: vec![]
            }));
        }
    }
    */
}
*/
