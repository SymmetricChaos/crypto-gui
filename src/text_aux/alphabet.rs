// use itertools::Itertools;
// use rand::prelude::StdRng;
// use std::fmt;

// use super::{preset_alphabet::PresetAlphabet, VecString, keyed_alphabet};

// // An VecString is a small collection of unique chars with an arbitrary ordering
// // All methods matain the uniqueness invariant
// #[derive(Clone, Debug)]
// pub struct VecString(VecString);

// impl VecString {
//     // Get the character at some position
//     pub fn get_char_at(&self, n: usize) -> Option<char> {
//         self.0.get_char(n)
//     }

//     // Get the position of some character
//     pub fn get_pos_of(&self, c: char) -> Option<usize> {
//         self.0.get_pos(c)
//     }

//     // Get the character at some position with a (positive or negative) offset
//     pub fn get_char_offset(&self, index: usize, offset: i32) -> Option<char> {
//         self.0.get_char_offset(index, offset)
//     }

//     pub fn get_pos_offset(&self, c: char, offset: i32) -> Option<usize> {
//         self.0.get_pos_offset(c, offset)
//     }

//     // Get the position of some character with a (positive or negative) offset
//     // pub fn get_pos_of_offset(&self, c: char, offset: i32) -> Option<usize> {
//     //     let shift = (self.len() as i32 - offset) as usize % self.len();
//     //     self.0.get(&c).map(|n| (*n as usize + shift) % self.len())
//     // }

//     // Get the character that is some (positive or negative) offset different from a provided char
//     pub fn get_shifted_char(&self, c: char, offset: i32) -> Option<char> {
//         let p = self.get_pos_of(c)?;
//         self.get_char_offset(p, offset)
//     }

//     pub fn get_rand_char(&self, rng: &mut StdRng) -> char {
//         self.0.get_rand_char(rng)
//     }

//     pub fn get_rand_chars_replace(&self, n: usize, rng: &mut StdRng) -> Vec<char> {
//         self.0.get_rand_chars_replace(n, rng)
//     }



//     // Builder Methods
//     pub fn new() -> Self {
//         VecString(VecString::new())
//     }

//     pub fn from_key(key: &str, alphabet: &str) -> Self {
//         Self(keyed_alphabet(key, alphabet).chars().collect())
//     }



//     // Mutating methods
//     // All of these reduce the input or do nothing on an invalid input
//     pub fn rotate_left(&mut self, mid: usize) {
//         self.0.rotate_left(mid % self.len())
//     }

//     pub fn rotate_right(&mut self, mid: usize) {
//         self.0.rotate_right(mid % self.len())
//     }

//     pub fn rotate_to(&mut self, c: char) {
//         if let Some(start) = self.get_pos_of(c) {
//             self.0.rotate_right(start)
//         }
//     }

//     // Other methods
//     pub fn contains(&self, c: char) -> bool {
//         self.0.contains(c)
//     }

//     pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
//         self.0.iter().map(|c| *c)
//     }

//     pub fn len(&self) -> usize {
//         self.0.len()
//     }
// }



// impl fmt::Display for VecString {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0.to_string())
//     }
// }

// impl From<VecString> for VecString {
//     fn from(str: VecString) -> Self {
//         Self(str.iter().map(|c| *c).unique().collect())
//     }
// }

// impl From<String> for VecString {
//     fn from(str: String) -> Self {
//         Self(str.chars().unique().collect())
//     }
// }

// impl From<&String> for VecString {
//     fn from(str: &String) -> Self {
//         Self(str.chars().unique().collect())
//     }
// }

// impl From<&str> for VecString {
//     fn from(str: &str) -> Self {
//         Self(str.chars().unique().collect())
//     }
// }

// impl From<PresetAlphabet> for VecString {
//     fn from(str: PresetAlphabet) -> Self {
//         VecString::from(str.slice())
//     }
// }

// #[cfg(test)]
// mod alphabet_tests {
//     use super::*;

//     #[test]
//     fn char() {
//         let alphabet = VecString::from("ABCD");
//         assert_eq!(alphabet.get_char_at(1).unwrap(), 'B');
//     }

//     #[test]
//     fn pos() {
//         let alphabet = VecString::from("ABCD");
//         assert_eq!(alphabet.get_pos_of('C').unwrap(), 2);
//     }

//     // #[test]
//     // fn char_offset() {
//     //     let alphabet = VecString::from("ABCD");
//     //     assert_eq!(alphabet.get_char_at_offset(1, 1).unwrap(), 'C');
//     // }

//     // #[test]
//     // fn pos_offset() {
//     //     let alphabet = VecString::from("ABCD");
//     //     assert_eq!(alphabet.get_pos_of_offset('C', 1).unwrap(), 1);
//     // }

//     // #[test]
//     // fn shifted_char() {
//     //     let alphabet = VecString::from("ABCD");
//     //     assert_eq!(alphabet.get_shifted_char('A', 2).unwrap(), 'C');
//     // }

//     // #[test]
//     // fn char_offset_neg() {
//     //     let alphabet = VecString::from("ABCD");
//     //     assert_eq!(alphabet.get_char_at_offset(3, -1).unwrap(), 'C');
//     // }

//     // #[test]
//     // fn pos_offset_neg() {
//     //     let alphabet = VecString::from("ABCD");
//     //     assert_eq!(alphabet.get_pos_of_offset('C', -1).unwrap(), 3);
//     // }

//     // #[test]
//     // fn shifted_char_neg() {
//     //     let alphabet = VecString::from("ABCD");
//     //     assert_eq!(alphabet.get_shifted_char('A', -1).unwrap(), 'D');
//     // }
// }
