use super::PolybiusSquare;
use crate::traits::Cipher;
use num::integer::Roots;
use utils::{errors::GeneralError, vecstring::VecString};

pub struct Nihilist {
    pub polybius: PolybiusSquare,
    keyword: Vec<usize>,
}

impl Default for Nihilist {
    fn default() -> Self {
        let mut polybius = PolybiusSquare::default();
        polybius.spaced = true;
        Self {
            polybius,
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
    ) -> Result<(), GeneralError> {
        self.polybius.square = VecString::keyed_alphabet(polybius_keyword, alphabet);
        self.polybius.side_len = alphabet.chars().count().sqrt();
        self.keyword = self
            .polybius
            .encrypt(additive_keyword)
            .or(Err(GeneralError::input("invalid additive key")))?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| usize::from_str_radix(&s, 10).unwrap())
            .collect();
        Ok(())
    }

    pub fn keyword_vec(&self) -> &Vec<usize> {
        &self.keyword
    }
}

impl Cipher for Nihilist {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::with_capacity(text.chars().count() * 2);

        let polybius_encrypt = self.polybius.encrypt(text)?;

        for (n, key_n) in polybius_encrypt
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .zip(self.keyword.iter().cycle())
        {
            out.push_str(&format!("{} ", n + key_n))
        }
        out.pop();
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let mut temp = String::with_capacity(text.len() / 3);
        for (group, key_n) in text
            .split(' ')
            .filter(|s| !s.is_empty())
            .zip(self.keyword.iter().cycle())
        {
            let n = usize::from_str_radix(group, 10)
                .map_err(|_| GeneralError::invalid_input_group(group))?;
            let t = n
                .checked_sub(*key_n)
                .ok_or(GeneralError::input("invalid subtraction occured"))?;
            temp.push_str(&format!("{} ", t));
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
    const CIPHERTEXT: &'static str = "37 87 65 93 63 35 43 64 77 73 72 64 36 45 43 106 92 103 55 69 63 43 65 65 73 26 59 26 65 76 106 74 43 45 46";

    #[test]
    fn encrypt_test() {
        let mut cipher = Nihilist::default();
        _ = cipher.assign_keys("INVENTORY", "RUSSIAN", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Nihilist::default();
        _ = cipher.assign_keys("INVENTORY", "RUSSIAN", Alphabet::BasicLatinNoQ.into());
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
