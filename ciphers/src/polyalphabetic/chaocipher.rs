use crate::traits::Cipher;
use utils::errors::GeneralError;
use utils::vecstring::VecString;

pub struct Chaocipher {
    pub left: VecString,
    pub right: VecString,
}

pub fn left_permute(left: &mut VecString, n: usize) {
    left.rotate_left(n);
    let t = left.remove(1).unwrap();
    left.insert(13, t);
}

pub fn right_permute(right: &mut VecString, n: usize) {
    right.rotate_left(n + 1);
    let t = right.remove(2).unwrap();
    right.insert(13, t);
}

impl Chaocipher {
    pub fn assign_left(&mut self, s: &str) {
        self.left = VecString::unique_from(s)
    }

    pub fn assign_right(&mut self, s: &str) {
        self.right = VecString::unique_from(s)
    }
}

impl Default for Chaocipher {
    fn default() -> Self {
        Chaocipher {
            left: VecString::from("HXUCZVAMDSLKPEFJRIGTWOBNYQ"),
            right: VecString::from("PTLNBQDEOYSFAVZKGJRIHWXUMC"),
        }
    }
}

impl Cipher for Chaocipher {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = right
                .get_pos(c)
                .ok_or(GeneralError::invalid_input_char(c))?;
            out.push(*left.get_char(n).unwrap()); // Error will be caught by previous line
            left_permute(&mut left, n);
            right_permute(&mut right, n);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = left.get_pos(c).ok_or(GeneralError::invalid_input_char(c))?;
            out.push(*right.get_char(n).unwrap()); // Error will be caught by previous line
            left_permute(&mut left, n);
            right_permute(&mut right, n);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod chaocipher_tests {
    // http://www.chaocipher.com/ActualChaocipher/Chaocipher-Revealed-Algorithm.pdf
    use super::*;

    const PTEXT: &'static str = "WELLDONEISBETTERTHANWELLSAID";
    const CTEXT: &'static str = "OAHQHCNYNXTSZJRRHJBYHQKSOUJY";

    #[test]
    fn encrypt_test() {
        let cipher = Chaocipher::default();
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = Chaocipher::default();
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
