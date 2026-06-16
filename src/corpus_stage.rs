pub enum CorpusStage {
    Converted,
    Analysed,
    KorpMono,
}

impl std::str::FromStr for CorpusStage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "converted" => Ok(Self::Converted),
            "analysed" => Ok(Self::Analysed),
            "korp_mono" => Ok(Self::KorpMono),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for CorpusStage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Converted => "converted",
                Self::Analysed => "analysed",
                Self::KorpMono => "korp_mono",
            }
        )
    }
}
