use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HasherId {
    Blake,
    Blake2,
    Blake3,
    Fnv,
    Md4,
    Md5,
    Pearson,
    Poly1305,
    Sha1,
    Sha2,
    SipHash,
}

impl Default for HasherId {
    fn default() -> Self {
        Self::Md5
    }
}

impl HasherId {
    // Describe the history of the RNG
    pub fn description(&self) -> &'static str {
        match HASHER_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for HasherId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Blake => "BLAKE",
            Self::Blake2 => "BLAKE2",
            Self::Blake3 => "BLAKE3",
            Self::Fnv => "FNV Hash",
            Self::Md4 => "MD4",
            Self::Md5 => "MD5",
            Self::Pearson => "Pearson",
            Self::Poly1305 => "Poly1305",
            Self::Sha1 => "SHA-1",
            Self::Sha2 => "SHA-2",
            Self::SipHash => "SipHash",
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
