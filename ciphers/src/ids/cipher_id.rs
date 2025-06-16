use json::{iterators::Members, JsonValue};
use std::{fmt::Display, sync::LazyLock};

// Macro to make it easier to add new ciphers without writing it out three times.
macro_rules! cipher_ids_and_names {
    ($( $id: ident, $name: expr);+ $(;)?) => {

        #[derive(PartialEq, Eq, Debug, Clone, Copy)]
        pub enum CipherId {
            $(
                $id,
            )+
        }

        impl Display for CipherId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(
                        CipherId::$id => $name,
                    )+
                };
                write!(f, "{}", name)
            }
        }

    }
}

cipher_ids_and_names!(
    Adfgvx, "ADFGVX";
    Aes, "AES";
    AesGcm, "AES-GCM";
    Affine, "Affine";
    Alberti, "Alberti Cipher Disk";
    Amsco, "AMSCO";
    Ascon128, "ASCON-128";
    Ascon80pq, "ASCON-80pq";
    Aria, "ARIA";
    A51, "A5/1";
    A52, "A5/2";
    A53, "A5/3";
    B64, "B64";
    Batco, "BATCO";
    Bazeries, "Bazeries";
    Beaufort, "Beaufort";
    Bifid, "Bifid";
    Blowfish, "Blowfish";
    Caesar, "Caesar";
    Camellia, "Camellia";
    ChaCha, "ChaCha";
    ChaCha20Poly1305, "ChaCha20-Poly1305";
    Chaocipher, "Chaocipher";
    Checkerboard, "Straddling Checkerboard";
    Columnar, "Columnar Transposition";
    Decoder, "Decoder Ring";
    Des, "DES";
    DesX, "DES-X";
    DiagonalColumnar, "Diagonal Columnar";
    DiffieHellman, "Diffie-Hellman";
    Dryad, "DRYAD";
    E2, "E2";
    ElGamal, "ElGamal";
    Enigma, "Enigma";
    FealNx, "FEAL-NX";
    Fialka, "Fialka";
    FourSquare, "Four-Square";
    Gift, "GIFT";
    Gost, "GOST 28147-89";
    Grille, "Grille";
    Hc128, "HC-128";
    Hc256, "HC-256";
    Hebern, "Hebern";
    Hutton, "Hutton";
    Idea, "IDEA";
    Isaac, "ISAAC";
    Kasumi, "KASUMI";
    Khufu, "Khufu";
    Khafre, "Khafre";
    Lea, "LEA";
    Lorzen, "Lorenz";
    Lucifer, "Lucifer";
    M94, "M-94";
    M209, "M-209";
    Misty1, "MISTY1";
    Nihilist, "Nihilist";
    Playfair, "Playfair";
    Plugboard, "Plugboard";
    Present, "PRESENT";
    Polybius, "Polybius Square";
    PolybiusCube, "Polybius Cube";
    Porta, "Porta";
    Purple, "Purple";
    Quagmire, "Quagmire";
    Rabbit, "Rabbit";
    RailFence, "Rail Fence";
    Rc2, "RC2";
    Rc4, "RC4";
    Rc5, "RC5";
    Rsa, "RSA";
    Rs44, "RS44";
    Salsa20, "Salsa20";
    Scytale, "Scytale";
    Seal3, "SEAL 3.0";
    Seed, "SEED";
    SeriatedPlayfair, "Seriated Playfair";
    Serpent, "Serpent";
    Shamir, "Shamir's Secret Sharing";
    Sigaba, "SIGABA";
    Simon, "Simon";
    Skipjack, "Skipjack";
    Slidefair, "Slidefair";
    Solitaire, "Solitaire";
    Sm4, "SM4";
    Snow3G, "Snow 3G";
    Speck, "Speck";
    Substitution, "Substitution";
    Tea, "TEA";
    Threefish, "Threefish";
    Trifid, "Trifid";
    TripleDes, "Triple DES";
    TurningGrille, "Turning Grille";
    Twofish, "Twofish";
    TwoSquare, "Two-Square";
    Vic, "VIC";
    Vigenere, "Vigenère";
    XChaCha, "XChaCha";
    XorSplitting, "XOR Secret Splitting";
    Xtea, "XTEA";
    Xxtea, "XXTEA";
);

impl Default for CipherId {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherId {
    pub fn description(&self) -> &JsonValue {
        &CIPHER_INFORMATION[self.to_string()]["Description"]
    }

    pub fn authors(&self) -> &JsonValue {
        &CIPHER_INFORMATION[self.to_string()]["Authors"]
    }

    pub fn publication_date(&self) -> &JsonValue {
        &CIPHER_INFORMATION[self.to_string()]["Publication"]
    }

    pub fn traits(&self) -> Members {
        CIPHER_INFORMATION[self.to_string()]["Traits"].members()
    }

    pub fn names(&self) -> Members {
        CIPHER_INFORMATION[self.to_string()]["Names"].members()
    }
}

impl From<CipherId> for String {
    fn from(id: CipherId) -> Self {
        id.to_string()
    }
}

pub static CIPHER_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("cipher_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse cipher_descriptions.json")
});
