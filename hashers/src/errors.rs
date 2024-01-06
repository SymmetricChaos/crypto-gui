use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum HasherError {
    General(String),
}

impl HasherError {
    pub fn general(error: &str) -> Self {
        Self::General(format!("{error}"))
    }

    pub fn inner(self) -> String {
        match self {
            Self::General(e) => e,
            Self::State(e) => e,
        }
    }
}

impl Display for RngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Self::General(e) => format!("General Error: {e}"),
        };
        write!(f, "{error}")
    }
}
