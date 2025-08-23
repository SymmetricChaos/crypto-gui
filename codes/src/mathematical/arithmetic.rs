use utils::errors::GeneralError;

use crate::traits::Code;

pub struct ArithmeticCode {}

impl Default for ArithmeticCode {
    fn default() -> Self {
        Self {}
    }
}

impl ArithmeticCode {}

impl Code for ArithmeticCode {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        todo!()
    }
}
