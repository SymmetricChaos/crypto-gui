// use std::fmt;
// use itertools::Itertools;
// use num::integer::Roots;
//
// use super::Cipher;
// use crate::{errors::CipherError, text_functions::shuffled_str};
// use crate::text_functions::{keyed_alphabet};
// use crate::text_types::{PresetAlphabet::*, PresetAlphabet};

// pub struct SeriatedPlayfair {
//     alphabet: String,
//     square: String,
//     key_word: String,
//     spacer: char,
//     grid_side_len: usize,
//     period: usize,
// }

// impl SeriatedPlayfair {

//     // Silently ignores invalid characters
//     pub fn control_key(&mut self) -> &mut String {
//         self.square = keyed_alphabet(&self.key_word, &self.alphabet);
//         &mut self.key_word
//     }

//     pub fn set_key(&mut self, key_word: &str) {
//         self.key_word = key_word.to_string();
//         self.square = keyed_alphabet(key_word, &self.alphabet);
//     }

//     pub fn control_spacer(&mut self) -> &mut char {
//         &mut self.spacer
//     }

//     pub fn set_alphabet(&mut self, mode: PresetAlphabet) {
//         match mode {
//             BasicLatinNoJ | BasicLatinNoQ | BasicLatinWithDigits | Base64 => {
//                 self.alphabet = mode.string();
//                 self.square = mode.string();
//                 self.grid_side_len = mode.len().sqrt();
//             }
//             _ => ()
//         }
//     }

//     fn pairs(&self, text: &str) -> Vec<(char,char)> {
//         let mut symbols: Vec<char> = text.chars().rev().collect();
//         let mut out = Vec::with_capacity(text.len()/2);
//         while symbols.len() >= 2 {
//             //unwrap justified by condition above
//             let l = symbols.pop().unwrap();
//             let r = symbols.pop().unwrap();
//             if l == r {
//                 symbols.push(r);
//                 out.push((l,self.spacer));
//             } else {
//                 out.push((l,r));
//             }
//         }
//         if symbols.len() != 0 {
//             out.push( (symbols.pop().unwrap(), self.spacer) )
//         }
//         out
//     }

//     fn char_to_position(&self, symbol: char) -> Result<(usize,usize),CipherError> {
//         let num = match self.square.chars().position(|x| x == symbol) {
//             Some(n) => n,
//             None => return Err(CipherError::invalid_input_char(symbol)),
//         };
//         Ok((num / self.grid_side_len, num % self.grid_side_len))
//     }

//     // The inputs to this come only from internal functions that will never give invalid positions
//     fn position_to_char(&self, position: (usize,usize)) -> char {
//         let num = position.0*self.grid_side_len + position.1;
//         self.square.chars().nth(num).unwrap()
//     }

//     fn playfair_shift(&self, lpos: (usize,usize), rpos: (usize,usize), shift: usize, output: &mut String) {
//         let size = self.grid_side_len;
//         // The pairs() function ensures l and r never match so that case is not handled
//         if lpos.0 == rpos.0 {
//             let x = lpos.0;
//             output.push(self.position_to_char((x, (lpos.1+shift)%size )));
//             output.push(self.position_to_char((x, (rpos.1+shift)%size )));
//         } else if lpos.1 == rpos.1 {
//             let y = lpos.1;
//             output.push(self.position_to_char(( (lpos.0+shift)%size, y )));
//             output.push(self.position_to_char(( (rpos.0+shift)%size, y )));
//         } else {
//             output.push(self.position_to_char((lpos.0, rpos.1) ));
//             output.push(self.position_to_char((rpos.0, lpos.1) ));
//         }
//     }

// }

// impl Default for SeriatedPlayfair {
//     fn default() -> Self {
//         Self{ alphabet: String::from(PresetAlphabet::BasicLatinNoQ),
//               square: String::from(PresetAlphabet::BasicLatinNoQ),
//               spacer: 'X',
//               grid_side_len: 5,
//               key_word: String::new() ,
//               period: 10,
//         }
//     }
// }

// impl Cipher for SeriatedPlayfair {
//     fn encrypt(&self, text: &str) -> Result<String,CipherError> {
//         // self.validate_settings()?;

//         // let chunks = text.chars().chunks(self.period);
//         // let groups = chunks.into_iter().map(|x| x.collect_vec()).collect_vec();

//         // let shift = self.grid_side_len+1;
//         // let mut out = String::with_capacity(text.len());

//         // for pair in 0..groups.len()/2 {
//         //     //println!("{:?}\n{:?}\n",groups[2*pair],groups[2*pair+1]);
//         //     let mut row_a = String::with_capacity(self.period);
//         //     let mut row_b = String::with_capacity(self.period);
//         //     for pos in 0..self.period {
//         //         let a = groups[2*pair][pos];
//         //         let b = groups[2*pair+1][pos];
//         //         self.playfair_shift(a, b, shift, &mut out);
//         //     }
//         // }
//         // Ok(out)
//         todo!()
//     }

//     fn decrypt(&self, text: &str) -> Result<String,CipherError> {
//         todo!()
//     }

//     fn randomize(&mut self, rng: &mut StdRng) {
//         self.alphabet = shuffled_str(&self.alphabet, rng)
//     }

//     fn get_input_alphabet(&self) -> &String {
//         &self.square
//     }

//     fn get_mut_input_alphabet(&mut self) -> &mut String {
//         &mut self.square
//     }

//     fn validate_settings(&self) -> Result<(), CipherError> {
//         if self.period % 2 != 0 {
//             return Err(CipherError::Key(String::from("period must be an even number")))
//         }
//         // let tlen = text.chars().count();
//         // if tlen % (self.period) != 0 {
//         //     return Err(CipherError::Key(format!("number of characters in the text must be a multiple of the period: {}",self.period)))
//         // }
//         if !&self.alphabet.contains(self.spacer) {
//             return Err(CipherError::Key(format!("spacer character {} is not in the alphabet",self.spacer)))
//         }
//         Ok(())
//     }

//     fn reset(&mut self) {
//         *self = Self::default();
//     }
// }

// impl fmt::Display for SeriatedPlayfair {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut out = String::new();
//         for (n, c) in self.square.chars().enumerate() {
//             if n % self.grid_side_len == 0 {
//                 out.push_str("\n")
//             }
//             out.push_str(&format!("{} ",c))
//         };
//         write!(f, "{}", out)
//     }
// }

// #[cfg(test)]
// mod seriated_playfair_tests {
//     use super::*;

//     // Note Q replaced by K and the X used as padding
//     const PLAINTEXT: &'static str =  "THEKUICKBROWNFOXJUMPSOVERTHELAZYDOGX";
//     const CIPHERTEXT: &'static str = "WGVOEGAOAWNXKHXEGLNKCMULTWIZVDLWCPIT";

//     #[test]
//     fn check_stuff() {
//         let mut cipher = SeriatedPlayfair::default();

//     }

//     // #[test]
//     // fn encrypt_test() {
//     //     let mut cipher = SeriatedPlayfair::default();
//     //     cipher.set_key("VUVUZELAS");
//     //     assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
//     // }

//     // #[test]
//     // fn decrypt_test() {
//     //     let mut cipher = SeriatedPlayfair::default();
//     //     cipher.set_key("VUVUZELAS");
//     //     assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
//     // }
// }
