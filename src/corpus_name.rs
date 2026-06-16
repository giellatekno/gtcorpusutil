use std::ffi::OsStr;

/// The name of a corpus directory. Corpus names are on the form
/// `corpus-LANG[-orig][-x-(closed | sensitive)][-x-OTHERXPROP1[-x-OTHERXPROP2]...]`.
///
/// For example `corpus-sme`, `corpus-xxx-x-sensitive`, or `corpus-xxx-orig-x-closed`.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusName {
    /// Which language this is. Usually a 3-letter ISO-639-3 code, but doesn't
    /// have to be. Cannot contain dashes.
    pub lang: String,

    /// Is this the repository storing the original files? Recognized by having the
    /// "-orig" substring as part of its name.
    pub is_orig: bool,

    /// Which visiblity this corpus has.
    pub visibility: CorpusNameVisibility,

    /// After the "-x-" in the name, we expect "closed", or "sensitive",
    /// and those will be stored in the `visibility` field. However, in the case
    /// where we encounter something else after the "-x-" that we don't
    /// recognize, we store it in this vec. For future expansions.
    pub other_x_properties: Vec<String>,
}

impl CorpusName {
    //fn fmt_into_buf(&self, buf: &mut [u8]) -> Result<usize, ()> {
    //    if buf.len() < self.stringifed_len() {
    //        return Err(());
    //    }

    //    buf[0..7].copy_from_slice(b"corpus-");
    //    buf[7..10].copy_from_slice(self.lang.as_bytes());
    //    let mut n = 10;

    //    if self.is_orig {
    //        buf[10..15].copy_from_slice(b"-orig");
    //        n += 5;
    //    }

    //    let visibility = self.visibility.as_str();
    //    let len = visibility.len();
    //    buf[n..n + len].copy_from_slice(visibility.as_bytes());
    //    n += len;

    //    for prop in self.other_x_properties.iter() {
    //        let len = prop.len();
    //        buf[n..n + len].copy_from_slice(prop.as_bytes());
    //        n += len;
    //    }

    //    Ok(n)
    //}

    //fn stringifed_len(&self) -> usize {
    //    let mut n = "corpus-".len() + self.lang.len();

    //    if self.is_orig {
    //        n += "-orig".len();
    //    }

    //    n += self.visibility.as_str().len();

    //    for s in self.other_x_properties.iter() {
    //        n += "-x-".len() + s.len();
    //    }

    //    n
    //}
}

/// The visiblity of a corpus.
#[derive(Debug, Clone, PartialEq)]
pub enum CorpusNameVisibility {
    /// An open corpus. Recognized in the name by not having "-x-closed",
    /// nor "-x-sensitive".
    Open,
    /// A closed corpus. Recognized in the name by having a "-x-closed"
    /// substring as part of its name.
    Closed,
    /// A "sensitive" corpus. Think of it as "extra closed". Recognized in the
    /// name by having a "x-sensitive" substring as part of its name.
    Sensitive,
}

impl CorpusNameVisibility {
    /// How the visibility specifier appears in the directory name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "",
            Self::Closed => "-x-closed",
            Self::Sensitive => "-x-sensitive",
        }
    }

    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed | Self::Sensitive)
    }

    pub fn is_sensitive(&self) -> bool {
        matches!(self, Self::Sensitive)
    }
}

impl std::fmt::Display for CorpusName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "corpus-")?;
        write!(f, "{}", self.lang)?;
        if self.is_orig {
            write!(f, "-orig")?;
        }
        write!(f, "{}", self.visibility.as_str())?;
        for s in self.other_x_properties.iter() {
            write!(f, "-x-")?;
            write!(f, "{s}")?;
        }
        Ok(())
    }
}

/// Error returned when a name is not a corpus name.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum NotCorpusNameError {
    /// The name doesn't start with `corpus-`.
    #[error("doesn't start with 'corpus-'")]
    InvalidPrefix,

    /// The name starts with `corpus-`, but the language that is supposed
    /// to follow, is missing. Essentially, this means the corpus name was
    /// just `corpus-`.
    #[error("missing lang after corpus-")]
    MissingLang,

    /// The language after "corpus-", i.e. the "xxx" in "corpus-xxx" is not
    /// a language. This only means that it wasn't valid utf-8, as we technically
    /// don't have other restrictions on what the lang could be. Most languages
    /// are 3-ascii-letters ISO-639-3 codes, but they don't have to be.
    #[error("invalid language")]
    InvalidLang,

    /// Malformed, e.g. if the directory name ends with `...-x`.
    #[error("directory name malformed")]
    Malformed,

    /// The name contained a "property" that we don't know what is. This error
    /// is returned when encountering a "property" other than "-x", "-orig",
    /// "-closed", or "-sensitive" has already been read, and we read something
    /// that is neither of these things.
    #[error("unknown property")]
    UnknownProperty,
}

/// Error returned from [CorpusName::from_path_component].
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum FromPathComponentError {
    /// The given [path::Component][::std::path::Component] is not a
    /// [path::Component::Normal][::std::path::Component::Normal].
    #[error("component not normal path")]
    NotNormalDir,

    #[error("not a corpus name")]
    NotCorpusName(#[from] NotCorpusNameError),
}

impl std::str::FromStr for CorpusName {
    type Err = NotCorpusNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CorpusName::from_os_str(OsStr::new(s))
    }
}

impl CorpusName {
    /// Parse a [CorpusName] from a [&path::Component][::std::path::Component].
    /// Internally checks that the component is a
    /// [path::Component::Normal][::std::path::Component::Normal], and delegates to
    /// [from_os_str][CorpusName::from_os_str].
    pub fn from_path_component(
        component: &std::path::Component,
    ) -> Result<CorpusName, FromPathComponentError> {
        if !matches!(component, &std::path::Component::Normal(_)) {
            return Err(FromPathComponentError::NotNormalDir);
        }

        Ok(Self::from_os_str(component.as_os_str())?)
    }

    /// Parse a [CorpusName] from an [OsStr]. If the name is not a corpus directory name,
    /// a fitting error is returned.
    pub fn from_os_str(s: &OsStr) -> Result<CorpusName, NotCorpusNameError> {
        let bytes: &[u8] = s.as_encoded_bytes();

        let Some(bytes) = bytes.strip_prefix(b"corpus-") else {
            return Err(NotCorpusNameError::InvalidPrefix);
        };

        let mut splits = bytes.split(|&b| b == b'-');

        let lang = splits
            .next()
            .expect("split() always has at least 1 item, even if input string is empty");

        if lang.len() == 0 {
            return Err(NotCorpusNameError::MissingLang);
        }

        let lang =
            String::from_utf8(lang.to_vec()).map_err(|_e| NotCorpusNameError::InvalidLang)?;

        let mut visibility = CorpusNameVisibility::Open;
        let mut is_orig = false;
        let mut other_x_properties = Vec::new();
        let mut saw_x = false;

        loop {
            if saw_x {
                saw_x = false;
                match splits.next() {
                    Some(b"closed") => {
                        visibility = CorpusNameVisibility::Closed;
                    }
                    Some(b"sensitive") => {
                        visibility = CorpusNameVisibility::Sensitive;
                    }
                    Some(b"x") => {
                        if saw_x {
                            // double -x-x-
                            return Err(NotCorpusNameError::Malformed);
                        }
                    }
                    Some(other) => {
                        other_x_properties.push(String::from_utf8_lossy(other).into_owned());
                    }
                    None => {
                        return Err(NotCorpusNameError::Malformed);
                    }
                }
            } else {
                match splits.next() {
                    Some(b"x") => {
                        saw_x = true;
                    }
                    Some(b"orig") => {
                        is_orig = true;
                    }
                    Some(_other) => {
                        return Err(NotCorpusNameError::Malformed);
                    }
                    None => {
                        return Ok(CorpusName {
                            lang,
                            is_orig,
                            visibility,
                            other_x_properties,
                        });
                    }
                }
            }
        }
    }

    /// Is this corpus open?
    pub fn is_open(&self) -> bool {
        self.visibility.is_open()
    }

    /// Is this corpus closed? (Note: This will also return true for
    /// `sensitive` corpora as well! Use `is_sensitive` to check specifically
    /// for that.)
    pub fn is_closed(&self) -> bool {
        self.visibility.is_closed()
    }

    /// Is this corpus sensitive?
    pub fn is_sensitive(&self) -> bool {
        self.visibility.is_sensitive()
    }
}

#[cfg(test)]
mod tests {
    use super::{CorpusName, NotCorpusNameError};

    fn expect_ok(name: &str) -> CorpusName {
        match name.parse::<CorpusName>() {
            Ok(val) => val,
            Err(e) => panic!("expecting Ok(_), not Err({e})"),
        }
    }

    fn expect_err(name: &str, error: NotCorpusNameError) {
        match name.parse::<CorpusName>() {
            Ok(_) => panic!("expecting Err({error}), got Ok(_)"),
            Err(e) if e == error => {}
            Err(e) => panic!("expecting Err({error}), got Err({e})"),
        }
    }

    #[test]
    fn non_corpus_prefix() {
        expect_err("something", NotCorpusNameError::InvalidPrefix);
    }

    #[test]
    fn missing_dash() {
        expect_err("corpus", NotCorpusNameError::InvalidPrefix);
    }

    #[test]
    fn missing_lang() {
        expect_err("corpus-", NotCorpusNameError::MissingLang);
    }

    #[test]
    fn basic() {
        let n = "corpus-xyz";
        assert_eq!(expect_ok(n).to_string(), n);
    }

    #[test]
    fn orig() {
        let n = "corpus-xyz-orig";
        assert_eq!(expect_ok(n).to_string(), n);
    }

    #[test]
    fn closed() {
        let n = "corpus-xyz-x-closed";
        assert_eq!(expect_ok(n).to_string(), n);
    }

    #[test]
    fn sensitive() {
        let n = "corpus-xyz-x-sensitive";
        assert_eq!(expect_ok(n).to_string(), n);
    }

    #[test]
    fn orig_closed() {
        let n = "corpus-xyz-orig-x-closed";
        assert_eq!(expect_ok(n).to_string(), n);
    }

    #[test]
    fn orig_sensitive() {
        let n = "corpus-xyz-orig-x-sensitive";
        assert_eq!(expect_ok(n).to_string(), n);
    }
}
