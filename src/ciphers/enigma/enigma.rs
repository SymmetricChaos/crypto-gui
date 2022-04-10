use crate::{errors::CipherError, ciphers::Cipher};
use super::{ROTOR_MAP,REFLECTORS,Rotor,Reflector,Plugboard,char_to_usize,usize_to_char};
use rand::prelude::StdRng;
 
 
 
 
// This will be the mutating inner state of the Enigma machine. Each time we 
// encrypt with Enigma this state is cloned and run.
// Cloning Rotors and Reflectors is cheap as they are Copy. Plugboard is 
// small and so should be cheap to Clone.
#[derive(Clone,Debug)]
pub struct EnigmaState {
    pub plugboard_pairs: String,
    pub plugboard: Plugboard,
    pub rotors: [Rotor; 3],
    pub reflector: Reflector,
}
 
impl EnigmaState {
    fn advance_rotors(&mut self) {
        let mut on_notch = self.rotors[2].position == self.rotors[2].notch.0 || self.rotors[2].position == self.rotors[2].notch.1;
        self.rotors[2].step();
        if on_notch {
            on_notch = self.rotors[1].position == self.rotors[1].notch.0 || self.rotors[1].position == self.rotors[1].notch.1;
            self.rotors[1].step();
            if on_notch {
                self.rotors[0].step();
            }
        }
    }
 
    // The message key
    pub fn set_rotors(&mut self, rotor_positions: (usize,usize,usize)) {
        self.rotors[0].position = rotor_positions.0;
        self.rotors[1].position = rotor_positions.1;
        self.rotors[2].position = rotor_positions.2;
    }

    pub fn set_rings(&mut self, rotor_ring_positions: (usize,usize,usize)) {
        self.rotors[0].ring = rotor_ring_positions.0;
        self.rotors[1].ring = rotor_ring_positions.1;
        self.rotors[2].ring = rotor_ring_positions.2;
    }

    pub fn set_plugboard(&mut self) -> Result<(),CipherError> {
        self.plugboard.set_plugboard(&self.plugboard_pairs)
    }
 
    // Notice that the signal goes through the rotors starting on the right with the 3rd rotor, 
    // then through the reflector, and back through from left to right starting with the 1st rotor
    fn encrypt_char(&mut self, c: char) -> char {
        self.advance_rotors();
        let mut x = char_to_usize(self.plugboard.swap(c));
        x = self.rotors[2].encrypt_rtl(x);
        x = self.rotors[1].encrypt_rtl(x);
        x = self.rotors[0].encrypt_rtl(x);
        x = self.reflector.encrypt(x);
        x = self.rotors[0].encrypt_ltr(x);
        x = self.rotors[1].encrypt_ltr(x);
        x = self.rotors[2].encrypt_ltr(x);
        self.plugboard.swap(usize_to_char(x))
    }
}

impl Default for EnigmaState {
    fn default() -> Self {
        Self { plugboard_pairs: String::new(), plugboard: Default::default(), rotors: [ROTOR_MAP["I"], ROTOR_MAP["II"], ROTOR_MAP["III"]], reflector: REFLECTORS["A"] }
    }
}
 
 
 
#[derive(Clone,Debug)]
pub struct EnigmaM3 {
    pub state: EnigmaState,
}
 
impl Cipher for EnigmaM3 {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut inner_state = self.state.clone();
        inner_state.set_plugboard()?;
        Ok(text.chars().map(|c| inner_state.encrypt_char(c)).collect())
    }
 
    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        self.encrypt(text)
    }

    fn randomize(&mut self, rng: &mut StdRng) {
        todo!("{:?}",rng)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
 
 
impl Default for EnigmaM3 {
    fn default() -> Self {
        Self { state: Default::default() }
    }
}