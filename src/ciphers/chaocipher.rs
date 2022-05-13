use crate::errors::CipherError;
use crate::text_aux::vecstring::VecString;

use super::Cipher;

pub struct Chaocipher {
    pub left: VecString,
    pub right: VecString,
}

impl Chaocipher {
    fn left_permute(left: &mut VecString, n: usize) {
        left.rotate_left(n);
        let t = left.remove(1).unwrap();
        left.insert(13, t);
    }

    fn right_permute(right: &mut VecString, n: usize) {
        right.rotate_left(n + 1);
        let t = right.remove(2).unwrap();
        right.insert(13, t);
    }

    pub fn step(&mut self, n: usize) {
        Chaocipher::left_permute(&mut self.left, n);
        Chaocipher::right_permute(&mut self.right, n);
    }

    pub fn set_left(&mut self, s: &str) {
        self.left = VecString::from(s)
    }

    pub fn set_right(&mut self, s: &str) {
        self.right = VecString::from(s)
    }

    // pub fn previous_state(&mut self) {
    //     self.left.replace(self.prev_state.0.clone());
    //     self.right.replace(self.prev_state.1.clone());
    // }
}

impl Default for Chaocipher {
    fn default() -> Self {
        Chaocipher {
            left: VecString::from("HXUCZVAMDSLKPEFJRIGTWOBNYQ"),
            right: VecString::from("PTLNBQDEOYSFAVZKGJRIHWXUMC"),
            //prev_state: (VecString::from(PresetAlphabet::BasicLatin), VecString::from("AZDNBUHYFWJLVGRCQMPSOEXTKI"))
        }
    }
}

impl Cipher for Chaocipher {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = right.get_pos(c).unwrap();
            out.push(left[n]);
            Chaocipher::left_permute(&mut left, n);
            Chaocipher::right_permute(&mut right, n);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = left.get_pos(c).unwrap();
            out.push(right[n]);
            Chaocipher::left_permute(&mut left, n);
            Chaocipher::right_permute(&mut right, n);
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
        self.left.shuffle(rng);
        self.right.shuffle(rng);
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod chaocipher_tests {
    // http://www.chaocipher.com/ActualChaocipher/Chaocipher-Revealed-Algorithm.pdf
    use super::*;

    const PLAINTEXT: &'static str = "WELLDONEISBETTERTHANWELLSAID";
    const CIPHERTEXT: &'static str = "OAHQHCNYNXTSZJRRHJBYHQKSOUJY";

    #[test]
    fn encrypt_test() {
        let cipher = Chaocipher::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Chaocipher::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
