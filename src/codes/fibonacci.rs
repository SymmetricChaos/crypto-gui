use std::collections::HashMap;

use crate::errors::Error;

use super::Code;

// https://en.wikipedia.org/wiki/Fibonacci_coding
pub struct FibStr {
    vector: Vec<usize>,
    n: usize,
    cur_fib: usize,
    next_fib: usize,
}

impl FibStr {
    pub fn new() -> FibStr {
        let mut vector = Vec::with_capacity(10); //Should allocate enough space most of the time
        vector.push(1);
        let n = 1;
        let cur_fib = 1;
        let next_fib = 2;
        FibStr {
            vector,
            n,
            cur_fib,
            next_fib,
        }
    }
}

impl Iterator for FibStr {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // Go through the bits backward adding a 1 or 0 depending on if its part
        // of the partition
        let mut bits = String::with_capacity(self.vector.len() + 1);
        bits.push('1');
        let mut val = self.n;
        for f in self.vector.iter().rev() {
            if *f <= val {
                bits.push('1');
                val -= f;
            } else {
                bits.push('0')
            }
        }

        // Reverse the bits, collect them into a String
        let output = bits.chars().rev().collect::<String>();

        // Increment the counter and append the next fibonacci number if it has
        // been reached
        self.n += 1;
        if self.next_fib == self.n {
            self.vector.push(self.next_fib);
            let t = self.next_fib;
            self.next_fib += self.cur_fib;
            self.cur_fib = t;
        }

        Some(output)
    }
}

pub struct FibonacciCode {
    map: HashMap<char, String>,
    map_inv: HashMap<String, char>,
    pub alphabet: String,
    old_alphabet: String,
    max_code_len: usize,
}

impl FibonacciCode {
    // This needs to be called before encoding or decoding to be
    // sure that the maps are up to date. In the egui interface
    // this is taken care of by embedding it in the chars_codes()
    // method.
    // It would make more sense to put it in the control_alphabet()
    // method but that causes a panic due to interaction with
    // the chars_codes() method.
    pub fn set_maps(&mut self) {
        if self.alphabet != self.old_alphabet {
            let codes = FibStr::new();
            self.map.clear();
            self.map_inv.clear();
            for (l, c) in self.alphabet.chars().zip(codes) {
                self.map.insert(l, c.clone());
                self.map_inv.insert(c.clone(), l);
            }
            self.max_code_len = self.map[&self.alphabet.chars().last().unwrap()]
                .chars()
                .count();
            self.old_alphabet = self.alphabet.clone();
        }
    }

    pub fn new(alphabet: &str) -> Self {
        let codes = FibStr::new();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l, c) in alphabet.chars().zip(codes) {
            map.insert(l, c.clone());
            map_inv.insert(c, l);
        }
        FibonacciCode {
            map,
            map_inv,
            alphabet: alphabet.to_string(),
            old_alphabet: alphabet.to_string(),
            max_code_len: 8,
        }
    }

    pub fn chars_codes(&mut self) -> impl Iterator<Item = (char, &String)> + '_ {
        self.set_maps();
        self.alphabet
            .chars()
            .map(|x| (x, self.map.get(&x).unwrap()))
    }
}

impl Default for FibonacciCode {
    fn default() -> Self {
        Self::new("ETAOINSHRDLCUMWFGYPBVKJXQZ")
    }
}

impl Code for FibonacciCode {
    fn encode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        for s in text.chars() {
            output.push_str(&self.map[&s])
        }
        Ok(output)
    }

    fn decode(&self, text: &str) -> Result<String, Error> {
        let mut output = String::new();
        let mut buffer = String::with_capacity(self.max_code_len);
        let mut prev = '0';
        for b in text.chars() {
            buffer.push(b);
            if prev == '1' && b == '1' {
                match self.map_inv.get(&buffer) {
                    Some(s) => {
                        output.push(*s);
                        buffer.clear();
                        prev = '0';
                        continue;
                    }
                    None => (),
                }
            }
            prev = b;
        }
        Ok(output)
    }

    fn randomize(&mut self) {}

    fn reset(&mut self) {}
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    const ENCODEDTEXT: &'static str = "01100001111101000110000011000111010111000001101010111000111011010001110011001001110110010001101000011000001110000111001011010111011000000111110001101100001111001011001100010011000101101001110111010011";

    #[test]
    fn encrypt_test() {
        let code = FibonacciCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = FibonacciCode::default();
        assert_eq!(code.decode(ENCODEDTEXT).unwrap(), PLAINTEXT);
    }
}
