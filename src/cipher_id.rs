use std::{fmt::Display, collections::HashMap};
use lazy_static::lazy_static;
use json::parse;
use std::fs;

// Not sure if this will work when compiled for web
lazy_static! {
    static ref CIPHER_DESCRIPTIONS: HashMap<String,String> = {
        let file_cts = fs::read_to_string("src\\cipher_descriptions.json")
            .expect("Something went wrong reading cipher_descriptions.json");
        let json = parse(&file_cts)
            .expect("error parsing cipher_descriptions.json");
        json.entries().map(|(name,desc)| (name.to_string(),desc.to_string())).collect::<HashMap<_,_>>()
    };
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CipherID {
    Caesar,
    Affine,
    Decoder,
    Substitution,
    M209,
    Playfair,
    CyclicKey,
    Autokey,
    ProgressiveKey,
}

impl Default for CipherID {
    fn default() -> Self {
        Self::Caesar
    }
}

impl CipherID {
    pub fn description(&self) -> &'static str {
        match CIPHER_DESCRIPTIONS.get(&self.to_string()) {
            Some(text) => &text,
            None => "MISSING DESCRIPTION",
        }
    }
}

impl Display for  CipherID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CipherID::Caesar => "Caesar",
            CipherID::Affine => "Affine",
            CipherID::Decoder => "Decoder Ring",
            CipherID::Substitution => "General Substittution",
            CipherID::M209 => "M209",
            CipherID::Playfair => "Playfair",
            CipherID::CyclicKey => "Cyclic Key",
            CipherID::Autokey => "Autokey",
            CipherID::ProgressiveKey => "Progressive Key",
        };
        write!(f,"{}",name)
    }
}