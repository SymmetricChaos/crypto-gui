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
    Ars, "ARS";
    BlumBlumShub, "Blum-Blum-Shub";
    ChaCha, "ChaCha";
    Clcg, "Combined Linear Congruential Generator";
    DualEcDrbg, "Dual_EC_DRBG";
    Geffe, "Geffe";
    Gmwc, "Generalized Multiply-with-Carry";
    Hc128, "HC-128";
    Hc256, "HC-256";
    Halton, "Halton";
    Isaac, "ISAAC";
    Jsf, "JSF";
    Kiss, "KISS";
    Kiss11, "KISS11";
    Lcg, "Linear Congruential Generator";
    Lehmer, "Lehmer";
    Lfg, "Lagged Fibonacci Generator";
    Lfsr, "Linear Feedback Shift Register";
    MersenneTwister, "Mersenne Twister";
    MiddleSquare, "Middle Square";
    MiddleSquareBinary, "Middle Square Binary";
    MultipleRecursive, "Multiple Recursive Generator";
    Mwc, "Multiply-with-Carry";
    NaorReingold, "Naor-Reingold";
    Pcg, "Permuted Congruential Generator";
    Plcg, "Polynomial Congruential Generator";
    Philox, "Philox";
    Rabbit, "Rabbit";
    Randu, "RANDU";
    Rc4, "RC4";
    Rule30, "Rule 30";
    Salsa20, "Salsa20";
    SelfShrinkingGenerator, "Self Shrinking Generator";
    ShrinkingGenerator, "Shrinking Generator";
    Splitmix, "Splitmix64";
    Squares, "Squares";
    Threefry, "Threefry";
    Tt800, "TT800";
    Vmpcr, "VMPC-R";
    Well, "WELL";
    Weyl, "Weyl";
    WichmannHill, "Wichmann-Hill";
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
    pub fn description(&self) -> &JsonValue {
        &RNG_INFORMATION[self.to_string()]["Description"]
    }

    pub fn authors(&self) -> &JsonValue {
        &RNG_INFORMATION[self.to_string()]["Authors"]
    }

    pub fn publication_date(&self) -> &JsonValue {
        &RNG_INFORMATION[self.to_string()]["Publication"]
    }

    pub fn traits(&self) -> &JsonValue {
        &RNG_INFORMATION[self.to_string()]["Traits"]
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
