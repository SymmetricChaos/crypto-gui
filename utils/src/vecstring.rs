use itertools::Itertools;
use rand::{prelude::SliceRandom, Rng};

use crate::text_functions::keyed_alphabet;

use super::preset_alphabet::Alphabet;
use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecString(VecDeque<char>);

impl VecString {
    // number of characters
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /////////////////////////
    // constructor methods //
    /////////////////////////
    pub fn new() -> Self {
        VecString(VecDeque::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        VecString(VecDeque::with_capacity(n))
    }

    pub fn unique_from(text: &str) -> Self {
        Self(text.chars().unique().collect())
    }

    pub fn keyed_alphabet(key: &str, alphabet: &str) -> Self {
        Self(keyed_alphabet(key, alphabet).chars().collect())
    }

    ////////////////////
    // getter methods //
    ////////////////////
    // First character
    pub fn front(&self) -> Option<&char> {
        self.0.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut char> {
        self.0.front_mut()
    }

    // Last character
    pub fn back(&self) -> Option<&char> {
        self.0.back()
    }

    pub fn back_mut(&mut self) -> Option<&mut char> {
        self.0.back_mut()
    }

    // Get the character at some position
    pub fn get_char(&self, n: usize) -> Option<&char> {
        self.0.iter().nth(n)
    }

    pub fn get_char_mut(&mut self, n: usize) -> Option<&mut char> {
        self.0.iter_mut().nth(n)
    }

    // Find a position
    pub fn get_pos(&self, c: char) -> Option<usize> {
        self.0.iter().position(|x| x == &c)
    }

    // Get one random character
    pub fn get_rand_char<R: Rng>(&self, rng: &mut R) -> char {
        *self.get_char(rng.gen_range(0..self.len())).unwrap()
    }

    // Get multiple random characters, replacing each time
    // For sampling without replacement shuffle and iterate
    pub fn get_rand_chars_replace<R: Rng>(&self, n: usize, rng: &mut R) -> Vec<char> {
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.get_rand_char(rng));
        }
        out
    }

    ////////////////////
    // offset methods //
    ///////////////////

    // Get an index offset by a positive or negative value that wraps around and the ends
    fn offset(&self, index: usize, offset: i32) -> usize {
        ((index + self.len()) as i32 + offset) as usize % self.len()
    }

    pub fn get_char_offset(&self, n: usize, offset: i32) -> Option<&char> {
        self.0.iter().nth(self.offset(n, offset))
    }

    pub fn get_char_offset_mut(&mut self, n: usize, offset: i32) -> Option<&mut char> {
        let pos = self.offset(n, offset);
        self.0.iter_mut().nth(pos)
    }

    // Get the character that is some (positive or negative) offset different from a provided char
    // pub fn get_shifted_char(&self, c: char, offset: i32) -> Option<&char> {

    // }

    ////////////////////////////////////
    // methods for finding characters //
    ////////////////////////////////////
    pub fn contains(&self, c: char) -> bool {
        self.0.contains(&c)
    }

    pub fn get_pos_offset(&self, c: char, offset: i32) -> Option<usize> {
        let shift = (self.len() as i32 - offset) as usize % self.len();
        Some((self.0.iter().position(|x| *x == c)? + shift) % self.len())
    }

    // pub fn offset_from_char(&self, c: char, offset: i32) -> Option<char> {
    //     let p = self.get_pos_of(c)?;
    //     self.get_char_offset(p, offset)
    // }

    ////////////////////////
    // conversion methods //
    ////////////////////////
    pub fn to_string(&self) -> String {
        self.0.iter().collect()
    }

    pub fn to_string_offset(&self, offset: i32) -> String {
        let shift = (self.len() as i32 + offset) as usize % self.len();
        let mut out = String::with_capacity(self.0.len());
        let s = self.to_string();
        out.push_str(&s[shift..]);
        out.push_str(&s[0..shift]);
        out
    }

    //////////////////
    // iter methods //
    //////////////////
    pub fn iter(&self) -> Iter<'_, char> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, char> {
        self.0.iter_mut()
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.0.iter().map(|c| *c)
    }

    ////////////////////////
    // reordering methods //
    ////////////////////////
    // mid is reduced modulo self.len() and does not panic
    pub fn rotate_left(&mut self, mid: usize) {
        self.0.rotate_left(mid % self.len())
    }

    // mid is reduced modulo self.len() and does not panic
    pub fn rotate_right(&mut self, mid: usize) {
        self.0.rotate_right(mid % self.len())
    }

    // does nothing if c does not exist
    pub fn rotate_to(&mut self, c: char) {
        if let Some(start) = self.get_pos(c) {
            self.0.rotate_right(start)
        }
    }

    // push and pop
    pub fn push(&mut self, val: char) {
        self.0.push_back(val)
    }

    pub fn push_front(&mut self, val: char) {
        self.0.push_front(val)
    }

    pub fn push_back(&mut self, val: char) {
        self.0.push_back(val)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.0.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<char> {
        self.0.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<char> {
        self.0.pop_back()
    }

    // insert and delete
    pub fn insert(&mut self, index: usize, val: char) {
        self.0.insert(index, val)
    }

    pub fn remove(&mut self, index: usize) -> Option<char> {
        self.0.remove(index)
    }

    // Sort the VecString
    pub fn sort(&mut self) {
        self.0.make_contiguous().sort()
    }

    // Return a sorted clone of the VecString
    pub fn sorted(&mut self) -> Self {
        let mut vs = self.clone();
        vs.sort();
        vs
    }

    // Shuffle the VecString
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.0.make_contiguous().shuffle(rng)
    }

    // Return a shuffled clone of the VecString
    pub fn shuffled<R: Rng>(&self, rng: &mut R) -> Self {
        let mut vs = self.clone();
        vs.shuffle(rng);
        vs
    }

    // Swap i and j, does nothing if either index out of bounds
    pub fn swap_indicies(&mut self, i: usize, j: usize) {
        if i < self.len() && j < self.len() {
            self.0.swap(i, j)
        }
    }

    // Swap the first instance of a with the first instance of b, does nothing if either does not exist
    pub fn swap_chars(&mut self, a: char, b: char) {
        if let (Some(i), Some(j)) = (self.get_pos(a), self.get_pos(b)) {
            self.0.swap(i, j)
        }
    }
}

// Indexing Traits
// impl Index<usize> for VecString {
//     type Output = char;

//     fn index(&self, n: usize) -> &Self::Output {
//         &self.0[n]
//     }
// }

// impl IndexMut<usize> for VecString {
//     fn index_mut(&mut self, n: usize) -> &mut Self::Output {
//         &mut self.0[n]
//     }
// }

// Display
impl Display for VecString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Lots of From<_> Traits
impl From<VecString> for String {
    fn from(vstr: VecString) -> Self {
        vstr.to_string()
    }
}

impl FromIterator<char> for VecString {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut vdq = VecDeque::new();
        for c in iter {
            vdq.push_back(c)
        }
        VecString(vdq)
    }
}

impl From<String> for VecString {
    fn from(str: String) -> Self {
        VecString(str.chars().collect::<VecDeque<char>>())
    }
}

impl From<&String> for VecString {
    fn from(str: &String) -> Self {
        VecString(str.chars().collect::<VecDeque<char>>())
    }
}

impl From<&str> for VecString {
    fn from(str: &str) -> Self {
        VecString(str.chars().collect::<VecDeque<char>>())
    }
}

impl From<Alphabet> for VecString {
    fn from(alpha: Alphabet) -> Self {
        VecString::from(alpha.slice())
    }
}

#[cfg(test)]
mod vecstring_tests {
    use super::*;

    #[test]
    fn show_offset() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.to_string_offset(1), "BCDA");
    }

    #[test]
    fn nth_offset() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(*alphabet.get_char_offset(1, 1).unwrap(), 'C');
    }

    #[test]
    fn get_pos_offset() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C', 1).unwrap(), 1);
    }

    // Offset should behave as expected even if it is negative
    #[test]
    fn show_offset_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.to_string_offset(-1), "DABC");
    }

    #[test]
    fn nth_offset_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(*alphabet.get_char_offset(3, -1).unwrap(), 'C');
    }

    #[test]
    fn get_pos_offset_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C', -1).unwrap(), 3);
    }
}
