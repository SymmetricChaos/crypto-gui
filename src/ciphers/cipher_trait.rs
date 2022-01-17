use rand::prelude::ThreadRng;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,&'static str>;
    fn decrypt(&self, text: &str) -> Result<String,&'static str>;
    fn randomize(&mut self, rng: &mut ThreadRng);
    fn input_alphabet(&mut self) -> &mut String;
    fn output_alphabet(&mut self) -> &mut String;
}

pub const LATIN: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
