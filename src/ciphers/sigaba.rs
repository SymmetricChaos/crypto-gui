use super::Cipher;

#[derive(Clone,Debug)]
pub struct Rotor {
    wiring_rtl: Vec<usize>,
    wiring_ltr: Vec<usize>,
    position: usize,
    size: usize,
    pub wiring_str: &'static str,
    pub name: &'static str,
}
 
impl Rotor {
    pub fn new(name: &str, wiring_str:  &str) -> Rotor {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = Vec::new();
        let mut wiring_ltr = Vec::new();
        for w in wiring_str.chars().map(|x| char_to_usize(x) ).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor{ name, wiring_rtl, wiring_ltr, position: 0, size, wiring_str }
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.size
    }
 
    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    pub fn encrypt_rtl(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_rtl[inner_position];
        (inner + self.size - self.position) % self.size
    }
 
    pub fn ltr(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_ltr[inner_position];
        (inner + self.size - self.position) % self.size
    }
}



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
 
    fn produce_signal(&self) -> [bool; 10] {
        todo!("put in the four live inputs and return live outputs")
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
 
 
 
// Internal machine state that must mutate during encryption
pub struct SigabaState {
    index_rotors: IndexRotors,
    control_rotors: ControlRotors,
    cipher_rotors: CipherRotors,
}
 
impl SigabaState {

    fn char_to_usize(&self, c: char) -> usize {
        todo!()
    }

    fn usize_to_char(&self, n: usize) -> char {
        todo!()
    }

    fn step(&mut self) {
        todo!("control rotors send signal to index rotors which send signal to cipher rotors")
    }
 
    fn encrypt(&self, text: &str) -> Result<String,crate::errors::CipherError> {
        let nums = text.chars().map(|c| self.char_to_usize(c));
        let out = String::with_capacity(text.chars().count());
        for n in nums {
            let val = self.cipher_rotors.encrypt(n);
            out.push(self.usize_to_char(val));
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