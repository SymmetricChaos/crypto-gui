use rand::prelude::ThreadRng;
use rand::Fill;
use super::Cipher;
use crate::text_functions::{LATIN_UPPER,random_char_vec};
use lazy_static::lazy_static;
use std::{collections::VecDeque, fmt};

use itertools::Itertools;

#[derive(Copy,Clone,Debug)]
pub struct Cage {
    lugs: [(usize,usize); 27]
}

impl Default for Cage {
    fn default() -> Self {
        Self { lugs: [
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)
            ]}
    }
}


impl Cage {
    pub fn set_lugs(&mut self, lugs: [(usize,usize); 27]) {
        self.lugs = lugs
    }
}


impl fmt::Display for Cage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "Cage\n".to_string();
        for b in self.lugs.chunks(9).collect_vec() {
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
    pub pins: Vec<char>,
    pub active: usize,
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

    pub fn rotor_length(&self) -> usize {
        self.alphabet.len()
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
    pub lugs: [(usize,usize); 27],
    alphabet: String,
}

impl Default for M209 {
    fn default() -> Self {
        Self { 
            wheels: M209_ROTORS.clone(), 
            lugs: [ (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                    (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)
            ], 
            alphabet: String::from(LATIN_UPPER) 
        }
    }
}

impl M209 {

    pub fn set_pins(&mut self, pins: [&str; 6]) -> Result<(),&'static str> {
        for (r, p) in self.get_wheels().zip(pins) {
            r.set_pins(p)?
        }
        Ok(())
    }

    pub fn set_lugs(&mut self, lugs: [(usize,usize); 27]) {
        self.lugs = lugs
    }

    pub fn set_wheels(&mut self, settings: &str) {
        for (r, c) in self.wheels.iter_mut().zip(settings.chars()) {
            r.set_display(c)
        }
    }

    pub fn get_wheels(&mut self) -> std::slice::IterMut<'_, Rotor> {
        self.wheels.iter_mut()
    }

    pub fn step(&mut self) {
        for w in self.wheels.iter_mut() {
            w.step()
        }
    }

    pub fn print_cage(&self) -> String {
        let mut out = "Cage\n".to_string();
        for b in self.lugs.chunks(9).collect_vec() {
            for lug in b {
                let entry = format!("{}-{}  ",lug.0,lug.1);
                out.push_str(&entry)
            }
            out.push('\n')
        }
        out
    }

    pub fn print_wheels(&self) -> String {
        let mut out = String::new();
        for wheel in self.wheels.iter() {
            out.push_str(&wheel.to_string());
            out.push('\n');
        }
        out
    }
}

impl Cipher for M209 {
    fn encrypt(&self, text: &str) -> Result<String,&'static str> {
        let nums = text.chars().map(|x| char_to_usize(x)).collect_vec();
        let mut out = String::with_capacity(text.len());

        // The wheels move during encryption but we don't want the cipher to get into an unknown position so we just clone them all
        let mut wheels = self.wheels.clone();
        
        for n in nums {
            let mut sh = 0;
            
            // Each tuple represents the two lugs of a bar
            // A lug set to zero is inactive and is ignored
            // If either lug hits an active effective pin increase the shift by one
            for (lug_a, lug_b) in self.lugs {
                if lug_a != 0 {
                    if wheels[lug_a-1].active_is_effective() {
                        sh += 1;
                        continue;
                    }
                }
                if lug_b != 0 {
                    if wheels[lug_b-1].active_is_effective() {
                        sh += 1;
                        continue;
                    }
                }
            }

            // This encryption step should be a modified atbash
            let c = usize_to_char(atbash_encrypt(n,sh,26));
            out.push(c);
            
            /*
            finally advance all the wheels by one step
            because the wheels all have coprime lengths this steps them through every possible permutation
            */
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
        // Fill up an array with random bytes. Then map that to pairs of usize.
        // Unwrap here is justified by the fixed sizes of everything involved.
        let mut data = [0u8; 54];
        data.try_fill(rng).unwrap();
        self.lugs = data.chunks_exact(2).map(|x| ((x[0]%7) as usize, (x[1]%7) as usize)).collect::<Vec<(usize,usize)>>().try_into().unwrap();

        let pins1 = random_char_vec("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 13, rng);
        let pins2 = random_char_vec("ABCDEFGHIJKLMNOPQRSTUVXYZ", 12, rng);
        let pins3 = random_char_vec("ABCDEFGHIJKLMNOPQRSTUVX",12, rng);
        let pins4 = random_char_vec("ABCDEFGHIJKLMNOPQRSTU",12, rng);
        let pins5 = random_char_vec("ABCDEFGHIJKLMNOPQRS",12, rng);
        let pins6 = random_char_vec("ABCDEFGHIJKLMNOPQ",12, rng);

        for (rotor, new_pins) in self.get_wheels().zip([pins1, pins2, pins3, pins4, pins5, pins6].iter()) {
            rotor.pins = new_pins.clone()
        }

    }

    fn input_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }

    fn output_alphabet(&mut self) -> &mut String {
        &mut self.alphabet
    }
}