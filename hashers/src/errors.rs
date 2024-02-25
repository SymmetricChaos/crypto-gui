use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum HasherError {
    General(String),
    Key(String),
}

impl HasherError {
    pub fn general(error: &str) -> Self {
        Self::General(format!("{error}"))
    }

    pub fn key(error: &str) -> Self {
        Self::Key(format!("{error}"))
    }

    pub fn inner(self) -> String {
        match self {
            Self::General(e) => e,
            Self::Key(e) => e,
        }
    }
}

impl Display for HasherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Self::General(e) => format!("General Error: {e}"),
            Self::Key(e) => format!("Key Error: {e}"),
        };
        write!(f, "{error}")
    }
}
