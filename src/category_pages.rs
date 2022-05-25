use crate::cipher_id::CipherID;

#[derive(Debug, PartialEq, Eq)]
pub enum CipherCategory {
    Substituion,
    Polyalphabetic,
    RotorMachine,
    Transposition,
    Playfair,
    Tactical,
    Polybius,

}

impl CipherCategory {
    pub fn description(&self) -> &'static str {
        match self {
            CipherCategory::Substituion => "Simple substitution ciphers are likely the oldest kind of cipher and certainly the easiest to understand. Each symbol is replaced with a different symbol in every place that it appears. Mathematically we say there must be a one-to-one relationship between the set symbols the plaintext is written in and the set of symbols that the ciphertext is written in. In many cases these sets are the same.\n\nAll simple substitution ciphers share a weakness to frequency analysis. In all natural languages some symbols or patterns of symbols appear more often than others. When analyzing a large amount of ciphertext an attacker can search for these patterns and then use that knowledge to guess surrounding symbols and eventually reveal the entire plaintext.",
            CipherCategory::Polyalphabetic => "Polyalphabetic ciphers are a broad category of substitution ciphers in which the substitution rule changes with every symbol enciphered. Rotor machines could also be classified here but have their own section. Simple polyaphabetic ciphers just repeat their key and can be attacked by identifying the period and then solving them by parts.\n\nThe first polyaphabetic ciphers received an incorrect reputation for being unbreakable due to their ability to defeat all forms of attack developed for simple substitution. Much later, in 1945, Claude Shannon proved that if the key were truly random and was never reused a polyalphabetic cipher was in fact immune to all forms of cryptanalysis. However huge amounts of high quality random data are difficult to produce and even more difficult to communicate making this system, called a one-time pad usable only in rare scenarios.",
            CipherCategory::RotorMachine => "Rotor Machines were the premier information security technology in the middle of the 20th century including the famous Enigma Machine. They are all electromechanical devices that rely on rotors to change their internal wiring each time a letter is typed. In the simplest form, such as the Hebern Machine, this is simply a Vigenere cipher with an extremely long period. The regular movement of the rotors and the fact that most never move at all is a serious design flaw that exposes such a device to attacks similar to those used against the Vigenere cipher. The machines built for military used additions that confounded anyone relying on this such as the Enigma's plugboard and the SIGABA's stepping maze. Though rendered technically obsolete by in 1970s there were some rotor machines still in use into the late 1980s.",
            CipherCategory::Transposition => "Transposition ciphers disguise text by changing the positions of the letters in it. Unlike other kinds of ciphers this means that a shorter plaintext is less secure than a long one. Although transposition theoretically has a very large key-space, growing exponentially as the size of the text increases, in practice it is very difficult to communicate an arbitrary permutation of some text as a key so the key-space is fairly small. The most powerful use of transposition is to combine it with some other kind of cipher as the transposition will destroy patterns in the plaintext the attacker might otherwise take advantage of.",
            CipherCategory::Playfair => "NEED DESCRIPTION",
            CipherCategory::Tactical => "Tactical Ciphers are designed for use on an active battlefield where there is a need to communicate tactical information quickly but without revealing it to the enemy. The Playfair cipher was also intended as a tactical cipher but has its own section due to the existence of a family of related ciphers. In general tactical ciphers gain security from three places: First tactical information is not relevant for long and thus by the time cryptanalysis can be applied should not help the enemy. Second messages are kept short, limiting the amount of text available to an attacker. Third the key is changed frequently, in both examples a new message key is picked for every transmission and a new page is switched to every few hours.",
            CipherCategory::Polybius => "The Polybius Square is a very slight modification of a simple substitution cipher which encodes each symbol as a pair of symbols. The idea can be extended to a Polybius cube, provided among the examples, or even generalized to arbitrary Polybius hypercubes. On their own none of these provide any increase in security over ordinary substitution. Instead the other members of this family of ciphers use the Polybius square followed by a transposition in order to produce much strong composite ciphers. The information from a single symbol becomes diffused across multiple symbols of the ciphertext.",
        }
    }

    pub fn ciphers(&self) -> &[CipherID] {
        match self {
            CipherCategory::Substituion => &[
                                    CipherID::Caesar,
                                    CipherID::Decoder,
                                    CipherID::Affine,
                                    CipherID::Substitution,],
            CipherCategory::Polyalphabetic => &[
                                    CipherID::Vigenere,
                                    CipherID::Beaufort,
                                    CipherID::M94,
                                    CipherID::Alberti,
                                    CipherID::Bazeries,
                                    CipherID::Porta,
                                    CipherID::Quagmire,
                                    CipherID::Chaocipher, 
                                    CipherID::Hutton],
            CipherCategory::RotorMachine =>  &[
                                    CipherID::M209, 
                                    CipherID::Enigma, 
                                    CipherID::Sigaba,],
            CipherCategory::Transposition =>  &[
                                    CipherID::Columnar,
                                    CipherID::Grille,
                                    CipherID::RailFence,
                                    CipherID::Scytale,],
            CipherCategory::Playfair => &[
                                    CipherID::Playfair,
                                    CipherID::Slidefair,
                                    CipherID::TwoSquare,
                                    CipherID::FourSquare,],
            CipherCategory::Tactical => &[
                                    CipherID::Batco, 
                                    CipherID::Dryad],
            CipherCategory::Polybius => &[
                                    CipherID::Polybius,
                                    CipherID::PolybiusCube,
                                    CipherID::Adfgvx,
                                    CipherID::B64,
                                    CipherID::Bifid,
                                    CipherID::Trifid,
                                    CipherID::Checkerboard,],
        }
    }
}
