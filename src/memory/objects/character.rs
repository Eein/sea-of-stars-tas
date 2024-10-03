#[derive(Debug, Default, Clone)]
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
    // pub fn as_str(&self) -> &'static str {
    //     match self {
    //         Self::None => "None",
    //         Self::Zale => "Zale",
    //         Self::Valere => "Valere",
    //         Self::Garl => "Garl",
    //         Self::Serai => "Seraï",
    //         Self::Reshan => "Resh'an",
    //         Self::Bst => "B'st",
    //         Self::Moraine => "Moraine",
    //         Self::Unknown => "Unknown",
    //     }
    // }

    pub fn parse(value: &str) -> Self {
        match value.to_lowercase().as_ref() {
            "none" => Self::None,
            "zale" => Self::Zale,
            "valere" => Self::Valere,
            "garl" => Self::Garl,
            "seraï" => Self::Serai,
            "resh'an" => Self::Reshan,
            "b'st" => Self::Bst,
            "moraine" => Self::Moraine,
            _ => Self::Unknown,
        }
    }
}
