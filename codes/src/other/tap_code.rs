use crate::traits::Code;
use itertools::Itertools;
use utils::{errors::GeneralError, preset_alphabet::Alphabet};

pub struct TapCode {
    pub grid: Vec<char>,
    side_len: usize,
    pub symbol: char,
}

impl Default for TapCode {
    fn default() -> Self {
        Self {
            grid: Alphabet::BasicLatinNoC.chars().collect_vec(),
            side_len: 5,
            symbol: '.',
        }
    }
}

impl TapCode {
    pub fn set_alphabet(&mut self, alphabet_string: &str) {
        let new_alpha_len = alphabet_string.chars().count();

        self.grid = alphabet_string.chars().unique().collect();
        self.side_len = (new_alpha_len as f64).sqrt().ceil() as usize;
    }

    pub fn alphabet_len(&self) -> usize {
        self.grid.len()
    }

    fn char_to_position(&self, symbol: char) -> Result<(usize, usize), GeneralError> {
        let num = self
            .grid
            .iter()
            .position(|x| x == &symbol)
            .ok_or_else(|| GeneralError::invalid_input_char(symbol))?;
        Ok((num / self.side_len, num % self.side_len))
    }

    pub fn show_grid(&self) -> String {
        let size = (self.side_len + 2) * (self.side_len + 1);
        let mut square = String::with_capacity(size);

        for (n, c) in self.grid.iter().enumerate() {
            if n % self.side_len == 0 {
                square.push_str(&format!("\n"));
            }
            square.push(*c);
            square.push(' ');
        }
        square
    }
}

impl Code for TapCode {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = Vec::new();

        for c in text.chars() {
            let (row, col) = self.char_to_position(c)?;

            out.push(format!("{} {}", ".".repeat(row + 1), ".".repeat(col + 1)));
        }
        Ok(out.join("  "))
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut out = String::new();
        let pairs = text.split("  ");
        for pair in pairs {
            if let Some((row, col)) = pair.split(" ").collect_tuple() {
                let r = row.chars().count() - 1;
                let c = col.chars().count() - 1;
                let nth = r * self.side_len + c;
                if r >= self.side_len {
                    return Err(GeneralError::input(format!("Invalid code group {}", row)));
                }
                if c >= self.side_len {
                    return Err(GeneralError::input(format!("Invalid code group {}", col)));
                }
                out.push(*self.grid.iter().nth(nth).unwrap())
            } else {
                return Err(GeneralError::input(format!(
                    "Unable to correctly segment code groups. Found pair {}",
                    pair
                )));
            }
        }
        Ok(out)
    }
}

// impl fmt::Display for TapCode {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         let mut square = String::from("  ");
//         for xlab in self.labels.chars().take(self.side_len) {
//             square.push_str(&format!("{xlab} "))
//         }
//         for (n, c) in self.grid.chars().enumerate() {
//             if n % self.side_len == 0 {
//                 let ylab = self.labels.chars().nth(n / self.side_len).unwrap();
//                 square.push_str(&format!("\n{ylab} "));
//             }
//             square.push_str(&format!("{c} "))
//         }
//         write!(f, "{square}")
//     }
// }

#[cfg(test)]
mod polybius_tests {
    use super::*;

    // Note Q replaced by K
    const PLAINTEXT: &'static str = "AFL";
    const CODETEXT: &'static str = ". .  .. .  ... .";

    #[test]
    fn encode_test() {
        let cipher = TapCode::default();
        assert_eq!(cipher.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decode_test() {
        let cipher = TapCode::default();
        assert_eq!(cipher.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
