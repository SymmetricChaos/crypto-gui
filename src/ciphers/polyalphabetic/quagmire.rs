use std::{iter::Cycle, slice::Iter};

use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{random_sample_replace, PresetAlphabet, VecString},
};

#[derive(Debug, PartialEq, Eq)]
pub enum QuagmireVersion {
    V1,
    V2,
    V3,
    V4,
}

pub struct Quagmire {
    pub version: QuagmireVersion,
    pub alphabet_string: String,
    alphabet: VecString,
    pub pt_key_string: String,
    pt_key: VecString,
    pub ct_key_string: String,
    ct_key: VecString,
    pub ind_key_string: String,
    ind_key: Vec<i32>,
    pub indicator: char,
}

impl Default for Quagmire {
    fn default() -> Quagmire {
        Self {
            version: QuagmireVersion::V1,
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            pt_key_string: String::from(PresetAlphabet::BasicLatin),
            pt_key: VecString::from(PresetAlphabet::BasicLatin),
            ct_key_string: String::from(PresetAlphabet::BasicLatin),
            ct_key: VecString::from(PresetAlphabet::BasicLatin),
            ind_key_string: String::new(),
            ind_key: Vec::new(),
            indicator: 'A',
        }
    }
}

impl Quagmire {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet_string = alphabet.to_string();
        self.set_alphabet();
    }

    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn show_alphabet(&self) -> String {
        self.alphabet.to_string()
    }

    pub fn assign_pt_key(&mut self, key: &str) {
        self.pt_key_string = key.to_string();
        self.set_pt_key();
    }

    pub fn set_pt_key(&mut self) {
        self.pt_key = VecString::keyed_alphabet(&self.pt_key_string, &self.alphabet_string);
    }

    pub fn show_pt_key(&self) -> String {
        self.pt_key.to_string()
    }

    pub fn assign_ct_key(&mut self, key: &str) {
        self.ct_key_string = key.to_string();
        self.set_ct_key();
    }

    pub fn set_ct_key(&mut self) {
        self.ct_key = VecString::keyed_alphabet(&self.ct_key_string, &self.alphabet_string);
    }

    pub fn show_ct_key(&self) -> String {
        self.ct_key.to_string()
    }

    pub fn assign_ind_key(&mut self, key: &str) {
        self.ind_key_string = key.to_string();
        self.set_ind_key();
    }

    // Converts the ind_key_string into a vector of usize that represent how
    // many spaces the ct_alphabet is rotated relative to its starting position
    pub fn set_ind_key(&mut self) {
        self.ind_key.clear();
        let ind_pos = self.indicator_position() as i32;
        let len = self.alphabet.len() as i32;
        let ct = match self.version {
            QuagmireVersion::V1 => &self.alphabet,
            QuagmireVersion::V2 => &self.pt_key,
            QuagmireVersion::V3 => &self.pt_key,
            QuagmireVersion::V4 => &self.ct_key,
        };
        for c in self.ind_key_string.chars() {
            let sh = len + ind_pos
                - (ct
                    .get_pos_of(c)
                    .expect(&format!("unknown character `{}` in indicator key", c))
                    as i32);
            self.ind_key.push(sh % len)
        }
    }

    pub fn indicator_position(&self) -> usize {
        match self.version {
            QuagmireVersion::V2 => self
                .alphabet
                .get_pos_of(self.indicator)
                .expect(&format!("invalid indicator character `{}`", self.indicator)),
            _ => self
                .pt_key
                .get_pos_of(self.indicator)
                .expect(&format!("invalid indicator character `{}`", self.indicator)),
        }
    }

    pub fn indicator_cyclic_key(&self) -> Cycle<Iter<'_, i32>> {
        self.ind_key.iter().cycle()
    }
}

impl Cipher for Quagmire {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let (pt, ct) = match self.version {
            QuagmireVersion::V1 => (&self.pt_key, &self.alphabet),
            QuagmireVersion::V2 => (&self.alphabet, &self.pt_key),
            QuagmireVersion::V3 => (&self.pt_key, &self.pt_key),
            QuagmireVersion::V4 => (&self.pt_key, &self.ct_key),
        };
        let ind_key = self.indicator_cyclic_key();
        let mut out = String::with_capacity(text.len());
        for (c, k) in text.chars().zip(ind_key) {
            let p = pt.get_pos_of(c).unwrap();
            let new_c = ct.get_char_offset(p, -*k).unwrap();
            out.push(new_c);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::errors::Error> {
        let (ct, pt) = match self.version {
            QuagmireVersion::V1 => (&self.pt_key, &self.alphabet),
            QuagmireVersion::V2 => (&self.alphabet, &self.pt_key),
            QuagmireVersion::V3 => (&self.pt_key, &self.pt_key),
            QuagmireVersion::V4 => (&self.pt_key, &self.ct_key),
        };
        let ind_key = self.indicator_cyclic_key();
        let mut out = String::with_capacity(text.len());
        for (c, k) in text.chars().zip(ind_key) {
            let p = pt.get_pos_of(c).unwrap();
            let new_c = ct.get_char_offset(p, *k).unwrap();
            out.push(new_c);
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        let rng = &mut get_global_rng();
        self.assign_ct_key(&random_sample_replace(&self.alphabet_string, 9, rng));
        self.assign_pt_key(&random_sample_replace(&self.alphabet_string, 9, rng));
        self.assign_ind_key(&random_sample_replace(&self.alphabet_string, 9, rng));
        self.indicator = self.alphabet.get_rand_char(rng);
    }

    fn reset(&mut self) {
        *self = Self::default()
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
        cipher.assign_ind_key("BRANDT");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V1);
    }

    #[test]
    fn decrypt_test_v1() {
        let mut cipher = Quagmire::default();
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT");
        assert_eq!(cipher.decrypt(CIPHERTEXT_V1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v2() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V2;
        cipher.indicator = 'C';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V2);
    }

    #[test]
    fn decrypt_test_v2() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V2;
        cipher.indicator = 'C';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT");
        assert_eq!(cipher.decrypt(CIPHERTEXT_V2).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v3() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V3;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V3);
    }

    #[test]
    fn decrypt_test_v3() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V3;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ind_key("BRANDT");
        assert_eq!(cipher.decrypt(CIPHERTEXT_V3).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v4() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V4;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ct_key("BRANDT");
        cipher.assign_ind_key("COUNTRY");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V4);
    }

    #[test]
    fn decrypt_test_v4() {
        let mut cipher = Quagmire::default();
        cipher.version = QuagmireVersion::V4;
        cipher.indicator = 'P';
        cipher.assign_pt_key("PAULBRANDT");
        cipher.assign_ct_key("BRANDT");
        cipher.assign_ind_key("COUNTRY");
        assert_eq!(cipher.decrypt(CIPHERTEXT_V4).unwrap(), PLAINTEXT);
    }
}
