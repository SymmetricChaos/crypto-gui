use rand::Rng;
use std::iter::Iterator;

use crate::{ciphers::Cipher, errors::Error, global_rng::GLOBAL_RNG};

pub struct RailFence {
    pub rails: usize, // the slider to control this should be limited
}

impl Default for RailFence {
    fn default() -> RailFence {
        RailFence { rails: 5 }
    }
}

impl Cipher for RailFence {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        if self.rails < 2 {
            return Err(Error::key("Rail Fence key must be greater than 1"));
        }

        let mut rows = Vec::new();
        for _ in 0..self.rails {
            rows.push(Vec::<char>::new());
        }

        let mut positions: Vec<usize> = (0..self.rails).collect();
        for p in 2..self.rails {
            positions.push(self.rails - p)
        }

        for (c, n) in text.chars().zip(positions.iter().cycle()) {
            rows[*n].push(c)
        }

        let mut out = String::new();
        for row in rows {
            for c in row {
                out.push(c)
            }
        }

        Ok(out)
    }

    // There's probably an easier way to do this.
    fn decrypt(&self, text: &str) -> Result<String, Error> {
        if self.rails < 2 {
            return Err(Error::key("Rail Fence key must be greater than 1"));
        }

        // Count how many letters must be on each rail
        let mut chunks = vec![0usize; self.rails];
        let mut rail = 0;
        let mut down = true;

        for _ in text.chars() {
            chunks[rail] += 1;
            match down {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == 0 || rail == self.rails - 1 {
                down = !down
            }
        }

        // Rebuild the rails
        let mut fence = Vec::new();
        let mut ctr = 0;
        for num in chunks {
            fence.push(text[ctr..ctr + num].chars());
            ctr += num
        }

        // Read up and down the rails
        let mut rail = 0;
        let mut down = true;
        let mut out = String::with_capacity(text.chars().count());

        for _ in text.chars() {
            let c = fence[rail].next();
            match c {
                Some(symbol) => out.push(symbol),
                None => break,
            }
            match down {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == 0 || rail == self.rails - 1 {
                down = !down
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.rails = GLOBAL_RNG.lock().unwrap().gen_range(2..10);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod railfence_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "TBJRDHKRXUETYOECOOMVHZGQIWFPOEAUNSL";

    #[test]
    fn encrypt_test() {
        let mut cipher = RailFence::default();
        cipher.rails = 5;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = RailFence::default();
        cipher.rails = 5;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
