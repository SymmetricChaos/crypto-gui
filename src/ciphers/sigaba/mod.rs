pub mod sigaba;
pub use sigaba::Sigaba;

pub mod rotors;
pub use rotors::{CipherRotor, IndexRotor, BIG_ROTOR_MAP, BIG_ROTOR_VEC, INDEX_ROTOR_VEC};

// References
// https://www.nsa.gov/portals/75/documents/about/cryptologic-heritage/historical-figures-publications/publications/technology/The_SIGABA_ECM_Cipher_Machine_A_Beautiful_Idea3.pdf
// https://scholarworks.sjsu.edu/cgi/viewcontent.cgi?article=1125&context=etd_projects

// These two functions are justified as only ASCII uppercase letters they should not be used elsewhere
pub(super) fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

pub(super) fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}
