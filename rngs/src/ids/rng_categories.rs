use json::JsonValue;
use std::{fmt::Display, sync::LazyLock};

#[derive(Debug, PartialEq, Eq)]
pub enum RngCategory {
    PRNG,
    CSPRNG,
    QRNG,
    TRNG,
}

impl Default for RngCategory {
    fn default() -> Self {
        Self::PRNG
    }
}

impl RngCategory {
    pub fn description(&self) -> &'static str {
        match RNG_CATEGORY_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for RngCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RngCategory::PRNG => "PRNG",
            RngCategory::CSPRNG => "CSPRNG",
            RngCategory::QRNG => "QRNG",
            RngCategory::TRNG => "True Random",
        };
        write!(f, "{}", name)
    }
}

impl From<RngCategory> for String {
    fn from(id: RngCategory) -> Self {
        id.to_string()
    }
}

pub static RNG_CATEGORY_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("rng_category_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse rng_category_descriptions.json")
});
