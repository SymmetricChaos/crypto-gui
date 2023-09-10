use itertools::Itertools;
use utils::{preset_alphabet::Alphabet, text_functions::rank_str};

use crate::{transposition::Columnar, Cipher, CipherError};

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
        let mut v = text.chars().map(|c| Self::char_to_digit(c)).collect_vec();
        for i in 0..n {
            let t = (v[i] + v[i + 1]) % 10;
            v.push(t)
        }
        v.into_iter()
            .skip(text.chars().count())
            .map(|d| char::from_digit(d, 10).unwrap())
            .join("")
    }

    fn digital_addition(a: char, b: char) -> char {
        let t = (Self::char_to_digit(a) + Self::char_to_digit(b)) % 10;
        char::from_digit(t, 10).unwrap()
    }

    fn digital_subtraction(a: char, b: char) -> char {
        let t = (10 + Self::char_to_digit(a) - Self::char_to_digit(b)) % 10;
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

    fn char_to_digit(c: char) -> u32 {
        u32::from(c) - 48
    }

    pub fn key_derivation_string(
        &self,
        key_group: &str,
        date: &str,
        phrase: &str,
        pin: u32,
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
            let mut temp = c.clone();
            temp.push_str(&Self::chain_addition(&c, 5));
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
        derivation.push_str(&block[..20]);
        derivation.push_str("\nL: ");
        derivation.push_str(&block[10..20]);
        derivation.push_str("\nM: ");
        derivation.push_str(&block[20..30]);
        derivation.push_str("\nN: ");
        derivation.push_str(&block[30..40]);
        derivation.push_str("\nP: ");
        derivation.push_str(&block[40..50]);

        // Derive key lengths
        let key_lengths = {
            let mut last_digits = block.chars().rev();
            let mut a = last_digits.next().unwrap();
            let mut b = last_digits.next().unwrap();
            while a == b {
                a = b;
                b = last_digits.next().unwrap();
            }
            (
                (Self::char_to_digit(b) + pin) as usize,
                (Self::char_to_digit(a) + pin) as usize,
            )
        };

        derivation.push_str(&format!(
            "\n\nThe last two unequal digits are {} and {} so the key lengths will be {} and {}\n",
            key_lengths.0 - pin as usize,
            key_lengths.1 - pin as usize,
            key_lengths.0,
            key_lengths.1
        ));

        // Line-Q
        let mut columnar = Columnar::default();
        columnar.assign_key(&j, "1234567890").unwrap();
        let encrypted_block = columnar.encrypt(&block)?;
        derivation.push_str("\nQ: ");
        derivation.push_str(&encrypted_block[..key_lengths.0]);

        derivation.push_str("\nR: ");
        derivation.push_str(&encrypted_block[key_lengths.0..key_lengths.0 + key_lengths.1]);

        derivation.push_str("\nS: ");
        derivation.push_str(&self.sequencing(&block[40..50], "1234567890")?);

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
    const PIN: u32 = 6;

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
