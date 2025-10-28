use crate::Cipher;
use itertools::Itertools;
use rand::{
    rngs::{StdRng, ThreadRng},
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng, SeedableRng,
};

pub struct Homophonic {
    characters: Vec<char>,
    group_sizes: Vec<usize>,
    groups: Vec<Vec<String>>,
    nulls: Vec<String>,
    null_rate: f64,
}

impl Default for Homophonic {
    fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(347856);

        let pairs = {
            let mut p = Vec::new();
            for i in utils::preset_alphabet::Alphabet::BasicLatin.chars() {
                for j in utils::preset_alphabet::Alphabet::BasicLatin.chars() {
                    p.push(format!("{}{}", i, j));
                }
            }
            p
        };
        let mut x = pairs
            .into_iter()
            .map(|x| x.to_string())
            .into_iter()
            .collect_vec();
        x.shuffle(&mut rng);

        let mut groups = Vec::new();
        let mut idx = 0;
        for i in [
            // this totals to 512
            40, 7, 15, 25, 60, 15, 10, 30, 35, 3, 3, 20, 15, 35, 35, 10, 3, 30, 30, 45, 15, 5, 10,
            3, 10, 3,
        ] {
            groups.push(x[idx..idx + i].to_vec());
            idx += i;
        }

        Self {
            characters: vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ],
            group_sizes: vec![
                40, 7, 15, 25, 60, 15, 10, 30, 35, 3, 3, 20, 15, 35, 35, 10, 3, 30, 30, 45, 15, 5,
                10, 3, 10, 3,
            ],
            groups,
            nulls: x[idx..].to_vec(), // should have 164 elements
            null_rate: 0.5,
        }
    }
}

impl Homophonic {
    pub fn random_null(&self, rng: &mut ThreadRng) -> &String {
        &self.nulls.iter().choose(rng).expect("nulls was empty")
    }
}

impl Cipher for Homophonic {
    fn encrypt(&self, text: &str) -> Result<String, utils::errors::GeneralError> {
        let mut out = String::new();
        let mut rng = thread_rng();
        for c in text.chars() {
            // Possibly insert a null
            if rng.gen_bool(self.null_rate) {
                out.push_str(self.random_null(&mut rng));
            }
            // Insert a random code group assigned to the character
            match self.characters.iter().position(|x| x == &c) {
                Some(n) => {
                    out.push_str(
                        self.groups[n]
                            .iter()
                            .choose(&mut rng)
                            .expect("group was empty"),
                    );
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

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use rand::seq::SliceRandom;
    use utils::preset_alphabet::Alphabet;

    use super::*;

    #[test]
    fn shuffled() {
        let mut rng = thread_rng();
        let pairs = {
            let mut p = Vec::new();
            for i in Alphabet::BasicLatin.chars() {
                for j in Alphabet::BasicLatin.chars() {
                    p.push(format!("{}{}", i, j));
                }
            }
            p
        };
        let mut x = pairs
            .into_iter()
            .map(|x| x.to_string())
            .into_iter()
            .collect_vec();
        x.shuffle(&mut rng);
        let mut idx = 0;
        for i in [
            40, 7, 15, 25, 60, 15, 10, 30, 35, 3, 3, 20, 15, 35, 35, 10, 3, 30, 30, 45, 15, 5, 10,
            3, 10, 3,
        ] {
            println!("vec!{:?},", &x[idx..idx + i]);
            idx += i;
        }
        println!("\n\nvec!{:?},", &x[idx..]);
        // println!("{:?}", x);
    }

    #[test]
    fn encrypt_decrypt_test() {
        let cipher = Homophonic::default();
        let ptext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(ptext, cipher.decrypt(&ctext).unwrap())
    }
}
