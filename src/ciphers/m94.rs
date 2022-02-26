use rand::{prelude::{ThreadRng, SliceRandom}, Rng};
use super::Cipher;
use crate::text_types::{PresetAlphabet::*};
use crate::errors::CipherError;


const M94_WHEELS: [&'static str; 25] = [
        "ABCEIGDJFVUYMHTQKZOLRXSPWN",
        "ACDEHFIJKTLMOUVYGZNPQXRWSB",
        "ADKOMJUBGEPHSCZINXFYQRTVWL",
        "AEDCBIFGJHLKMRUOQVPTNWYXZS",
        "AFNQUKDOPITJBRHCYSLWEMZVXG",
        "AGPOCIXLURNDYZHWBJSQFKVMET",
        "AHXJEZBNIKPVROGSYDULCFMQTW",
        "AIHPJOBWKCVFZLQERYNSUMGTDX",
        "AJDSKQOIVTZEFHGYUNLPMBXWCR",
        "AKELBDFJGHONMTPRQSVZUXYWIC",
        "ALTMSXVQPNOHUWDIZYCGKRFBEJ",
        "AMNFLHQGCUJTBYPZKXISRDVEWO",
        "ANCJILDHBMKGXUZTSWQYVORPFE",
        "AODWPKJVIUQHZCTXBLEGNYRSMF",
        "APBVHIYKSGUENTCXOWFQDRLJZM",
        "AQJNUBTGIMWZRVLXCSHDEOKFPY",
        "ARMYOFTHEUSZJXDPCWGQIBKLNV",
        "ASDMCNEQBOZPLGVJRKYTFUIWXH",
        "ATOJYLFXNGWHVCMIRBSEKUPDZQ",
        "AUTRZXQLYIOVBPESNHJWMDGFCK",
        "AVNKHRGOXEYBFSJMUDQCLZWTIP",
        "AWVSFDLIEBHKNRJQZGMXPUCOTY",
        "AXKWREVDTUFOYHMLSIQNJCPGBZ",
        "AYJPXMVKBQWUGLOSTECHNZFRID",
        "AZDNBUHYFWJLVGRCQMPSOEXTKI",
    ];
 
pub struct M94 {
    pub offset: usize,
    pub wheels: Vec<&'static str>, //wheels can be reordered
    alphabet: String,
}

impl M94 {
    pub fn randomize_wheels(&mut self, rng: &mut ThreadRng) {
        self.wheels.shuffle(rng);
    }
}
 
impl Default for M94 {
    fn default() -> M94 {
        let wheels = Vec::from(M94_WHEELS);
        let alphabet = String::from(BasicLatin);
        M94{ offset: 0, wheels, alphabet }
    }
 
}

impl Cipher for M94 {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        if text.len() != 25 {
            return Err(CipherError::Input("M94 messages must have exactly 25 characters".to_string()))
        }
        let mut out = String::with_capacity(text.len());
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.offset) % 26;
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        if text.len() != 25 {
            return Err(CipherError::Input("M94 messages must have exactly 25 characters".to_string()))
        }
        let mut out = String::with_capacity(text.len());
        let rev_offset = 26 - self.offset;
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + rev_offset) % 26;
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.wheels.shuffle(rng);
        self.offset = rng.gen_range(1..25);
    }

    fn get_mut_input_alphabet(&mut self) -> &mut String {
        unimplemented!("the M94 alphabet cannot be changed")
    }

    fn get_input_alphabet(&self) -> &String {
        &self.alphabet
    }

    fn validate_settings(&self) -> Result<(), CipherError> {
        unimplemented!("It shouldn't be possible to get the M94 into an invalid state")
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
