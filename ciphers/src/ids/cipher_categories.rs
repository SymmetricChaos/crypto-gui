use json::JsonValue;
use std::{fmt::Display, sync::LazyLock};

#[derive(Debug, PartialEq, Eq)]
pub enum CipherCategory {
    Substituion,
    Polyalphabetic,
    Electromechanical,
    Transposition,
    Playfair,
    Tactical,
    Polybius,
    DigitalBlock,
    DigitalStream,
    Sharing,
    Composite,
    PublicKey,
}

impl Default for CipherCategory {
    fn default() -> Self {
        Self::Substituion
    }
}

impl CipherCategory {
    pub fn description(&self) -> &'static str {
        match CIPHER_CATEGORY_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for CipherCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CipherCategory::Composite => "Composite",
            CipherCategory::DigitalBlock => "Digital (Block)",
            CipherCategory::DigitalStream => "Digital (Stream)",
            CipherCategory::Electromechanical => "Electromechanical",
            CipherCategory::Playfair => "Playfair",
            CipherCategory::Polyalphabetic => "Polyalphabetic",
            CipherCategory::Polybius => "Polybius",
            CipherCategory::PublicKey => "Public Key",
            CipherCategory::Sharing => "Secret Sharing",
            CipherCategory::Substituion => "Substitution",
            CipherCategory::Tactical => "Tactical",
            CipherCategory::Transposition => "Transposition",
        };
        write!(f, "{}", name)
    }
}

impl From<CipherCategory> for String {
    fn from(id: CipherCategory) -> Self {
        id.to_string()
    }
}

pub static CIPHER_CATEGORY_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("cipher_category_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse cipher_category_descriptions.json")
});
