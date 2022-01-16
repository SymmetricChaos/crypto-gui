use rand::prelude::ThreadRng;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,&'static str>;
    fn decrypt(&self, text: &str) -> Result<String,&'static str>;
    fn randomize(&mut self, rng: &mut ThreadRng);
}

pub const LATIN: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
