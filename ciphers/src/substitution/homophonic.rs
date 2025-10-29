use crate::Cipher;
use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use utils::{errors::GeneralError, text_functions::string_chunks};

pub struct Homophonic {
    characters: Vec<char>,
    groups: Vec<Vec<String>>,
    nulls: Vec<String>,
    null_rate: f64,
    seed: u64,
}

impl Default for Homophonic {
    fn default() -> Self {
        let mut pairs = Vec::new();
        for i in utils::preset_alphabet::Alphabet::BasicLatin.chars() {
            for j in utils::preset_alphabet::Alphabet::BasicLatin.chars() {
                pairs.push(format!("{}{}", i, j));
            }
        }

        // Seeded for consistency, specific ordering doesn't matter.
        let mut rng = StdRng::seed_from_u64(347856);
        pairs.shuffle(&mut rng);

        let mut groups = Vec::new();
        let mut idx = 0;
        for i in [
            // this totals to 512
            40, 7, 15, 25, 60, 15, 10, 30, 35, 3, 3, 20, 15, 35, 35, 10, 3, 30, 30, 45, 15, 5, 10,
            3, 10, 3,
        ] {
            groups.push(pairs[idx..idx + i].to_vec());
            idx += i;
        }

        Self {
            characters: vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ],
            groups,
            nulls: pairs[idx..].to_vec(), // should have 164 elements
            null_rate: 0.5,
            seed: 0xBAD5EED0BAD5EED0,
        }
    }
}

impl Homophonic {
    pub fn set_groups(
        &mut self,
        characters: Vec<char>,
        groups_sizes: Vec<usize>,
        seed: u64,
    ) -> Result<(), GeneralError> {
        if groups_sizes.iter().sum::<usize>() > 676 {
            return Err(GeneralError::input("only 676 code groups can be assigned"));
        }

        self.characters = characters;

        let mut pairs = Vec::new();
        for i in utils::preset_alphabet::Alphabet::BasicLatin.chars() {
            for j in utils::preset_alphabet::Alphabet::BasicLatin.chars() {
                pairs.push(format!("{}{}", i, j));
            }
        }

        // Seeded for consistency, specific ordering doesn't matter.
        let mut rng = StdRng::seed_from_u64(seed);
        pairs.shuffle(&mut rng);

        self.groups.clear();
        self.nulls.clear();

        let mut idx = 0;
        for i in groups_sizes {
            self.groups.push(pairs[idx..idx + i].to_vec());
            idx += i;
        }

        self.nulls = pairs[idx..].to_vec();

        Ok(())
    }

    pub fn random_null(&self, rng: &mut StdRng) -> Result<&String, utils::errors::GeneralError> {
        self.nulls
            .iter()
            .choose(rng)
            .ok_or(utils::errors::GeneralError::state("no nulls assigned"))
    }
}

impl Cipher for Homophonic {
    fn encrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let mut out = String::new();
        let mut rng = StdRng::seed_from_u64(self.seed);
        for c in text.chars() {
            // Possibly insert a null
            if rng.gen_bool(self.null_rate) {
                out.push_str(self.random_null(&mut rng)?);
            }
            // Insert a random code group assigned to the character
            match self.characters.iter().position(|x| x == &c) {
                Some(n) => {
                    out.push_str(self.groups[n].iter().choose(&mut rng).ok_or(
                        utils::errors::GeneralError::state("no groups assigned to letter"),
                    )?);
                }
                None => return Err(utils::errors::GeneralError::invalid_alphabet_char(c)),
            }
        }
        // Allow a null at the end
        if rng.gen_bool(self.null_rate) {
            out.push_str(&self.random_null(&mut rng)?);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let mut out = String::new();

        for c in string_chunks(text, 2) {
            if !self.nulls.contains(&c) {
                for (n, group) in self.groups.iter().enumerate() {
                    if group.contains(&c) {
                        out.push(self.characters[n]);
                        continue;
                    }
                }
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let cipher = Homophonic::default();
        let ptext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOGTHEQUICKBROWNFOXJUMPSOVERTHELAZYDOGTHEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(ptext, cipher.decrypt(&ctext).unwrap())
    }
}
