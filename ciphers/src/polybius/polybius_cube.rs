use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use utils::vecstring::VecString;

pub struct PolybiusCube {
    pub grid: VecString,
    pub labels: VecString,
    pub side_len: usize,
}

impl Default for PolybiusCube {
    fn default() -> Self {
        let grid = VecString::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        let labels = VecString::from("123456789");
        Self {
            grid,
            labels,
            side_len: 3,
        }
    }
}

impl PolybiusCube {
    pub fn assign_grid(&mut self, keyword: &str, alphabet: &str) {
        self.grid = VecString::keyed_alphabet(keyword, alphabet);
    }

    pub fn assign_labels(&mut self, labels: &str) {
        self.labels = VecString::unique_from(labels);
    }

    fn triplets(&self, text: &str) -> Result<Vec<(char, char, char)>, CipherError> {
        if text.chars().count() % 3 != 0 {
            return Err(CipherError::input(
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
        let num = self.grid.get_pos(symbol).unwrap();
        let l = self.side_len;
        let x = num / (l * l);
        let y = (num / l) % l;
        let z = num % l;
        (x, y, z)
    }

    fn position_to_char(&self, position: (char, char, char)) -> Result<char, CipherError> {
        let x = self
            .labels
            .get_pos(position.0)
            .ok_or(CipherError::invalid_input_char(position.0))?;
        let y = self
            .labels
            .get_pos(position.1)
            .ok_or(CipherError::invalid_input_char(position.1))?;
        let z = self
            .labels
            .get_pos(position.2)
            .ok_or(CipherError::invalid_input_char(position.2))?;

        let l = self.side_len;
        let num = x * (l * l) + y * l + z;
        Ok(*self.grid.get_char(num).unwrap())
    }

    fn check_labels(&self) -> Result<(), CipherError> {
        if self.labels.len() < self.side_len {
            return Err(CipherError::key("not enough labels for grid size"));
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
                        .get_char((n / self.side_len) % self.side_len)
                        .unwrap_or(&' ');
                    grids[idx].push('\n');
                    grids[idx].push(*ylab);
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
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_labels()?;

        let mut out = String::with_capacity(text.chars().count() * 3);

        for c in text.chars() {
            let pos = self.char_to_position(c);
            out.push(
                *self
                    .labels
                    .get_char(pos.0)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
            out.push(
                *self
                    .labels
                    .get_char(pos.1)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
            out.push(
                *self
                    .labels
                    .get_char(pos.2)
                    .ok_or(CipherError::invalid_input_char(c))?,
            );
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.check_labels()?;

        if text.len() == 0 {
            return Ok(String::new());
        }

        if text.chars().count() % 3 != 0 {
            return Err(CipherError::input(
                "Input text must have a length that is a multiple of three.",
            ));
        }

        let triplets = self.triplets(text)?;
        let mut out = String::with_capacity(text.chars().count() / 3);

        for p in triplets {
            out.push(self.position_to_char(p)?);
        }
        Ok(out)
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
        cipher.assign_grid("INVENTORY", "ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusCube::default();
        cipher.assign_grid("INVENTORY", "ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
