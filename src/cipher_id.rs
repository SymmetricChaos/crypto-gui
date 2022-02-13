use std::fmt::Display;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CipherID {
    Caesar,
    Affine,
    Decoder,
    Substitution,
    Polybius,

    M209,
    Enigma,
    SIGABA,

    Playfair,
    Slidefair,

    Vigenere,
    Beaufort,
    Alberti,
    Bazieres,
    M94,

    Columnar,

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
            CipherID::Polybius => "The Polybius Square is an ancient substitutuion cipher that converts each character of the plaintext into a pair that describes its coordinates in a grid. Though it provides no special security on its own it is a key component of very strong composite ciphers.",
            
            CipherID::M209 => "The M209 was an entirely mechanical cipher machine used by the US Military with very complex key settings. The positions of the pins and lugs were set once a day. The exteral positions of the rotors were changed with each message.",
            CipherID::Enigma => "The Enigma machine is probably the most famous rotor machine from the brief era in which they dominated encryption. It was remarkable for its simplicity and compact size. Although it contained critical flaws ultimately the failure of Engima was caused by operational mistakes in the Nazi military that leaked information to the Allies.",
            CipherID::SIGABA => "SIGABA was the most complex rotor machine of its era and is not known to have been successfully attacked during its use. Despite its complexity the United States was extremely paranoid about the device and did not allow allies direct access to it.",

            CipherID::Playfair => "The Playfair Cipher swaps letters on a grid to encrypt letters pair by pair.",
            CipherID::Slidefair => "The Slidefair Cipher",
            
            CipherID::Vigenere => "Vigenere",
            CipherID::Beaufort => "Beaufort",
            CipherID::Alberti => "The Alberti Cipher",
            CipherID::Bazieres => "The Baziers Cipher",
            CipherID::M94 => "The M94 Cipher was a low security tactical cipher US Army that consisted of 25 wheels each with a scrambled alphabet, placed sequentially on a rod. The order of the wheels was changed daily. To send a message the wheels were turned to display it and then an arbitrary other line was used. Decryption relied on the reciever searching for the only sensible line on their own set of wheels but this implementation specifies an offset for each message. Messages had to be sent with exactly 25 letters at a time, padded if the message was too short and broken into pices if it was too long.",

            CipherID::Columnar => "The Columnar Transposition Cipher",

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
            CipherID::Alberti => "Alberti Cipher Disk",
            CipherID::Polybius => "Polybius Square",
            CipherID::Enigma => "Enigma",
            CipherID::SIGABA => "SIGABA",
            CipherID::Slidefair => "Slidefair",
            CipherID::Columnar => "Columnar Transposition",
            CipherID::Bazieres => "Bazieres",
            CipherID::M94 => "M94",
            CipherID::Vigenere => "Vigenere",
            CipherID::Beaufort => "Beaufort",
        };
        write!(f,"{}",name)
    }
}

impl From<CipherID> for String {
    fn from(id: CipherID) -> Self {
        id.to_string()
    }
}