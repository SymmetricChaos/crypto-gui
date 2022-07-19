use crate::{
    ciphers::{substitution::Plugboard, Cipher},
    codes::romaji::to_romaji_ks,
    errors::CipherError,
    text_aux::VecString,
};
use lazy_static::lazy_static;

use super::switch::{Switch, SwitchSpeed};

#[derive(Clone)]
pub struct Switches {
    sixes: Switch<6_usize>,
    twenties: [Switch<20_usize>; 3],
}

impl Default for Switches {
    fn default() -> Self {
        Self {
            sixes: Switch::<6_usize>::sixes(),
            twenties: Switch::<20_usize>::twenties(),
        }
    }
}

impl Switches {
    pub fn step(&mut self) {
        let spos = self.sixes.position;
        let mpos = self.get_switch(SwitchSpeed::Middle).position;

        // Sixes always steps
        self.sixes.step();

        // Exactly one of the Twenties steps at a time
        if spos == 23 && mpos == 24 {
            self.get_switch(SwitchSpeed::Slow).step();
        } else if spos == 24 {
            self.get_switch(SwitchSpeed::Fast).step();
        } else {
            self.get_switch(SwitchSpeed::Middle).step();
        }
    }

    pub fn encrypt_num(&self, n: usize) -> usize {
        if n < 6 {
            self.sixes.encrypt(n)
        } else {
            let n = self.twenties[2].encrypt(n - 6);
            let n = self.twenties[1].encrypt(n);
            self.twenties[0].encrypt(n)+6
        }
    }

    pub fn decrypt_num(&self, n: usize) -> usize {
        if n < 6 {
            self.sixes.decrypt(n)
        } else {
            let n = self.twenties[0].decrypt(n - 6);
            let n = self.twenties[1].decrypt(n);
            self.twenties[2].decrypt(n)+6
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

pub struct Purple {
    switches: Switches, // this will be cloned during execution and then mutated
    plugboard: Plugboard,
}

impl Default for Purple {
    fn default() -> Self {
        let plugboard = Plugboard::build_purple("NOKTYUXEQLHBRMPDICJASVWGZF");
        Self {
            switches: Default::default(),
            plugboard,
        }
    }
}

lazy_static! {
    pub static ref PURPLE_ALPHABET: VecString = VecString::from("AEIOUYBCDFGHJKLMNPQRSTVWXZ");
}

impl Purple {
    fn text_to_nums(text: &str) -> Result<Vec<usize>, CipherError> {
        let mut out = Vec::with_capacity(text.len());
        for c in text.chars() {
            let n = PURPLE_ALPHABET
                .get_pos(c)
                .ok_or(CipherError::input("invalid character"))?;
            out.push(n);
        }
        Ok(out)
    }

    fn nums_to_text(nums: Vec<usize>) -> Result<String, CipherError> {
        let mut out = String::with_capacity(nums.len());
        for n in nums {
            let c = PURPLE_ALPHABET
                .get_char(n)
                .ok_or(CipherError::input("invalid character"))?;
            out.push(c);
        }
        Ok(out)
    }
}

impl Cipher for Purple {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // convert kana to romaji if needed
        let text = to_romaji_ks(text);

        // pass all the letters through the plugboard
        // this can be done all at once because the plugboard doesn't change
        let from_pb = self.plugboard.encrypt(&text)?;

        // Convert the plugboard values into numbers
        let mut nums = Self::text_to_nums(&from_pb)?;

        // Clone switches then encrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        for n in nums.iter_mut() {
            *n = switches.encrypt_num(*n);
            switches.step();
        }

        self.plugboard.encrypt(&Self::nums_to_text(nums)?)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let text = to_romaji_ks(text);

        let from_pb = self.plugboard.decrypt(&text)?;

        let mut nums = Self::text_to_nums(&from_pb)?;

        let mut switches = self.switches.clone();
        for n in nums.iter_mut() {
            *n = switches.decrypt_num(*n);
            switches.step();
        }

        Self::nums_to_text(nums)
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

    const PLAINTEXT: &'static str = "ZTXODNWKCCMAVNZXYWEETUQTCIMNVEUVIWBLUAXRRTLVA";
    const CIPHERTEXT: &'static str = "FOVTATAKIDASINIMUIMINOMOXIWOIRUBESIFYXXFCKZZR";

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
