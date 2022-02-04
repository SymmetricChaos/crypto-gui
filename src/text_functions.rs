use itertools::Itertools;
use rand::prelude::{ThreadRng, SliceRandom, IteratorRandom};

use crate::{errors::CipherError};

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

pub fn random_sample_replace(s: &str, n: usize, rng: &mut ThreadRng) -> String {
    let mut out = String::with_capacity(n);
    for _ in 0..n {
        out.push(s.chars().choose(rng).unwrap())
    }
    out
}

pub fn random_char_vec(s: &str, n: usize, rng: &mut ThreadRng) -> Vec<char> {
    s.chars().choose_multiple(rng, n)
}

pub fn validate_alphabet(alphabet: &str) -> Result<(),CipherError> {

    // Most basic check, symbols in an alphabet must be unique
    if alphabet.chars().count() != alphabet.chars().unique().count() {
        return Err(CipherError::Alphabet(String::from("characters must all be unique")))
    }

    // Eliminate potentiually confusing characters
    for symbol in alphabet.chars() {
        if symbol.is_control() || symbol.is_whitespace() {
            return Err(CipherError::Alphabet(String::from("whitespace and control characters are not allowed")))
        }
    };

    Ok(())
}

// Standard provisos about unicode character apply
pub fn string_pairs(text: &str) -> Vec<&str> {
    let mut idxs = text.char_indices();
    let mut out = Vec::with_capacity(text.len()/2);
    let mut start = 0;
    let last = text.len();
    idxs.next();
    loop {
        idxs.next();
        let end = match idxs.next() {
            Some(n) => n.0,
            None => {
                out.push(&text[start..last]);
                return out
            }
    };
    out.push(&text[start..end]);
    start = end;
   
    }
}


// use itertools::{sorted,equal};

// We generally need to check anagrams for alphabets which are short (less than 100 characters) and should have all unique symbols
// so more complex anagram checking is pointless
// fn is_anagram(a: &str, word: &str) -> bool {
//     equal(sorted(a.chars()),sorted(word.chars()))
// }

// // Test multiple possible words. Need this for Tableaux
// fn are_anagrams(a: &str, words: &Vec<&str>) -> bool {
//     let standard = sorted(a.chars());
//     for word in words {
//         if !equal(standard.clone(),sorted(word.chars())) {
//             return false
//         }
//     }
//     true
// }


pub fn keyed_alphabet(keyword: &str, alphabet: &str) -> Result<String,CipherError> {
    let mut keyed_alpha = String::with_capacity(alphabet.len());
    for c in keyword.chars() {
        if !alphabet.contains(c) {
            return Err(CipherError::invalid_key_char(c))
        }
        if keyed_alpha.contains(c) {
            continue
        } else {
            keyed_alpha.push(c)
        }
    }
 
    for a in alphabet.chars() {
        if keyed_alpha.contains(a) {
            continue
        } else {
            keyed_alpha.push(a)
        }
    }
    Ok(keyed_alpha)
}
 