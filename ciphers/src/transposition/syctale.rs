use crate::traits::Cipher;
use utils::errors::GeneralError;

pub struct Scytale {
    pub num_rails: usize,
    pub padding: char,
}

impl Default for Scytale {
    fn default() -> Scytale {
        Scytale {
            num_rails: 4,
            padding: 'X',
        }
    }
}

impl Cipher for Scytale {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        if self.num_rails <= 1 {
            return Err(GeneralError::key("Scytale key must have at least 2 rails"));
        }

        let n_cols = num::Integer::div_ceil(&text.chars().count(), &self.num_rails);
        let mut symbols = text.chars();
        let mut rows = Vec::with_capacity(self.num_rails);

        for _ in 0..self.num_rails {
            let mut v = Vec::with_capacity(n_cols);
            for _ in 0..n_cols {
                v.push(symbols.next().unwrap_or(self.padding))
            }
            rows.push(v)
        }

        let mut out = String::with_capacity(text.len());

        for x in 0..n_cols {
            for y in 0..self.num_rails {
                out.push(rows[y][x])
            }
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        if self.num_rails <= 1 {
            return Err(GeneralError::key("Scytale key must be 2 or greater"));
        }

        let n_cols = num::Integer::div_ceil(&text.chars().count(), &self.num_rails);
        let mut symbols = text.chars();
        let mut rows = Vec::with_capacity(n_cols);

        for _ in 0..n_cols {
            let mut v = Vec::with_capacity(self.num_rails);
            for _ in 0..self.num_rails {
                v.push(symbols.next().unwrap_or(self.padding))
            }
            rows.push(v)
        }

        let mut out = String::with_capacity(text.len());

        for x in 0..self.num_rails {
            for y in 0..n_cols {
                out.push(rows[y][x])
            }
        }

        Ok(out)
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
        cipher.num_rails = 5;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Scytale::default();
        cipher.num_rails = 5;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
