use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    General(String),
    Input(String),
    Key(String),
    Alphabet(String),
    State(String),
}

impl Error {
    pub fn general(error: &str) -> Self {
        Error::General(format!("{error}"))
    }

    pub fn input(error: &str) -> Self {
        Error::Input(format!("{error}"))
    }

    pub fn key(error: &str) -> Self {
        Error::Key(format!("{error}"))
    }

    pub fn alphabet(error: &str) -> Self {
        Error::Alphabet(format!("{error}"))
    }

    pub fn state(error: &str) -> Self {
        Error::State(format!("{error}"))
    }

    pub fn invalid_input_char(c: char) -> Self {
        Error::Input(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_input_group(s: &str) -> Self {
        Error::Input(format!("invalid group `{s}`, alphabets are case sensitive"))
    }

    pub fn invalid_key_char(c: char) -> Self {
        Error::Key(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_key_group(c: char) -> Self {
        Error::Key(format!("invalid group `{c}`, alphabets are case sensitive"))
    }

    pub fn invalid_alphabet_char(c: char) -> Self {
        Error::Alphabet(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn inner(self) -> String {
        match self {
            Error::General(e) => e,
            Error::Input(e) => e,
            Error::Key(e) => e,
            Error::Alphabet(e) => e,
            Error::State(e) => e,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Error::General(e) => format!("General Error: {e}"),
            Error::Input(e) => format!("Input Error: {e}"),
            Error::Key(e) => format!("Key Error: {e}"),
            Error::Alphabet(e) => format!("Alphabet Error: {e}"),
            Error::State(e) => format!("State Error: {e}"),
        };
        write!(f, "{error}")
    }
}
