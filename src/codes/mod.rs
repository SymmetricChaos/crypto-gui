pub mod godel;
pub use godel::Godel;
use rand::prelude::ThreadRng;


pub trait Code {
    fn encode(&self, text: &str) -> Result<String,&'static str>;
    fn decode(&self, text: &str) -> Result<String,&'static str>;
    fn randomize(&mut self, rng: &mut ThreadRng);
    fn input_alphabet(&mut self) -> &mut String;
    fn output_alphabet(&mut self) -> &mut String;
}
