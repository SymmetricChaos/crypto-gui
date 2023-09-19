use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum RngError {
    General(String),
    State(String),
}

impl RngError {
    pub fn general(error: &str) -> Self {
        RngError::General(format!("{error}"))
    }

    pub fn state(error: &str) -> Self {
        RngError::State(format!("{error}"))
    }

    pub fn inner(self) -> String {
        match self {
            RngError::General(e) => e,
            RngError::State(e) => e,
        }
    }
}

impl Display for RngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            RngError::General(e) => format!("General Error: {e}"),
            RngError::State(e) => format!("State Error: {e}"),
        };
        write!(f, "{error}")
    }
}
