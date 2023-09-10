use itertools::Itertools;
use utils::{preset_alphabet::Alphabet, text_functions::rank_str, vecstring::VecString};

use crate::{Cipher, CipherError};

pub struct Vic {
    alphabet: VecString,
}

impl Default for Vic {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(Alphabet::BasicLatin),
        }
    }
}

impl Vic {
    fn vic_sequencing(&self, text: &str) -> Result<String, CipherError> {
        Ok(rank_str(&text, &self.alphabet.to_string())
            .map_err(|e| CipherError::Key(format!("{:?}", e)))?
            .iter()
            .map(|n| char::from_digit(((n + 1) % 10).try_into().unwrap(), 10).unwrap())
            .join(""))
    }

    fn chain_addition(&self, text: &str, n: usize) -> String {
        let mut v = text.chars().map(|c| u32::from(c) - 48).collect_vec();
        for i in 0..n {
            let t = (v[i] + v[i + 1]) % 10;
            v.push(t)
        }
        v.into_iter()
            .map(|d| char::from_digit(d, 10).unwrap())
            .join("")
    }

    fn digital_addition(a: char, b: char) -> char {
        let t = ((u32::from(a) - 48) + (u32::from(b) - 48)) % 10;
        char::from_digit(t, 10).unwrap()
    }

    fn digital_subtraction(a: char, b: char) -> char {
        let t = (10 + (u32::from(a) - 48) - (u32::from(b) - 48)) % 10;
        char::from_digit(t, 10).unwrap()
    }

    pub fn key_derivation_string(
        &self,
        key_group: &str,
        date: &str,
        phrase: &str,
    ) -> Result<String, CipherError> {
        let mut derivation = String::new();
        // Line-A
        derivation.push_str(&key_group[..5]);
        derivation.push('\n');

        // Line-B
        derivation.push_str(&date[..5]);
        derivation.push('\n');

        // Line-C
        let mut c = String::new();
        for (c1, c2) in key_group.chars().zip(date.chars()) {
            c.push(Self::digital_subtraction(c1, c2))
        }
        derivation.push_str(&c);
        derivation.push('\n');

        // Line-D
        derivation.push_str(&phrase[0..10]);
        derivation.push(' ');
        derivation.push_str(&phrase[10..20]);
        derivation.push('\n');

        // Line-E1
        let e1 = self.vic_sequencing(&phrase[0..10])?;
        derivation.push_str(&e1);
        derivation.push(' ');

        // Line-E2
        let e2 = self.vic_sequencing(&phrase[10..20])?;
        derivation.push_str(&e2);
        derivation.push('\n');

        // Line-F
        let f = {
            let mut temp = self.chain_addition(&c, 5);
            temp.push_str("1234567890");
            temp
        };
        derivation.push_str(&f[0..10]);
        derivation.push(' ');
        derivation.push_str(&f[10..20]);
        derivation.push('\n');

        // Line-G
        let g = {
            let mut temp = String::new();
            for (c1, c2) in e1.chars().zip(f[0..10].chars()) {
                temp.push(Self::digital_addition(c1, c2))
            }
            temp
        };
        derivation.push_str(&g);
        derivation.push('\n');

        Ok(derivation)
    }

    pub fn key_derivation(&self) {
        todo!()
    }
}

// impl Cipher for Vic {
//     fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
//         todo!()
//     }

//     fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
//         todo!()
//     }
// }

#[cfg(test)]
mod vic_tests {

    use super::*;

    const KEY_GROUP: &'static str = "72401";
    const DATE: &'static str = "139195";
    const PHRASE: &'static str = "TWASTHENIGHTBEFORECHRISTMAS";

    #[test]
    fn derivation_test() {
        let cipher = Vic::default();
        println!(
            "{}",
            cipher
                .key_derivation_string(KEY_GROUP, DATE, PHRASE)
                .unwrap()
        );
    }

    // #[test]
    // fn encrypt_test() {

    // }

    // #[test]
    // fn decrypt_test() {

    // }
}
