use json::JsonValue;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RngId {
    A51,
    A52,
    AlternatingStep,
    BlumBlumShub,
    ChaCha,
    Geffe,
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
    Salsa20,
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
    pub fn description(&self) -> Option<&'static str> {
        RNG_INFORMATION[self.to_string()]["Description"].as_str()
    }

    pub fn authors(&self) -> Option<&'static str> {
        RNG_INFORMATION[self.to_string()]["Authors"].as_str()
    }

    pub fn publication_date(&self) -> Option<&'static str> {
        RNG_INFORMATION[self.to_string()]["Publication"].as_str()
    }

    pub fn traits(&self) -> Option<&'static str> {
        RNG_INFORMATION[self.to_string()]["Traits"].as_str()
    }
}

impl Display for RngId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RngId::A51 => "A5/1",
            RngId::A52 => "A5/2",
            RngId::AlternatingStep => "Alternating Step Generator",
            RngId::BlumBlumShub => "Blum-Blum-Shub",
            RngId::ChaCha => "ChaCha",
            RngId::Geffe => "Geffe",
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
            RngId::Salsa20 => "Salsa20",
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
