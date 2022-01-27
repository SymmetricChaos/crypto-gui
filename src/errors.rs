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