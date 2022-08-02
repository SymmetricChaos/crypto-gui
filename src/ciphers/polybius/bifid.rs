use rand::Rng;

use crate::{ciphers::Cipher, errors::Error, global_rng::GLOBAL_RNG};

use super::PolybiusSquare;

/// The Bifid Cipher combines a Polybius Square with a simple transposition
pub struct Bifid {
    pub polybius: PolybiusSquare,
    pub block_size: usize,
}

impl Default for Bifid {
    fn default() -> Self {
        Self {
            polybius: Default::default(),
            block_size: 7,
        }
    }
}

impl Bifid {
    pub fn set_alphabet(&mut self) -> Result<(), Error> {
        let new_alpha_len = self.polybius.alphabet_string.chars().count();
        if !new_alpha_len.is_power_of_two() {
            return Err(Error::alphabet(
                "alphabet length must be a power of two to fill the grid",
            ));
        }

        self.polybius.set_alphabet()
    }
}

impl Cipher for Bifid {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let vector: Vec<char> = text.chars().collect();
        let len = vector.len();
        if len % self.block_size != 0 {
            return Err(Error::input(
                "Input length must be a multiple of the block size",
            ));
        };
        let mut out = String::with_capacity(len * 2);

        for block in vector.chunks(self.block_size).map(|x| x.to_vec()) {
            let clip: String = block.iter().collect();
            let poly = self.polybius.encrypt(&clip)?;
            let mut left = String::with_capacity(len);
            let mut right = String::with_capacity(len);
            for (pos, s) in poly.chars().enumerate() {
                if (pos % 2) == 0 {
                    left.push(s)
                } else {
                    right.push(s)
                }
            }
            left.push_str(&right);
            out.push_str(&self.polybius.decrypt(&left)?)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        // turn text into a vector and prepare a string to fill with the output
        let vector: Vec<char> = text.chars().collect();
        if vector.len() % self.block_size != 0 {
            return Err(Error::input(
                "Input length must be a multiple of the block size",
            ));
        };
        let mut out = String::with_capacity(vector.len());

        // Divide the vector into chunks of the block size
        for block in vector.chunks(self.block_size).map(|x| x.to_vec()) {
            // Turn the block into a String then encrypt it with the Polybius cipher
            let clip: String = block.iter().collect();
            let poly: String = self.polybius.encrypt(&clip)?;

            dbg!(&clip);

            // Divide the encrypted string in half
            // TODO: This will likely panic with non-ASCII inputs
            let left = &poly[0..self.block_size];
            let right = &poly[self.block_size..self.block_size * 2];

            // Take characters from left and right as pairs and write them into a new string
            let mut sorted = String::with_capacity(self.block_size * 2);
            for (l, r) in left.chars().zip(right.chars()) {
                sorted.push(l);
                sorted.push(r);
            }

            // Decrypt the result and push it onto the output string
            out.push_str(&self.polybius.decrypt(&sorted)?)
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.block_size = GLOBAL_RNG.lock().unwrap().gen_range(3..=30);
        self.polybius.randomize();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod bifid_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "RCRDOESKSXFGWPOINUOXCODREEIOKZCGETW";

    #[test]
    fn encrypt_test() {
        let cipher = Bifid::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Bifid::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
