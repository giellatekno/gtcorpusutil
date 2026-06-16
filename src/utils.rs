use std::path::{Path, PathBuf};

/// Turn the given path into a PathBuf, prepending the current working directory
/// if it is relative, and finally canoicalizing the path.
pub fn path_make_absolute_and_canonicalize<P: AsRef<Path>>(
    path: P,
) -> Result<PathBuf, std::io::Error> {
    let path = path.as_ref();
    let p = if path.is_relative() {
        let mut absolute = std::env::current_dir()?;
        absolute.push(path);
        absolute
    } else {
        path.to_path_buf()
    };

    Ok(p.canonicalize()?)
}

/// Error returned when trying to find the root directory
/// from the gut config.
#[derive(thiserror::Error, Debug)]
pub enum FindGutRootError {
    /// There is no gut config dir on the system
    #[error("no config dir for user")]
    NoConfigDir,
    /// Can't read the gut app.toml config file. Contains the inner
    /// [`std::io::Error`].
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    /// Error in the app.toml: contains no "root" property
    #[error("no 'root' property in app.toml")]
    NoRootPropertyInAppToml,
}

/// Return the path to the `gut` root on this system, as a `PathBuf`, by
/// reading the `root` property of `$HOME/.config/gut/app.toml`.
pub fn find_gut_root() -> Result<PathBuf, FindGutRootError> {
    use crate::strip;

    let app_toml_path = dirs::config_dir()
        .ok_or(FindGutRootError::NoConfigDir)?
        .join("gut")
        .join("app.toml");
    let app_toml_contents = std::fs::read_to_string(app_toml_path)?;

    for line in app_toml_contents.lines() {
        let Some((prop, value)) = line.split_once('=') else {
            // line doesn't contain a '='
            continue;
        };
        let prop = prop.trim();
        if prop != "root" {
            continue;
        }

        let value = value.trim();
        let value = match strip(value, "\"") {
            Some(stripped) => stripped,
            None => value,
        };

        let mut pathbuf = PathBuf::from(value);
        pathbuf.push("giellalt");
        return Ok(pathbuf);
    }

    Err(FindGutRootError::NoRootPropertyInAppToml)
}

/// Check if the path component has a name on the form "corpus-XYZ",
/// where X, Y, Z are ascii lowercase letters ('a'..='z')
pub fn is_corpusdir_name(component: &std::path::Component) -> bool {
    // OsString's encoding is a superset of utf-8, so we can compare our ascii
    // bytes like this directly.... right?
    let bytes = component.as_os_str().as_encoded_bytes();

    bytes.len() >= 10
        && bytes[..7] == [b'c', b'o', b'r', b'p', b'u', b's', b'-']
        && bytes[7..10].iter().all(|c| matches!(c, b'a'..=b'z'))
}

#[cfg(test)]
mod tests {
    use super::is_corpusdir_name;
    use std::path::{PathBuf};

    #[test]
    fn rejects_too_short() {
        let path = PathBuf::from("/sme");
        let mut components = path.components().skip(1);
        let component = components.next().unwrap();
        assert!(!is_corpusdir_name(&component));
    }

    #[test]
    fn rejects_invalid_prefix() {
        let path = PathBuf::from("/korpus-sme");
        let mut components = path.components().skip(1);
        let component = components.next().unwrap();
        assert!(!is_corpusdir_name(&component));
    }

    #[test]
    fn rejects_invalid_lang() {
        let path = PathBuf::from("/korpus-SME");
        let mut components = path.components().skip(1);
        let component = components.next().unwrap();
        assert!(!is_corpusdir_name(&component));
    }

    #[test]
    fn accepts_valid_exact_len() {
        let path = PathBuf::from("/corpus-sme");
        let mut components = path.components().skip(1);
        let component = components.next().unwrap();
        assert!(is_corpusdir_name(&component));
    }

    #[test]
    fn accepts_valid_with_extras() {
        let path = PathBuf::from("/corpus-abc-x-whatever");
        let mut components = path.components().skip(1);
        let component = components.next().unwrap();
        assert!(is_corpusdir_name(&component));
    }
}
