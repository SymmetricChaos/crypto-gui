use rand::prelude::{StdRng, SliceRandom};

use super::preset_alphabet::PresetAlphabet;
use std::collections::VecDeque;
use std::collections::vec_deque::{Iter,IterMut};
use std::fmt::Display;
use std::ops::{Index,IndexMut};
 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecString(VecDeque<char>);
 
// A Python-like string type for when we need need to constantly manipulate it
// The *_offset methods allow the equivalent of shifting without actually
// changing anything
impl VecString {
 
    // number of characters
    pub fn len(&self) -> usize {
        self.0.len()
    }
 
    ////////////////////
    // getter methods //
    ////////////////////
    pub fn get(&self, index: usize) -> Option<&char> {
        self.0.get(index)
    }
 
    pub fn get_mut(&mut self, index: usize) -> Option<&mut char> {
        self.0.get_mut(index)
    }
 
    pub fn get_offset(&self, index: usize, offset: i32) -> Option<&char> {
        let len = self.len();
        let idx = ((index + len) as i32 + offset) as usize % len;
        self.0.get(idx)
    }
 
    pub fn get_mut_offset(&mut self, index: usize, offset: i32) -> Option<&mut char> {
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
 
    pub fn pos(&self, c: char) -> Option<usize> {
        Some(self.0.iter().position(|x| *x == c)?)
    }
 
    pub fn pos_offset(&self, c: char, offset: i32) -> Option<usize> {
        let shift = (self.len() as i32 - offset) as usize % self.len();
        Some((self.0.iter().position(|x| *x == c)? + shift) % self.len())
    }
 
    pub fn char_offset(&self, c: char, offset: i32) -> Option<&char> {
        let p = self.pos(c)?;
        self.get_offset(p, offset)
    }
 
    //////////////////
    // iter methods //
    //////////////////
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
 
    ////////////////////////
    // reordering methods //
    ////////////////////////
    pub fn rotate_left(&mut self, mid: usize) {
        self.0.rotate_left(mid)
    }
 
    pub fn rotate_right(&mut self, mid: usize) {
        self.0.rotate_right(mid)
    }
 
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
}
 
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

impl Display for VecString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.to_string())
    }
}




 
impl From<VecString> for String {
    fn from(vstr: VecString) -> Self {
        vstr.to_string()
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
        assert_eq!(alphabet.to_string_offset(1),"BCDA");
    }
 
    #[test]
    fn nth_offset()  {
        let alphabet = VecString::from("ABCD");
        assert_eq!(*alphabet.get_offset(1,1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.pos_offset('C',1).unwrap(),1);
    }

    #[test]
    fn offset_char() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(*alphabet.char_offset('C',1).unwrap(),'D');
    }
 
	// Offset should behave as expected even if it is negative
    #[test]
    fn show_offset_neg() {
        let alphabet = VecString::from("ABCD");
            assert_eq!(alphabet.to_string_offset(-1),"DABC");
    }
 
    #[test]
    fn nth_offset_neg()  {
        let alphabet = VecString::from("ABCD");
        assert_eq!(*alphabet.get_offset(3,-1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(alphabet.pos_offset('C',-1).unwrap(),3);
    }

    #[test]
    fn offset_char_neg() {
        let alphabet = VecString::from("ABCD");
        assert_eq!(*alphabet.char_offset('C',-1).unwrap(),'B');
    }
 
}