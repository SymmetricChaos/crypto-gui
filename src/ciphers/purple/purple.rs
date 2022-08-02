use std::collections::HashMap;

use crate::{
    ciphers::Cipher, codes::romaji::to_romaji_ks, errors::Error, text_aux::VecString,
};
use lazy_static::lazy_static;

use super::switch::{Switch, SwitchSpeed};

#[derive(Clone)]
pub struct Switches {
    pub sixes: Switch<6>,
    pub twenties: [Switch<20>; 3],
}

impl Default for Switches {
    fn default() -> Self {
        Self {
            sixes: Switch::sixes(),
            twenties: Switch::twenties(),
        }
    }
}

impl Switches {
    pub fn set_switch_speed_1() {}

    pub fn set_switch_speed_2() {}

    pub fn set_switch_speed_3() {}

    pub fn step(&mut self) {
        let spos = self.sixes.position;
        let mpos = self.get_switch(SwitchSpeed::Middle).position;

        // Sixes always steps
        self.sixes.step();

        // Exactly one of the Twenties steps at a time
        if spos == 23 && mpos == 24 {
            self.get_switch(SwitchSpeed::Slow).step();
        } else if spos == 24 {
            self.get_switch(SwitchSpeed::Middle).step();
        } else {
            self.get_switch(SwitchSpeed::Fast).step();
        }
    }

    pub fn encrypt_num(&self, n: usize) -> usize {
        if n < 6 {
            self.sixes.encrypt(n)
        } else {
            let n = self.twenties[0].encrypt(n - 6);
            let n = self.twenties[1].encrypt(n);
            self.twenties[2].encrypt(n) + 6
        }
    }

    pub fn decrypt_num(&self, n: usize) -> usize {
        if n < 6 {
            self.sixes.decrypt(n)
        } else {
            let n = self.twenties[2].decrypt(n - 6);
            let n = self.twenties[1].decrypt(n);
            self.twenties[0].decrypt(n) + 6
        }
    }

    fn get_switch(&mut self, speed: SwitchSpeed) -> &mut Switch<20> {
        for switch in self.twenties.iter_mut() {
            if switch.speed == speed {
                return switch;
            }
        }
        unreachable!("every switch speed must be represented")
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
        }
    }
}

impl Purple {
    pub fn set_plugboard(&mut self) -> Result<(), Error> {
        if self.plugboard_string.chars().count() != 26 {
            return Err(Error::key(
                "plugboard must have exactly 26 characters",
            ));
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
        //let text = to_romaji_ks(text);

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
        let text = to_romaji_ks(text);

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
