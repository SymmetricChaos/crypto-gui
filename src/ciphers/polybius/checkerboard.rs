use std::char;

use rand::prelude::SliceRandom;

use crate::{ciphers::Cipher, errors::Error, global_rng::get_global_rng, text_aux::keyed_alphabet};

// Use this to fill partial inputs for the interface
const CHECKERBOARD_ALPHABET: &'static str = "ABCDEFGHIJKLM/NOPQRSTUVWXYZ.";

pub struct StraddlingCheckerboard {
    rows: Vec<char>,
    pub gaps: (usize, usize),
    pub alphabet: String,
}

impl Default for StraddlingCheckerboard {
    fn default() -> Self {
        let rows = "ETAONRISBCDFGHJKLMPQ/UVWXYZ.".chars().collect();
        let gaps = (2, 6);
        StraddlingCheckerboard {
            rows,
            gaps,
            alphabet: "ETAONRISBCDFGHJKLMPQ/UVWXYZ.".to_string(),
        }
    }
}

// need to handle the digit encoding scheme
impl StraddlingCheckerboard {
    pub fn set_alphabet(&mut self) {
        self.rows = keyed_alphabet(&self.alphabet, CHECKERBOARD_ALPHABET)
            .chars()
            .collect();
    }

    fn char_to_num(&self, c: char) -> Result<usize, Error> {
        if let Some(mut n) = self.rows.iter().position(|x| *x == c) {
            if n >= self.gaps.0 {
                n += 1
            }
            if n >= self.gaps.1 {
                n += 1
            }
            Ok(n)
        } else {
            Err(Error::invalid_input_char(c))
        }
    }

    fn encrypt_char(&self, num: usize, output: &mut String) -> Result<(), Error> {
        let qt = num / 10;
        let rem = num % 10;
        match qt {
            0 => output.push_str(&format!("{}", rem)),
            1 => output.push_str(&format!("{}{}", self.gaps.0, rem)),
            2 => output.push_str(&format!("{}{}", self.gaps.1, rem)),
            _ => return Err(Error::input("invalid character")),
        }
        Ok(())
    }

    pub fn cipher_page(&self) -> String {
        let mut page = String::with_capacity(87);
        page.push_str("  0 1 2 3 4 5 6 7 8 9\n ");
        let mut symbols = self.rows.iter();

        for idx in 0..10 {
            page.push(' ');
            if self.gaps.0 == idx || self.gaps.1 == idx {
                page.push(' ');
            } else {
                page.push(*symbols.next().unwrap())
            }
        }

        page.push_str(&format!("\n{}", self.gaps.0));
        for _ in 0..10 {
            page.push(' ');
            page.push(*symbols.next().unwrap())
        }

        page.push_str(&format!("\n{}", self.gaps.1));
        for _ in 0..10 {
            page.push(' ');
            page.push(*symbols.next().unwrap())
        }
        page
    }
}

impl Cipher for StraddlingCheckerboard {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len());
        let mut digit_mode = false;

        for c in text.chars() {
            // If in digit mode push the character directly onto the output
            // then turn off digit_mode
            if digit_mode {
                // check that c is a character and return Error if not
                if !c.is_ascii_digit() {
                    return Err(Error::input("only digits 0 to 9 can be coded as digits"));
                }
                out.push(c);
                digit_mode = false;
            } else {
                // Otherwise convert it to a number and encrypt
                let n = self.char_to_num(c)?;
                self.encrypt_char(n, &mut out)?;
            }

            // If c is the escape symbol turn on digit mode
            if c == '/' {
                digit_mode = true
            }
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len());
        let mut numbers = text.chars().map(|c| c.to_digit(10).unwrap() as usize);

        // This needs to handle gaps correctly
        while let Some(n) = numbers.next() {
            let c = if n == self.gaps.0 {
                let x = numbers.next().unwrap();
                *self.rows.iter().nth(x + 8).unwrap()
            } else if n == self.gaps.1 {
                let x = numbers.next().unwrap();
                *self.rows.iter().nth(x + 18).unwrap()
            } else {
                if n >= self.gaps.1 {
                    *self.rows.iter().nth(n - 2).unwrap()
                } else if n >= self.gaps.0 {
                    *self.rows.iter().nth(n - 1).unwrap()
                } else {
                    *self.rows.iter().nth(n).unwrap()
                }
            };
            out.push(c);
            if c == '/' {
                let n = (numbers.next().unwrap() + 48) as u8 as char;
                out.push(n)
            }
        }

        Ok(out)
    }

    fn randomize(&mut self) {
        let mut rng = get_global_rng();
        self.rows.shuffle(&mut *rng);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod checkerboard_tests {
    // http://www.chaocipher.com/ActualChaocipher/Chaocipher-Revealed-Algorithm.pdf
    use super::*;
    const PLAINTEXT: &'static str = "ATTACKTHEQUICKBROWNFOXAT/0/5/3/1";
    const CIPHERTEXT: &'static str = "31132127125061638212720746552346631620625623621";

    #[test]
    fn encrypt_test() {
        let cipher = StraddlingCheckerboard::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = StraddlingCheckerboard::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
