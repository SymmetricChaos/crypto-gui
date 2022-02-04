// use std::collections::VecDeque;

// use rand::prelude::ThreadRng;
// use super::Cipher;
// use crate::errors::CipherError;


// pub struct Composite<'a> {
//     ciphers: Vec<&'a dyn Cipher>,
// }

// impl<'a> Composite<'a> {

//     fn validate_keys(&self) -> Result<(),CipherError> {
//         todo!()
//     }

//     fn validate_input(&self, text: &str) -> Result<(),CipherError> {
//         todo!()
//     }


// }

// impl Default for Composite<'_> {
//     fn default() -> Self {
//         todo!()
//     }
// }

// impl Cipher for Composite<'_> {
//     fn encrypt(&self, text: &str) -> Result<String,CipherError> {
//         todo!()
//     }

//     fn decrypt(&self, text: &str) -> Result<String,CipherError> {
//         todo!()
//     }

//     fn randomize(&mut self, rng: &mut ThreadRng) {
//         todo!()
//     }

//     fn get_input_alphabet(&mut self) -> &String {
//         todo!()
//     }

//     fn get_output_alphabet(&mut self) -> &String {
//         todo!()
//     }

//     fn get_mut_input_alphabet(&mut self) -> &mut String {
//         todo!()
//     }

//     fn get_mut_output_alphabet(&mut self) -> &mut String {
//         todo!()
//     }

//     fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
//         todo!()
//     }
// }