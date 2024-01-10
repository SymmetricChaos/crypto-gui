use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CipherId {
    Adfgvx,
    Affine,
    Alberti,
    Amsco,
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
    DiagonalColumnar,
    Dryad,
    Enigma,
    Fialka,
    FourSquare,
    Grille,
    Hebern,
    Hutton,
    M94,
    M209,
    Nihilist,
    Playfair,
    Plugboard,
    Polybius,
    PolybiusCube,
    Porta,
    Purple,
    Quagmire,
    RailFence,
    Rc4,
    Rs44,
    Scytale,
    SeriatedPlayfair,
    Shamir,
    Sigaba,
    Slidefair,
    Substitution,
    Trifid,
    TurningGrille,
    TwoSquare,
    Vic,
    Vigenere,
}

impl Default for CipherId {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherId {
    // Describe the history of the cipher
    pub fn description(&self) -> &'static str {
        match CIPHER_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for CipherId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CipherId::Adfgvx => "ADFGVX",
            CipherId::Affine => "Affine",
            CipherId::Alberti => "Alberti Cipher Disk",
            CipherId::Amsco => "AMSCO",
            CipherId::B64 => "B64",
            CipherId::Batco => "BATCO",
            CipherId::Bazeries => "Bazeries",
            CipherId::Beaufort => "Beaufort",
            CipherId::Bifid => "Bifid",
            CipherId::Caesar => "Caesar",
            CipherId::Chaocipher => "Chaocipher",
            CipherId::Checkerboard => "Straddling Checkerboard",
            CipherId::Columnar => "Columnar Transposition",
            CipherId::Decoder => "Decoder Ring",
            CipherId::DiagonalColumnar => "Diagonal Columnar",
            CipherId::Dryad => "DRYAD",
            CipherId::Enigma => "Enigma",
            // CipherID::Fialka => "Fialka",
            CipherId::FourSquare => "Four-Square",
            CipherId::Grille => "Grille",
            CipherId::Hebern => "Hebern",
            CipherId::Hutton => "Hutton",
            CipherId::M94 => "M94",
            CipherId::M209 => "M209",
            CipherId::Nihilist => "Nihilist",
            CipherId::Playfair => "Playfair",
            CipherId::Plugboard => "Plugboard",
            CipherId::Polybius => "Polybius Square",
            CipherId::PolybiusCube => "Polybius Cube",
            CipherId::Porta => "Porta",
            CipherId::Purple => "Purple",
            CipherId::Quagmire => "Quagmire",
            CipherId::RailFence => "Rail Fence",
            CipherId::Rc4 => "RC4",
            CipherId::Rs44 => "RS44",
            CipherId::Scytale => "Scytale",
            CipherId::SeriatedPlayfair => "Seriated Playfair",
            CipherId::Shamir => "Shamir's Secret Sharing",
            CipherId::Sigaba => "SIGABA",
            CipherId::Slidefair => "Slidefair",
            CipherId::Substitution => "Substitution",
            CipherId::Trifid => "Trifid",
            CipherId::TurningGrille => "Turning Grille",
            CipherId::TwoSquare => "Two-Square",
            CipherId::Vic => "VIC",
            CipherId::Vigenere => "Vigenère",
            _ => "<<<MISSING NAME>>>",
        };
        write!(f, "{}", name)
    }
}

impl From<CipherId> for String {
    fn from(id: CipherId) -> Self {
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
