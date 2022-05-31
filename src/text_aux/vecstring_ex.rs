use itertools::Itertools;
use rand::Rng;
use rand::prelude::{SliceRandom, StdRng};

use super::keyed_alphabet;
use super::preset_alphabet::PresetAlphabet;
use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecStringExtended<'a>(VecDeque<&'a str>);

impl VecStringExtended<'_> {
    // number of grapheme clusters
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /////////////////////////
    // constructor methods //
    /////////////////////////
    pub fn new() -> Self {
        VecStringExtended(VecDeque::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        VecStringExtended(VecDeque::with_capacity(n))
    }

    pub fn keyed_alphabet(key: &str, alphabet: &str) -> Self {
        Self(keyed_alphabet(key, alphabet).chars().collect())
    }

    pub fn unique_from(text: &str) -> Self {
        Self(text.chars().unique().collect())
    }

    ////////////////////
    // getter methods //
    ////////////////////
    // Get the character at some position
    pub fn get_char(&self, n: usize) -> Option<char> {
        self.0.iter().nth(n).map(|c| *c)
    }

    pub fn get_char_at(&self, n: usize) -> Option<char> {
        self.0.iter().nth(n).map(|c| *c)
    }

    // Get the position of some character
    pub fn get_pos(&self, c: char) -> Option<usize> {
        self.0.iter().position(|x| x == &c)
    }

    pub fn get_pos_of(&self, c: char) -> Option<usize> {
        self.0.iter().position(|x| x == &c)
    }

    // Get a mutable reference to the character as some position
    pub fn get_char_mut(&mut self, n: usize) -> Option<&mut char> {
        self.0.iter_mut().nth(n)
    }

    // Get a char at a position with some offset
    pub fn get_char_offset(&self, index: usize, offset: i32) -> Option<char> {
        let len = self.len();
        let idx = ((index + len) as i32 + offset) as usize % len;
        self.0.get(idx).map(|c| *c)
    }

    // Get a mutable reference to the char at a position with some offset
    pub fn get_char_offset_mut(&mut self, index: usize, offset: i32) -> Option<&mut char> {
        let len = self.len();
        let idx = ((index + len) as i32 + offset) as usize % len;
        self.0.get_mut(idx)
    }

    // Get the character that is some (positive or negative) offset different from a provided char
    pub fn get_shifted_char(&self, c: char, offset: i32) -> Option<char> {
        let p = self.get_pos_of(c)?;
        self.get_char_offset(p, offset)
    }

    // Get one random character
    pub fn get_rand_char(&self, rng: &mut StdRng) -> char {
        self.get_char(rng.gen_range(0..self.len())).unwrap()
    }

    // Get multiple random characters, replacing each time
    // For sampling without replacement shuffle and iterate
    pub fn get_rand_chars_replace(&self, n: usize, rng: &mut StdRng) -> Vec<char> {
        let mut out = Vec::with_capacity(n);
        for i in out.iter_mut() {
            *i = self.get_rand_char(rng);
        }
        out
    }

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

    pub fn offset_from_char(&self, c: char, offset: i32) -> Option<char> {
        let p = self.get_pos(c)?;
        self.get_char_offset(p, offset)
    }

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

    // Sort the VecStringExtended
    pub fn sort(&mut self) {
        self.0.make_contiguous().sort()
    }

    // Return a sorted clone of the VecStringExtended
    pub fn sorted(&mut self) -> Self {
        let mut vs = self.clone();
        vs.sort();
        vs
    }

    // Shuffle the VecStringExtended
    pub fn shuffle(&mut self, rng: &mut StdRng) {
        self.0.make_contiguous().shuffle(rng)
    }

    // Return a shuffled clone of the VecStringExtended
    pub fn shuffled(&mut self, rng: &mut StdRng) -> Self {
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
impl Index<usize> for VecStringExtended {
    type Output = char;

    fn index(&self, n: usize) -> &Self::Output {
        &self.0[n]
    }
}

impl IndexMut<usize> for VecStringExtended {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        &mut self.0[n]
    }
}

// Display
impl Display for VecStringExtended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Lots of From<_> Traits
impl From<VecStringExtended> for String {
    fn from(vstr: VecStringExtended) -> Self {
        vstr.to_string()
    }
}

impl FromIterator<char> for VecStringExtended {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut vdq = VecDeque::new();
        for c in iter {
            vdq.push_back(c)
        }
        VecStringExtended(vdq)
    }
}

impl From<String> for VecStringExtended {
    fn from(str: String) -> Self {
        VecStringExtended(str.chars().collect::<VecDeque<char>>())
    }
}

impl From<&String> for VecStringExtended {
    fn from(str: &String) -> Self {
        VecStringExtended(str.chars().collect::<VecDeque<char>>())
    }
}

impl From<&str> for VecStringExtended {
    fn from(str: &str) -> Self {
        VecStringExtended(str.chars().collect::<VecDeque<char>>())
    }
}

impl From<PresetAlphabet> for VecStringExtended {
    fn from(alpha: PresetAlphabet) -> Self {
        VecStringExtended::from(alpha.slice())
    }
}

#[cfg(test)]
mod VecStringExtended_tests {
    use super::*;

    #[test]
    fn show_offset() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.to_string_offset(1), "BCDA");
    }

    #[test]
    fn nth_offset() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.get_char_offset(1, 1).unwrap(), 'C');
    }

    #[test]
    fn get_pos_offset() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C', 1).unwrap(), 1);
    }

    #[test]
    fn offset_char() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.offset_from_char('C', 1).unwrap(), 'D');
    }

    // Offset should behave as expected even if it is negative
    #[test]
    fn show_offset_neg() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.to_string_offset(-1), "DABC");
    }

    #[test]
    fn nth_offset_neg() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.get_char_offset(3, -1).unwrap(), 'C');
    }

    #[test]
    fn get_pos_offset_neg() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C', -1).unwrap(), 3);
    }

    #[test]
    fn offset_char_neg() {
        let alphabet = VecStringExtended::from("ABCD");
        assert_eq!(alphabet.offset_from_char('C', -1).unwrap(), 'B');
    }
}
