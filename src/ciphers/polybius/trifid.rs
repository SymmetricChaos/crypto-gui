use rand::Rng;

use crate::{errors::CipherError, ciphers::Cipher};

use super::PolybiusCube;

// The Trifid Cipher combines a Polybius Cube with a simple transposition
pub struct Trifid {
    pub cube: PolybiusCube,
    pub block_size: usize,
}

impl Default for Trifid {
    fn default() -> Self {
        Self {
            cube: PolybiusCube::default(),
            block_size: 7,
        }
    }
}

impl Cipher for Trifid {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let vector: Vec<char> = text.chars().collect();
        let len = vector.len();
        if len % self.block_size != 0 {
            return Err(CipherError::input(
                "Input length must be a multiple of the block size",
            ));
        };
        let mut out = String::with_capacity(len * 3);

        for block in vector.chunks(self.block_size).map(|x| x.to_vec()) {
            let clip: String = block.iter().collect();
            let poly = self.cube.encrypt(&clip)?;
            let mut first = String::with_capacity(len);
            let mut second = String::with_capacity(len);
            let mut third = String::with_capacity(len);
            for (pos, s) in poly.chars().enumerate() {
                match pos % 3 {
                    0 => first.push(s),
                    1 => second.push(s),
                    2 => third.push(s),
                    _ => unreachable!("n % 3 can only be 0, 1, or 2")
                }
            }
            first.push_str(&second);
            first.push_str(&third);
            out.push_str(&self.cube.decrypt(&first)?)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // turn text into a vector and prepare a string to fill with the output
        let vector: Vec<char> = text.chars().collect();
        if vector.len() % self.block_size != 0 {
            return Err(CipherError::input(
                "Input length must be a multiple of the block size",
            ));
        };
        let mut out = String::with_capacity(vector.len());

        // Divide the vector into chunks of the block size
        for block in vector.chunks(self.block_size).map(|x| x.to_vec()) {
            // Turn the block into a String then encrypt it with the Polybius cipher
            let clip: String = block.iter().collect();
            let poly: String = self.cube.encrypt(&clip)?;

            dbg!(&clip);

            // Divide the encrypted string in half
            // TODO: This will likely panic with non-ASCII inputs
            let left = &poly[0..self.block_size];
            let right = &poly[self.block_size..self.block_size * 3];

            // Take characters from left and right as pairs and write them into a new string
            let mut sorted = String::with_capacity(self.block_size * 3);
            for (l, r) in left.chars().zip(right.chars()) {
                sorted.push(l);
                sorted.push(r);
            }

            // Decrypt the result and push it onto the output string
            out.push_str(&self.cube.decrypt(&sorted)?)
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
        self.block_size = rng.gen_range(3..=30);
        self.cube.randomize(rng)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod trifid_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "";

    #[test]
    fn encrypt_test() {
        let cipher = Trifid::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Trifid::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}