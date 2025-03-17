use json::JsonValue;
use std::{fmt::Display, sync::LazyLock};

// Macro to make it easier to add new RNGs without writing it out three times.
macro_rules! rng_ids_and_names {
    ($( $id: ident, $name: expr);+ $(;)?) => {

        #[derive(PartialEq, Eq, Debug, Clone, Copy)]
        pub enum RngId {
            $(
                $id,
            )+
        }

        impl Display for RngId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(
                        RngId::$id => $name,
                    )+
                };
                write!(f, "{}", name)
            }
        }

    }
}

rng_ids_and_names!(
    A51, "A5/1";
    A52, "A5/2";
    Acorn, "ACORN";
    AlternatingStep, "Alternating Step Generator";
    BlumBlumShub, "Blum-Blum-Shub";
    ChaCha, "ChaCha";
    DualEcDrbg, "Dual_EC_DRBG";
    Geffe, "Geffe";
    Halton, "Halton";
    Isaac, "ISAAC";
    Jsf, "JSF";
    Lcg, "Linear Congruential Generator";
    Lfg, "Lagged Fibonacci Generator";
    Lfsr, "Linear Feedback Shift Register";
    MersenneTwister, "Mersenne Twister";
    MiddleSquare, "Middle Square";
    MiddleSquareBinary, "Middle Square Binary";
    Mwc, "Multiply-with-Carry";
    NaorReingold, "Naor-Reingold";
    Pcg, "Permuted Congruential Generator";
    Rc4, "RC4";
    Rule30, "Rule 30";
    Salsa20, "Salsa20";
    SelfShrinkingGenerator, "Self Shrinking Generator";
    ShrinkingGenerator, "Shrinking Generator";
    Splitmix, "Splitmix64";
    Tt800, "TT800";
    Vmpcr, "VMPC-R";
    Well, "WELL";
    Weyl, "Weyl";
    Xorshift64, "Xorshift64";
    Xoroshiro, "Xoroshiro";
    Xoshiro, "Xoshiro";
);

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

impl From<RngId> for String {
    fn from(id: RngId) -> Self {
        id.to_string()
    }
}

pub static RNG_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("rng_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse rng_descriptions.json")
});
