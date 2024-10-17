#[derive(Debug, Default, Clone, PartialEq)]
pub enum PlayerPartyCharacter {
    #[default]
    None,
    Zale,
    Valere,
    Garl,
    Serai,
    Reshan,
    Bst,
    Moraine,
    Unknown,
}

impl PlayerPartyCharacter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Zale => "Zale",
            Self::Valere => "Valere",
            Self::Garl => "Garl",
            Self::Serai => "Seraï",
            Self::Reshan => "Resh'an",
            Self::Bst => "B'st",
            Self::Moraine => "Moraine",
            Self::Unknown => "Unknown",
        }
    }

    /// Parses the name provided to the enum
    ///
    /// In game code this can be:
    /// - normally cased with special characters
    /// - normally cased without special characters
    /// - capitalized with no special characters
    ///
    /// This handles all cases gracefully.
    pub fn parse(value: &str) -> Self {
        match value.to_lowercase().as_ref() {
            "none" => Self::None,
            "zale" => Self::Zale,
            "valere" => Self::Valere,
            "garl" => Self::Garl,
            "seraï" => Self::Serai,
            "serai" => Self::Serai,
            "resh'an" => Self::Reshan,
            "reshan" => Self::Reshan,
            "b'st" => Self::Bst,
            "bst" => Self::Bst,
            "moraine" => Self::Moraine,
            _ => Self::Unknown,
        }
    }
}
