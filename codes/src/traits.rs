pub trait Code {
    fn encode(&self, text: &str) -> Result<String, utils::errors::GeneralError>;
    fn decode(&self, text: &str) -> Result<String, utils::errors::GeneralError>;
}
