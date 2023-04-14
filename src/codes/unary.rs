use super::Code;
use crate::errors::Error;
use bimap::BiMap;

pub struct UnaryCode {
    map: BiMap<char, String>,
    pub alphabet: String,
}

impl UnaryCode {
    pub fn set_map(&mut self) {
        let mut code = String::from('0');
        self.map.clear();
        for c in self.alphabet.chars() {
            self.map.insert(c, code.clone());
            code = format!("1{code}");
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.alphabet
            .chars()
            .map(|x| (x, self.map.get_by_left(&x).unwrap()))
    }
}

impl Default for UnaryCode {
    fn default() -> Self {
        let alphabet = "ETAOINSHRDLCUMWFGYPBVKJXQZ";
        let mut code = String::from("0");
        let mut map = BiMap::new();
        for c in alphabet.chars() {
            map.insert(c, code.clone());
            code = format!("1{code}");
        }
        UnaryCode {
            map,
            alphabet: alphabet.to_string(),
        }
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        for s in text.chars() {
            output.push_str(
                &self
                    .map
                    .get_by_left(&s)
                    .ok_or(Error::invalid_input_char(s))?,
            )
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.map.len());
        for b in text.chars() {
            buffer.push(b);
            if b == '0' {
                match self.map.get_by_right(&buffer) {
                    Some(s) => {
                        output.push(*s);
                        buffer.clear();
                    }
                    None => {
                        output.push('�');
                        buffer.clear();
                    }
                }
            }
        }
        Ok(output)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod unary_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "1011111110011111111111111111111111101111111111110111101111111111101111111111111111111110111111111111111111101111111101110111111111111110111110111111111111111011101111111111111111111111101111111111111111111111011111111111101111111111111011111111111111111101111110111011111111111111111111001111111101011111110011111111110110111111111111111111111111101111111111111111101111111110111011111111111111110";

    #[test]
    fn encrypt_test() {
        let code = UnaryCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = UnaryCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
