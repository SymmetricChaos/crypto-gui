use itertools::Itertools;
use utils::{preset_alphabet::Alphabet, text_functions::rank_str, vecstring::VecString};

use crate::{Cipher, CipherError};

pub struct Vic {
    alphabet: String,
}

impl Default for Vic {
    fn default() -> Self {
        Self {
            alphabet: String::from(Alphabet::BasicLatin),
        }
    }
}

impl Vic {
    fn sequencing(&self, text: &str, alphabet: &str) -> Result<String, CipherError> {
        Ok(rank_str(&text, alphabet)
            .map_err(|e| CipherError::Key(format!("{:?}", e)))?
            .iter()
            .map(|n| char::from_digit(((n + 1) % 10).try_into().unwrap(), 10).unwrap())
            .join(""))
    }

    fn chain_addition(text: &str, n: usize) -> String {
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

    fn digit_encoding(a: &str, b: &str) -> String {
        let mut out = String::new();
        for ch in a.chars() {
            let mut n = (u32::from(ch) - 48) as usize;
            n = (n + 9) % 10; // Equivalent to subtracting 1, without overflowing
            out.push(b.chars().nth(n).unwrap())
        }
        out
    }

    pub fn key_derivation_string(
        &self,
        key_group: &str,
        date: &str,
        phrase: &str,
        pin: usize,
    ) -> Result<String, CipherError> {
        let mut derivation = String::new();
        // Line-A
        derivation.push_str("A: ");
        derivation.push_str(&key_group[..5]);

        // Line-B
        derivation.push_str("\nB: ");
        derivation.push_str(&date[..5]);

        // Line-C
        let mut c = String::new();
        for (c1, c2) in key_group.chars().zip(date.chars()) {
            c.push(Self::digital_subtraction(c1, c2))
        }
        derivation.push_str("\nC: ");
        derivation.push_str(&c);

        // Line-D
        derivation.push_str("\nD: ");
        derivation.push_str(&phrase[0..10]);
        derivation.push(' ');
        derivation.push_str(&phrase[10..20]);

        // Line-E
        let e1 = self.sequencing(&phrase[0..10], &self.alphabet)?;
        let e2 = self.sequencing(&phrase[10..20], &self.alphabet)?;
        derivation.push_str("\nE: ");
        derivation.push_str(&e1);
        derivation.push(' ');
        derivation.push_str(&e2);

        // Line-F
        let f = {
            let mut temp = Self::chain_addition(&c, 5);
            temp.push_str("1234567890");
            temp
        };
        derivation.push_str("\nF: ");
        derivation.push_str(&f[0..10]);
        derivation.push(' ');
        derivation.push_str(&f[10..20]);

        // Line-G
        let g = {
            let mut temp = String::new();
            for (c1, c2) in e1.chars().zip(f[0..10].chars()) {
                temp.push(Self::digital_addition(c1, c2))
            }
            temp
        };
        derivation.push_str("\nG: ");
        derivation.push_str(&g);

        // Line-H
        let h = Self::digit_encoding(&g, &e2);
        derivation.push_str("\nH: ");
        derivation.push_str(&h);

        // Line-J (there is no Line-I)
        let j = self.sequencing(&h, "1234567890")?;
        derivation.push_str("\nJ: ");
        derivation.push_str(&j);

        // Line-K through Line-P (there is no Line-O)
        let block = Self::chain_addition(&h, 50);
        derivation.push_str("\nK: ");
        derivation.push_str(&block[10..20]);
        derivation.push_str("\nL: ");
        derivation.push_str(&block[20..30]);
        derivation.push_str("\nM: ");
        derivation.push_str(&block[30..40]);
        derivation.push_str("\nN: ");
        derivation.push_str(&block[40..50]);
        derivation.push_str("\nP: ");
        derivation.push_str(&block[50..60]);

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
    const PIN: usize = 6;

    #[test]
    fn derivation_test() {
        let cipher = Vic::default();
        println!(
            "{}",
            cipher
                .key_derivation_string(KEY_GROUP, DATE, PHRASE, PIN)
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
