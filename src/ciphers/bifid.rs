use crate::{ciphers::polybius::Polybius, errors::CipherError};
use super::Cipher;

/// The Bifid Cipher combines a Polybius Square with a simple transposition
pub struct Bifid {
    polybius: Polybius,
    block_size: usize,
}


impl Bifid {

}

impl Default for Bifid {
    fn default() -> Self {
        Self { polybius: Default::default(), block_size: Default::default() }
    }
}

impl Cipher for Bifid {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let len = text.chars().count();
        let vector: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(len*2);

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

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let len = text.chars().count();

        let vector: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(len);

        for block in vector.chunks(self.block_size).map(|x| x.to_vec()) {
            let clip: String = block.iter().collect();
            let poly: String = self.polybius.encrypt(&clip)?;

            let left = &poly[0..self.block_size];
            let right = &poly[self.block_size..self.block_size*2];

            let mut sorted = String::with_capacity(self.block_size);
            for (l, r) in left.chars().zip(right.chars()) {
                sorted.push(l);
                sorted.push(r);
            }

            out.push_str(&self.polybius.decrypt(&sorted)?)
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
        self.polybius.randomize(rng)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}