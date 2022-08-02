use super::PolybiusCube;
use crate::{ciphers::Cipher, errors::Error, global_rng::GLOBAL_RNG};
use num::Integer;
use rand::Rng;

fn is_power_of_three(a: usize) -> bool {
    let mut p = 1;
    while p < a {
        if a == p {
            return true;
        } else {
            p *= 3
        }
    }
    false
}

// The Trifid Cipher combines a Polybius Cube with a simple transposition
pub struct Trifid {
    pub polybius: PolybiusCube,
    pub block_size: usize,
}

impl Default for Trifid {
    fn default() -> Self {
        Self {
            polybius: PolybiusCube::default(),
            block_size: 7,
        }
    }
}

impl Trifid {
    pub fn set_alphabet(&mut self) -> Result<(), Error> {
        let new_alpha_len = self.polybius.alphabet_string.chars().count();

        if !is_power_of_three(new_alpha_len) {
            return Err(Error::alphabet(
                "alphabet length must be a power of three to fill the grid",
            ));
        }
        self.polybius.set_alphabet()
    }
}

impl Cipher for Trifid {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let vector: Vec<char> = text.chars().collect();
        let len = vector.len();
        if !len.is_multiple_of(&self.block_size) {
            return Err(Error::input(
                "Input length must be a multiple of the block size",
            ));
        };
        let mut out = String::with_capacity(len * 3);

        for block in vector
            .chunks(self.block_size)
            .map(|x| x.to_vec().iter().collect::<String>())
        {
            let poly = self.polybius.encrypt(&block)?;
            let mut first = String::with_capacity(len);
            let mut second = String::with_capacity(len);
            let mut third = String::with_capacity(len);
            for (pos, s) in poly.chars().enumerate() {
                match pos % 3 {
                    0 => first.push(s),
                    1 => second.push(s),
                    2 => third.push(s),
                    _ => unreachable!("n % 3 can only be 0, 1, or 2"),
                }
            }
            first.push_str(&second);
            first.push_str(&third);
            out.push_str(&self.polybius.decrypt(&first)?)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        // turn text into a vector and prepare a string to fill with the output
        let vector: Vec<char> = text.chars().collect();
        if !vector.len().is_multiple_of(&self.block_size) {
            return Err(Error::input(
                "Input length must be a multiple of the block size",
            ));
        };
        let mut out = String::with_capacity(vector.len());

        // Divide the vector into chunks of the block size
        for block in vector
            .chunks(self.block_size)
            .map(|x| x.to_vec().iter().collect::<String>())
        {
            // Turn the block into a String then encrypt it with the Polybius cipher
            let poly: String = self.polybius.encrypt(&block)?;

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
mod trifid_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
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
