use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{keyed_alphabet, shuffled_str, PresetAlphabet, VecString},
};

#[derive(PartialEq, Eq)]
pub enum HuttonVersion {
    V1,
    V2,
}

pub struct Hutton {
    pub version: HuttonVersion,
    pub alphabet_string: String,
    alphabet: VecString,
    pub key_string: String,
    keyed_alpha: VecString,
    pub password_string: String,
    password: Vec<usize>,
}

impl Default for Hutton {
    fn default() -> Self {
        Self {
            version: HuttonVersion::V1,
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            key_string: Default::default(),
            keyed_alpha: VecString::with_capacity(26),
            password_string: Default::default(),
            password: Default::default(),
        }
    }
}

impl Hutton {
    pub fn password_values_cycle(&self) -> std::iter::Cycle<std::slice::Iter<'_, usize>> {
        self.password.iter().cycle()
    }

    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet_string = alphabet.to_string();
        self.set_alphabet();
    }

    pub fn set_password(&mut self) {
        self.password = self
            .password_string
            .chars()
            .map(|c| self.alphabet.get_pos_of(c).unwrap() + 1)
            .collect();
    }

    pub fn assign_password(&mut self, password: &str) {
        self.password_string = password.to_string();
        self.set_password();
    }

    pub fn set_key(&mut self) {
        self.keyed_alpha = VecString::from(keyed_alphabet(&self.key_string, &self.alphabet_string));
    }

    pub fn assign_key(&mut self, key: &str) {
        self.key_string = key.to_string();
        self.set_key();
    }

    pub fn keyed_alphabet(&self) -> String {
        self.keyed_alpha.to_string()
    }
}

impl Cipher for Hutton {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len());
        // mutable alphabet for use while function runs
        let mut inner_alpha = self.keyed_alpha.clone();

        let len = self.alphabet.len();

        for (c, p) in text.chars().zip(self.password_values_cycle()) {
            // add the password number to the position of the character in the keyed alphabet
            let mut value = inner_alpha.get_pos(c).unwrap() + p;
            // in Version 2 add the plain alphabet position of the first symbol in the keyed alphabet
            if self.version == HuttonVersion::V2 {
                value += self
                    .alphabet
                    .get_pos_of(inner_alpha.get_char(0).unwrap())
                    .unwrap();
                value += 1;
            }
            // reduce modulo alphabet length and push the character at that position in the keyed alphabet to the ciphertext
            value %= len;
            out.push(inner_alpha.get_char(value).unwrap());

            inner_alpha.swap_indicies(inner_alpha.get_pos(c).unwrap(), value);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len());
        let mut inner_alphabet = self.keyed_alpha.clone();
        let len = self.alphabet.len();

        // this offset allows us to avoid dealing with negative numbers
        let offset = self.alphabet.len() * 2;
        for (c, p) in text.chars().zip(self.password_values_cycle()) {
            let mut value = offset + inner_alphabet.get_pos(c).unwrap() - p;
            if self.version == HuttonVersion::V2 {
                value -= self
                    .alphabet
                    .get_pos_of(inner_alphabet.get_char(0).unwrap())
                    .unwrap();
                value -= 1;
            }
            value %= len;
            out.push(inner_alphabet.get_char(value).unwrap());

            inner_alphabet.swap_indicies(inner_alphabet.get_pos(c).unwrap(), value);
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.assign_key(&shuffled_str(&self.alphabet_string, &mut get_global_rng()));
        self.assign_password(&shuffled_str(&self.alphabet_string, &mut get_global_rng()));
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod hutton_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT_V1: &'static str = "ZLZJUPIMUKVJLFVFFBGZYVVDBVVANEPYEZB";
    const CIPHERTEXT_V2: &'static str = "KVQPFLIRUTGZEUZEHUNBIYMPBMHLTREMUQU";

    #[test]
    fn encrypt_test_v1() {
        let mut cipher = Hutton::default();
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V1);
    }

    #[test]
    fn decrypt_test_v1() {
        let mut cipher = Hutton::default();
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE");
        assert_eq!(cipher.decrypt(CIPHERTEXT_V1).unwrap(), PLAINTEXT);
    }

    #[test]
    fn encrypt_test_v2() {
        let mut cipher = Hutton::default();
        cipher.version = HuttonVersion::V2;
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT_V2);
    }

    #[test]
    fn decrypt_test_v2() {
        let mut cipher = Hutton::default();
        cipher.version = HuttonVersion::V2;
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE");
        assert_eq!(cipher.decrypt(CIPHERTEXT_V2).unwrap(), PLAINTEXT);
    }
}
