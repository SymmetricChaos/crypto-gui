use std::fmt;
use itertools::Itertools;
use indexmap::IndexMap;

use super::preset_alphabet::PresetAlphabet;

// An Alphabet is a small collection of unique chars with an arbitrary
// ordering (ie the order of their insertion).
// IndexMap will provided the necessary functionality.
#[derive(Clone,Debug)]
pub struct Alphabet {
    set: IndexMap<char,u8>
}

impl Alphabet {

    // Get the character at some position
    pub fn get_char_at(&self, n: usize) -> Option<char> {
        self.set.keys().nth(n).map(|c| *c)
    }
    
    // Get the position of some character
    pub fn get_pos_of(&self, c: char) -> Option<usize> {
        self.set.get(&c).map(|n| *n as usize)
    }

    // pub fn get_pos_of_unchecked(&self, c: char) -> usize {
    //     self.set[&c] as usize
    // }
    
    // Get the character at some position with a (positive or negative) offset
    pub fn get_char_at_offset(&self, n: usize, offset: i32) -> Option<char> {
        let idx = ((n + self.len()) as i32 + offset) as usize % self.len();
        self.get_char_at(idx)
    }
    
    // Get the position of some character with a (positive or negative) offset
    pub fn get_pos_of_offset(&self, c: char, offset: i32) -> Option<usize> {
        let shift = (self.len() as i32 - offset) as usize % self.len();
        self.set.get(&c).map(|n| (*n as usize + shift) % self.len() )
    }

    // Get the character that is some (positive or negative) offset different from a provided char
    pub fn get_shifted_char(&self, c: char, offset: i32) -> Option<char> {
        let p = self.get_pos_of(c)?;
        self.get_char_at_offset(p, offset)
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
        let mut set = IndexMap::new();
        for (pos, c) in str.chars().unique().enumerate() {
            set.insert(c, pos as u8);
        }
        Self{ set }
    }
}

impl From<&String> for Alphabet {
    fn from(str: &String) -> Self {
        let mut set = IndexMap::new();
        for (pos, c) in str.chars().unique().enumerate() {
            set.insert(c, pos as u8);
        }
        Self{ set }
    }
}

impl From<&str> for Alphabet {
    fn from(str: &str) -> Self {
        let mut set = IndexMap::new();
        for (pos, c) in str.chars().unique().enumerate() {
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
    fn build_alphabet() {
        let alpha = Alphabet::from("ZAGX");
        assert_eq!(format!("{:?}",alpha),"Alphabet { set: {'Z': 0, 'A': 1, 'G': 2, 'X': 3} }");

    }

    #[test]
    fn char()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_char_at(1).unwrap(),'B');
    }
 
    #[test]
    fn pos() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_pos_of('C').unwrap(),2);
    }

    #[test]
    fn char_offset()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_char_at_offset(1,1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_pos_of_offset('C',1).unwrap(),1);
    }

    #[test]
    fn shifted_char() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_shifted_char('A',2).unwrap(),'C');
    }
 
    #[test]
    fn char_offset_neg()  {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_char_at_offset(3,-1).unwrap(),'C');
    }
 
    #[test]
    fn pos_offset_neg() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_pos_of_offset('C',-1).unwrap(),3);
    }

    #[test]
    fn shifted_char_neg() {
        let alphabet = Alphabet::from("ABCD");
        assert_eq!(alphabet.get_shifted_char('A',-1).unwrap(),'D');
    }
}