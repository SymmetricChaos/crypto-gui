use itertools::Itertools;
use rand::prelude::{ThreadRng, SliceRandom, IteratorRandom};

use crate::errors::CipherError;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetAlphabet {
    BasicLatin,
    BasicLatinNoJ,
    BasicLatinNoQ,
    BasicLatinWithDigits,
    Digits0,
    Digits1,
    Ascii94, // The printing ASCII symbols without the space
    Ascii95, // The printing ASCII symbols with the space
    Greek,
    Latin, //Classical Latin
    Base64, // 64 safe to use ASCII symbols, low chance of being interpreted if the string is parsed
    Spanish,
    German,
}

impl PresetAlphabet {

    // Pointer to a static string slice
    pub fn slice(&self) -> &'static str {
        match self {
            PresetAlphabet::BasicLatin => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            PresetAlphabet::BasicLatinNoJ => "ABCDEFGHIKLMNOPQRSTUVWXYZ",
            PresetAlphabet::BasicLatinNoQ => "ABCDEFGHIJKLMNOPRSTUVWXYZ",
            PresetAlphabet::BasicLatinWithDigits => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            PresetAlphabet::Digits0 => "0123456789",
            PresetAlphabet::Digits1 => "1234567890",
            PresetAlphabet::Ascii94 => "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            PresetAlphabet::Ascii95 => " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            PresetAlphabet::Greek => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ", //All of these are Unicode Greek even the ones draw identically to ASCII
            PresetAlphabet::Latin => "ABCDEFGHIKLMNOPQRSTVXY",
            PresetAlphabet::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            PresetAlphabet::Spanish => "ABCDEFGHIJKLMNÑOPQRSTUVWXYZ",
            PresetAlphabet::German => "ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜß",
        }
    }

    // Owned string
    pub fn string(&self) -> String {
        self.slice().to_string()
    }

    // Length in Unicode characters
    pub fn len(&self) -> usize {
        // This could be a match statement but this is easier
        self.slice().chars().count()
    }

    // Iterate over characters
    pub fn chars(&self) -> std::str::Chars {
        self.slice().chars()
    }
}

impl From<PresetAlphabet> for String {
    fn from(alphabet: PresetAlphabet) -> Self {
        alphabet.string()
    }
}

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

/*
Rank the characters of a string by their order in the alphabet, making every entry unique and using the smallest possible numbers
The text APPLE with the BasicLatin alphabet would give: [0, 3, 4, 2, 1, 5]
*/
pub fn rank_str(text: &str, alphabet: &str) -> Vec<usize> {
    let mut values = text.chars().map(|x| alphabet.chars().position(|c| x == c).unwrap()).collect::<Vec<usize>>();

    let len = values.len();
    let biggest = alphabet.chars().count();

    let mut out = vec![0usize;len];

    for i in 0..len {
        let m = values.iter().min().unwrap();
        for (pos,v) in values.iter().enumerate() {
            if v == m {
                out[pos] = i;
                values[pos] = biggest;
                break
            }
        }
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
            continue
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
    keyed_alpha
}


pub fn dedup_alphabet(s: &str) -> String {
    let mut seen: Vec<char> = Vec::with_capacity(s.len());
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if seen.contains(&c) {
            continue
        } else {
            out.push(c);
            seen.push(c)
        }
    }
    out
}



#[cfg(test)]
mod affine_tests {
    use super::*;

    #[test]
    fn string_ranking() {
        let text = "APPLES";
        let alphabet = PresetAlphabet::BasicLatin.slice();
        assert_eq!(vec![0, 3, 4, 2, 1, 5],rank_str(text, alphabet));
    }

}
