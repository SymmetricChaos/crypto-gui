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
    Ascon128,
    Ascon80pq,
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
    FealNx,
    Fialka,
    FourSquare,
    Gost,
    Grille,
    Hebern,
    Hutton,
    Idea,
    Lea,
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
    Seed,
    SeriatedPlayfair,
    Shamir,
    Sigaba,
    Simon,
    Slidefair,
    Sm4,
    Speck,
    Substitution,
    Tea,
    Trifid,
    TripleDes,
    TurningGrille,
    TwoSquare,
    Vic,
    Vigenere,
    XChaCha,
    XorSplitting,
    Xtea,
    Xxtea,
}

impl Default for CipherId {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherId {
    pub fn description(&self) -> Option<&'static str> {
        CIPHER_INFORMATION[self.to_string()]["Description"].as_str()
    }

    pub fn authors(&self) -> Option<&'static str> {
        CIPHER_INFORMATION[self.to_string()]["Authors"].as_str()
    }

    pub fn publication_date(&self) -> Option<&'static str> {
        CIPHER_INFORMATION[self.to_string()]["Publication"].as_str()
    }

    pub fn traits(&self) -> Option<&'static str> {
        CIPHER_INFORMATION[self.to_string()]["Traits"].as_str()
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
            CipherId::Ascon128 => "Ascon-128",
            CipherId::Ascon80pq => "Ascon-80pq",
            CipherId::B64 => "B64",
            CipherId::Batco => "BATCO",
            CipherId::Bazeries => "Bazeries",
            CipherId::Beaufort => "Beaufort",
            CipherId::Bifid => "Bifid",
            CipherId::Blowfish => "Blowfish",
            CipherId::Caesar => "Caesar",
            CipherId::ChaCha20Poly1305 => "ChaCha20-Poly1305",
            CipherId::ChaCha => "ChaCha",
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
            CipherId::FealNx => "FEAL-NX",
            CipherId::Fialka => "Fialka",
            CipherId::FourSquare => "Four-Square",
            CipherId::Gost => "GOST 28147-89",
            CipherId::Grille => "Grille",
            CipherId::Hebern => "Hebern",
            CipherId::Hutton => "Hutton",
            CipherId::Idea => "IDEA",
            CipherId::Lea => "LEA",
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
            CipherId::Seed => "SEED",
            CipherId::SeriatedPlayfair => "Seriated Playfair",
            CipherId::Shamir => "Shamir's Secret Sharing",
            CipherId::Sigaba => "SIGABA",
            CipherId::Simon => "Simon",
            CipherId::Slidefair => "Slidefair",
            CipherId::Sm4 => "SM4",
            CipherId::Speck => "Speck",
            CipherId::Substitution => "Substitution",
            CipherId::Tea => "TEA",
            CipherId::Trifid => "Trifid",
            CipherId::TripleDes => "Triple-DES",
            CipherId::TurningGrille => "Turning Grille",
            CipherId::TwoSquare => "Two-Square",
            CipherId::Vic => "VIC",
            CipherId::Vigenere => "Vigenère",
            CipherId::XorSplitting => "XOR Secret Splitting",
            CipherId::XChaCha => "XChaCha",
            CipherId::Xtea => "XTEA",
            CipherId::Xxtea => "XXTEA",
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
