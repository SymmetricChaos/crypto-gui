use std::fmt::Display;

#[derive(Debug,Clone)]
pub enum CipherError {
    General(String),
    Input(String),
    Output(String),
    Key(String),
    Alphabet(String),
}

impl CipherError {
    pub fn new(error: &str) -> Self {
        CipherError::General(format!("{}",error))
    }

    pub fn input(error: &str) -> Self {
        CipherError::Input(format!("{}",error))
    }

    pub fn output(error: &str) -> Self {
        CipherError::Output(format!("{}",error))
    }

    pub fn key(error: &str) -> Self {
        CipherError::Key(format!("{}",error))
    }

    pub fn alphabet(error: &str) -> Self {
        CipherError::Alphabet(format!("{}",error))
    }
}

impl Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            CipherError::General(e) => format!("Cipher Error {}", e),
            CipherError::Input(e) => format!("Input Error {}", e),
            CipherError::Output(e) => format!("Output Error {}", e),
            CipherError::Key(e) => format!("Key Error {}", e),
            CipherError::Alphabet(e) => format!("Alphabet Error {}", e),
        };
        write!(f, "{}",error)
    }
}