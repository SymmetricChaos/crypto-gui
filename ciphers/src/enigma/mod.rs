pub mod enigma;
pub use enigma::EnigmaM3;

pub mod rotors;
pub use rotors::{Reflector, Rotor, REFLECTORS, ROTOR_MAP, ROTOR_VEC};

pub mod plugboard;
pub use plugboard::EnigmaPlugboard;

// References
// This is the M3 Enigma
// https://github.com/aurbano/EnigmaM3_py
// https://cryptii.com/pipes/EnigmaM3-machine

// These two functions are justified as only ASCII uppercase letters are used in Enigma
// they should not be used in any other context
pub(super) fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

pub(super) fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}
