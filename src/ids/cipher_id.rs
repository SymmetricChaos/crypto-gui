use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CipherID {
    Adfgvx,
    Affine,
    Alberti,
    B64,
    Batco,
    Bazeries,
    Beaufort,
    Bifid,
    Caesar,
    Chaocipher,
    Checkerboard,
    Columnar,
    Decoder,
    Dryad,
    Enigma,
    Fialka,
    FourSquare,
    Grille,
    Hebern,
    Hutton,
    M94,
    M209,
    Playfair,
    Plugboard,
    Polybius,
    PolybiusCube,
    Porta,
    Purple,
    Quagmire,
    RailFence,
    Rs44,
    Scytale,
    Sigaba,
    Slidefair,
    Substitution,
    Trifid,
    TurningGrille,
    TwoSquare,
    Vic,
    Vigenere,
}

impl Default for CipherID {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherID {
    // Describe the history of the cipher
    pub fn description(&self) -> &'static str {
        match CIPHER_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for CipherID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CipherID::Adfgvx => "ADFGVX",
            CipherID::Affine => "Affine",
            CipherID::Alberti => "Alberti Cipher Disk",
            CipherID::B64 => "B64",
            CipherID::Batco => "BATCO",
            CipherID::Bazeries => "Bazeries",
            CipherID::Beaufort => "Beaufort",
            CipherID::Bifid => "Bifid",
            CipherID::Caesar => "Caesar",
            CipherID::Chaocipher => "Chaocipher",
            CipherID::Checkerboard => "Straddling Checkerboard",
            CipherID::Columnar => "Columnar Transposition",
            CipherID::Decoder => "Decoder Ring",
            CipherID::Dryad => "DRYAD",
            CipherID::Enigma => "Enigma",
            // CipherID::Fialka => "Fialka",
            CipherID::FourSquare => "Four-Square",
            CipherID::Grille => "Grille",
            CipherID::Hebern => "Hebern",
            CipherID::Hutton => "Hutton",
            CipherID::M94 => "M94",
            CipherID::M209 => "M209",
            CipherID::Playfair => "Playfair",
            CipherID::Plugboard => "Plugboard",
            CipherID::Polybius => "Polybius Square",
            CipherID::PolybiusCube => "Polybius Cube",
            CipherID::Porta => "Porta",
            CipherID::Purple => "Purple",
            CipherID::Quagmire => "Quagmire",
            CipherID::RailFence => "Rail Fence",
            CipherID::Rs44 => "RS44",
            CipherID::Scytale => "Scytale",
            CipherID::Sigaba => "SIGABA",
            CipherID::Slidefair => "Slidefair",
            CipherID::Substitution => "Substitution",
            CipherID::Trifid => "Trifid",
            CipherID::TurningGrille => "Turning Grille",
            CipherID::TwoSquare => "Two-Square",
            // CipherID::Vic => "VIC",
            CipherID::Vigenere => "Vigenère",
            _ => "<<<MISSING NAME>>>",
        };
        write!(f, "{}", name)
    }
}

impl From<CipherID> for String {
    fn from(id: CipherID) -> Self {
        id.to_string()
    }
}

const JSON_CIPHER_INFORMATION: &'static str = include_str!("cipher_descriptions.json");

lazy_static! {
    pub static ref CIPHER_INFORMATION: JsonValue = {
        json::parse(&JSON_CIPHER_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse cipher_descriptions")
    };
}
