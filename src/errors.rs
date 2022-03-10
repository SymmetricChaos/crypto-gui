use std::fmt::Display;


#[derive(Debug,Clone)]
pub enum CipherError {
    General(String),
    Input(String),
    Key(String),
    Alphabet(String),
}

impl CipherError {
    pub fn new_static(error: &str) -> Self {
        CipherError::General(format!("{error}"))
    }


    pub fn invalid_input_char(c: char) -> Self {
        CipherError::Input(format!("invalid character `{c}`, alphabets are case sensitive"))
    }

    pub fn input(error: &str) -> Self {
        CipherError::Input(format!("{error}"))
    }


    pub fn invalid_key_char(c: char) -> Self {
        CipherError::Key(format!("invalid character `{c}`, alphabets are case sensitive"))
    }

    pub fn key(error: &str) -> Self {
        CipherError::Key(format!("{error}"))
    }


    pub fn invalid_alphabet_char(c: char) -> Self {
        CipherError::Alphabet(format!("invalid character `{c}`, alphabets are case sensitive"))
    }

    pub fn alphabet(error: &str) -> Self {
        CipherError::Alphabet(format!("{error}"))
    }
}

impl Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            CipherError::General(e) => format!("Cipher Error: {e}"),
            CipherError::Input(e) => format!("Input Error: {e}"),
            CipherError::Key(e) => format!("Key Error: {e}"),
            CipherError::Alphabet(e) => format!("Alphabet Error: {e}"),
        };
        write!(f, "{error}")
    }
}





#[derive(Debug,Clone)]
pub enum CodeError {
    General(String),
    Input(String),
    Key(String),
    Alphabet(String),
}

impl CodeError {
    pub fn new_static(error: &str) -> Self {
        CodeError::General(format!("{error}"))
    }


    pub fn invalid_input_char(c: char) -> Self {
        CodeError::Input(format!("invalid character `{c}`, alphabets are case sensitive"))
    }

    pub fn input(error: &str) -> Self {
        CodeError::Input(format!("{error}"))
    }


    pub fn invalid_key_char(c: char) -> Self {
        CodeError::Key(format!("invalid character `{c}`, alphabets are case sensitive"))
    }

    pub fn key(error: &str) -> Self {
        CodeError::Key(format!("{error}"))
    }


    pub fn invalid_alphabet_char(c: char) -> Self {
        CodeError::Alphabet(format!("invalid character `{c}`, alphabets are case sensitive"))
    }

    pub fn alphabet(error: &str) -> Self {
        CodeError::Alphabet(format!("{error}"))
    }
}

impl Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            CodeError::General(e) => format!("Code Error: {e}"),
            CodeError::Input(e) => format!("Input Error: {e}"),
            CodeError::Key(e) => format!("Key Error: {e}"),
            CodeError::Alphabet(e) => format!("Alphabet Error: {e}"),
        };
        write!(f, "{error}")
    }
}