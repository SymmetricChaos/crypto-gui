use std::collections::HashMap;

use crate::errors::Error;

use super::Code;
pub struct UnaryCode {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    pub alphabet: String,
    old_alphabet: String,
}

impl UnaryCode {
    pub fn set_maps(&mut self) {
        if self.alphabet != self.old_alphabet {
            let mut code = String::from("0");
            self.map.clear();
            self.map_inv.clear();
            for c in self.alphabet.chars() {
                self.map.insert(c, code.clone());
                self.map_inv.insert(code.clone(), c);
                code = format!("1{code}");
            }
            self.old_alphabet = self.alphabet.clone();
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.set_maps();
        self.alphabet
            .chars()
            .map(|x| (x, self.map.get(&x).unwrap()))
    }
}

impl Default for UnaryCode {
    fn default() -> Self {
        let alphabet = "ETAOINSHRDLCUMWFGYPBVKJXQZ";
        let mut code = String::from("0");
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for c in alphabet.chars() {
            map.insert(c, code.clone());
            map_inv.insert(code.clone(), c);
            code = format!("1{code}");
        }
        UnaryCode {
            map,
            map_inv,
            alphabet: alphabet.to_string(),
            old_alphabet: alphabet.to_string(),
        }
    }
}

impl Code for UnaryCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        for s in text.chars() {
            output.push_str(&self.map[&s])
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.map.len());
        for b in text.chars() {
            buffer.push(b);
            if b == '0' {
                match self.map_inv.get(&buffer) {
                    Some(s) => {
                        output.push(*s);
                        buffer.clear();
                        continue;
                    }
                    None => (),
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
