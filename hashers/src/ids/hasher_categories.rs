use json::JsonValue;
use std::{fmt::Display, sync::LazyLock};

#[derive(Debug, PartialEq, Eq)]
pub enum HasherCategory {
    NonCryptographic,
    Cryptographic,
}

impl Default for HasherCategory {
    fn default() -> Self {
        Self::NonCryptographic
    }
}

impl HasherCategory {
    pub fn description(&self) -> &'static str {
        match HASHER_CATEGORY_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for HasherCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::NonCryptographic => "Non-cryptographic",
            Self::Cryptographic => "Cryptographic",
        };
        write!(f, "{}", name)
    }
}

impl From<HasherCategory> for String {
    fn from(id: HasherCategory) -> Self {
        id.to_string()
    }
}

const JSON_HASHER_CATEGORY_INFORMATION: &'static str =
    include_str!("hasher_category_descriptions.json");

pub static HASHER_CATEGORY_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&JSON_HASHER_CATEGORY_INFORMATION.replace('\u{feff}', ""))
        .expect("unable to parse hasher_category_descriptions.json")
});
