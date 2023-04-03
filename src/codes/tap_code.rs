use itertools::Itertools;

use crate::{
    errors::Error,
    text_aux::{
        PresetAlphabet::{self, BasicLatinNoC, BasicLatinNoJ, BasicLatinNoQ},
        VecString,
    },
};

use super::Code;

pub struct TapCode {
    pub alphabet_string: String,
    grid: VecString,
    side_len: usize,
    pub symbol: char,
}

impl Default for TapCode {
    fn default() -> Self {
        Self {
            alphabet_string: String::from(BasicLatinNoC),
            grid: VecString::from(BasicLatinNoC),
            side_len: 5,
            symbol: '.',
        }
    }
}

impl TapCode {
    pub fn assign_alphabet(&mut self, mode: PresetAlphabet) {
        match mode {
            BasicLatinNoC | BasicLatinNoJ | BasicLatinNoQ => {
                self.alphabet_string = String::from(mode);
                self.grid = VecString::from(mode);
                self.side_len = (mode.len() as f64).sqrt().ceil() as usize;
            }
            _ => (),
        }
    }

    pub fn set_alphabet(&mut self) -> Result<(), Error> {
        let new_alpha_len = self.alphabet_string.chars().count();

        if new_alpha_len > 100 {
            return Err(Error::alphabet(
                "alphabet length currently limited to 100 characters",
            ));
        }

        self.grid = VecString::unique_from(&self.alphabet_string);
        self.side_len = (new_alpha_len as f64).sqrt().ceil() as usize;

        Ok(())
    }

    pub fn alphabet_len(&self) -> usize {
        self.grid.len()
    }

    fn char_to_position(&self, symbol: char) -> Result<(usize, usize), Error> {
        let num = self
            .grid
            .get_pos_of(symbol)
            .ok_or_else(|| Error::invalid_input_char(symbol))?;
        Ok((num / self.side_len, num % self.side_len))
    }

    pub fn show_grid(&self) -> String {
        let size = (self.side_len + 2) * (self.side_len + 1);
        let mut square = String::with_capacity(size);

        for (n, c) in self.grid.chars().enumerate() {
            if n % self.side_len == 0 {
                square.push_str(&format!("\n"));
            }
            square.push(c);
            square.push(' ');
        }
        square
    }
}

impl Code for TapCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut out = Vec::new();

        for c in text.chars() {
            let (row, col) = self.char_to_position(c)?;

            out.push(format!("{} {}", ".".repeat(row + 1), ".".repeat(col + 1)));
        }
        Ok(out.join("  "))
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut out = String::new();
        let pairs = text.split("  ");
        for pair in pairs {
            if let Some((row, col)) = pair.split(" ").collect_tuple() {
                let r = row.chars().count() - 1;
                let c = col.chars().count() - 1;
                let nth = r * self.side_len + c;
                if r >= self.side_len {
                    return Err(Error::Input(format!("Invalid code group {}", row)));
                }
                if c >= self.side_len {
                    return Err(Error::Input(format!("Invalid code group {}", col)));
                }
                out.push(self.alphabet_string.chars().nth(nth).unwrap())
            } else {
                return Err(Error::Input(format!(
                    "Unable to correctly segment code groups. Found pair {}",
                    pair
                )));
            }
        }
        Ok(out)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
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
    const CIPHERTEXT: &'static str = ". .  .. .  ... .";

    #[test]
    fn encode_test() {
        let cipher = TapCode::default();
        assert_eq!(cipher.encode(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decode_test() {
        let cipher = TapCode::default();
        assert_eq!(cipher.decode(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
