use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CipherID {
    Caesar,
    Affine,
    Decoder,
    Substitution,
    Plugboard,

    M209,
    Enigma,
    Sigaba,
    Fialka,
    Hebern,

    Playfair,
    Slidefair,
    TwoSquare,
    FourSquare,

    Vigenere,
    Beaufort,
    Alberti,
    Bazeries,
    M94,
    Porta,

    Columnar,
    Grille,
    TurningGrille,
    RailFence,
    Scytale,

    Polybius,
    PolybiusCube,
    Adfgvx,
    Bifid,
    Trifid,
    B64,
    Checkerboard,

    Batco,
    Dryad,
    Rs44,

    Chaocipher,

    Hutton,
    Quagmire,

    Vic,
    Purple,
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
            CipherID::Caesar => "Caesar",
            CipherID::Affine => "Affine",
            CipherID::Decoder => "Decoder Ring",
            CipherID::Substitution => "Substitution",
            CipherID::M209 => "M209",
            CipherID::Playfair => "Playfair",
            CipherID::Alberti => "Alberti Cipher Disk",
            CipherID::Polybius => "Polybius Square",
            CipherID::PolybiusCube => "Polybius Cube",
            CipherID::Enigma => "Enigma",
            CipherID::Sigaba => "SIGABA",
            CipherID::Slidefair => "Slidefair",
            CipherID::Columnar => "Columnar Transposition",
            CipherID::Bazeries => "Bazeries",
            CipherID::M94 => "M94",
            CipherID::Vigenere => "Vigenère",
            CipherID::Beaufort => "Beaufort",
            CipherID::Adfgvx => "ADFGVX",
            CipherID::Bifid => "Bifid",
            CipherID::Trifid => "Trifid",
            CipherID::B64 => "B64",
            CipherID::Grille => "Grille",
            CipherID::Chaocipher => "Chaocipher",
            CipherID::TurningGrille => "Turning Grille",
            // CipherID::Vic => "VIC",
            CipherID::Batco => "BATCO",
            CipherID::Dryad => "DRYAD",
            CipherID::RailFence => "Rail Fence",
            CipherID::Scytale => "Scytale",
            CipherID::Checkerboard => "Straddling Checkerboard",
            CipherID::Porta => "Porta",
            CipherID::TwoSquare => "Two-Square",
            CipherID::FourSquare => "Four-Square",
            CipherID::Hutton => "Hutton",
            CipherID::Quagmire => "Quagmire",
            // CipherID::Fialka => "Fialka",
            CipherID::Plugboard => "Plugboard",
            CipherID::Rs44 => "RS44",
            CipherID::Hebern => "Hebern",
            CipherID::Purple => "Purple",
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
            .expect("unable to parse cipher_descriptions.json")
    };
}
