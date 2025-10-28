use crate::Cipher;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Homophonic {
    characters: Vec<char>,
    groups: Vec<String>,
    nulls: Vec<String>,
    null_rate: f64,
}

impl Default for Homophonic {
    fn default() -> Self {
        Self {
            characters: Vec::default(),
            groups: Vec::default(),
            nulls: Vec::default(),
            null_rate: 0.05,
        }
    }
}

impl Homophonic {
    pub fn random_null(&self, rng: &mut ThreadRng) -> &String {
        &self.nulls[rng.gen_range(0..self.nulls.len())]
    }
}

impl Cipher for Homophonic {
    fn encrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let mut out = String::new();
        let mut rng = thread_rng();
        for c in text.chars() {
            // Possibly insert a random null
            if rng.gen_bool(self.null_rate) {
                out.push_str(self.random_null(&mut rng));
            }
            // Insert a random code group assigned to the character
            match self.characters.iter().position(|x| x == &c) {
                Some(n) => {
                    out.push_str(todo!("position"));
                }
                None => return Err(utils::errors::GeneralError::invalid_alphabet_char(c)),
            }
        }
        // Allow a null at the end
        if rng.gen_bool(self.null_rate) {
            out.push_str(&self.random_null(&mut rng));
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        todo!()
    }
}
