use std::fmt::Display;

use itertools::Itertools;


pub struct CipherErrors {
    errors: Vec<CipherError>
}

impl CipherErrors {
    pub fn new(errors: Vec<CipherError>) -> Self {
        Self { errors }
    }
}

impl Display for CipherErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_list = self.errors.clone().into_iter().map(|x| x.to_string()).join("\n");
        write!(f, "{}",err_list)
    }
}

#[derive(Debug,Clone)]
pub enum CipherError {
    General(String),
    Input(String),
    Key(String),
    Alphabet(String),
}

impl CipherError {
    pub fn new_static(error: &str) -> Self {
        CipherError::General(format!("{}",error))
    }


    pub fn invalid_input_char(c: char) -> Self {
        CipherError::Input(format!("invalid character `{}`, alphabets are case sensitive",c))
    }

    pub fn input(error: &str) -> Self {
        CipherError::Input(format!("{}",error))
    }


    pub fn invalid_key_char(c: char) -> Self {
        CipherError::Key(format!("invalid character `{}`, alphabets are case sensitive",c))
    }

    pub fn key(error: &str) -> Self {
        CipherError::Key(format!("{}",error))
    }


    pub fn invalid_alphabet_char(c: char) -> Self {
        CipherError::Alphabet(format!("invalid character `{}`, alphabets are case sensitive",c))
    }

    pub fn alphabet(error: &str) -> Self {
        CipherError::Alphabet(format!("{}",error))
    }
}

impl Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            CipherError::General(e) => format!("Cipher Error: {}", e),
            CipherError::Input(e) => format!("Input Error: {}", e),
            CipherError::Key(e) => format!("Key Error: {}", e),
            CipherError::Alphabet(e) => format!("Alphabet Error: {}", e),
        };
        write!(f, "{}",error)
    }
}