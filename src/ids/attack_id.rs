use std::fmt::Display;

use json::JsonValue;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum AttackId {
    Caesar,
    Substitution,
}

impl Default for AttackId {
    fn default() -> Self {
        Self::Caesar
    }
}

impl AttackId {
    pub fn description(&self) -> &'static str {
        match CODE_INFORMATION[self.to_string()].as_str() {
            Some(s) => s,
            None => "<<<MISSING DESCRIPTION>>>",
        }
    }
}

impl Display for AttackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AttackId::Caesar => "Caesar",
            AttackId::Substitution => "Substitution",
        };
        write!(f, "{}", name)
    }
}

impl From<AttackId> for String {
    fn from(id: AttackId) -> Self {
        id.to_string()
    }
}

const JSON_CODE_INFORMATION: &'static str = include_str!("attack_descriptions.json");

lazy_static! {
    pub static ref CODE_INFORMATION: JsonValue = {
        json::parse(&JSON_CODE_INFORMATION.replace('\u{feff}', ""))
            .expect("unable to parse attack_descriptions")
    };
}
