use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RngId {
    Halton,
    Lcg,
    Lfg,
    Lfsr,
    Md4,
    MiddleSquare,
    Pcg,
    Rc4,
    Splitmix,
    Weyl,
    Xorshift,
    Xoshiro,
}

impl Default for RngId {
    fn default() -> Self {
        Self::Lcg
    }
}

impl RngId {
    // Describe the history of the RNG
    pub fn description(&self) -> &'static str {
        match RNG_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for RngId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RngId::Halton => "Halton Sequence",
            RngId::Lcg => "Linear Congruential Generator",
            RngId::Lfg => "Lagged Fibonacci Generator",
            RngId::Lfsr => "Linear Feedback Shift Register",
            RngId::Md4 => "MD4",
            RngId::MiddleSquare => "Middle Square",
            RngId::Pcg => "Permuted Congruential Generator",
            RngId::Rc4 => "RC4",
            RngId::Splitmix => "Splitmix64",
            RngId::Weyl => "Weyl Sequence",
            RngId::Xorshift => "Xorshift",
            RngId::Xoshiro => "Xoshiro",
            // _ => "<<<MISSING NAME>>>",
        };
        write!(f, "{}", name)
    }
}

impl From<RngId> for String {
    fn from(id: RngId) -> Self {
        id.to_string()
    }
}

const JSON_RNG_INFORMATION: &'static str = include_str!("rng_descriptions.json");

lazy_static! {
    pub static ref RNG_INFORMATION: JsonValue = {
        json::parse(&JSON_RNG_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse rng_descriptions")
    };
}
