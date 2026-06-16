use std::path::Path;

/// Every corpus is divided up into modules, depending on where in the processing stage they are.
// CorpusTools (python) defines these as follows:
// MODULES = [
//    "correct-no-gs/converted",
//    "goldstandard/converted",
//    "stable/converted",
//    "stable/tmx",
//    "analysed",
//    "converted",
//    "korp_mono",
//    "korp_tmx",
//    "tmx",
//]
#[derive(Copy, Clone)]
pub enum Module {
    Analysed,
    Converted,
    CorrectNoGsConverted,
    GoldstandardConverted,
    KorpMono,
    KorpTmx,
    StableConverted,
    StableTmx,
    Tmx,
}

impl Module {
    /// Given a [Path] that starts with anything that matches a module path, extract that
    /// module path from it, and return that [Module], as well as the part of the input
    /// [Path] after the module has been stripped away, or [None] if the given [Path]
    /// does not start with a module.
    pub fn path_strip_module_prefix(path: &Path) -> Option<(Module, &Path)> {
        let mut it = path.components();

        macro_rules! next_component {
            () => {{
                let comp = it.next()?;
                let std::path::Component::Normal(comp) = comp else {
                    return None;
                };
                comp.to_str()?
            }};
        }

        match next_component!() {
            "analysed" => Some((Module::Analysed, it.as_path())),
            "converted" => Some((Module::Converted, it.as_path())),
            "korp_mono" => Some((Module::KorpMono, it.as_path())),
            "korp_tmx" => Some((Module::KorpTmx, it.as_path())),
            "tmx" => Some((Module::Tmx, it.as_path())),
            "correct-no-gs" => (next_component!() == "converted")
                .then_some((Module::CorrectNoGsConverted, it.as_path())),
            "goldstandard" => (next_component!() == "converted")
                .then_some((Module::GoldstandardConverted, it.as_path())),
            "stable" => match next_component!() {
                "converted" => Some((Module::StableConverted, it.as_path())),
                "tmx" => Some((Module::StableTmx, it.as_path())),
                _ => None,
            }
            _ => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid module name")]
pub struct InvalidModuleError;

impl std::str::FromStr for Module {
    type Err = InvalidModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "analysed" => Ok(Module::Analysed),
            "converted" => Ok(Module::Converted),
            "correct-no-gs/converted" => Ok(Module::CorrectNoGsConverted),
            "goldstandard/converted" => Ok(Module::GoldstandardConverted),
            "korp_mono" => Ok(Module::KorpMono),
            "korp_tmx" => Ok(Module::KorpTmx),
            "stable/converted" => Ok(Module::StableConverted),
            "stable/tmx" => Ok(Module::StableTmx),
            "tmx" => Ok(Module::Tmx),
            _ => Err(InvalidModuleError),
        }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Module::Analysed => "analysed",
                Module::Converted => "converted",
                Module::CorrectNoGsConverted => "correct-no-gs/converted",
                Module::GoldstandardConverted => "goldstandard/converted",
                Module::KorpMono => "korp_mono",
                Module::KorpTmx => "korp_tmx",
                Module::StableConverted => "stable/converted",
                Module::StableTmx => "stable/tmx",
                Module::Tmx => "tmx",
            }
        )
    }
}

impl AsRef<Path> for Module {
    fn as_ref(&self) -> &Path {
        match self {
            Module::Analysed => Path::new("analysed"),
            Module::Converted => Path::new("converted"),
            Module::CorrectNoGsConverted => Path::new("correct-no-gs/converted"),
            Module::GoldstandardConverted => Path::new("goldstandard/converted"),
            Module::KorpMono => Path::new("korp_mono"),
            Module::KorpTmx => Path::new("korp_tmx"),
            Module::StableConverted => Path::new("stable/converted"),
            Module::StableTmx => Path::new("stable/tmx"),
            Module::Tmx => Path::new("tmx"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::Module;

    #[test]
    fn strip_module() {
        match Module::path_strip_module_prefix(Path::new("analysed/foo/bar/baz.xml")) {
            Some((Module::Analysed, path)) if path == Path::new("foo/bar/baz.xml") => {},
            Some(_) | None => panic!("incorrect"),
        }
    }
}
