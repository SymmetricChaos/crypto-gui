use std::collections::HashMap;

use crate::{
    ciphers::Cipher,
    codes::romaji::to_romaji_ks,
    errors::CipherError,
    text_aux::VecString,
};
use lazy_static::lazy_static;

use super::switch::{Switch, SwitchSpeed};

#[derive(Clone)]
pub struct Switches {
    sixes: Switch<6>,
    twenties: [Switch<20>; 3],
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
    plugboard: HashMap<char,usize>,
    plugboard_inv: HashMap<usize,char>,
}

impl Default for Purple {
    fn default() -> Self {
        let plugboard = HashMap::from([('N', 0), ('O', 1), ('K', 2), ('T', 3), ('Y', 4), ('U', 5), ('X', 6), ('E', 7), ('Q', 8), ('L', 9), ('H', 10), ('B', 11), ('R', 12), ('M', 13), ('P', 14), ('D', 15), ('I', 16), ('C', 17), ('J', 18), ('A', 19), ('S', 20), ('V', 21), ('W', 22), ('G', 23), ('Z', 24), ('F', 25)]);
        let plugboard_inv = HashMap::from([('N', 0), ('O', 1), ('K', 2), ('T', 3), ('Y', 4), ('U', 5), ('X', 6), ('E', 7), ('Q', 8), ('L', 9), ('H', 10), ('B', 11), ('R', 12), ('M', 13), ('P', 14), ('D', 15), ('I', 16), ('C', 17), ('J', 18), ('A', 19), ('S', 20), ('V', 21), ('W', 22), ('G', 23), ('Z', 24), ('F', 25)].map(|(a,b)| (b,a)));
        Self {
            switches: Default::default(),
            plugboard,
            plugboard_inv,
        }
    }
}

lazy_static! {
    pub static ref PURPLE_ALPHABET: VecString = VecString::from("AEIOUYBCDFGHJKLMNPQRSTVWXZ");
}

// impl Purple {
//     fn text_to_nums(text: &str) -> Result<Vec<usize>, CipherError> {
//         let mut out = Vec::with_capacity(text.len());
//         for c in text.chars() {
//             let n = PURPLE_ALPHABET
//                 .get_pos(c)
//                 .ok_or(CipherError::input("invalid character"))?;
//             out.push(n);
//         }
//         Ok(out)
//     }

//     fn nums_to_text(nums: Vec<usize>) -> Result<String, CipherError> {
//         let mut out = String::with_capacity(nums.len());
//         for n in nums {
//             let c = PURPLE_ALPHABET
//                 .get_char(n)
//                 .ok_or(CipherError::input("invalid character"))?;
//             out.push(c);
//         }
//         Ok(out)
//     }
// }

impl Cipher for Purple {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // convert kana to romaji if needed
        let text = to_romaji_ks(text);

        // Clone switches then encrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let n = self.plugboard.get(&c).ok_or(CipherError::input("invalid character"))?;
            let encrypted = switches.encrypt_num(*n);
            out.push(PURPLE_ALPHABET.get_char_at(encrypted).ok_or(CipherError::input("invalid character"))?);
            switches.step();
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // convert kana to romaji if needed
        let text = to_romaji_ks(text);

        // Clone switches then decrypt letters one by one, stepping each time
        let mut switches = self.switches.clone();
        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let n = PURPLE_ALPHABET.get_pos(c).ok_or(CipherError::input("invalid character"))?;
            let decrypted = switches.decrypt_num(n);
            out.push(*self.plugboard_inv.get(&decrypted).ok_or(CipherError::input("invalid character"))?);
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

    // const PLAINTEXT: &'static str = "ZTXODNWKCCMAVNZXYWEETUQTCIMNVEUVIWBLUAXRRTLVA";
    // const CIPHERTEXT: &'static str = "FOVTATAKIDASINIMUIMINOMOXIWOIRUBESIFYXXFCKZZR";

    // Checked by hand for Purple::default()
    // N will go through the Sixes
    const PLAINTEXT_V: &'static str = "N";
    const CIPHERTEXT_V: &'static str = "Y";

    // Checked by hand for Purple::default(), but this contradicts the tests for the reference, decryption also fails
    // X will go through the Twenties
    const PLAINTEXT_C: &'static str = "X";
    const CIPHERTEXT_C: &'static str = "F";

    #[test]
    fn encrypt_vowel() {
        let cipher = Purple::default();
        assert_eq!(cipher.encrypt(PLAINTEXT_V).unwrap(), CIPHERTEXT_V);
    }

    #[test]
    fn decrypt_vowel() {
        let cipher = Purple::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT_V).unwrap(), PLAINTEXT_V);
    }

    #[test]
    fn encrypt_consonant() {
        let cipher = Purple::default();
        assert_eq!(cipher.encrypt(PLAINTEXT_C).unwrap(), CIPHERTEXT_C);
    }

    #[test]
    fn derypt_consonant() {
        let cipher = Purple::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT_C).unwrap(), PLAINTEXT_C);
    }

}
