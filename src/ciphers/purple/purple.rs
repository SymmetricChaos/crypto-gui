use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ciphers::Cipher, codes::romaji::romaji::to_romaji, errors::Error, text_aux::VecString,
};
use lazy_static::lazy_static;

use super::switch::Switch;

use crate::codes::romaji::NIHON_SHIKI;

#[derive(Clone)]
pub struct Switches {
    pub sixes: Switch<6>,
    pub twenties: [Rc<RefCell<Switch<20>>>; 3],
    pub slow: Rc<RefCell<Switch<20>>>,
    pub middle: Rc<RefCell<Switch<20>>>,
    pub fast: Rc<RefCell<Switch<20>>>,
}

impl Default for Switches {
    fn default() -> Self {
        let twenties = Switch::twenties();
        let slow = twenties[0].clone();
        let middle = twenties[2].clone();
        let fast = twenties[1].clone();
        Self {
            sixes: Switch::sixes(),
            twenties,
            slow,
            middle,
            fast,
        }
    }
}

impl Switches {
    pub fn validate_switches(&self) -> Result<(), Error> {
        for switch in self.twenties.iter() {
            if Rc::strong_count(&switch) != 2 {
                return Err(Error::key(
                    "each Twenties switch must have a different speed",
                ));
            }
        }
        Ok(())
    }

    pub fn set_slow(&mut self, switch: Rc<RefCell<Switch<20>>>) {
        self.slow = switch
    }

    pub fn set_middle(&mut self, switch: Rc<RefCell<Switch<20>>>) {
        self.middle = switch
    }

    pub fn set_fast(&mut self, switch: Rc<RefCell<Switch<20>>>) {
        self.fast = switch
    }

    pub fn step(&mut self) {
        let spos = self.sixes.position;
        let mpos = self.middle.borrow().position;

        // Sixes always steps
        self.sixes.step();

        // Exactly one of the Twenties steps at a time
        if spos == 23 && mpos == 24 {
            self.slow.borrow_mut().step();
        } else if spos == 24 {
            self.middle.borrow_mut().step();
        } else {
            self.fast.borrow_mut().step();
        }
    }

    pub fn encrypt_num(&self, n: usize) -> usize {
        if n < 6 {
            self.sixes.encrypt(n)
        } else {
            let n = self.twenties[0].borrow().encrypt(n - 6);
            let n = self.twenties[1].borrow().encrypt(n);
            self.twenties[2].borrow().encrypt(n) + 6
        }
    }

    pub fn decrypt_num(&self, n: usize) -> usize {
        if n < 6 {
            self.sixes.decrypt(n)
        } else {
            let n = self.twenties[2].borrow().decrypt(n - 6);
            let n = self.twenties[1].borrow().decrypt(n);
            self.twenties[0].borrow().decrypt(n) + 6
        }
    }
}

lazy_static! {
    pub static ref PURPLE_ALPHABET: VecString = VecString::from("AEIOUYBCDFGHJKLMNPQRSTVWXZ");
}

pub struct Purple {
    pub switches: Switches, // this will be cloned during execution and then mutated
    pub plugboard_string: String,
    plugboard: HashMap<char, usize>,
    plugboard_inv: HashMap<usize, char>,
    pub use_kana: bool,
}

impl Default for Purple {
    fn default() -> Self {
        let plugboard = HashMap::from([
            ('N', 0),
            ('O', 1),
            ('K', 2),
            ('T', 3),
            ('Y', 4),
            ('U', 5),
            ('X', 6),
            ('E', 7),
            ('Q', 8),
            ('L', 9),
            ('H', 10),
            ('B', 11),
            ('R', 12),
            ('M', 13),
            ('P', 14),
            ('D', 15),
            ('I', 16),
            ('C', 17),
            ('J', 18),
            ('A', 19),
            ('S', 20),
            ('V', 21),
            ('W', 22),
            ('G', 23),
            ('Z', 24),
            ('F', 25),
        ]);
        let plugboard_inv = HashMap::from(
            [
                ('N', 0),
                ('O', 1),
                ('K', 2),
                ('T', 3),
                ('Y', 4),
                ('U', 5),
                ('X', 6),
                ('E', 7),
                ('Q', 8),
                ('L', 9),
                ('H', 10),
                ('B', 11),
                ('R', 12),
                ('M', 13),
                ('P', 14),
                ('D', 15),
                ('I', 16),
                ('C', 17),
                ('J', 18),
                ('A', 19),
                ('S', 20),
                ('V', 21),
                ('W', 22),
                ('G', 23),
                ('Z', 24),
                ('F', 25),
            ]
            .map(|(a, b)| (b, a)),
        );
        Self {
            switches: Default::default(),
            plugboard_string: "NOKTYUXEQLHBRMPDICJASVWGZF".into(),
            plugboard,
            plugboard_inv,
            use_kana: false,
        }
    }
}

impl Purple {
    pub fn set_plugboard(&mut self) -> Result<(), Error> {
        if self.plugboard_string.chars().count() != 26 {
            return Err(Error::key("plugboard must have exactly 26 characters"));
        }
        self.plugboard.clear();
        self.plugboard_inv.clear();
        for (n, c) in self.plugboard_string.chars().enumerate() {
            self.plugboard.insert(c, n);
            self.plugboard_inv.insert(n, c);
        }
        Ok(())
    }
}

impl Cipher for Purple {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        // convert kana to romaji if needed
        let text = if self.use_kana {
            let text = to_romaji(text, &NIHON_SHIKI);
            if let Err(e) = text {
                return Err(Error::General(e.to_string()));
            }
            text.unwrap()
        } else {
            text.to_string()
        };

        // Clone switches then encrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let n = self
                .plugboard
                .get(&c)
                .ok_or(Error::input("invalid character"))?;
            let encrypted = switches.encrypt_num(*n);
            out.push(
                *self
                    .plugboard_inv
                    .get(&encrypted)
                    .ok_or(Error::input("invalid character"))?,
            );
            switches.step();
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        // convert kana to romaji if needed
        let text = if self.use_kana {
            let text = to_romaji(text, &NIHON_SHIKI);
            if let Err(e) = text {
                return Err(Error::General(e.to_string()));
            }
            text.unwrap()
        } else {
            text.to_string()
        };

        // Clone switches then decrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let n = self
                .plugboard
                .get(&c)
                .ok_or(Error::input("invalid character"))?;
            let encrypted = switches.decrypt_num(*n);
            out.push(
                *self
                    .plugboard_inv
                    .get(&encrypted)
                    .ok_or(Error::input("invalid character"))?,
            );
            switches.step();
        }

        Ok(out)
    }

    fn randomize(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod purple_tests {

    use super::*;

    const PLAINTEXT: &'static str = "FOVTATAKIDASINIMUIMINOMOXIWOIRUBESIFYXXFCKZZR";
    const CIPHERTEXT: &'static str = "ZTXODNWKCCMAVNZXYWEETUQTCIMNVEUVIWBLUAXRRTLVA";

    #[test]
    fn encrypt() {
        let cipher = Purple::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt() {
        let cipher = Purple::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
