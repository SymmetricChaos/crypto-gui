use bimap::BiMap;
use itertools::Itertools;
use num::Integer;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use std::hash::Hash;
use strsim::damerau_levenshtein;

// Mutate a string so that it contains only characters in a provided alphabet
pub fn filter_string<S: AsRef<str>>(string: &mut String, alphabet: &S) {
    *string = string
        .chars()
        .filter(|c| alphabet.as_ref().contains(*c))
        .collect();
}

// Mutate a string so that it contains only unique characters
pub fn unique_string(string: &mut String) {
    *string = string.chars().unique().collect();
}

// Mutate a string so that it contains only characters in a provided alphabet and only unique characters
pub fn filter_unique_string<S: AsRef<str>>(string: &mut String, alphabet: &S) {
    *string = string
        .chars()
        .filter(|c| alphabet.as_ref().contains(*c))
        .unique()
        .collect();
}

pub fn bimap_from_iter<I, S, T>(iter: I) -> BiMap<S, T>
where
    I: Iterator<Item = (S, T)>,
    S: Hash + Eq,
    T: Hash + Eq,
{
    let mut map = BiMap::new();
    for (l, r) in iter {
        map.insert(l, r);
    }
    map
}

pub fn chunk_and_join(text: &str, width: usize, sep: char) -> String {
    text.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % width == 0 {
                Some(sep)
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

pub fn string_chunks(text: &str, width: usize) -> Vec<String> {
    text.chars()
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect_vec()
}

pub fn shuffled_str<R: Rng>(s: &str, rng: &mut R) -> String {
    let mut characters = s.chars().collect::<Vec<char>>();
    let slice = characters.as_mut_slice();
    slice.shuffle(rng);
    slice.iter().map(|x| *x).collect::<String>()
}

pub fn random_string_sample<R: Rng>(s: &str, n: usize, rng: &mut R) -> String {
    s.chars().choose_multiple(rng, n).iter().collect()
}

pub fn random_string_sample_replace<R: Rng>(s: &str, n: usize, rng: &mut R) -> String {
    let mut out = String::with_capacity(n);
    for _ in 0..n {
        out.push(s.chars().choose(rng).unwrap())
    }
    out
}

pub fn swap_ab(a: char, b: char, text: &str) -> String {
    text.chars()
        .map(|c| {
            if c == a {
                b
            } else if c == b {
                a
            } else {
                c
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumRep {
    Binary,
    Octal,
    Decimal,
    HexLower,
    HexUpper,
}

impl NumRep {
    pub fn radix(&self) -> u32 {
        match self {
            NumRep::Binary => 2,
            NumRep::Octal => 8,
            NumRep::Decimal => 10,
            NumRep::HexLower => 16,
            NumRep::HexUpper => 16,
        }
    }
}

pub fn u8_to_string(n: u8, rep: NumRep) -> String {
    match rep {
        NumRep::Binary => format!("{n:08b}"),
        NumRep::Octal => format!("{n:03o}"),
        NumRep::Decimal => format!("{n}"),
        NumRep::HexLower => format!("{n:02x}"),
        NumRep::HexUpper => format!("{n:02X}"),
    }
}

pub fn u16_to_string(n: u16, rep: NumRep) -> String {
    match rep {
        NumRep::Binary => format!("{n:016b}"),
        NumRep::Octal => format!("{n:06o}"),
        NumRep::Decimal => format!("{n}"),
        NumRep::HexLower => format!("{n:04x}"),
        NumRep::HexUpper => format!("{n:04X}"),
    }
}

pub fn u32_to_string(n: u32, rep: NumRep) -> String {
    match rep {
        NumRep::Binary => format!("{n:032b}"),
        NumRep::Octal => format!("{n:011o}"),
        NumRep::Decimal => format!("{n}"),
        NumRep::HexLower => format!("{n:08x}"),
        NumRep::HexUpper => format!("{n:08X}"),
    }
}

pub fn u8_to_string_with_radix(byte: &u8, radix: u8) -> String {
    if byte == &0 {
        return String::from("0");
    }
    let mut b = *byte;
    let mut s = Vec::new();
    while b != 0 {
        let (q, r) = b.div_rem(&radix);
        s.push(num_to_digit(r as u32).expect("remainder should always be less than 36"));
        b = q;
    }
    s.into_iter().rev().collect()
}

pub fn u8_to_string_with_radix_and_width(byte: &u8, radix: u8, width: usize) -> String {
    assert!(radix > 1);
    assert!(radix < 37);
    if byte == &0 {
        return "0".repeat(width);
    }
    let mut b = *byte;
    let mut s = Vec::with_capacity(8); // Largest size needed for a valid radix
    while b != 0 {
        let (q, r) = b.div_rem(&radix);
        if r < 10 {
            s.push(r + 48) // shift to start of ASCII numbers
        } else {
            s.push(r + 55) // shift to start of ASCII uppercase letters
        }
        b = q;
    }
    let zeroes = std::iter::repeat('0' as u8).take(width - s.len());
    String::from_utf8(zeroes.chain(s.into_iter().rev()).collect()).unwrap()
}

pub fn random_char_vec<R: Rng>(s: &str, n: usize, rng: &mut R) -> Vec<char> {
    s.chars().choose_multiple(rng, n)
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

pub fn num_to_digit(n: u32) -> Option<char> {
    if n > 36 {
        None
    } else if n < 10 {
        char::from_u32(n + 48)
    } else {
        char::from_u32(n + 55)
    }
}

/*
Rank the characters of a string by their order in the alphabet, making every entry unique and using the smallest possible numbers
The text APPLE with the BasicLatin alphabet would give: [0, 3, 4, 2, 1, 5]
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringRankError;

pub fn rank_str(text: &str, alphabet: &str) -> Result<Vec<usize>, StringRankError> {
    if text.is_empty() {
        return Ok(Vec::new());
    }

    let mut values = Vec::new();
    for c in text.chars() {
        values.push(
            alphabet
                .chars()
                .position(|x| x == c)
                .ok_or(StringRankError)?,
        );
    }

    let len = values.len();
    let biggest = alphabet.chars().count();

    let mut out = vec![0usize; len];

    for i in 0..len {
        let m = values.iter().min().unwrap(); // justified by test for empty string
        for (pos, v) in values.iter().enumerate() {
            if v == m {
                out[pos] = i;
                values[pos] = biggest;
                break;
            }
        }
    }

    Ok(out)
}

// This ignores repeated numbers
pub fn rank_vec<O: Ord>(vec: &Vec<O>) -> Vec<usize> {
    let mut out = Vec::with_capacity(vec.len());
    let ranks = vec.iter().sorted().dedup().collect_vec();

    for o in ranks.iter() {
        out.push(vec.iter().position(|x| &x == o).unwrap())
    }

    out
}

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

// Takes a word and a corpus. Return a tuple (a,b) where a is the index of the (leftmost) best match in the corpus and the b is the damerau-levenshtein distance between the word and its best match
pub fn closest_match<T: AsRef<str>>(word: T, corpus: &[T]) -> (usize, usize) {
    let mut idx = 0_usize;
    let mut best_distance = usize::MAX;
    for (i, candidate) in corpus.into_iter().enumerate() {
        let candidate_distance = damerau_levenshtein(word.as_ref(), candidate.as_ref());
        // Short circuit on exact mathc
        if candidate_distance == 0 {
            return (i, candidate_distance);
        }
        if candidate_distance < best_distance {
            best_distance = candidate_distance;
            idx = i
        }
    }
    (idx, best_distance)
}

// pub fn dedup_alphabet(s: &str) -> String {
//     let mut seen: Vec<char> = Vec::with_capacity(s.len());
//     let mut out = String::with_capacity(s.len());
//     for c in s.chars() {
//         if seen.contains(&c) {
//             continue;
//         } else {
//             out.push(c);
//             seen.push(c)
//         }
//     }
//     out
// }

#[cfg(test)]
mod text_function_tests {

    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn string_ranking() {
        let text = "APPLES";
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        assert_eq!(vec![0, 3, 4, 2, 1, 5], rank_str(text, alphabet).unwrap());
    }

    #[test]
    fn num_ranking() {
        let text = vec![5, 0, 1, 3, 2, 4];
        assert_eq!(vec![1, 2, 4, 3, 5, 0], rank_vec(&text));
    }

    #[test]
    fn shuffled_alphabet() {
        let mut rng = StdRng::seed_from_u64(3141592654);
        let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for _ in 0..26 {
            println!("{}", shuffled_str(alpha, &mut rng))
        }
    }

    #[test]
    fn joined_chunks() {
        let s = "1234567890";
        assert_eq!("123 456 789 0", chunk_and_join(s, 3, ' '))
    }

    #[test]
    fn vec_chunks() {
        let s = "1234567890";
        let chunks = string_chunks(s, 3);
        assert_eq!(
            "[\"123\", \"456\", \"789\", \"0\"]",
            format!("{:?}", chunks)
        )
    }
}
