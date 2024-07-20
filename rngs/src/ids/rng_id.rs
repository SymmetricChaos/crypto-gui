use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RngId {
    AlternatingStep,
    BlumBlumShub,
    ChaCha,
    Geffe,
    Glfsr,
    Halton,
    Jsf,
    Lcg,
    Lfg,
    Lfsr,
    MersenneTwister,
    MiddleSquare,
    MiddleSquareBinary,
    Pcg,
    Rc4,
    SelfShrinkingGenerator,
    ShrinkingGenerator,
    Splitmix,
    Vmpcr,
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
            RngId::AlternatingStep => "Alternating Step Generator",
            RngId::BlumBlumShub => "Blum-Blum-Shub",
            RngId::ChaCha => "ChaCha",
            RngId::Geffe => "Geffe",
            RngId::Glfsr => "Galois Linear Feedback Shift Register",
            RngId::Halton => "Halton Sequence",
            RngId::Jsf => "JSF",
            RngId::Lcg => "Linear Congruential Generator",
            RngId::Lfg => "Lagged Fibonacci Generator",
            RngId::Lfsr => "Linear Feedback Shift Register",
            RngId::MersenneTwister => "Mersenne Twister",
            RngId::MiddleSquare => "Middle Square",
            RngId::MiddleSquareBinary => "Middle Square Binary",
            RngId::Pcg => "Permuted Congruential Generator",
            RngId::Rc4 => "RC4",
            RngId::SelfShrinkingGenerator => "Self Shrinking Generator",
            RngId::ShrinkingGenerator => "Shrinking Generator",
            RngId::Splitmix => "Splitmix64",
            RngId::Vmpcr => "VMPC-R",
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
