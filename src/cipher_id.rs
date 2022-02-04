use std::{fmt::Display};

// use std::collections::HashMap;
// use lazy_static::lazy_static;
// use json::parse;
// use std::fs;
// // Not sure if this will work when compiled for web
// lazy_static! {
//     static ref CIPHER_DESCRIPTIONS: HashMap<String,String> = {
//         let file_cts = fs::read_to_string("src\\cipher_descriptions.json")
//             .expect("Something went wrong reading cipher_descriptions.json");
//         let json = parse(&file_cts)
//             .expect("error parsing cipher_descriptions.json");
//         json.entries().map(|(name,desc)| (name.to_string(),desc.to_string())).collect::<HashMap<_,_>>()
//     };
// }

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
        match self {
            CipherID::Caesar => "The Caesar Cipher is perhaps the oldest and simplest of ciphers. Each letter is simply shifted some number of positions along the alphabet, wrapping around if needed. For example with the standard English alphabet a shift of 2 turns A in C and Y into A.",
            CipherID::Affine => "The Affine Cipher is a simple extension of the Caesar Cipher that applies an affine transform to the alphabet rather than simply shifting the position. This means each letter is assigned a value based on its position then the first key value is added to it and it is multiplied by the second key value. This often gives a position not in the alphabet so the value is reduced by the modulo operation. The multiplication steps adds some extra complexity as multiplicative key must have an inverse modulo the length of the alphabet.",
            CipherID::Decoder => "A Decoder Ring (as popularized by Little Orphan Annie and Captain Midnight, presets exist for each) is a minor variation on the Caesar Cipher. Rather than simply shift the letter's position instead a number is assigned a number and the key is added to that number then reduced by the modulo operation. The original decoder rings were keyed in a slightly more complex way by telling listeners to match a specific letter to a specific number but this is equivalent to simply adding a value.",
            CipherID::Substitution => "The General Substituion Cipher maps a set of symbols one-to-one onto another arbitary set. This implementation allows only maping the symbols of an alphabet but all simple substitution ciphers are included in principle.",
            CipherID::M209 => "The M209 was an entirely mechanical cipher machine used by the US Military with very complex key settings. The positions of the pins and lugs were set once a day. The exteral positions of the rotors were changed with each message.",
            CipherID::Playfair => "The Playfair Cipher swaps letters on a grid to encrypt letters pair by pair.",
            CipherID::CyclicKey => "Cyclic Ley Ciphers repeat their keyword over and over.",
            CipherID::Autokey => "Autokey Ciphers draw their key from the text itself.",
            CipherID::ProgressiveKey => "Progressive key ciphers repeat their key like a cyclic key cipher but apply a shift at each repetition to stretch it out.",
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