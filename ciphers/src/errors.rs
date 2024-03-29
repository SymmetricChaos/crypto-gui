use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum CipherError {
    General(String),
    Input(String),
    Key(String),
    Alphabet(String),
    State(String),
}

impl CipherError {
    pub fn general(error: &str) -> Self {
        CipherError::General(format!("{error}"))
    }

    pub fn input(error: &str) -> Self {
        CipherError::Input(format!("{error}"))
    }

    pub fn key(error: &str) -> Self {
        CipherError::Key(format!("{error}"))
    }

    pub fn alphabet(error: &str) -> Self {
        CipherError::Alphabet(format!("{error}"))
    }

    pub fn state(error: &str) -> Self {
        CipherError::State(format!("{error}"))
    }

    pub fn invalid_input_char(c: char) -> Self {
        CipherError::Input(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_input_group(s: &str) -> Self {
        CipherError::Input(format!("invalid group `{s}`, alphabets are case sensitive"))
    }

    pub fn invalid_key_char(c: char) -> Self {
        CipherError::Key(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_key_group(c: char) -> Self {
        CipherError::Key(format!("invalid group `{c}`, alphabets are case sensitive"))
    }

    pub fn invalid_alphabet_char(c: char) -> Self {
        CipherError::Alphabet(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn inner(self) -> String {
        match self {
            CipherError::General(e) => e,
            CipherError::Input(e) => e,
            CipherError::Key(e) => e,
            CipherError::Alphabet(e) => e,
            CipherError::State(e) => e,
        }
    }
}

impl Display for CipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            CipherError::General(e) => format!("General Error: {e}"),
            CipherError::Input(e) => format!("Input Error: {e}"),
            CipherError::Key(e) => format!("Key Error: {e}"),
            CipherError::Alphabet(e) => format!("Alphabet Error: {e}"),
            CipherError::State(e) => format!("State Error: {e}"),
        };
        write!(f, "{error}")
    }
}
