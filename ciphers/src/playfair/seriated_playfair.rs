use itertools::Itertools;
use utils::functions::string_chunks;

use crate::{Cipher, CipherError};

use super::Playfair;

pub struct SeriatedPlayfair {
    pub period: usize,
    pub playfair: Playfair,
}

impl Default for SeriatedPlayfair {
    fn default() -> Self {
        Self {
            period: 4,
            playfair: Playfair::default(),
        }
    }
}

impl SeriatedPlayfair {
    pub fn groups(&self, text: &str) -> Vec<String> {
        let mut chunks = string_chunks(text, self.period);

        // if there are an even number of chunks fill ot the last one with spacers
        if chunks.len() % 2 == 0 {
            let x = chunks.last_mut().unwrap();
            while x.len() != self.period {
                x.push(self.playfair.spacer)
            }
        // if there are an odd number of chunks split the last one in half and fill out the second half with spacers
        } else {
            let last = chunks.pop().unwrap();
            let len = last.len();
            let left: String = last.chars().take(len / 2 + 1).collect();
            let right: String = last.chars().skip(len / 2 + 1).collect();
            chunks.push(left);
            chunks.push(right);
            let x = chunks.last_mut().unwrap();
            while x.len() != len / 2 + 1 {
                x.push(self.playfair.spacer)
            }
        }
        chunks
    }
}

impl Cipher for SeriatedPlayfair {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let groups = self.groups(text);
        let mut out = String::with_capacity(text.len() + 4); // maximum spacer size is four bytes
        for (l_str, r_str) in groups
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|c| c.collect_tuple().unwrap())
        {
            for (l, r) in l_str.chars().zip(r_str.chars()) {
                if l == r {
                    return Err(CipherError::Input(format!(
                        "found repeated character {}, a spacer should be inserted",
                        l
                    )));
                }
                let lpos = self.playfair.char_to_position(l)?;
                let rpos = self.playfair.char_to_position(r)?;
                let pair = self.playfair.playfair_shift(lpos, rpos, true);
                out.push(pair.0);
                out.push(pair.1);
            }
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        if text.chars().count() % 2 != 0 {
            return Err(CipherError::input(
                "decrypting a seriated playfair requires an even number of characters",
            ));
        }

        let mut out = String::with_capacity(text.len());
        let mut left = String::with_capacity(text.len() / 2);
        let mut right = String::with_capacity(text.len() / 2);
        for (l, r) in text
            .chars()
            .chunks(2)
            .into_iter()
            .map(|c| c.collect_tuple().unwrap())
        {
            if l == r {
                return Err(CipherError::Input(format!(
                    "found repeated character {}, a spacer should be inserted",
                    l
                )));
            }
            let lpos = self.playfair.char_to_position(l)?;
            let rpos = self.playfair.char_to_position(r)?;
            let pair = self.playfair.playfair_shift(lpos, rpos, false);
            left.push(pair.0);
            right.push(pair.1);
        }
        out.push_str(&left);
        out.push_str(&right);

        Ok(out)
    }
}

#[cfg(test)]
mod seriated_playfair_tests {
    use super::*;

    #[test]
    fn chunks() {
        let mut cipher = SeriatedPlayfair::default();
        cipher.period = 5;
        assert_eq!(
            cipher.groups("AAAAAAAAAAAAAAAAAAAAAAA"),
            ["AAAAA", "AAAAA", "AAAAA", "AAAAA", "AA", "AX"]
        );
        cipher.period = 6;
        assert_eq!(
            cipher.groups("AAAAAAAAAAAAAAAAAAAAAAA"),
            ["AAAAAA", "AAAAAA", "AAAAAA", "AAAAAX"]
        );
    }

    #[test]
    fn encrypt_test() {
        let cipher = SeriatedPlayfair::default();
        assert_eq!(cipher.encrypt("THEKUICX").unwrap(), "UPIJADMV");
    }

    #[test]
    fn decrypt_test() {
        let cipher = SeriatedPlayfair::default();
        assert_eq!(cipher.decrypt("UPIJADMV").unwrap(), "THEKUICX");
    }
}
