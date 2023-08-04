#[derive(Debug, PartialEq, Eq)]
pub enum CodeCategory {
    Text,
    Binary,
    ErrorCorrecting,
    Mathematical,
    Other,
}

impl Default for CodeCategory {
    fn default() -> Self {
        Self::Text
    }
}

impl CodeCategory {
    pub fn description_of_category(&self) -> &'static str {
        match self {
            CodeCategory::Text => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::Binary => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::ErrorCorrecting => "<<<DESCRIPTION NEEDED>>>",
            CodeCategory::Mathematical => "<<<DESCRIPTION NEEDED>>",
            CodeCategory::Other => "<<<DESCRIPTION NEEDED>>",
        }
    }
}
