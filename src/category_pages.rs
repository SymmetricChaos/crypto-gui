use crate::cipher_id::CipherID;

#[derive(Debug, PartialEq, Eq)]
pub enum CipherCategory {
    Substituion,
    Polyalphabetic,
    RotorMachine,
    Transposition,
    Playfair,
    Tactical,
    Mutating,
    Polybius,

}

impl CipherCategory {
    pub fn description(&self) -> &'static str {
        match self {
            CipherCategory::Substituion => "Substitution ciphers are the simplest form of cipher.",
            CipherCategory::Polyalphabetic => "NEED DESCRIPTION",
            CipherCategory::RotorMachine => "NEED DESCRIPTION",
            CipherCategory::Transposition => "NEED DESCRIPTION",
            CipherCategory::Playfair => "NEED DESCRIPTION",
            CipherCategory::Tactical => "NEED DESCRIPTION",
            CipherCategory::Mutating => "NEED DESCRIPTION",
            CipherCategory::Polybius => "NEED DESCRIPTION",
        }
    }

    pub fn ciphers(&self) -> &[CipherID] {
        match self {
            CipherCategory::Substituion => &[
                                            CipherID::Caesar,
                                            CipherID::Decoder,
                                            CipherID::Affine,
                                            CipherID::Substitution,],
            CipherCategory::Polyalphabetic => &[],
            CipherCategory::RotorMachine =>  &[],
            CipherCategory::Transposition =>  &[],
            CipherCategory::Playfair => &[],
            CipherCategory::Tactical => &[],
            CipherCategory::Mutating => &[],
            CipherCategory::Polybius => &[],
        }
    }
}
