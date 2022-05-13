// use crate::{text_aux::{Alphabet, VecString, keyed_alphabet}, errors::CipherError};

// use super::Cipher;

// pub struct Hutton {
//     version: u8,
//     alphabet_string: String,
//     alphabet: Alphabet,
//     key_string: String,
//     keyed_alpha: VecString,
//     password_string: String,
//     password: Vec<usize>
// }

// impl Hutton {
//     fn password_values(&self) -> std::iter::Cycle<std::slice::Iter<usize>> {
//         self.password.iter().cycle()
//     }
    
//     fn set_password(&mut self) {
//         self.password = self.password_string.chars().map(|c| self.alphabet.get_pos_of(c).unwrap() + 1).collect();
//     }
    
//     fn assign_password(&mut self, password: &str) {
//         self.password_string = password.to_string();
//         self.set_password();
//     }
    
//     fn set_key(&mut self) {
//         self.keyed_alpha = VecString::from(keyed_alphabet(&self.key_string, &self.alphabet_string));
//     }
    
//     fn assign_key(&mut self, key: &str) {
//         self.key_string = key.to_string();
//         self.set_key();
//     }
// }

// impl Cipher for Hutton {
//     fn encrypt(&self, text: &str) -> Result<String,CipherError> {
//         let mut out = String::with_capacity(text.len());
//         // mutable alphabet for use while function runs
//         let mut keyed_alpha = self.keyed_alpha.clone();
        
//         for (c, p) in text.chars().zip(self.password_values()) {
//             // add the password number to the position of the character in the keyed alphabet
//             let mut value = keyed_alpha.get_pos_of(c).unwrap() + p;
//             // in Version 2 add the plain alphabet position of the first symbol in the keyed alphabet
//             if self.version == 2 {
//                 value += self.alphabet.get_pos_of(keyed_alpha.get_char_at(0));
//             }
//             // reduce modulo alphabet length and push the character at that position in the keyed alphabet to the ciphertext
//             value %= self.alphabet.len();
//             out.push(keyed_alpha.get_char_at(value));
            
//             keyed_alpha.swap(keyed_alpha.get_pos_of(c), value);
//         }
//         Ok(out)
//     }
    
//     fn decrypt(&self, text: &str) -> Result<String,CipherError> {
//         let mut out = String::with_capacity(text.len());
//         let mut keyed_alpha = self.keyed_alpha.clone();
//         // this offset allows us to avoid dealing with negative numbers
//         // since no more than two subtractions occur we know it will not underflow
//         let mut offset = self.alphabet.len()*2;
//         for (p, c) in text.chars().zip(self.password_values()) {
//             let mut value = offset + self.keyed_alpha.get_pos_of(c) - p;
//             if self.version == 2 {
//                 value -= self.alphabet.get_pos_of(self.keyed_alpha.get_char_at(0));
//             }
//             value %= self.alphabet.len();
//             out.push(self.keyed_alpha.get_char_at(value));
            
//             keyed_alpha.swap(keyed_alpha.get_pos_of(c), value);
//         }
//         Ok(out)
//     }

//     fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
//         todo!()
//     }

//     fn reset(&mut self) {
//         todo!()
//     }
// }