use rand::prelude::{ThreadRng, SliceRandom, IteratorRandom};

pub const LATIN_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LATIN_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn shuffled_str(s: &str, rng: &mut ThreadRng) -> String {
    let mut characters = s.chars().collect::<Vec<char>>();
    let slice = characters.as_mut_slice();
    slice.shuffle(rng);
    slice.iter().map(|x| *x).collect::<String>()
}

pub fn random_char_vec(s: &str, n: usize, rng: &mut ThreadRng) -> Vec<char> {
    s.chars().choose_multiple(rng, n)
}