use std::{collections::HashMap, fmt};
use itertools::Itertools;

#[derive(Clone,Debug)]
pub struct Alphabet {
    characters: Vec<char>,
    positions: HashMap<char,usize>
}

impl Alphabet {

    pub fn empty() -> Self {
        Self{ characters: Vec::new(), positions: HashMap::new() }
    }

    pub fn from(alphabet: &str) -> Self {
        let characters = alphabet.chars().unique().collect_vec();
        let mut positions = HashMap::with_capacity(characters.len());
        for (pos,c) in characters.iter().enumerate() {
            positions.insert(*c, pos);
        }
        Self{ characters, positions }
    }

    pub fn get_char(&self, n: usize) -> Option<char> {
        self.characters.get(n).map(|n| *n)
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
        write!(f, "{}", self.characters.iter().collect::<String>())
    }
}