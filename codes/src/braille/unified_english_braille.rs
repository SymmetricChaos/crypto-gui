use crate::{errors::CodeError, traits::Code};

pub struct UnifiedEnglishBraille {}

impl Default for UnifiedEnglishBraille {
    fn default() -> Self {
        Self {}
    }
}

impl Code for UnifiedEnglishBraille {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}
