use crate::{errors::CodeError, traits::Code};
use itertools::Itertools;

pub struct BarbierCode {
    pub symbol: char,
}

impl Default for BarbierCode {
    fn default() -> Self {
        Self { symbol: '.' }
    }
}

impl BarbierCode {
    const SIDE_LEN: usize = 6;

    pub const GRID: [&'static str; 36] = [
        "a", "i", "o", "u", "é", "è", "an", "in", "on", "un", "eu", "ou", "b", "d", "g", "j", "v",
        "z", "p", "t", "q", "ch", "f", "s", "l", "m", "n", "r", "gn", "ll", "oi", "oin", "ian",
        "ien", "ion", "ieu",
    ];

    fn str_to_position(&self, symbol: &str) -> Result<(usize, usize), CodeError> {
        let num = Self::GRID
            .iter()
            .position(|x| x == &symbol)
            .ok_or_else(|| CodeError::invalid_input_group(symbol))?;
        Ok((num / Self::SIDE_LEN, num % Self::SIDE_LEN))
    }
}

impl Code for BarbierCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for s in text.split(" ") {
            let (row, col) = self.str_to_position(s)?;

            out.push(format!("{} {}", ".".repeat(row + 1), ".".repeat(col + 1)));
        }
        Ok(out.join("  "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = String::new();
        for pair in text.split("  ") {
            if let Some((row, col)) = pair.split(" ").collect_tuple() {
                let r = row.chars().count() - 1;
                let c = col.chars().count() - 1;
                let nth = r * Self::SIDE_LEN + c;
                if r >= Self::SIDE_LEN {
                    return Err(CodeError::Input(format!("Invalid code group {}", row)));
                }
                if c >= Self::SIDE_LEN {
                    return Err(CodeError::Input(format!("Invalid code group {}", col)));
                }
                out.push_str(*Self::GRID.iter().nth(nth).unwrap());
                out.push(' ');
            } else {
                return Err(CodeError::Input(format!(
                    "Unable to correctly segment code groups. Found pair {}",
                    pair
                )));
            }
        }
        out.pop();
        Ok(out)
    }
}

// impl fmt::Display for TapCode {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         let mut square = String::from("  ");
//         for xlab in self.labels.chars().take(Self::SIDE_LEN) {
//             square.push_str(&format!("{xlab} "))
//         }
//         for (n, c) in self.grid.chars().enumerate() {
//             if n % Self::SIDE_LEN == 0 {
//                 let ylab = self.labels.chars().nth(n / Self::SIDE_LEN).unwrap();
//                 square.push_str(&format!("\n{ylab} "));
//             }
//             square.push_str(&format!("{c} "))
//         }
//         write!(f, "{square}")
//     }
// }

#[cfg(test)]
mod barbier_tests {
    use super::*;

    const PLAINTEXT: &'static str = "é ou ien";
    const CODETEXT: &'static str = ". .....  .. ......  ...... ....";

    #[test]
    fn encode_test() {
        let cipher = BarbierCode::default();
        assert_eq!(cipher.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decode_test() {
        let cipher = BarbierCode::default();
        assert_eq!(cipher.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
