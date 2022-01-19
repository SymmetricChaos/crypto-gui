use crate::math_functions::PrimeSieve;
use num::{BigUint, Integer, Num, One};
use rand::prelude::ThreadRng;

/// An encoding of arbitrary information to a subset of the integers similar to that used by Godel
pub struct Godel {
    alphabet: String,
}

impl Godel {

    pub fn new(alphabet: &str) -> Godel {
        Godel{ alphabet: String::from(alphabet) }
    }

    fn char_map(&self) -> String {
        let mut out = String::new();
        let length = self.alphabet.chars().count();
        for (l,n) in self.alphabet.chars().zip(1..=length) {
            out.push_str(&format!("{}  ^{}\n",l,n))
        }
        out
    }

}


impl super::Code for Godel {

    fn encode(&self, text: &str) -> Result<String,&'static str> {
        // We need big integers to encode anything even slightly useful
        let mut out = BigUint::one();
        let mut pset = PrimeSieve::new();
        let alpha_vec: Vec<char> = self.alphabet.chars().collect();

        for t in text.chars() {
            let n = alpha_vec.iter().position(|x| *x == t).unwrap() + 1;
            let p = pset.next().unwrap();
            let t = p.pow(n as u32);
            out *= BigUint::from(t);
        }

        Ok(out.to_str_radix(10))
    }

    fn decode(&self, text: &str) -> Result<String,&'static str> {
        let mut num = BigUint::from_str_radix(text,10).unwrap();
        let mut pset = PrimeSieve::new();
        let alpha_vec: Vec<char> = self.alphabet.chars().collect();
        let mut out = String::new();
        
        while !num.is_one() {
            let p = BigUint::from(pset.next().unwrap());
            let mut ctr = 0;
            while num.is_multiple_of(&p) {
                ctr += 1;
                num = num.div_floor(&p)
            }
            out.push(alpha_vec[ctr-1])
        }

        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!()
    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        unimplemented!("output alphabet is fixed")
    }


}