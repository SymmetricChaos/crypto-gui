use crate::{ciphers::Cipher, errors::Error, global_rng::GLOBAL_RNG};
use rand::Rng;

pub struct Scytale {
    pub key: usize,
    padding: char,
}

impl Default for Scytale {
    fn default() -> Scytale {
        Scytale {
            key: 4,
            padding: 'X',
        }
    }
}

impl Cipher for Scytale {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        if self.key <= 1 {
            return Err(Error::key("Scytale key must be 2 or greater"));
        }

        let n_cols = num::Integer::div_ceil(&text.len(), &self.key);
        let mut symbols = text.chars();
        let mut rows = Vec::with_capacity(self.key);

        for _ in 0..self.key {
            let mut v = Vec::with_capacity(n_cols);
            for _ in 0..n_cols {
                v.push(symbols.next().unwrap_or(self.padding))
            }
            rows.push(v)
        }

        let mut out = String::with_capacity(text.len());

        for x in 0..n_cols {
            for y in 0..self.key {
                out.push(rows[y][x])
            }
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        if self.key <= 1 {
            return Err(Error::key("Scytale key must be 2 or greater"));
        }

        let n_cols = num::Integer::div_ceil(&text.len(), &self.key);
        let mut symbols = text.chars();
        let mut rows = Vec::with_capacity(n_cols);

        for _ in 0..n_cols {
            let mut v = Vec::with_capacity(self.key);
            for _ in 0..self.key {
                v.push(symbols.next().unwrap_or(self.padding))
            }
            rows.push(v)
        }

        let mut out = String::with_capacity(text.len());

        for x in 0..self.key {
            for y in 0..n_cols {
                out.push(rows[y][x])
            }
        }

        Ok(out)
    }

    fn randomize(&mut self) {
        self.key = GLOBAL_RNG.lock().unwrap().gen_range(2..10);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod scytale_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "TKOOLHBXVAERJEZQOURYUWMTDINPHOCFSEG";

    #[test]
    fn encrypt_test() {
        let mut cipher = Scytale::default();
        cipher.key = 5;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Scytale::default();
        cipher.key = 5;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
