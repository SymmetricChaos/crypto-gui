use std::collections::HashSet;

use itertools::Itertools;

use crate::{ciphers::Cipher, errors::CipherError};
use super::{Rotor, CONTROL_ROTOR_VEC, INDEX_ROTOR_VEC, CIPHER_ROTOR_VEC, char_to_usize, usize_to_char};

pub enum SigabaMode {
    Off,
    Plaintext,
    Reset,
    Encipher,
    Decipher,
}
 
 
#[derive(Clone,Debug)]
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
#[derive(Clone,Debug)]
pub struct IndexRotors {
    rotors: [Rotor; 5]
}
 
impl IndexRotors {
    pub fn encrypt(&self, n: usize) -> usize {
        let mut out = n;
        for rotor in self.rotors.iter() {
            out = rotor.ltr(out)
        }
        out
    }
 
    fn pass_signal(&self, signal_in: Vec<usize>) -> Vec<usize> {
 
        let mut signal_out: HashSet<usize> = HashSet::new();
        for sig in signal_in.iter().map(|s| self.encrypt(*s)) {
            match sig {
                0|1 => {signal_out.insert(0);},
                2|3 => {signal_out.insert(1);},
                4|5 => {signal_out.insert(2);},
                6|7 => {signal_out.insert(3);},
                8|9 => {signal_out.insert(4);},
                _ => unreachable!("SIGABA index rotors should not produce values greater than 9"),
            }
        }
        signal_out.into_iter().collect_vec()
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
#[derive(Clone,Debug)]
pub struct CipherRotors {
    rotors: [Rotor; 5]
}
 
impl CipherRotors {
    pub fn encrypt(&self, n: usize) -> usize {
        let mut out = n;
        for rotor in self.rotors.iter() {
            out = rotor.ltr(out)
        }
        out
    }
 
    pub fn decrypt(&self, n: usize) -> usize {
        let mut out = n;
        for rotor in self.rotors.iter().rev() {
            out = rotor.rtl(out)
        }
        out
    }
 
    pub fn step(&mut self, signal: Vec<usize>) {
        for sig in signal {
            self.rotors[sig].step()
        }
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
#[derive(Clone,Debug)]
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
        let sig = self.control_rotors.produce_signal();
        let sig = self.index_rotors.pass_signal(sig);
        self. cipher_rotors.step(sig);
    }
 
    fn encrypt_single(&self, n: usize) -> usize {
        self.cipher_rotors.encrypt(n)
    }
 
    fn decrypt_single(&self, n: usize) -> usize {
        self.cipher_rotors.decrypt(n)
    }
 
    fn encrypt(&mut self, text: &str) -> String {
        let mut nums: Vec<usize> = text.chars().map(|c| char_to_usize(c)).collect();
        for n in nums.iter_mut() {
            *n = self.encrypt_single(*n);
            self.step()
        }
        nums.iter().map(|n| usize_to_char(*n)).collect()
    }

    fn decrypt(&mut self, text: &str) -> String {
        let mut nums: Vec<usize> = text.chars().map(|c| char_to_usize(c)).collect();
        for n in nums.iter_mut() {
            *n = self.decrypt_single(*n);
            self.step()
        }
        nums.iter().map(|n| usize_to_char(*n)).collect()
    }
}
 
 
// Interface for the cipher
pub struct Sigaba {
    state: SigabaState,
}
 
impl Default for Sigaba {
    fn default() -> Self {
        Self { state: Default::default() }
    }
}
 
impl Cipher for Sigaba {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut state = self.state.clone();
        Ok(state.encrypt(text))
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut state = self.state.clone();
        Ok(state.decrypt(text))
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


// TODO: These tests only confirm that encrypting and decrypting properly reverse
// need to validated correctness, check wiring diagram
#[cfg(test)]
mod sigaba_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXIUMPSOVERTHELAZYDOG";
    const CIPHERTEXT: &'static str = "GIUPTZNKCMFPUNZUBPICOCXOCYFCFQPJBTY";
    //SIGABA is not perfectly reversible
    const DECRYPT_TEXT: &'static str = "THEQUICKBROWNFOXIUMPSOVERTHELAXYDOG";

    #[test]
    fn encrypt() {
        let cipher = Sigaba::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt() {
        let cipher = Sigaba::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
