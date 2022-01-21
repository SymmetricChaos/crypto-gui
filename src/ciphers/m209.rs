use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::LATIN;
use lazy_static::lazy_static;
use std::{collections::VecDeque, fmt};

use itertools::Itertools;

#[derive(Clone,Debug)]
pub struct Cage {
    bars: [(usize,usize); 27]
}

impl Default for Cage {
    fn default() -> Self {
        Self { bars: [
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)
            ]  }
    }
}


impl Cage {
    pub fn set_bars(&mut self, bars: [(usize,usize); 27]) {
        self.bars = bars
    }
}


impl fmt::Display for Cage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "Cage\n".to_string();
        for b in self.bars.chunks(9).collect_vec() {
            for lug in b {
                let entry = format!("{}-{}  ",lug.0,lug.1);
                s.push_str(&entry)
            }
            s.push('\n')
        }
        write!(f, "{}", s)
    }
}





#[derive(Clone,Debug)]
pub struct Rotor {
    alphabet: VecDeque<char>,
    pins: Vec<char>,
    active: usize,
}

impl Rotor {
    pub fn new(alphabet: &str, active: usize) -> Rotor {
        let alphabet: VecDeque<char> = alphabet.chars().collect();
        Rotor{ alphabet, pins: Vec::new(), active }
    }

    pub fn step(&mut self) {
        self.alphabet.rotate_left(1)
    }

    pub fn set_pins(&mut self, pins: &str) -> Result<(),&'static str> {
        for p in pins.chars() {
            if !self.alphabet.contains(&p) {
                return Err("effective pins must be in the Rotor's alphabet")
            }
        }
        self.pins = pins.chars().collect();
        Ok(())
    }

    pub fn get_pins(&mut self) -> &mut Vec<char> {
        &mut self.pins
    }

    pub fn set_active(&mut self, c: char) {
        while self.alphabet[self.active] != c {
            self.alphabet.rotate_left(1)
        }
    }

    pub fn set_display(&mut self, c: char) {
        while self.alphabet[0] != c {
            self.alphabet.rotate_left(1)
        }
    }

    pub fn get_active(&self) -> char {
        self.alphabet[self.active]
    }

    pub fn active_is_effective(&self) -> bool {
        self.pins.contains(&self.alphabet[self.active])
    }

}

// This could be simplified since all the real rotors used ASCII characters but this library tries to work with Unicode as much as possible
impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (pos,letter) in self.alphabet.iter().enumerate() {
            if pos == self.active {
                // bracket the active position
                s.push_str(&format!("[{}]",letter));
            } else {
                s.push(*letter)
            }
        }
        s.push_str(&format!(" ({})",self.pins.iter().collect::<String>()));
        write!(f, "{}", s)
    }
}


//The rotor alphabets all have coprime lengths
lazy_static! {
    pub static ref M209_ROTORS: [Rotor; 6] = {
        [Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ",15),
         Rotor::new("ABCDEFGHIJKLMNOPQRSTUVXYZ",14),
         Rotor::new("ABCDEFGHIJKLMNOPQRSTUVX",13),
         Rotor::new("ABCDEFGHIJKLMNOPQRSTU",12),
         Rotor::new("ABCDEFGHIJKLMNOPQRS",11),
         Rotor::new("ABCDEFGHIJKLMNOPQ",10),
        ]
    };
}





fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}

fn atbash_encrypt(n: usize, k: usize, l: usize) -> usize {
    ((l-1)*(n+1)+k) % l
}




pub struct M209 {
    wheels: [Rotor; 6],
    cage: Cage,
    alphabet: String,
}

impl Default for M209 {
    fn default() -> Self {
        Self { wheels: M209_ROTORS.clone(), cage: Default::default(), alphabet: String::from(LATIN) }
    }
}

impl M209 {

    pub fn set_pins(&mut self, settings: &str) {
        for (r, c) in self.wheels.iter_mut().zip(settings.chars()) {
            r.set_display(c)
        }
    }

    pub fn set_wheels(&mut self, settings: &str) {
        for (r, c) in self.wheels.iter_mut().zip(settings.chars()) {
            r.set_display(c)
        }
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            for w in self.wheels.iter_mut() {
                w.step()
            }
        }

    }

}

impl Cipher for M209 {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let nums = text.chars().map(|x| char_to_usize(x)).collect_vec();
        let mut out = String::with_capacity(text.len());

        let cage = &self.cage;
        let mut wheels = self.wheels.clone();
        
        for n in nums {
            let mut sh = 0;
            // Check each bar. 
            // If either lug hits an active effective pin increase the shift by one
            for (lug_a, lug_b) in cage.bars.iter() {
                if lug_a == &0 {
                    // do nothing
                } else {
                    if wheels[lug_a-1].active_is_effective() {
                        sh += 1;
                        continue;
                    }
                }
                if lug_b == &0 {
                    // do nothing
                } else {
                    if wheels[lug_b-1].active_is_effective() {
                        sh += 1;
                        continue;
                    }
                }

            }

            // This encryption step should be a modified atbash
            let c = usize_to_char(atbash_encrypt(n,sh,26));
            out.push(c);
            
            // advance the wheels
            for w in wheels.iter_mut() {
                w.step()
            }
        }
        Ok(out)
    }

    // The M209 is reciprocal
    fn decrypt(&self, text: &str) -> Result<String,&'static str> {
        self.encrypt(text)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!("randomization for M209 not yet implemented")
    }

    fn input_alphabet(&mut self) -> &mut String {
        unimplemented!("the M209 alphabet is fixed")
    }

    fn output_alphabet(&mut self) -> &mut String {
        unimplemented!("the M209 alphabet is fixed")
    }
}