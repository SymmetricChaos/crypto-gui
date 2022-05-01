use std::char;

use crate::errors::CipherError;

use super::Cipher;

// Use this to fill partial inputs for the interface
// const CHECKERBOARD_ALPHABET: &'static str = "ABCDEFGHIJKLM/NOPQRSTUVWXYZ.";

pub struct StraddlingCheckerboard {
    rows: Vec<char>,
    gaps: (usize,usize),
}

impl Default for StraddlingCheckerboard {
    fn default() -> Self {
        let rows = "ETAONRISBCDFGHJKLMPQ/UVWXYZ.".chars().collect();
        let gaps = (2,6);
        StraddlingCheckerboard{
            rows, 
            gaps, 
        }
    }
}

// need to handle the digit encoding scheme
impl StraddlingCheckerboard {

    fn char_to_num(&self, c: char) -> Result<usize,CipherError> {
        if let Some(mut n) = self.rows.iter().position(|x| *x == c) {
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

    fn encrypt_char(&self, num: usize, output: &mut String) -> Result<(),CipherError> {
        let qt = num / 10;
        let rem = num % 10;
        match qt {
            0 => output.push_str(&format!("{}",qt)),
            1 => output.push_str(&format!("{}{}",self.gaps.0,rem)),
            2 => output.push_str(&format!("{}{}",self.gaps.1,rem)),
            _ => return Err(CipherError::input("invalid character"))
        }
        Ok(())
    }
    
    pub fn cipher_page(&self) -> String {
        let mut s = "Straddling Checkerboard Cipher\n  0 1 2 3 4 5 6 7 8 9\n ".to_string();
        let mut symbols = self.rows.iter();

        for _ in 0..10 {
            s.push(' ');
            s.push(*symbols.next().unwrap())
        }

        s.push_str(&format!("\n{}",self.gaps.0));
        for _ in 0..10 {
            s.push(' ');
            s.push(*symbols.next().unwrap())
        }

        s.push_str(&format!("\n{}",self.gaps.1));
        for _ in 0..10 {
            s.push(' ');
            s.push(*symbols.next().unwrap())
        }
        s
    }

}

impl Cipher for StraddlingCheckerboard {
    
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = String::with_capacity(text.len());
        let mut digit_mode = false;
        for c in text.chars() {
        
            // If in digit mode push the character directly onto the output
            // then turn off digit_mode
            if digit_mode {
                // check that c is a character and return Error if not
                if !c.is_ascii_digit() {
                    return Err(CipherError::input("only digits 0 to 9 can be coded as digits"))
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

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = String::with_capacity(text.len());
        let mut numbers = text.chars().map(|c| c.to_digit(10).unwrap() as usize);

        // This needs to correct for gaps
        while let Some(n) = numbers.next() {

            if n == self.gaps.0 {
                let x = numbers.next().unwrap();
                out.push(*self.rows.iter().nth(x + 10).unwrap())
            
            } else if n == self.gaps.1 {
                let x = numbers.next().unwrap();
                out.push(*self.rows.iter().nth(x + 20).unwrap())
            
            } else {
                out.push(*self.rows.iter().nth(n).unwrap())
            }
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

}