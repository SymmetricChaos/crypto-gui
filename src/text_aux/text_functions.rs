use crate::errors::Error;
use itertools::Itertools;
use rand::prelude::{IteratorRandom, SliceRandom, StdRng};

use super::VecString;

pub fn shuffled_str(s: &str, rng: &mut StdRng) -> String {
    let mut characters = s.chars().collect::<Vec<char>>();
    let slice = characters.as_mut_slice();
    slice.shuffle(rng);
    slice.iter().map(|x| *x).collect::<String>()
}

pub fn random_sample_replace(s: &str, n: usize, rng: &mut StdRng) -> String {
    let mut out = String::with_capacity(n);
    for _ in 0..n {
        out.push(s.chars().choose(rng).unwrap())
    }
    out
}

pub fn random_char_vec(s: &str, n: usize, rng: &mut StdRng) -> Vec<char> {
    s.chars().choose_multiple(rng, n)
}

pub fn validate_alphabet(alphabet: &str) -> Result<(), Error> {
    // Most basic check, symbols in an alphabet must be unique
    if alphabet.chars().count() != alphabet.chars().unique().count() {
        return Err(Error::Alphabet(String::from(
            "characters must all be unique",
        )));
    }

    // Eliminate potentiually confusing characters
    for symbol in alphabet.chars() {
        if symbol.is_control() || symbol.is_whitespace() {
            return Err(Error::Alphabet(String::from(
                "whitespace and control characters are not allowed",
            )));
        }
    }

    Ok(())
}

// Standard provisos about unicode character apply
pub fn string_pairs(text: &str) -> Vec<&str> {
    let mut idxs = text.char_indices();
    let mut out = Vec::with_capacity(text.len() / 2);
    let mut start = 0;
    let last = text.len();
    idxs.next();
    loop {
        idxs.next();
        let end = match idxs.next() {
            Some(n) => n.0,
            None => {
                out.push(&text[start..last]);
                return out;
            }
        };
        out.push(&text[start..end]);
        start = end;
    }
}

/*
Rank the characters of a string by their order in the alphabet, making every entry unique and using the smallest possible numbers
The text APPLE with the BasicLatin alphabet would give: [0, 3, 4, 2, 1, 5]
*/
pub fn rank_str(text: &str, alphabet: &str) -> Vec<usize> {
    let mut values = text
        .chars()
        .map(|x| alphabet.chars().position(|c| x == c).unwrap())
        .collect::<Vec<usize>>();

    let len = values.len();
    let biggest = alphabet.chars().count();

    let mut out = vec![0usize; len];

    for i in 0..len {
        let m = values.iter().min().unwrap();
        for (pos, v) in values.iter().enumerate() {
            if v == m {
                out[pos] = i;
                values[pos] = biggest;
                break;
            }
        }
    }

    out
}

// This ignores repeated numbers
pub fn rank_vec<O: Ord>(vec: &Vec<O>) -> Vec<usize> {
    let mut out = Vec::with_capacity(vec.len());
    let ranks = vec.clone().iter().sorted().dedup().collect_vec();

    for o in ranks.iter() {
        out.push(vec.iter().position(|x| &x == o).unwrap())
    }

    out
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

// Silently ignores invalid characters
pub fn keyed_alphabet(keyword: &str, alphabet: &str) -> String {
    let mut keyed_alpha = String::with_capacity(alphabet.len());
    for c in keyword.chars() {
        if !alphabet.contains(c) {
            continue;
        }
        if keyed_alpha.contains(c) {
            continue;
        } else {
            keyed_alpha.push(c)
        }
    }

    for a in alphabet.chars() {
        if keyed_alpha.contains(a) {
            continue;
        } else {
            keyed_alpha.push(a)
        }
    }
    keyed_alpha
}

pub fn dedup_alphabet(s: &str) -> String {
    let mut seen: Vec<char> = Vec::with_capacity(s.len());
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if seen.contains(&c) {
            continue;
        } else {
            out.push(c);
            seen.push(c)
        }
    }
    out
}

pub fn prep_text(text: &str, alphabet: &str) -> Result<String, Error> {
    let mut out = String::with_capacity(text.len());
    for t in text.chars() {
        if alphabet.contains(t) {
            out.push(t)
        } else if t.is_whitespace() || t.is_ascii_punctuation() {
            // ignore any Unicode whitespace and
            // any ASCII punctuation
        } else if alphabet.contains(t.to_ascii_lowercase()) {
            // try converting the character to lowercase
            // this only works with ASCII at the moment
            // because unicode can change the number of
            // characters between upper and lower case
            out.push(t.to_ascii_lowercase())
        } else if alphabet.contains(t.to_ascii_uppercase()) {
            // As above
            out.push(t.to_ascii_uppercase())
        } else {
            return Err(Error::invalid_input_char(t));
        }
    }
    Ok(out)
}

pub fn validate_text(text: &str, alphabet: &VecString) -> Result<(), Error> {
    if text.len() == 0 {
        return Err(Error::Input(String::from("No input text provided")));
    }
    for c in text.chars() {
        if !alphabet.contains(c) {
            return Err(Error::invalid_input_char(c));
        }
    }
    Ok(())
}

#[cfg(test)]
mod text_function_tests {

    use crate::global_rng::{get_global_rng, seed_global_rng};

    use super::*;

    #[test]
    fn string_ranking() {
        let text = "APPLES";
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        assert_eq!(vec![0, 3, 4, 2, 1, 5], rank_str(text, alphabet));
    }

    #[test]
    fn num_ranking() {
        let text = vec![5, 0, 1, 3, 2, 4];
        assert_eq!(vec![1, 2, 4, 3, 5, 0], rank_vec(&text));
    }

    #[test]
    fn shuffled_alphabet() {
        seed_global_rng(3141592654);
        let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for _ in 0..26 {
            println!("{}", shuffled_str(alpha, &mut get_global_rng()))
        }
    }
}
