use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

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
    ElGamal, "ElGamal";
    Enigma, "Enigma";
    FealNx, "FEAL-NX";
    Fialka, "Fialka";
    FourSquare, "Four-Square";
    Gift, "GIFT";
    Gost, "GOST 28147-89";
    Grille, "Grille";
    Hebern, "Hebern";
    Hutton, "Hutton";
    Idea, "IDEA";
    Lea, "LEA";
    M94, "M-94";
    M209, "M-209";
    Nihilist, "Nihilist";
    Playfair, "Playfair";
    Plugboard, "Plugboard";
    Present, "PRESENT";
    Polybius, "Polybius Square";
    PolybiusCube, "Polybius Cube";
    Porta, "Porta";
    Purple, "Purple";
    Quagmire, "Quagmire";
    RailFence, "Rail Fence";
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
    Slidefair, "Slidefair";
    Sm4, "SM4";
    Speck, "Speck";
    Substitution, "Substitution";
    Tea, "TEA";
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
