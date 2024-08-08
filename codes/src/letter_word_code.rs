use itertools::Itertools;
use strum::{Display, EnumIter};

use crate::errors::CodeError;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum IOMode {
    Letter,
    Word,
    Integer,
}

// For relating characters and words to their positions in the list
pub struct IntegerCodeMaps {
    pub alphabet: String,
    pub words: Vec<String>,
}

impl IntegerCodeMaps {
    pub fn new() -> Self {
        Self {
            alphabet: String::new(),
            words: Vec::new(),
        }
    }
}

impl IntegerCodeMaps {
    pub fn char_to_int(&self, c: char) -> Result<usize, CodeError> {
        self.alphabet
            .find(|x| x == c)
            .ok_or_else(|| CodeError::invalid_input_char(c))
    }

    pub fn int_to_char(&self, n: usize) -> Result<char, CodeError> {
        self.alphabet
            .chars()
            .nth(n)
            .ok_or_else(|| CodeError::input("no character at position given"))
    }

    pub fn word_to_int(&self, s: &str) -> Result<usize, CodeError> {
        self.words
            .iter()
            .position(|x| x == s)
            .ok_or_else(|| CodeError::invalid_input_group(s))
    }

    pub fn int_to_word(&self, n: usize) -> Result<&String, CodeError> {
        self.words
            .iter()
            .nth(n)
            .ok_or_else(|| CodeError::input("no character at position given"))
    }

    pub fn set_words(&mut self, s: &str) {
        self.words = s
            .split(",")
            .map(|w| w.trim().to_string())
            .filter(|word| !word.is_empty())
            .unique()
            .collect_vec();
    }

    pub fn ints_chars(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        self.alphabet.chars().enumerate()
    }

    pub fn ints_words(&self) -> impl Iterator<Item = (usize, &String)> + '_ {
        self.words.iter().enumerate()
    }
}

// #[macro_export]
// macro_rules! impl_code_for_integer_code {
//     ($name: ident) => {
//         impl Code for $name {
//             fn encode(&self, text: &str) -> Result<String, CodeError> {
//                 let mut output = String::new();
//                 match self.mode {
//                     IOMode::Letter => {
//                         for c in text.chars() {
//                             let code = self.maps.char_to_int(c)? as u32;
//                             output.push_str(&self.integer_code.encode_u32(code));
//                             if self.spaced {
//                                 output.push(' ');
//                             }
//                         }
//                     }
//                     IOMode::Word => {
//                         for w in text.split(" ") {
//                             let code = self.maps.word_to_int(w)? as u32;
//                             output.push_str(&self.integer_code.encode_u32(code));
//                             if self.spaced {
//                                 output.push(' ');
//                             }
//                         }
//                     }
//                     IOMode::Integer => {
//                         for s in text.split(" ") {
//                             let n = u32::from_str_radix(s, 10)
//                                 .map_err(|_| CodeError::invalid_input_group(s))?;
//                             output.push_str(&self.integer_code.encode_u32(n));
//                             if self.spaced {
//                                 output.push(' ');
//                             }
//                         }
//                     }
//                 }

//                 if self.spaced {
//                     output.pop();
//                 }

//                 Ok(output)
//             }

//             fn decode(&self, text: &str) -> Result<String, CodeError> {
//                 let text = &text.replace(" ", "");
//                 let mut output = String::new();
//                 let maybe_codes = self.integer_code.decode_to_u32(text).into_iter();
//                 match self.mode {
//                     IOMode::Letter => {
//                         for n in maybe_codes {
//                             if let Some(val) = n {
//                                 match self.maps.alphabet.chars().nth(val as usize) {
//                                     Some(w) => output.push(w),
//                                     None => output.push('�'),
//                                 }
//                             } else {
//                                 output.push('�')
//                             }
//                         }
//                     }
//                     IOMode::Word => {
//                         for n in maybe_codes {
//                             if let Some(val) = n {
//                                 match self.maps.words.get(val as usize) {
//                                     Some(w) => output.push_str(w),
//                                     None => output.push('�'),
//                                 }
//                             } else {
//                                 output.push('�')
//                             }
//                         }
//                     }
//                     IOMode::Integer => {
//                         for n in maybe_codes {
//                             output.push_str(&decode_or_err_char(n))
//                         }
//                     }
//                 }
//                 Ok(output)
//             }
//         }
//     };
// }
