use crate::traits::Cipher;
use std::iter::Iterator;
use utils::errors::GeneralError;

pub struct RailFence {
    pub num_rails: usize, // the slider to control this should be limited
    pub start_rail: usize,
    pub falling: bool,
}

impl Default for RailFence {
    fn default() -> Self {
        Self {
            num_rails: 3,
            start_rail: 0,
            falling: true,
        }
    }
}

impl RailFence {
    pub fn positions(&self) -> std::iter::Cycle<std::vec::IntoIter<usize>> {
        let mut positions = Vec::new();
        let mut idx = self.start_rail;

        let mut falling = self.falling;

        for _ in 0..(self.num_rails * 2 - 2) {
            positions.push(idx);
            if idx >= self.num_rails - 1 {
                falling = false;
            }
            if idx == 0 {
                falling = true;
            }
            match falling {
                true => idx += 1,
                false => idx -= 1,
            };
        }
        positions.into_iter().cycle()
    }
}

impl Cipher for RailFence {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        if self.num_rails < 2 {
            return Err(GeneralError::key("Rail Fence must have at least two rails"));
        }
        if self.num_rails <= self.start_rail {
            return Err(GeneralError::key("invalid starting rail"));
        }

        let mut rows: Vec<Vec<char>> = vec![Vec::new(); self.num_rails];

        for (c, n) in text.chars().zip(self.positions()) {
            rows[n].push(c)
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
    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        if self.num_rails < 2 {
            return Err(GeneralError::key("Rail Fence must have at least two rails"));
        }
        if self.num_rails <= self.start_rail {
            return Err(GeneralError::key("invalid starting rail"));
        }

        // Count how many letters must be on each rail
        let mut chunks = vec![0usize; self.num_rails];
        let mut rail = 0;
        let mut down = true;

        for _ in text.chars() {
            chunks[rail] += 1;
            match down {
                true => rail += 1,
                false => rail -= 1,
            }
            if rail == 0 || rail == self.num_rails - 1 {
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
            if rail == 0 || rail == self.num_rails - 1 {
                down = !down
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod railfence_tests {
    use super::*;

    const PTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CTEXT: &'static str = "TBJRDHKRXUETYOECOOMVHZGQIWFPOEAUNSL";

    #[test]
    fn encrypt_test() {
        let mut cipher = RailFence::default();
        cipher.num_rails = 5;
        cipher.start_rail = 0;
        cipher.falling = false;
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = RailFence::default();
        cipher.num_rails = 5;
        cipher.start_rail = 0;
        cipher.falling = false;
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
