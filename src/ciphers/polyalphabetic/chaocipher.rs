use crate::ciphers::Cipher;
use crate::errors::Error;
use crate::global_rng::get_global_rng;
use crate::text_aux::text_functions::validate_text;
use crate::text_aux::VecString;

pub struct Chaocipher {
    pub left_string: String,
    left: VecString,
    pub right_string: String,
    right: VecString,
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

    pub fn set_left(&mut self) {
        self.left = VecString::unique_from(&self.left_string)
    }

    pub fn set_right(&mut self) {
        self.right = VecString::unique_from(&self.right_string)
    }

    pub fn assign_left(&mut self, s: &str) {
        self.left_string = String::from(s);
        self.set_left();
    }

    pub fn assign_right(&mut self, s: &str) {
        self.right_string = String::from(s);
        self.set_right();
    }
}

impl Default for Chaocipher {
    fn default() -> Self {
        Chaocipher {
            left: VecString::from("HXUCZVAMDSLKPEFJRIGTWOBNYQ"),
            right: VecString::from("PTLNBQDEOYSFAVZKGJRIHWXUMC"),
            left_string: String::from("HXUCZVAMDSLKPEFJRIGTWOBNYQ"),
            right_string: String::from("PTLNBQDEOYSFAVZKGJRIHWXUMC"),
        }
    }
}

impl Cipher for Chaocipher {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        validate_text(text, &self.left)?;

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

    fn decrypt(&self, text: &str) -> Result<String, Error> {
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

    fn randomize(&mut self) {
        let rng = &mut get_global_rng();
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
