use rand::prelude::{SliceRandom, StdRng};

use super::preset_alphabet::PresetAlphabet;
use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecString(VecDeque<char>);

impl VecString {
    // number of characters
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // Constructor methods
    pub fn new() -> Self {
        VecString(VecDeque::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        VecString(VecDeque::with_capacity(n))
    }

    ////////////////////
    // getter methods //
    ////////////////////
    // Get the character at some position
    pub fn get_char(&self, n: usize) -> Option<char> {
        self.0.iter().nth(n).map(|c| *c)
    }

    // Get the position of some character
    pub fn get_pos(&self, c: char) -> Option<usize> {
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

    pub fn chars(&self) -> Iter<'_, char> {
        self.0.iter()
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

    pub fn pop_front(&mut self) ->  Option<char> {
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

    pub fn sort(&mut self) {
        self.0.make_contiguous().sort()
    }

    pub fn shuffle(&mut self, rng: &mut StdRng) {
        self.0.make_contiguous().shuffle(rng)
    }

    // Does nothing if either index out of bounds
    pub fn swap_indicies(&mut self, i: usize, j: usize) {
        if i < self.len() && j < self.len() {
            self.0.swap(i, j)
        }
    }

    // Does nothing is either character doesn't exist
    pub fn swap_chars(&mut self, a: char, b: char) {
        if let (Some(i), Some(j)) = (self.get_pos(a), self.get_pos(b)) {
            self.0.swap(i, j)
        }
    }
}


// Indexing Traits
impl Index<usize> for VecString {
    type Output = char;

    fn index(&self, n: usize) -> &Self::Output {
        &self.0[n]
    }
}

impl IndexMut<usize> for VecString {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        &mut self.0[n]
    }
}



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
    fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
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

impl From<PresetAlphabet> for VecString {
    fn from(alpha: PresetAlphabet) -> Self {
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
        assert_eq!(alphabet.get_char_offset(1, 1).unwrap(), 'C');
    }

    #[test]
    fn get_pos_offset() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C', 1).unwrap(), 1);
    }

    #[test]
    fn offset_char() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.offset_from_char('C', 1).unwrap(), 'D');
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
        assert_eq!(alphabet.get_char_offset(3, -1).unwrap(), 'C');
    }

    #[test]
    fn get_pos_offset_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C', -1).unwrap(), 3);
    }

    #[test]
    fn offset_char_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.offset_from_char('C', -1).unwrap(), 'B');
    }
}
