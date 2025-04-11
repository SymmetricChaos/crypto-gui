use json::JsonValue;
use std::{fmt::Display, sync::LazyLock};

// Macro to make it easier to add new hashers without writing it out three times.
macro_rules! hasher_ids_and_names {
    ($( $id: ident, $name: expr);+ $(;)?) => {

        #[derive(PartialEq, Eq, Debug, Clone, Copy)]
        pub enum HasherId {
            $(
                $id,
            )+
        }

        impl Display for HasherId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(
                        HasherId::$id => $name,
                    )+
                };
                write!(f, "{}", name)
            }
        }

    }
}

hasher_ids_and_names!(
    Adler32, "Adler-32";
    Argon2, "Argon2";
    Ascon, "Ascon";
    Balloon, "BalloonHash";
    Bcrypt, "bcrypt";
    Belt, "BelT";
    Blake, "BLAKE";
    Blake2, "BLAKE2";
    Blake3, "BLAKE3";
    CityHash, "CityHash";
    Crypt, "crypt";
    Fletcher, "Fletcher";
    Fnv, "FNV Hash";
    FxHash, "FxHash";
    Ghash, "GHASH";
    Gost, "GOST R 34.11-94";
    Haval, "HAVAL";
    Hkdf, "HKDF";
    Hmac, "HMAC";
    Jh, "JH";
    Lm, "LM";
    Lsh, "LSH";
    Md2,"MD2";
    Md4, "MD4";
    Md5, "MD5";
    Md6, "MD6";
    Mgf1, "MGF1";
    MurmurHash3, "MurmurHash3";
    OneAtATime,"OneAtATime";
    Pbkdf1, "PBKDF1";
    Pbkdf2, "PBKDF2";
    Pearson, "Pearson";
    Poly1305, "Poly1305";
    RadioGatun, "RadioGatún";
    RipeMd, "RIPEMD";
    Scrypt, "scrypt";
    Sha0, "SHA-0";
    Sha1, "SHA-1";
    Sha2, "SHA-2";
    Sha3, "SHA-3 (Keccak)";
    Shabal, "Shabal";
    SipHash, "SipHash";
    Skein, "Skein";
    Sm3, "SM3";
    Snefru, "Snefru";
    Streebog, "Streebog";
    Tiger, "Tiger";
    Vsh, "VSH";
);

impl Default for HasherId {
    fn default() -> Self {
        Self::Md5
    }
}

impl HasherId {
    pub fn description(&self) -> &JsonValue {
        &HASHER_INFORMATION[self.to_string()]["Description"]
    }

    pub fn authors(&self) -> &JsonValue {
        &HASHER_INFORMATION[self.to_string()]["Authors"]
    }

    pub fn publication_date(&self) -> &JsonValue {
        &HASHER_INFORMATION[self.to_string()]["Publication"]
    }

    pub fn traits(&self) -> &JsonValue {
        &HASHER_INFORMATION[self.to_string()]["Traits"]
    }
}

impl From<HasherId> for String {
    fn from(id: HasherId) -> Self {
        id.to_string()
    }
}

pub static HASHER_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("hasher_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse hasher_descriptions.json")
});
