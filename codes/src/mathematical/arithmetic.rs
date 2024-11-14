use crate::{errors::CodeError, traits::Code};

pub struct ArithmeticCode {

}

impl Default for ArithmeticCode {
    fn default() -> Self {
        Self {  }
    }
}

impl ArithmeticCode {

}

impl Code for ArithmeticCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        todo!()
    }
}