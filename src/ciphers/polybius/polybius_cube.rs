use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{shuffled_str, text_functions::validate_text, VecString},
};
use itertools::Itertools;
use num::Integer;

pub struct PolybiusCube {
    pub alphabet_string: String,
    grid: VecString,
    pub labels_string: String,
    labels: VecString,
    side_len: usize,
    pub key_word: String,
}

impl Default for PolybiusCube {
    fn default() -> Self {
        let alphabet = VecString::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        let labels = VecString::from("123456789");
        Self {
            alphabet_string: "ABCDEFGHIJKLMNOPQRSTUVWXYZ+".to_string(),
            grid: alphabet,
            labels_string: "123456789".to_string(),
            labels,
            side_len: 3,
            key_word: String::new(),
        }
    }
}

impl PolybiusCube {
    pub fn alphabet(&self) -> &str {
        &self.alphabet_string
    }

    pub fn assign_key(&mut self, key_word: &str) {
        self.key_word = key_word.to_string();
        self.set_key();
    }

    pub fn set_key(&mut self) {
        self.grid = VecString::keyed_alphabet(&self.key_word, &self.alphabet_string);
    }

    pub fn assign_labels(&mut self, labels: &str) {
        self.labels_string = labels.to_string();
        self.set_labels();
    }

    pub fn set_labels(&mut self) {
        self.labels = VecString::unique_from(&self.labels_string);
    }

    pub fn set_alphabet(&mut self) -> Result<(), Error> {
        let new_alpha_len = self.alphabet_string.chars().count();

        if new_alpha_len < 8 {
            return Err(Error::alphabet("alphabet length must be at least 8"));
        }

        if new_alpha_len > 125 {
            return Err(Error::alphabet(
                "alphabet length currently limited to 125 characters",
            ));
        }

        self.grid = VecString::from(&self.alphabet_string);
        self.side_len = (new_alpha_len as f64).cbrt().ceil() as usize;

        Ok(())
    }

    fn triplets(&self, text: &str) -> Result<Vec<(char, char, char)>, Error> {
        if text.chars().count() % 3 != 0 {
            dbg!(text);
            dbg!(text.chars().count());
            return Err(Error::input(
                "ciphertext length must be a multiple of three.",
            ));
        }
        let out = text
            .chars()
            .chunks(3)
            .into_iter()
            .map(|x| x.collect_tuple().unwrap())
            .collect();
        Ok(out)
    }

    pub fn alphabet_len(&self) -> usize {
        self.grid.len()
    }

    fn char_to_position(&self, symbol: char) -> (usize, usize, usize) {
        let num = self.grid.get_pos_of(symbol).unwrap();
        let l = self.side_len;
        let x = num / (l * l);
        let y = (num / l) % l;
        let z = num % l;
        (x, y, z)
    }

    fn position_to_char(&self, position: (char, char, char)) -> char {
        let x = self.labels.get_pos_of(position.0).unwrap();
        let y = self.labels.get_pos_of(position.1).unwrap();
        let z = self.labels.get_pos_of(position.2).unwrap();

        let l = self.side_len;
        let num = x * (l * l) + y * l + z;
        self.grid.get_char_at(num).unwrap()
    }

    fn check_labels(&self) -> Result<(), Error> {
        if self.labels.len() < self.side_len {
            return Err(Error::key("not enough labels for grid size"));
        }
        Ok(())
    }

    pub fn show_grids(&self) -> [String; 3] {
        let size = (self.side_len + 2) * (self.side_len + 1);
        let mut grids = [
            String::with_capacity(size),
            String::with_capacity(size),
            String::with_capacity(size),
        ];

        // We produce a vector of positions and characters.
        // The infinite cycle of blanks fills out empty spaces.
        // Without the cycle we'll can be missing contents for the last chunk and panic later on
        let blanks = " ".chars().cycle();
        let grid_idxs = self
            .grid
            .chars()
            .chain(blanks)
            .take(self.side_len.pow(3))
            .enumerate()
            .collect::<Vec<(usize, char)>>();
        let grid_chunks = grid_idxs
            .chunks(self.side_len * self.side_len)
            .collect_vec();

        for idx in 0..3 {
            // Append x-axis labels
            grids[idx].push_str("  ");
            for xlab in self.labels.chars().take(self.side_len) {
                grids[idx].push(xlab);
                grids[idx].push(' ');
            }

            // Append y-axis labels followed by rows
            for (n, c) in grid_chunks[idx] {
                if n % self.side_len == 0 {
                    let ylab = self
                        .labels
                        .get_char_at((n / self.side_len) % self.side_len)
                        .unwrap_or(' ');
                    grids[idx].push('\n');
                    grids[idx].push(ylab);
                    grids[idx].push(' ')
                }
                grids[idx].push(*c);
                grids[idx].push(' ');
            }
        }

        grids
    }
}

impl Cipher for PolybiusCube {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        self.check_labels()?;
        validate_text(text, &self.grid)?;

        let mut out = String::with_capacity(text.chars().count() * 3);

        for c in text.chars() {
            let pos = self.char_to_position(c);
            out.push(self.labels.get_char_at(pos.0).unwrap());
            out.push(self.labels.get_char_at(pos.1).unwrap());
            out.push(self.labels.get_char_at(pos.2).unwrap());
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.check_labels()?;
        validate_text(text, &self.labels)?;
        if !text.chars().count().is_multiple_of(&3) {
            return Err(Error::input(
                "Input text must have a length that is a multiple of three.",
            ));
        }

        let triplets = self.triplets(text)?;
        let mut out = String::with_capacity(text.chars().count() / 3);

        for p in triplets {
            out.push(self.position_to_char(p));
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        self.key_word = shuffled_str(&self.alphabet_string, &mut get_global_rng());
        self.set_key();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod polybius_cube_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICK";
    const CIPHERTEXT: &'static str = "122223121313322111212232";

    #[test]
    fn encrypt_test() {
        let mut cipher = PolybiusCube::default();
        cipher.assign_key("INVENTORY");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusCube::default();
        cipher.assign_key("INVENTORY");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
