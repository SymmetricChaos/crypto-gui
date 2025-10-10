use utils::{preset_alphabet::Alphabet, text_functions::keyed_alphabet, vecstring::VecString};
use utils::errors::GeneralError;
use crate::{traits::Cipher};

#[derive(PartialEq, Eq)]
pub enum HuttonVersion {
    V1,
    V2,
}

pub struct Hutton {
    pub version: HuttonVersion,
    plain_alphabet: VecString,
    keyed_alpha: VecString,
    password: Vec<usize>,
}

impl Default for Hutton {
    fn default() -> Self {
        Self {
            version: HuttonVersion::V1,
            plain_alphabet: VecString::from(Alphabet::BasicLatin),
            keyed_alpha: VecString::from(Alphabet::BasicLatin),
            password: Default::default(),
        }
    }
}

impl Hutton {
    pub fn password_values_cycle(&self) -> std::iter::Cycle<std::slice::Iter<'_, usize>> {
        self.password.iter().cycle()
    }

    pub fn assign_key(&mut self, key: &str, alphabet: &str) {
        self.plain_alphabet = VecString::unique_from(alphabet);
        self.keyed_alpha = VecString::from(keyed_alphabet(key, &self.plain_alphabet.to_string()));
    }

    pub fn assign_password(&mut self, password: &str) {
        self.password = password
            .chars()
            .map(|c| self.plain_alphabet.get_pos(c).unwrap() + 1)
            .collect();
    }
}

impl Cipher for Hutton {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        // mutable alphabet for use while function runs
        let mut inner_alphabet = self.keyed_alpha.clone();

        let len = self.plain_alphabet.len();

        for (c, p) in text.chars().zip(self.password_values_cycle()) {
            // add the password number to the position of the character in the keyed alphabet
            let mut value = inner_alphabet.get_pos(c).unwrap() + p;
            // in Version 2 add the plain alphabet position of the first symbol in the keyed alphabet
            if self.version == HuttonVersion::V2 {
                value += self
                    .plain_alphabet
                    .get_pos(*inner_alphabet.get_char(0).unwrap())
                    .unwrap();
                value += 1;
            }
            // reduce modulo alphabet length and push the character at that position in the keyed alphabet to the ciphertext
            value %= len;
            out.push(*inner_alphabet.get_char(value).unwrap());

            inner_alphabet.swap_indicies(inner_alphabet.get_pos(c).unwrap(), value);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.len());
        let mut inner_alphabet = self.keyed_alpha.clone();
        let len = self.plain_alphabet.len();

        // this offset allows us to avoid dealing with negative numbers
        let offset = self.plain_alphabet.len() * 2;
        for (c, p) in text.chars().zip(self.password_values_cycle()) {
            let mut value = offset + inner_alphabet.get_pos(c).unwrap() - p;
            if self.version == HuttonVersion::V2 {
                value -= self
                    .plain_alphabet
                    .get_pos(*inner_alphabet.get_char(0).unwrap())
                    .unwrap();
                value -= 1;
            }
            value %= len;
            out.push(*inner_alphabet.get_char(value).unwrap());

            inner_alphabet.swap_indicies(inner_alphabet.get_pos(c).unwrap(), value);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod hutton_tests {
    use super::*;

    const PTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CTEXT_V1: &'static str = "ZLZJUPIMUKVJLFVFFBGZYVVDBVVANEPYEZB";
    const CTEXT_V2: &'static str = "KVQPFLIRUTGZEUZEHUNBIYMPBMHLTREMUQU";

    #[test]
    fn encrypt_test_v1() {
        let mut cipher = Hutton::default();
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE", Alphabet::BasicLatin.into());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT_V1);
    }

    #[test]
    fn decrypt_test_v1() {
        let mut cipher = Hutton::default();
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE", Alphabet::BasicLatin.into());
        assert_eq!(cipher.decrypt(CTEXT_V1).unwrap(), PTEXT);
    }

    #[test]
    fn encrypt_test_v2() {
        let mut cipher = Hutton::default();
        cipher.version = HuttonVersion::V2;
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE", Alphabet::BasicLatin.into());
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT_V2);
    }

    #[test]
    fn decrypt_test_v2() {
        let mut cipher = Hutton::default();
        cipher.version = HuttonVersion::V2;
        cipher.assign_password("VUVUZELAS");
        cipher.assign_key("OBSTACLE", Alphabet::BasicLatin.into());
        assert_eq!(cipher.decrypt(CTEXT_V2).unwrap(), PTEXT);
    }
}
