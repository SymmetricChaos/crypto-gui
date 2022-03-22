use std::collections::HashMap;

use crate::errors::CodeError;

use super::Code;

// https://en.wikipedia.org/wiki/Fibonacci_coding
pub struct FibStr {
    vector: Vec<usize>,
    n: usize,
    cur_fib: usize,
    next_fib: usize,
}
 
impl FibStr {
    pub fn new() -> FibStr  {
        let mut vector = Vec::with_capacity(30); //Should allocate enough space most of the time
        vector.push(1);
        let n = 1;
        let cur_fib = 1;
        let next_fib = 2;
        FibStr{ vector, n, cur_fib, next_fib }
    }
}
 
impl Iterator for FibStr {
    type Item = String;
 
    fn next(&mut self) -> Option<String> {
 
        // Go through the bits backward adding a 1 or 0 depending on if its part
        // of the partition
        let mut bits = String::with_capacity(self.vector.len()+1);
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
 
        // Reverse the bits, collect them into a String, and append a 1
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
    alphabet: String,
}
 
impl FibonacciCode {

    // pub fn control_alphabet(&mut self) -> &mut String {
    //     let codes = FibStr::new();
    //     let mut map = HashMap::new();
    //     let mut map_inv = HashMap::new();
    //     for (l,c) in self.alphabet.chars().zip(codes) {
    //         map.insert(l,c.clone() );
    //         map_inv.insert(c, l);
    //     }
    //     &mut self.alphabet
    // }
 
    pub fn new(alphabet: &str) -> Self {
        let codes = FibStr::new();
        let mut map = HashMap::new();
        let mut map_inv = HashMap::new();
        for (l,c) in alphabet.chars().zip(codes) {
            map.insert(l,c.clone() );
            map_inv.insert(c, l);
        }
        FibonacciCode{ map, map_inv, alphabet: alphabet.to_string() }
    }

    pub fn chars_codes(&self) -> impl Iterator<Item=(char, &String)> + '_ {
        self.alphabet.chars()
            .map(|x| (x, self.map.get(&x).unwrap()) )
    }

}

impl Default for FibonacciCode {
    fn default() -> Self {
        Self::new("ETAOINSHRDLCUMWFGYPBVKJXQZ")
    }
}
 
impl Code for FibonacciCode {
 
    fn encode(&self, text: &str) -> Result<String,CodeError> {
        let mut output = String::new();
        for s in text.chars() {
            output.push_str(&self.map[&s])
        }
        Ok(output)
    }
 
    fn decode(&self, text: &str) -> Result<String,CodeError> {
        let mut output = String::new();
        let mut buffer = String::new();
        for b in text.chars() {
            buffer.push(b);
            if self.map_inv.contains_key(&buffer) {
                output.push(self.map_inv[&buffer]);
                buffer.clear();
            }
        }
        Ok(output)
    }
 
}
 