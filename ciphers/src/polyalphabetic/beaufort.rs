use std::collections::VecDeque;

use utils::{preset_alphabet::Alphabet, vecstring::VecString};

use super::PolyMode;
use crate::{errors::CipherError, traits::Cipher};

pub struct Beaufort {
    pub keywords: Vec<String>,
    alphabet: VecString,
    pub prog_shift: usize,
    pub mode: PolyMode,
    pub multikey: bool,
}

impl Default for Beaufort {
    fn default() -> Self {
        Self {
            keywords: vec![String::new(), String::new(), String::new()],
            alphabet: VecString::from(Alphabet::BasicLatin),
            mode: PolyMode::CylicKey,
            prog_shift: 0,
            multikey: false,
        }
    }
}

impl Beaufort {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet);
    }

    // Some weirdness needed to make types match
    pub fn key(&self) -> impl Iterator<Item = usize> + '_ {
        if self.multikey {
            let mut effective_key = vec![0usize; self.key_len()];
            for key in self.keywords.iter().filter(|s| !s.is_empty()) {
                for (pos, sym) in key.chars().cycle().take(self.key_len()).enumerate() {
                    let p = self.alphabet.get_pos(sym).unwrap();
                    effective_key[pos] += p
                }
            }
            effective_key = effective_key
                .into_iter()
                .map(|v| v % self.alphabet_len())
                .collect();
            effective_key.into_iter()
        } else {
            let key: Vec<usize> = self.keywords[0]
                .chars()
                .map(|x| self.alphabet.get_pos(x).unwrap())
                .collect();
            key.into_iter()
        }
    }

    pub fn cyclic_key(&self) -> impl Iterator<Item = usize> + '_ {
        let v = self.key().collect::<Vec<usize>>();
        v.into_iter().cycle()
    }

    pub fn key_len(&self) -> usize {
        if self.multikey {
            // Should multiply together ignoring common factors. [9,6] should give 18
            self.keywords
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().count())
                .fold(1, num::integer::lcm)
        } else {
            self.keywords[0].chars().count()
        }
    }

    // Unwrap justified by bounds on key
    pub fn keyword(&self) -> String {
        if self.multikey {
            self.key()
                .map(|v| self.alphabet.get_char(v).unwrap())
                .collect()
        } else {
            self.keywords[0].clone()
        }
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    fn validate_key(&self) -> Result<(), CipherError> {
        for key in self.keywords.iter() {
            for c in key.chars() {
                if !self.alphabet.contains(c) {
                    return Err(CipherError::invalid_alphabet_char(c));
                }
            }
        }
        Ok(())
    }

    fn validate_input(&self, text: &str) -> Result<(), CipherError> {
        if text.len() == 0 {
            return Err(CipherError::Input(String::from("No input text provided")));
        }
        for c in text.chars() {
            if !self.alphabet.contains(c) {
                return Err(CipherError::invalid_input_char(c));
            }
        }
        Ok(())
    }

    fn autokey_prep(
        &self,
        text: &str,
    ) -> Result<(Vec<usize>, VecDeque<usize>, String), CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        let text_nums: Vec<usize> = text
            .chars()
            .map(|x| self.alphabet.get_pos(x).unwrap())
            .collect();
        let akey: VecDeque<usize> = self.key().collect();
        let out = String::with_capacity(text_nums.len());

        Ok((text_nums, akey, out))
    }

    // The Beaufort cipher is reciprocal so no decrypt method is needed on a character per character basis
    fn encrypt_char(&self, t: usize, k: usize) -> char {
        *self.alphabet.get_char_offset(k, -(t as i32)).unwrap()
    }

    fn encrypt_cyclic(&self, text: &str) -> Result<String, CipherError> {
        let nums: Vec<usize> = text
            .chars()
            .map(|x| self.alphabet.get_pos(x).unwrap())
            .collect();
        let mut out = String::with_capacity(nums.len());
        for (n, k) in nums.iter().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(*n, k))
        }
        Ok(out)
    }

    fn decrypt_cyclic(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt_cyclic(text)
    }

    fn encrypt_auto(&self, text: &str) -> Result<String, CipherError> {
        let (text_nums, mut akey, mut out) = self.autokey_prep(text)?;

        for n in text_nums {
            akey.push_back(n);
            let k = akey.pop_front().unwrap();
            out.push(self.encrypt_char(n, k))
        }

        Ok(out)
    }

    fn decrypt_auto(&self, text: &str) -> Result<String, CipherError> {
        let (text_nums, mut akey, mut out) = self.autokey_prep(text)?;

        for n in text_nums {
            let k = akey.pop_front().unwrap();
            let ptxt_char = self.encrypt_char(n, k);
            out.push(ptxt_char);
            let new_key_val = self.alphabet.get_pos(ptxt_char).unwrap();
            akey.push_back(new_key_val);
        }
        Ok(out)
    }

    fn encrypt_prog(&self, text: &str) -> Result<String, CipherError> {
        let text_nums: Vec<usize> = text
            .chars()
            .map(|x| self.alphabet.get_pos(x).unwrap())
            .collect();
        let mut out = String::with_capacity(text_nums.len());

        let mut cur_shift = 0 as usize;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(*n, k + cur_shift));
            ctr = (ctr + 1) % key_len;
            if ctr == 0 {
                cur_shift += self.prog_shift;
            }
        }
        Ok(out)
    }

    fn decrypt_prog(&self, text: &str) -> Result<String, CipherError> {
        let alpha_len = self.alphabet_len();
        let text_nums: Vec<usize> = text
            .chars()
            .map(|x| self.alphabet.get_pos(x).unwrap())
            .collect();
        let mut out = String::with_capacity(text_nums.len());

        let mut cur_shift = 0;
        let mut ctr = 0;
        let key_len = self.key_len();

        for (n, k) in text_nums.iter().zip(self.cyclic_key()) {
            out.push(self.encrypt_char(*n, (k + cur_shift) % alpha_len));
            ctr = (ctr + 1) % key_len;
            if ctr == 0 {
                cur_shift = (cur_shift + self.prog_shift) % alpha_len;
            }
        }
        Ok(out)
    }
}

impl Cipher for Beaufort {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        match self.mode {
            PolyMode::CylicKey => self.encrypt_cyclic(text),
            PolyMode::Autokey => self.encrypt_auto(text),
            PolyMode::ProgKey => self.encrypt_prog(text),
        }
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate_key()?;
        self.validate_input(text)?;
        match self.mode {
            PolyMode::CylicKey => self.decrypt_cyclic(text),
            PolyMode::Autokey => self.decrypt_auto(text),
            PolyMode::ProgKey => self.decrypt_prog(text),
        }
    }
}

#[cfg(test)]
mod beaufort_tests {
    use super::*;

    const PLAINTEXT: &'static str =         "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGANDTHENSOMEMORETOMAKEALONGERPLAINTEXTFORTHISCIPHERTOUSE";
    const CIPHERTEXT_CYCLIC: &'static str = "LGYIVLGEMNZGLKFLFTSYKKUPRVGACCZRQUINRKJRLGQABABOHLAUCNUJCNBGCKWPCCQCAERUZZLFILQMFPGYHWFOWJ";
    const CIPHERTEXT_AUTO: &'static str =   "LGYIVLGEMCTIDPUFBHFZEZKKGQNIESPXBDNHRIHSUQWSOJRFQAUECMUIOGQGIIJVTADBUCHYKXJPGLMVLQHNCOUYKE";
    const CIPHERTEXT_PROG: &'static str =   "LGYIVLGEMQCJONIOIWYEQQAVXBMJLLIAZDRWDWVDXSCMNPQDWAPJRCMBUFTYUCOKXXLXVZMPXXJDGJOKDQHZIXGPXK";

    #[test]
    fn encrypt_test_cyclic() {
        let mut cipher = Beaufort::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCYPTION");

        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_CYCLIC);
    }

    #[test]
    fn decrypt_test_cyclic() {
        let mut cipher = Beaufort::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCYPTION");
        cipher.mode = PolyMode::CylicKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_CYCLIC).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_auto() {
        let mut cipher = Beaufort::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_AUTO);
    }

    #[test]
    fn decrypt_test_auto() {
        let mut cipher = Beaufort::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCYPTION");
        cipher.mode = PolyMode::Autokey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_AUTO).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_prog() {
        let mut cipher = Beaufort::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCYPTION");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_PROG);
    }

    #[test]
    fn decrypt_test_prog() {
        let mut cipher = Beaufort::default();
        cipher.keywords[1] = String::from("GOOD");
        cipher.keywords[0] = String::from("ENCYPTION");
        cipher.prog_shift = 3;
        cipher.mode = PolyMode::ProgKey;
        assert_eq!(cipher.decrypt(CIPHERTEXT_PROG).unwrap(), PLAINTEXT);
    }
}
