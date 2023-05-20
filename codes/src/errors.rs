use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum CodeError {
    General(String),
    Input(String),
    Alphabet(String),
    State(String),
}

impl CodeError {
    pub fn general(error: &str) -> Self {
        CodeError::General(format!("{error}"))
    }

    pub fn input(error: &str) -> Self {
        CodeError::Input(format!("{error}"))
    }

    pub fn alphabet(error: &str) -> Self {
        CodeError::Alphabet(format!("{error}"))
    }

    pub fn state(error: &str) -> Self {
        CodeError::State(format!("{error}"))
    }

    pub fn invalid_input_char(c: char) -> Self {
        CodeError::Input(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_input_group(s: &str) -> Self {
        CodeError::Input(format!("invalid group `{s}`, alphabets are case sensitive"))
    }

    pub fn invalid_alphabet_char(c: char) -> Self {
        CodeError::Alphabet(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn inner(self) -> String {
        match self {
            CodeError::General(e) => e,
            CodeError::Input(e) => e,
            CodeError::Alphabet(e) => e,
            CodeError::State(e) => e,
        }
    }
}

impl Display for CodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            CodeError::General(e) => format!("General Error: {e}"),
            CodeError::Input(e) => format!("Input Error: {e}"),
            CodeError::Alphabet(e) => format!("Alphabet Error: {e}"),
            CodeError::State(e) => format!("State Error: {e}"),
        };
        write!(f, "{error}")
    }
}
