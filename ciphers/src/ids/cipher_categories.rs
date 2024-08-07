use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

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

const JSON_CIPHER_CATEGORY_INFORMATION: &'static str =
    include_str!("cipher_category_descriptions.json");

lazy_static! {
    pub static ref CIPHER_CATEGORY_INFORMATION: JsonValue = {
        json::parse(&JSON_CIPHER_CATEGORY_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse cipher_category_descriptions.json")
    };
}
