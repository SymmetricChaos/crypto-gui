use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use num::integer::Roots;
use utils::{functions::string_chunks, vecstring::VecString};

use super::PolybiusSquare;

pub struct Nihilist {
    pub polybius: PolybiusSquare,
    keyword: Vec<usize>,
}

impl Default for Nihilist {
    fn default() -> Self {
        Self {
            polybius: PolybiusSquare::default(),
            keyword: Vec::new(),
        }
    }
}

impl Nihilist {
    pub fn assign_keys(
        &mut self,
        polybius_keyword: &str,
        additive_keyword: &str,
        alphabet: &str,
    ) -> Result<(), CipherError> {
        self.polybius.square = VecString::keyed_alphabet(polybius_keyword, alphabet);
        self.polybius.side_len = alphabet.chars().count().sqrt();
        self.keyword = string_chunks(&self.polybius.encrypt(additive_keyword)?, 2)
            .into_iter()
            .map(|s| usize::from_str_radix(&s, 10).unwrap())
            .collect();
        Ok(())
    }
}

impl Cipher for Nihilist {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut out = String::with_capacity(text.chars().count() * 2);

        let polybius_encrypt = self.polybius.encrypt(text)?;

        for (n, key_n) in polybius_encrypt
            .chars()
            .chunks(2)
            .into_iter()
            .map(|chunk| usize::from_str_radix(&chunk.collect::<String>(), 10).unwrap())
            .zip(self.keyword.iter().cycle())
        {
            out.push_str(&format!("{}", n + key_n))
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut temp = String::with_capacity(text.len() / 3);
        for (group, key_n) in text.split(' ').zip(self.keyword.iter().cycle()) {
            let n = usize::from_str_radix(group, 10)
                .map_err(|_| CipherError::invalid_input_group(group))?;
            let t = n
                .checked_sub(*key_n)
                .ok_or(CipherError::input("invalid subtraction occured"))?;
            temp.push_str(&format!("{}", t));
        }
        self.polybius.decrypt(&temp)
    }
}

#[cfg(test)]
mod nihilist_tests {
    use utils::preset_alphabet::Alphabet;

    use super::*;

    // Note Q replaced by K
    const PLAINTEXT: &'static str = "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str =
        "4423153145241331124235523421355325453341433551154244231532115554143522";

    #[test]
    fn encrypt_test() {
        let mut cipher = Nihilist::default();
        cipher.assign_keys("INVENTORY", "RUSSIAN", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Nihilist::default();
        cipher.assign_keys("INVENTORY", "RUSSIAN", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
