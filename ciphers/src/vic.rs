use itertools::Itertools;
use utils::{preset_alphabet::Alphabet, text_functions::rank_str};

use crate::{polybius::StraddlingCheckerboard, transposition::Columnar, Cipher, CipherError};

pub struct Vic {
    pub key_group: String,
    pub date: String,
    pub phrase: String,
    pub pin: u32,
    pub alphabet: String,
}

impl Default for Vic {
    fn default() -> Self {
        Self {
            key_group: String::new(),
            date: String::new(),
            phrase: String::new(),
            pin: 0,
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

    pub fn key_derivation_string(&self) -> Result<String, CipherError> {
        let mut derivation = String::new();
        // Line-A
        derivation.push_str("A: ");
        derivation.push_str(&self.key_group[..5]);

        // Line-B
        derivation.push_str("\nB: ");
        derivation.push_str(&self.date[..5]);

        // Line-C
        let mut c = String::new();
        for (c1, c2) in self.key_group.chars().zip(self.date.chars()) {
            c.push(Self::digital_subtraction(c1, c2))
        }
        derivation.push_str("\nC: ");
        derivation.push_str(&c);

        // Line-D
        derivation.push_str("\nD: ");
        derivation.push_str(&self.phrase[0..10]);
        derivation.push(' ');
        derivation.push_str(&self.phrase[10..20]);

        // Line-E
        let e1 = self.sequencing(&self.phrase[0..10], &self.alphabet)?;
        let e2 = self.sequencing(&self.phrase[10..20], &self.alphabet)?;
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
                (Self::char_to_digit(b) + self.pin) as usize,
                (Self::char_to_digit(a) + self.pin) as usize,
            )
        };

        derivation.push_str(&format!(
            "\n\nThe last two unequal digits are {} and {}, since the personal number is {} the key lengths will be {} and {}\n",
            key_lengths.0 - self.pin as usize,
            key_lengths.1 - self.pin as usize,
            self.pin,
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

    pub fn key_derivation(&self) -> Result<(String, String, String), CipherError> {
        let a = &self.key_group[..5];
        let b = &self.date[..5];

        let c = {
            let mut c = String::new();
            for (c1, c2) in a.chars().zip(b.chars()) {
                c.push(Self::digital_subtraction(c1, c2))
            }
            c
        };

        // Line-D is skipped

        let e1 = self.sequencing(&self.phrase[0..10], &self.alphabet)?;
        let e2 = self.sequencing(&self.phrase[10..20], &self.alphabet)?;

        let f = {
            let mut temp = c.clone();
            temp.push_str(&Self::chain_addition(&c, 5));
            temp.push_str("1234567890");
            temp
        };

        let g = {
            let mut temp = String::new();
            for (c1, c2) in e1.chars().zip(f[0..10].chars()) {
                temp.push(Self::digital_addition(c1, c2))
            }
            temp
        };

        let h = Self::digit_encoding(&g, &e2);

        let j = self.sequencing(&h, "1234567890")?;

        let block = Self::chain_addition(&h, 50);

        let key_lengths = {
            let mut last_digits = block.chars().rev();
            let mut a = last_digits.next().unwrap();
            let mut b = last_digits.next().unwrap();
            while a == b {
                a = b;
                b = last_digits.next().unwrap();
            }
            (
                (Self::char_to_digit(b) + self.pin) as usize,
                (Self::char_to_digit(a) + self.pin) as usize,
            )
        };

        // Line-Q
        let mut columnar = Columnar::default();
        columnar.assign_key(&j, "1234567890").unwrap();
        let encrypted_block = columnar.encrypt(&block)?;

        Ok((
            encrypted_block[..key_lengths.0].to_string(),
            encrypted_block[key_lengths.0..key_lengths.0 + key_lengths.1].to_string(),
            self.sequencing(&block[40..50], "1234567890")?.to_string(),
        ))
    }
}

impl Cipher for Vic {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        let (q, r, s) = self.key_derivation()?;

        let mut checkerboard = StraddlingCheckerboard::default();
        checkerboard.assign_top_row(&s);
        checkerboard.assign_alphabet(&self.alphabet);
        let mut ctext = checkerboard.encrypt(text)?;

        let mut columnar = Columnar::default();
        columnar.assign_key(&q, "1234567890").unwrap();
        ctext = columnar.encrypt(&ctext)?;

        let mut diagonal_columnar = Columnar::default();
        diagonal_columnar.assign_key(&r, "1234567890").unwrap();
        ctext = diagonal_columnar.encrypt(&ctext)?;

        Ok(ctext)
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        let (q, r, s) = self.key_derivation()?;

        let mut diagonal_columnar = Columnar::default();
        diagonal_columnar.assign_key(&r, "1234567890").unwrap();
        let mut ptext = diagonal_columnar.decrypt(&text)?;

        let mut columnar = Columnar::default();
        columnar.assign_key(&q, "1234567890").unwrap();
        ptext = columnar.decrypt(&ptext)?;

        let mut checkerboard = StraddlingCheckerboard::default();
        checkerboard.assign_top_row(&s);
        checkerboard.assign_alphabet(&self.alphabet);
        ptext = checkerboard.decrypt(&ptext)?;

        Ok(ptext)
    }
}

#[cfg(test)]
mod vic_tests {

    use super::*;

    const KEY_GROUP: &'static str = "72401";
    const DATE: &'static str = "139195";
    const PHRASE: &'static str = "TWASTHENIGHTBEFORECHRISTMAS";
    const PIN: u32 = 6;

    #[test]
    fn derivation_test() {
        let mut cipher = Vic::default();
        cipher.key_group = KEY_GROUP.to_string();
        cipher.date = DATE.to_string();
        cipher.phrase = PHRASE.to_string();
        cipher.pin = PIN;
        assert_eq!(
            "A: 72401\nB: 13919\nC: 69592\nD: TWASTHENIG HTBEFORECH\nE: 8017942653 6013589427\nF: 6959254417 1234567890\nG: 4966196060\nH: 3288628787\nJ: 3178429506\nK: 50648055525602850077\nL: 5602850077\nM: 1620350748\nN: 7823857125\nP: 5051328370\n\nThe last two unequal digits are 7 and 0, since the personal number is 6 the key lengths will be 13 and 6\n\nQ: 0668005552551\nR: 758838\nS: 5961328470",
            cipher
                .key_derivation_string()
                .unwrap()
        );
        /* The key derivation page looks like this
        A: 72401
        B: 13919
        C: 69592
        D: TWASTHENIG HTBEFORECH
        E: 8017942653 6013589427
        F: 6959254417 1234567890
        G: 4966196060
        H: 3288628787
        J: 3178429506
        K: 50648055525602850077
        L: 5602850077
        M: 1620350748
        N: 7823857125
        P: 5051328370

        The last two unequal digits are 7 and 0, since the personal number is 6 the key lengths will be 13 and 6

        Q: 0668005552551
        R: 758838
        S: 5961328470
        */
    }

    // #[test]
    // fn encrypt_test() {

    // }

    // #[test]
    // fn decrypt_test() {

    // }
}
