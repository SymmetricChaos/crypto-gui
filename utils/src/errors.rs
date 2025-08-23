use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneralError(String);

impl GeneralError {
    pub fn general<T: ToString>(error: T) -> Self {
        GeneralError(format!("{}", error.to_string()))
    }

    pub fn input<T: ToString>(error: T) -> Self {
        GeneralError(format!("Input Error: {}", error.to_string()))
    }

    pub fn key<T: ToString>(error: T) -> Self {
        GeneralError(format!("Key Error: {}", error.to_string()))
    }

    pub fn alphabet<T: ToString>(error: T) -> Self {
        GeneralError(format!("Alphabet Error: {}", error.to_string()))
    }

    pub fn state<T: ToString>(error: T) -> Self {
        GeneralError(format!("State Error: {}", error.to_string()))
    }

    pub fn invalid_key_char(c: char) -> Self {
        GeneralError(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_key_group(c: char) -> Self {
        GeneralError(format!("invalid group `{c}`, alphabets are case sensitive"))
    }

    pub fn invalid_input_char(c: char) -> Self {
        GeneralError(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }

    pub fn invalid_input_group<T: ToString>(s: T) -> Self {
        GeneralError(format!(
            "invalid group `{}`, alphabets are case sensitive",
            s.to_string()
        ))
    }

    pub fn invalid_alphabet_char(c: char) -> Self {
        GeneralError(format!(
            "invalid character `{c}`, alphabets are case sensitive"
        ))
    }
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
