use crate::{errors::CipherError, traits::Cipher};
use std::{iter::Cycle, slice::Iter};
use utils::{preset_alphabet::Alphabet, vecstring::VecString};

#[derive(Debug, PartialEq, Eq)]
pub enum QuagmireVersion {
    V1,
    V2,
    V3,
    V4,
}

pub struct Quagmire {
    pub version: QuagmireVersion,
    alphabet: VecString,
    pt_key: VecString,
    ct_key: VecString,
    ind_key: Vec<i32>,
    pub indicator: char,
}

impl Default for Quagmire {
    fn default() -> Quagmire {
        Self {
            version: QuagmireVersion::V1,
            alphabet: VecString::from(Alphabet::BasicLatin),
            pt_key: VecString::from(Alphabet::BasicLatin),
            ct_key: VecString::from(Alphabet::BasicLatin),
            ind_key: Vec::new(),
            indicator: 'A',
        }
    }
}

impl Quagmire {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(&alphabet);
    }

    pub fn show_alphabet(&self) -> String {
        self.alphabet.to_string()
    }

    pub fn assign_pt_key(&mut self, key: &str) {
        self.pt_key = VecString::keyed_alphabet(&key, &self.alphabet.to_string());
    }

    pub fn assign_ct_key(&mut self, key: &str) {
        self.ct_key = VecString::keyed_alphabet(&key, &self.alphabet.to_string());
    }

    pub fn ind_key(&self) -> &Vec<i32> {
        &self.ind_key
    }

    // Converts the ind_key_string into a vector of i32 that represents how
    // many spaces the ct_alphabet is rotated relative to its starting position
    pub fn assign_ind_key(&mut self, key: &str) -> Result<(), CipherError> {
        self.ind_key.clear();
        let ind_pos = self.indicator_position()? as i32;
        let len = self.alphabet.len() as i32;
        let ct = match self.version {
            QuagmireVersion::V1 => &self.alphabet,
            QuagmireVersion::V2 => &self.pt_key,
            QuagmireVersion::V3 => &self.pt_key,
            QuagmireVersion::V4 => &self.ct_key,
        };
        for c in key.chars() {
            let sh =
                len + ind_pos - (ct.get_pos(c).ok_or(CipherError::invalid_key_char(c))? as i32);
            self.ind_key.push(sh % len)
        }
        Ok(())
    }

    pub fn indicator_position(&self) -> Result<usize, CipherError> {
        match self.version {
            QuagmireVersion::V2 => self
                .alphabet
                .get_pos(self.indicator)
                .ok_or(CipherError::Key(format!(
                    "invalid indicator character `{}`",
                    self.indicator
                ))),
            _ => self
                .pt_key
                .get_pos(self.indicator)
                .ok_or(CipherError::Key(format!(
                    "invalid indicator character `{}`",
                    self.indicator
                ))),
        }
    }

    pub fn indicator_cyclic_key(&self) -> Cycle<Iter<'_, i32>> {
        self.ind_key.iter().cycle()
    }
}

impl Cipher for Quagmire {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let (pt, ct) = match self.version {
            QuagmireVersion::V1 => (&self.pt_key, &self.alphabet),
            QuagmireVersion::V2 => (&self.alphabet, &self.pt_key),
            QuagmireVersion::V3 => (&self.pt_key, &self.pt_key),
            QuagmireVersion::V4 => (&self.pt_key, &self.ct_key),
        };
        let ind_key = self.indicator_cyclic_key();
        let mut out = String::with_capacity(text.len());
        for (c, k) in text.chars().zip(ind_key) {
            let p = pt.get_pos(c).unwrap();
            let new_c = *ct.get_char_offset(p, -*k).unwrap();
            out.push(new_c);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
        let (ct, pt) = match self.version {
            QuagmireVersion::V1 => (&self.pt_key, &self.alphabet),
            QuagmireVersion::V2 => (&self.alphabet, &self.pt_key),
            QuagmireVersion::V3 => (&self.pt_key, &self.pt_key),
            QuagmireVersion::V4 => (&self.pt_key, &self.ct_key),
        };
        let ind_key = self.indicator_cyclic_key();
        let mut out = String::with_capacity(text.len());
        for (c, k) in text.chars().zip(ind_key) {
            let p = pt.get_pos(c).unwrap();
            let new_c = *ct.get_char_offset(p, *k).unwrap();
            out.push(new_c);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod quagmire_tests {
    use super::*;

    const PLAINTEXT: &'static str = "DONTLETANYONE";
    const CIPHERTEXT_V1: &'static str = "HIFUFCIRFKUYK";
    const CIPHERTEXT_V2: &'static str = "RMGXKEVLGUQQN";
    const CIPHERTEXT_V3: &'static str = "FXDIEOGNDBZII";
    const CIPHERTEXT_V4: &'static str = "KFBIFICEWQVII";

    #[test]
    fn encrypt_test_v1() {
        let mut cipher = Quagmire::default();
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V1);
    }

    #[test]
    fn decrypt_test_v1() {
        let mut cipher = Quagmire::default();
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT_V1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v2() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V2;
        cipher.indicator = 'C';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V2);
    }

    #[test]
    fn decrypt_test_v2() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V2;
        cipher.indicator = 'C';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT_V2).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v3() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V3;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V3);
    }

    #[test]
    fn decrypt_test_v3() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V3;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT_V3).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v4() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V4;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ct_key("BRANDT");
        cipher.assign_ind_key("COUNTRY").unwrap();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V4);
    }

    #[test]
    fn decrypt_test_v4() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V4;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ct_key("BRANDT");
        cipher.assign_ind_key("COUNTRY").unwrap();
        assert_eq!(cipher.decrypt(CIPHERTEXT_V4).unwrap(), PLAINTEXT);
    }
}
