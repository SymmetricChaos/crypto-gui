use json::JsonValue;
use std::{fmt::Display, sync::LazyLock};

#[derive(Debug, PartialEq, Eq)]
pub enum CodeCategory {
    BinaryToText,
    ErrorCorrecting,
    Integer,
    TextStandard,
    Commercial,
    Prefix,
    Other,
    Compression,
}

impl Default for CodeCategory {
    fn default() -> Self {
        Self::BinaryToText
    }
}

impl CodeCategory {
    pub fn description(&self) -> &'static str {
        match CODE_CATEGORY_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for CodeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CodeCategory::BinaryToText => "Binary-to-Text",
            CodeCategory::ErrorCorrecting => "Error Correcting",
            CodeCategory::Integer => "Integer",
            CodeCategory::TextStandard => "Text Standards",
            CodeCategory::Commercial => "Commercial",
            CodeCategory::Other => "Other",
            CodeCategory::Prefix => "Prefix",
            CodeCategory::Compression => "Compression",
        };
        write!(f, "{}", name)
    }
}

impl From<CodeCategory> for String {
    fn from(id: CodeCategory) -> Self {
        id.to_string()
    }
}

pub static CODE_CATEGORY_INFORMATION: LazyLock<JsonValue> = LazyLock::new(|| {
    json::parse(&include_str!("code_category_descriptions.json").replace('\u{feff}', ""))
        .expect("unable to parse code_descriptions")
});
