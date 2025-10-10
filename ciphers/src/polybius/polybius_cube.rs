use crate::traits::Cipher;
use itertools::Itertools;
use utils::{errors::GeneralError, vecstring::VecString};

fn icbrt(n: usize) -> usize {
    (n as f64).cbrt().floor() as usize
}

fn is_cube(n: usize) -> bool {
    let rt = icbrt(n);
    rt * rt * rt == n
}

pub struct PolybiusCube {
    pub cube: VecString,
    pub labels: VecString,
    pub side_len: usize,
}

impl Default for PolybiusCube {
    fn default() -> Self {
        Self::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ+", "123").unwrap()
    }
}

impl PolybiusCube {
    pub fn new(cube: &str, labels: &str) -> Result<Self, GeneralError> {
        let cube = VecString::from(cube);
        if !is_cube(cube.len()) {
            return Err(GeneralError::input(
                "the square must have a square number of characters",
            ));
        }
        let side_len = icbrt(cube.len());
        let labels = VecString::from(labels);
        if labels.len() != side_len {
            return Err(GeneralError::input(
                "the number of label characters must be equal to the square root of the square size",
            ));
        }
        Ok(Self {
            cube,
            labels,
            side_len,
        })
    }

    pub fn assign_grid(&mut self, keyword: &str, alphabet: &str) {
        self.cube = VecString::keyed_alphabet(keyword, alphabet);
    }

    pub fn assign_labels(&mut self, labels: &str) {
        self.labels = VecString::unique_from(labels);
    }

    fn triplets(&self, text: &str) -> Result<Vec<(char, char, char)>, GeneralError> {
        if text.chars().count() % 3 != 0 {
            return Err(GeneralError::input(
                "ciphertext length must be a multiple of three",
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
        self.cube.len()
    }

    fn char_to_position(&self, symbol: char) -> (usize, usize, usize) {
        let num = self.cube.get_pos(symbol).unwrap();
        let l = self.side_len;
        let x = num / (l * l);
        let y = (num / l) % l;
        let z = num % l;
        (x, y, z)
    }

    fn position_to_char(&self, position: (char, char, char)) -> Result<char, GeneralError> {
        let x = self
            .labels
            .get_pos(position.0)
            .ok_or(GeneralError::invalid_input_char(position.0))?;
        let y = self
            .labels
            .get_pos(position.1)
            .ok_or(GeneralError::invalid_input_char(position.1))?;
        let z = self
            .labels
            .get_pos(position.2)
            .ok_or(GeneralError::invalid_input_char(position.2))?;

        let l = self.side_len;
        let num = x * (l * l) + y * l + z;
        Ok(*self.cube.get_char(num).unwrap())
    }

    fn check_labels(&self) -> Result<(), GeneralError> {
        if self.labels.len() < self.side_len {
            return Err(GeneralError::key("not enough labels for grid size"));
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
            .cube
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
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.check_labels()?;

        let mut out = String::with_capacity(text.chars().count() * 3);

        for c in text.chars() {
            let pos = self.char_to_position(c);
            out.push(
                *self
                    .labels
                    .get_char(pos.0)
                    .ok_or(GeneralError::invalid_input_char(c))?,
            );
            out.push(
                *self
                    .labels
                    .get_char(pos.1)
                    .ok_or(GeneralError::invalid_input_char(c))?,
            );
            out.push(
                *self
                    .labels
                    .get_char(pos.2)
                    .ok_or(GeneralError::invalid_input_char(c))?,
            );
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        self.check_labels()?;

        if text.len() == 0 {
            return Ok(String::new());
        }

        if text.chars().count() % 3 != 0 {
            return Err(GeneralError::input(
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

    const PTEXT: &'static str = "THEQUICK";
    const CTEXT: &'static str = "122223121313322111212232";

    #[test]
    fn encrypt_test() {
        let mut cipher = PolybiusCube::default();
        cipher.assign_grid("INVENTORY", "ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = PolybiusCube::default();
        cipher.assign_grid("INVENTORY", "ABCDEFGHIJKLMNOPQRSTUVWXYZ+");
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT);
    }
}
