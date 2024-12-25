use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

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
    AsconHash, "Ascon";
    Bcrypt, "bcrypt";
    Belt, "BelT";
    Blake, "BLAKE";
    Blake2, "BLAKE2";
    Blake3, "BLAKE3";
    CityHash, "CityHash";
    Fletcher, "Fletcher";
    Fnv, "FNV Hash";
    Ghash, "GHASH";
    Gost, "GOST R 34.11-94";
    Haval, "HAVAL";
    Hmac, "HMAC";
    Lm, "LM";
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
    RipeMd, "RIPEMD";
    Scrypt, "scrypt";
    Sha0, "SHA-0";
    Sha1, "SHA-1";
    Sha2, "SHA-2";
    Sha3, "SHA-3 (Keccak)";
    Shabal, "Shabal";
    SipHash, "SipHash";
    Sm3, "SM3";
    Snefru, "Snefru";
    Streebog, "Streebog";
    Tiger, "Tiger";
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

const JSON_HASHER_INFORMATION: &'static str = include_str!("hasher_descriptions.json");

lazy_static! {
    pub static ref HASHER_INFORMATION: JsonValue = {
        json::parse(&JSON_HASHER_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse hasher_descriptions")
    };
}
