pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String,&'static str>;
    fn decrypt(&self, text: &str) -> Result<String,&'static str>;
    fn randomize(&mut self);
}

pub const LATIN: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
