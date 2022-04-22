use std::collections::BTreeMap;
use std::fmt;
use itertools::Itertools;

use crate::preset_alphabet::PresetAlphabet;

// An Alphabet is a small ordered collection of unique chars
// We want to be able to quickly know the index of any char and the char at
// any index. A BTreeMap should work for this, with u8 keys to save space.
// This is probably a needless optimization but it will save writing lots of
// iters and lambdas.
// No way to modify an Alphabet once created is provided.
#[derive(Clone,Debug)]
pub struct Alphabet {
    set: BTreeMap<char,u8>
}

impl Alphabet {

    // Get the character at some position
    pub fn get_char(&self, n: usize) -> Option<char> {
        self.set.keys().nth(n).map(|c| *c)
    }
    
    // Get the position of some character
    pub fn get_pos(&self, c: char) -> Option<usize> {
        self.set.get(&c).map(|n| *n as usize)
    }
    
    // Get the character at some position with a (positive or negative) offset
    pub fn get_char_offset(&self, n: usize, offset: i32) -> Option<char> {
        let idx = ((n + self.len()) as i32 + offset) as usize % self.len();
        self.set.keys().nth(idx).map(|c| *c)
    }
    
    // Get the position of some character with a (positive or negative) offset
    pub fn get_pos_offset(&self, c: char, offset: i32) -> Option<usize> {
        let shift = (self.len() as i32 - offset) as usize % self.len();
        self.set.get(&c).map(|n| (*n as usize + shift) % self.len() )
    }

    // Get the character that is some (positive or negative) offset different from a provided char
    pub fn get_shifted_char(&self, c: char, offset: i32) -> Option<char> {
        let p = self.get_pos(c)?;
        self.get_char_offset(p, offset)
    }

    pub fn contains(&self, c: char) -> bool {
        self.set.contains_key(&c)
    }
    
    pub fn chars(&self) -> impl Iterator<Item=char> +'_ {
        self.set.keys().map(|c| *c)
    }
    
    pub fn len(&self) -> usize {
        self.set.len()
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.set.keys().collect::<String>())
    }
}

impl From<String> for Alphabet {
    fn from(str: String) -> Self {
        let mut set = BTreeMap::new();
        for (pos,c) in str.chars().unique().enumerate() {
            set.insert(c, pos as u8);
        }
        Self{ set }
    }
}

impl From<&String> for Alphabet {
    fn from(str: &String) -> Self {
        let mut set = BTreeMap::new();
        for (pos,c) in str.chars().unique().enumerate() {
            set.insert(c, pos as u8);
        }
        Self{ set }
    }
}

impl From<&str> for Alphabet {
    fn from(str: &str) -> Self {
        let mut set = BTreeMap::new();
        for (pos,c) in str.chars().unique().enumerate() {
            set.insert(c, pos as u8);
        }
        Self{ set }
    }
}

impl From<PresetAlphabet> for Alphabet {
    fn from(str: PresetAlphabet) -> Self {
        Alphabet::from(str.slice())
    }
}

#[cfg(test)]
mod alphabet_tests {
    use super::*;
 
    #[test]
    fn nth_offset()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_char_offset(1,1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C',1).unwrap(),1);
    }
 
    #[test]
    fn nth_offset_neg()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_char_offset(3,-1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset_neg() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_pos_offset('C',-1).unwrap(),3);
    }
}