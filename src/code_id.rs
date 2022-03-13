use std::fmt::Display;


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CodeID {
    Ascii,
    Bacon,
    Morse,
    Godel,
}

impl Default for CodeID {
    fn default() -> Self {
        Self::Ascii
    }
}

impl CodeID {
    // Describe the history of the code
    pub fn description(&self) -> &'static str {
        match self {
            _ => "Missing description. Please complain to the author.",
        }
    }
}



impl Display for  CodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeID::Ascii => "ASCII",
            CodeID::Morse => "Morse",
            CodeID::Bacon => "Bacon",
            CodeID::Godel => "Godel",
        };
        write!(f,"{}",name)
    }
}

impl From<CodeID> for String {
    fn from(id: CodeID) -> Self {
        id.to_string()
    }
}