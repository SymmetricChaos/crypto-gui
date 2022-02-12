use rand::{prelude::{ThreadRng, SliceRandom}, Rng};
use super::Cipher;
use crate::text_functions::PresetAlphabet;
use std::fmt;
use crate::errors::CipherError;

use itertools::Itertools;
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
 
impl Default for M94 {
    fn default() -> M94 {
        let wheels = Vec::from(M94_WHEELS);
        let alphabet = String::from(PresetAlphabet::English);
        M94{ offset: 0, wheels, alphabet }
    }
 
}

impl Cipher for M94 {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // should require a 25 character message
        let mut out = String::with_capacity(text.len());
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.offset) % 26;
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // should require a 25 character message
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

    fn get_mut_output_alphabet(&mut self) -> &mut String {
        unimplemented!("the M94 alphabet cannot be changed")
    }

    fn get_input_alphabet(&mut self) -> &String {
        &self.alphabet
    }

    fn get_output_alphabet(&mut self) -> &String {
        &self.alphabet
    }

    fn validate_settings(&self) -> Result<(),crate::errors::CipherErrors> {
        todo!()
    }

}
