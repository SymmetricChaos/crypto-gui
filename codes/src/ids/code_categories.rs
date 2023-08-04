use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeCategory {
    BinaryToText,
    ErrorCorrecting,
    Mathematical,
    TextStandard,
    Commercial,
    Prefix,
    Other,
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
            CodeCategory::Mathematical => "Mathematical",
            CodeCategory::TextStandard => "Text Standards",
            CodeCategory::Commercial => "Commerial",
            CodeCategory::Other => "Other",
            CodeCategory::Prefix => "Prefix",
        };
        write!(f, "{}", name)
    }
}

impl From<CodeCategory> for String {
    fn from(id: CodeCategory) -> Self {
        id.to_string()
    }
}

const JSON_CODE_CATEGORY_INFORMATION: &'static str =
    include_str!("code_category_descriptions.json");

lazy_static! {
    pub static ref CODE_CATEGORY_INFORMATION: JsonValue = {
        json::parse(&JSON_CODE_CATEGORY_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse code_descriptions")
    };
}
