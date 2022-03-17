use std::collections::HashSet;

use itertools::Itertools;

use crate::ciphers::Cipher;
use super::{Rotor, CONTROL_ROTOR_VEC, INDEX_ROTOR_VEC, CIPHER_ROTOR_VEC, char_to_usize, usize_to_char};

pub enum SigabaMode {
    Off,
    Plaintext,
    Reset,
    Encipher,
    Decipher,
}
 
 
 
pub struct ControlRotors {
    rotors: [Rotor; 5],
    counter: usize,
}
 
impl ControlRotors {
    // rotor[2] (the middle rotor) steps every time. 
    // rotor[3] steps every 26 characters
    // rotor[1] steps every 676 characters
    // The other two rotors do not move
    fn step(&mut self) {
        self.rotors[2].step();
        if self.counter % 26 == 0 {
            self.rotors[3].step()
        }
        if self.counter % 676 == 0 {
            self.rotors[1].step()
        }
        self.counter += 1;
    }

    fn encrypt(&self, n: usize) -> usize {
        let mut out = n;
        for rotor in self.rotors.iter() {
            out = rotor.ltr(out)
        }
        out
    }
 
    fn produce_signal(&self) -> Vec<usize> {
        let signal_in = [self.encrypt(0),
                         self.encrypt(1),
                         self.encrypt(2),
                         self.encrypt(3),
                        ];
        let mut signal_out: HashSet<usize> = HashSet::new();
        for sig in signal_in {
            match sig {
                0 => {signal_out.insert(8);},
                1 => {signal_out.insert(7);},
                2 => {signal_out.insert(6);},
                3..=4 => {signal_out.insert(5);},
                5..=7 => {signal_out.insert(4);},
                8..=10 => {signal_out.insert(3);},
                11..=14 => {signal_out.insert(2);},
                15..=19 => {signal_out.insert(1);},
                20..=25 => {signal_out.insert(0);},
                _ => unreachable!("SIGABA control rotors should not produce values greater than 25"),
            }
        }
        signal_out.into_iter().collect_vec()
    }
}

impl Default for ControlRotors {
    fn default() -> Self {
        let rotors = [CONTROL_ROTOR_VEC[0].clone(),
                                CONTROL_ROTOR_VEC[1].clone(),
                                CONTROL_ROTOR_VEC[2].clone(),
                                CONTROL_ROTOR_VEC[3].clone(),
                                CONTROL_ROTOR_VEC[4].clone()
                            ];
        Self { rotors, counter: 0 }
    }
}
 
 
// These rotors do not move they only pass signals through them
pub struct IndexRotors {
    rotors: [Rotor; 5]
}
 
impl IndexRotors {
    fn pass_signal(&self, signal: [bool; 10]) -> [bool; 5] {
        todo!("take live inputs and return live outputs")
    }
}

impl Default for IndexRotors {
    fn default() -> Self {
        let rotors = [INDEX_ROTOR_VEC[0].clone(),
                                INDEX_ROTOR_VEC[1].clone(),
                                INDEX_ROTOR_VEC[2].clone(),
                                INDEX_ROTOR_VEC[3].clone(),
                                INDEX_ROTOR_VEC[4].clone(),
                            ];
        Self { rotors }
    }
}


 
// Rotors through which the text input passes
pub struct CipherRotors {
    rotors: [Rotor; 5]
}
 
impl CipherRotors {
    pub fn encrypt(&self, n: usize) -> usize {
        todo!("steal from Enigma")
    }
 
    pub fn decrypt_char(&self, n: usize) -> usize {
        todo!("good luck")
    }
 
    pub fn step(&mut self, signal: [bool; 5]) {
        todo!("take live inputs and move the rotors accordingly")
    }
}

impl Default for CipherRotors {
    fn default() -> Self {
        let rotors = [CIPHER_ROTOR_VEC[0].clone(),
                                CIPHER_ROTOR_VEC[1].clone(),
                                CIPHER_ROTOR_VEC[2].clone(),
                                CIPHER_ROTOR_VEC[3].clone(),
                                CIPHER_ROTOR_VEC[4].clone(),
                            ];
        Self { rotors }
    }
}
 
 
 
// Internal machine state that must mutate during encryption
pub struct SigabaState {
    index_rotors: IndexRotors,
    control_rotors: ControlRotors,
    cipher_rotors: CipherRotors,
}

impl Default for SigabaState {
    fn default() -> Self {
        Self { index_rotors: Default::default(), control_rotors: Default::default(), cipher_rotors: Default::default() }
    }
}
 
impl SigabaState {

    fn step(&mut self) {
        todo!("control rotors send signal to index rotors which send signal to cipher rotors")
    }
 
    fn encrypt(&self, text: &str) -> Result<String,crate::errors::CipherError> {
        let nums = text.chars().map(|c| char_to_usize(c));
        let mut out = String::with_capacity(text.chars().count());
        for n in nums {
            let val = self.cipher_rotors.encrypt(n);
            out.push(usize_to_char(val));
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String,crate::errors::CipherError> {
        todo!()
    }
}
 
 
// Interface for the cipher
pub struct Sigaba {
    state: SigabaState,
    pub mode: SigabaMode,
}
 
impl Sigaba {
 
}
 
impl Default for Sigaba {
    fn default() -> Self {
        Self { state: Default::default(), mode: SigabaMode::Encipher }
    }
}
 
impl Cipher for Sigaba {
    fn encrypt(&self, text: &str) -> Result<String,crate::errors::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String,crate::errors::CipherError> {
        todo!()
    }

    fn randomize(&mut self, rng: &mut rand::prelude::ThreadRng) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn get_input_alphabet(&self) -> &String {
        todo!()
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        todo!()
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherError> {
        todo!()
    }
}