use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HasherId {
    Argon2,
    AsconHash,
    AsconMac,
    Blake,
    Blake2,
    Blake3,
    Fnv,
    Ghash,
    Gost,
    Haval,
    Hmac,
    Lm,
    Md2,
    Md4,
    Md5,
    Md6,
    Mgf1,
    OneAtATime,
    Pbkdf1,
    Pbkdf2,
    Pearson,
    Poly1305,
    RipeMd,
    Sha0,
    Sha1,
    Sha2,
    Sha3,
    SipHash,
    Streebog,
    Tiger,
}

impl Default for HasherId {
    fn default() -> Self {
        Self::Md5
    }
}

impl HasherId {
    pub fn description(&self) -> Option<&'static str> {
        HASHER_INFORMATION[self.to_string()]["Description"].as_str()
    }

    pub fn authors(&self) -> Option<&'static str> {
        HASHER_INFORMATION[self.to_string()]["Authors"].as_str()
    }

    pub fn publication_date(&self) -> Option<&'static str> {
        HASHER_INFORMATION[self.to_string()]["Publication"].as_str()
    }
}

impl Display for HasherId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Argon2 => "Argon2",
            Self::AsconHash => "Ascon-Hash",
            Self::AsconMac => "Ascon-Hash",
            Self::Blake => "BLAKE",
            Self::Blake2 => "BLAKE2",
            Self::Blake3 => "BLAKE3",
            Self::Fnv => "FNV Hash",
            Self::Ghash => "GHASH",
            Self::Gost => "GOST R 34.11-94",
            Self::Haval => "HAVAL",
            Self::Hmac => "HMAC",
            Self::Lm => "LM",
            Self::Md2 => "MD2",
            Self::Md4 => "MD4",
            Self::Md5 => "MD5",
            Self::Md6 => "MD6",
            Self::Mgf1 => "MGF1",
            Self::OneAtATime => "OneAtATime",
            Self::Pbkdf1 => "PBKDF1",
            Self::Pbkdf2 => "PBKDF2",
            Self::Pearson => "Pearson",
            Self::Poly1305 => "Poly1305",
            Self::RipeMd => "RIPEMD",
            Self::Sha0 => "SHA-0",
            Self::Sha1 => "SHA-1",
            Self::Sha2 => "SHA-2",
            Self::Sha3 => "SHA-3 (Keccak)",
            Self::SipHash => "SipHash",
            Self::Streebog => "Streebog",
            Self::Tiger => "Tiger",
        };
        write!(f, "{}", name)
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
