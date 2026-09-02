/// In a corpus path, e.g.
/// `/home/user/gut/giellalt/corpus-abc/converted/admin/some/file/path.xml`
/// the `admin` part of the path is what we call the category, and it corresponds
/// the categories in the Korp interface.
///
/// Other known directories in this place of the path which ARE NOT categories,
/// are  "hist", "inc", and "tmp".
///
/// todo: what is "pma", "rmy-rka", "rmy-rlo"?
pub enum Category {
    Administration,
    Bible,
    Blogs,
    Facta,
    Ficti,
    Laws,
    Literature,
    News,
    Reindriftsnytt,
    Science,
    Speccorp,
    Wikipedia,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Administration => "admin",
            Category::Bible => "bible",
            Category::Blogs => "blogs",
            Category::Facta => "facta",
            Category::Ficti => "ficti",
            Category::Laws => "laws",
            Category::Literature => "literature",
            Category::News => "news",
            Category::Reindriftsnytt => "reindriftsnytt",
            Category::Science => "science",
            Category::Speccorp => "speccorp",
            Category::Wikipedia => "wikipedia",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NotCategoryError {
    #[error("unknown directory: {0}")]
    UnknownCategory(String),

    #[error("the hist directory is not itself a category")]
    Hist,

    #[error("the inc directory is not itself a category")]
    Inc,

    #[error("TODO: what is this 'pma' directory?")]
    Pma,

    #[error("TODO: what is this 'rmy-rka' directory?")]
    Rmyrka,

    #[error("TODO: what is this 'rmy-rlo' directory?")]
    Rmyrlo,

    #[error("the tmp directory, for temporary things")]
    Tmp,
}

impl Category {
    pub fn from_path_str(s: &str) -> Result<Self, NotCategoryError> {
        match s {
            "admin" => Ok(Category::Administration),
            "bible" => Ok(Category::Bible),
            "blogs" => Ok(Category::Blogs),
            "facta" => Ok(Category::Facta),
            "ficti" => Ok(Category::Ficti),
            "hist" => Err(NotCategoryError::Hist),
            "inc" => Err(NotCategoryError::Inc),
            "laws" => Ok(Category::Laws),
            "literature" => Ok(Category::Literature),
            "news" => Ok(Category::News),
            "pma" => Err(NotCategoryError::Pma),
            "reindriftsnytt" => Ok(Category::Reindriftsnytt),
            "rmy-rka" => Err(NotCategoryError::Rmyrka),
            "rmy-rlo" => Err(NotCategoryError::Rmyrlo),
            "science" => Ok(Category::Science),
            "speccorp" => Ok(Category::Speccorp),
            "tmp" => Err(NotCategoryError::Tmp),
            "wikipedia" => Ok(Category::Wikipedia),
            other => Err(NotCategoryError::UnknownCategory(other.to_string())),
        }
    }
}
