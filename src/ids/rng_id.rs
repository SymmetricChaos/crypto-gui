use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RngID {
    Lfsr,
}

impl Default for RngID {
    fn default() -> Self {
        Self::Lfsr
    }
}

impl RngID {
    pub fn description(&self) -> &'static str {
        match self {
            RngID::Lfsr => "The Linear Feedback Shift Register.",
            // _ => "Missing description. Please complain to the author.",
        }
    }
}

impl Display for RngID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RngID::Lfsr => "LFSR",
            // _ => "Missing name. Please complain to the author.",
        };
        write!(f, "{}", name)
    }
}

impl From<RngID> for String {
    fn from(id: RngID) -> Self {
        id.to_string()
    }
}
