use rand::prelude::{ThreadRng, SliceRandom, IteratorRandom};

pub const LATIN_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LATIN_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const LATIN_UPPER_NO_J: &'static str = "ABCDEFGHIKLMNOPQRSTUVWXYZ";
pub const LATIN_UPPER_NO_Q: &'static str = "ABCDEFGHIJKLMNOPRSTUVWXYZ";
pub const LATIN_UPPER_DIGITS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"; 
pub const DIGITS: &'static str = "0123456789";

pub fn shuffled_str(s: &str, rng: &mut ThreadRng) -> String {
    let mut characters = s.chars().collect::<Vec<char>>();
    let slice = characters.as_mut_slice();
    slice.shuffle(rng);
    slice.iter().map(|x| *x).collect::<String>()
}

pub fn random_char_vec(s: &str, n: usize, rng: &mut ThreadRng) -> Vec<char> {
    s.chars().choose_multiple(rng, n)
}