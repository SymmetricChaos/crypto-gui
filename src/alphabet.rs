use std::{collections::BTreeMap, fmt};
use itertools::Itertools;

#[derive(Clone,Debug)]
pub struct Alphabet {
    positions: BTreeMap<char,usize>
}

impl Alphabet {

    pub fn empty() -> Self {
        Self{ positions: BTreeMap::new() }
    }

    pub fn from(alphabet: &str) -> Self {
        let characters = alphabet.chars().unique();
        let mut positions = BTreeMap::new();
        for (pos,c) in characters.enumerate() {
            positions.insert(c, pos);
        }
        Self{ positions }
    }

    pub fn get_char(&self, n: usize) -> Option<char> {
        self.positions.keys().nth(n).map(|c| *c)
    }

    pub fn get_pos(&self, c: char) -> Option<usize> {
        self.positions.get(&c).map(|n| *n)
    }

    pub fn contains(&self, c: char) -> bool {
        self.positions.contains_key(&c)
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.positions.keys().collect::<String>())
    }
}