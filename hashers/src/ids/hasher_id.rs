use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HasherId {
    Md4,
    Md5,
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
            Self::Md4 => "MD4",
            Self::Md5 => "MD5",
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