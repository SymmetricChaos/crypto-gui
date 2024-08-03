use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CipherId {
    Adfgvx,
    Aes,
    Affine,
    Alberti,
    Amsco,
    A51,
    A52,
    A53,
    B64,
    Batco,
    Bazeries,
    Beaufort,
    Bifid,
    Blowfish,
    Caesar,
    ChaCha,
    ChaCha20Poly1305,
    ChaChaExtendedNonce,
    Chaocipher,
    Checkerboard,
    Columnar,
    Decoder,
    Des,
    DesX,
    DiagonalColumnar,
    DiffieHellman,
    Dryad,
    ElGamal,
    Enigma,
    Fialka,
    FourSquare,
    Grille,
    Hebern,
    Hutton,
    Idea,
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
    Rc5,
    Rsa,
    Rs44,
    Salsa20,
    Scytale,
    Seal3,
    SeriatedPlayfair,
    Shamir,
    Sigaba,
    Slidefair,
    Substitution,
    Tea,
    Trifid,
    TripleDes,
    TurningGrille,
    TwoSquare,
    Vic,
    Vigenere,
    XorSplitting,
    Xtea,
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
            CipherId::A51 => "A5/1",
            CipherId::A52 => "A5/2",
            CipherId::A53 => "A5/3",
            CipherId::Adfgvx => "ADFGVX",
            CipherId::Aes => "AES (Rijndael)",
            CipherId::Affine => "Affine",
            CipherId::Alberti => "Alberti Cipher Disk",
            CipherId::Amsco => "AMSCO",
            CipherId::B64 => "B64",
            CipherId::Batco => "BATCO",
            CipherId::Bazeries => "Bazeries",
            CipherId::Beaufort => "Beaufort",
            CipherId::Bifid => "Bifid",
            CipherId::Blowfish => "Blowfish",
            CipherId::Caesar => "Caesar",
            CipherId::ChaCha20Poly1305 => "ChaCha20-Poly1305",
            CipherId::ChaCha => "ChaCha",
            CipherId::ChaChaExtendedNonce => "ChaCha (IETF)",
            CipherId::Chaocipher => "Chaocipher",
            CipherId::Checkerboard => "Straddling Checkerboard",
            CipherId::Columnar => "Columnar Transposition",
            CipherId::Decoder => "Decoder Ring",
            CipherId::Des => "DES",
            CipherId::DesX => "DES-X",
            CipherId::DiagonalColumnar => "Diagonal Columnar",
            CipherId::DiffieHellman => "Diffie-Hellman",
            CipherId::Dryad => "DRYAD",
            CipherId::ElGamal => "ElGamal",
            CipherId::Enigma => "Enigma",
            CipherId::Fialka => "Fialka",
            CipherId::FourSquare => "Four-Square",
            CipherId::Grille => "Grille",
            CipherId::Hebern => "Hebern",
            CipherId::Hutton => "Hutton",
            CipherId::Idea => "IDEA",
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
            CipherId::Rc5 => "RC5",
            CipherId::Rs44 => "RS44",
            CipherId::Rsa => "RSA",
            CipherId::Salsa20 => "Salsa20",
            CipherId::Scytale => "Scytale",
            CipherId::Seal3 => "SEAL 3.0",
            CipherId::SeriatedPlayfair => "Seriated Playfair",
            CipherId::Shamir => "Shamir's Secret Sharing",
            CipherId::Sigaba => "SIGABA",
            CipherId::Slidefair => "Slidefair",
            CipherId::Substitution => "Substitution",
            CipherId::Tea => "TEA",
            CipherId::Trifid => "Trifid",
            CipherId::TripleDes => "Triple-DES",
            CipherId::TurningGrille => "Turning Grille",
            CipherId::TwoSquare => "Two-Square",
            CipherId::Vic => "VIC",
            CipherId::Vigenere => "Vigenère",
            CipherId::XorSplitting => "XOR Secret Splitting",
            CipherId::Xtea => "XTEA",
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
