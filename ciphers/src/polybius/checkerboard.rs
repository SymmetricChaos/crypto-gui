use crate::{errors::CipherError, traits::Cipher};
use std::char;
use utils::text_functions::keyed_alphabet;

// Use this to fill partial inputs for the interface
const CHECKERBOARD_ALPHABET: &'static str = "ABCDEFGHIJKLM/NOPQRSTUVWXYZ.";

pub struct StraddlingCheckerboard {
    pub top_row: String,
    pub letter_rows: Vec<char>,
    pub gaps: (usize, usize),
}

impl Default for StraddlingCheckerboard {
    fn default() -> Self {
        let top_row = String::from("0123456789");
        let letter_rows = "ETAONRISBCDFGHJKLMPQ/UVWXYZ.".chars().collect();
        let gaps = (2, 6);
        StraddlingCheckerboard {
            top_row,
            letter_rows,
            gaps,
        }
    }
}

// need to handle the digit encoding scheme
impl StraddlingCheckerboard {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.letter_rows = keyed_alphabet(alphabet, CHECKERBOARD_ALPHABET)
            .chars()
            .collect();
    }

    pub fn assign_top_row(&mut self, row: &str) {
        self.top_row = keyed_alphabet(row, "0123456789")
    }

    fn char_to_num(&self, c: char) -> Result<usize, CipherError> {
        if let Some(mut n) = self.letter_rows.iter().position(|x| *x == c) {
            if n >= self.gaps.0 {
                n += 1
            }
            if n >= self.gaps.1 {
                n += 1
            }
            Ok(n)
        } else {
            Err(CipherError::invalid_input_char(c))
        }
    }

    fn encrypt_char(&self, num: usize, output: &mut String) -> Result<(), CipherError> {
        let y_digit = num / 10;
        let x_digit = self.top_row.chars().nth(num % 10).unwrap();
        match y_digit {
            0 => output.push_str(&format!("{}", x_digit)),
            1 => output.push_str(&format!("{}{}", self.gaps.0, x_digit)),
            2 => output.push_str(&format!("{}{}", self.gaps.1, x_digit)),
            _ => return Err(CipherError::input("invalid character")),
        }
        Ok(())
    }

    fn x_position(&self, c: char) -> usize {
        self.top_row.chars().position(|x| x == c).unwrap()
    }

    pub fn cipher_page(&self) -> String {
        let mut page = String::with_capacity(87);
        page.push(' ');
        for digit in self.top_row.chars() {
            page.push(' ');
            page.push(digit);
        }
        page.push_str("\n ");
        let mut symbols = self.letter_rows.iter();

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
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.len());
        let mut digit_mode = false;

        for c in text.chars() {
            // If in digit mode push the character directly onto the output
            // then turn off digit_mode
            if digit_mode {
                // check that c is a digit and return Error if not
                if !c.is_ascii_digit() {
                    return Err(CipherError::input(
                        "only digits 0 to 9 can be coded as digits",
                    ));
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

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.len());
        let mut numbers = text.chars().map(|c| c.to_digit(10).unwrap() as usize);

        // This needs to handle gaps correctly
        while let Some(n) = numbers.next() {
            let c = if n == self.gaps.0 {
                let x = numbers.next().unwrap();
                *self.letter_rows.iter().nth(x + 8).unwrap()
            } else if n == self.gaps.1 {
                let x = numbers.next().unwrap();
                *self.letter_rows.iter().nth(x + 18).unwrap()
            } else {
                if n >= self.gaps.1 {
                    *self.letter_rows.iter().nth(n - 2).unwrap()
                } else if n >= self.gaps.0 {
                    *self.letter_rows.iter().nth(n - 1).unwrap()
                } else {
                    *self.letter_rows.iter().nth(n).unwrap()
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

    #[test]
    fn circular_test() {
        let mut cipher = StraddlingCheckerboard::default();
        cipher.assign_top_row("923");
        let ctext = cipher.encrypt("ESTONIA").unwrap();
        let ptext = cipher.decrypt(&ctext).unwrap();
        assert_eq!("ESTONIA", ptext);
    }
}
