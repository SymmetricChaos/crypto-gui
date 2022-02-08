use rand::prelude::ThreadRng;
use super::Cipher;
use crate::text_functions::PresetAlphabet;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};
use crate::errors::CipherError;

use itertools::Itertools;

lazy_static! {
    pub static ref M94_WHEELS: HashMap<char, &'static str> = {
        let mut m = HashMap::with_capacity(25);
        m.insert('B', "ABCEIGDJFVUYMHTQKZOLRXSPWN");
        m.insert('C', "ACDEHFIJKTLMOUVYGZNPQXRWSB");
        m.insert('D', "ADKOMJUBGEPHSCZINXFYQRTVWL");
        m.insert('E', "AEDCBIFGJHLKMRUOQVPTNWYXZS");
        m.insert('F', "AFNQUKDOPITJBRHCYSLWEMZVXG");
        m.insert('G', "AGPOCIXLURNDYZHWBJSQFKVMET");
        m.insert('H', "AHXJEZBNIKPVROGSYDULCFMQTW");
        m.insert('I', "AIHPJOBWKCVFZLQERYNSUMGTDX");
        m.insert('J', "AJDSKQOIVTZEFHGYUNLPMBXWCR");
        m.insert('K', "AKELBDFJGHONMTPRQSVZUXYWIC");
        m.insert('L', "ALTMSXVQPNOHUWDIZYCGKRFBEJ");
        m.insert('M', "AMNFLHQGCUJTBYPZKXISRDVEWO");
        m.insert('N', "ANCJILDHBMKGXUZTSWQYVORPFE");
        m.insert('O', "AODWPKJVIUQHZCTXBLEGNYRSMF");
        m.insert('P', "APBVHIYKSGUENTCXOWFQDRLJZM");
        m.insert('Q', "AQJNUBTGIMWZRVLXCSHDEOKFPY");
        m.insert('R', "ARMYOFTHEUSZJXDPCWGQIBKLNV");
        m.insert('S', "ASDMCNEQBOZPLGVJRKYTFUIWXH");
        m.insert('T', "ATOJYLFXNGWHVCMIRBSEKUPDZQ");
        m.insert('U', "AUTRZXQLYIOVBPESNHJWMDGFCK");
        m.insert('V', "AVNKHRGOXEYBFSJMUDQCLZWTIP");
        m.insert('W', "AWVSFDLIEBHKNRJQZGMXPUCOTY");
        m.insert('X', "AXKWREVDTUFOYHMLSIQNJCPGBZ");
        m.insert('Y', "AYJPXMVKBQWUGLOSTECHNZFRID");
        m.insert('Z', "AZDNBUHYFWJLVGRCQMPSOEXTKI");
        m
    };
}


pub struct M94 {
    wheels: Vec<&'static str>,
    length: usize,
    offset: usize,
    alphabet: String,
}

impl M94 {
    pub fn new(offset: usize, wheels: &str) -> M94 {
        let wheel_alphas: Vec<&'static str> = wheels.chars().map(|c| M94_WHEELS[&c]).collect();
        let length = 26;
        let alphabet = String::from(PresetAlphabet::Latin);
        M94{ wheels: wheel_alphas, length, offset, alphabet }
    }

}

impl Cipher for M94 {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // should require a 25 character message
        let mut out = String::with_capacity(text.chars().count());
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.offset) % self.length;
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        // should require a 25 character message
        let mut out = String::with_capacity(text.chars().count());
        let rev_offset = self.length - self.offset;
        let ckey = self.wheels.iter().cycle();
        for (k, c) in ckey.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + rev_offset) % self.length;
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn randomize(&mut self, rng: &mut ThreadRng) {
        todo!()
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

impl fmt::Display for M94 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphas = self.wheels.iter().join("\n");
        write!(f, "M94 Cipher\noffset: {}\nwheels: {}",self.offset,alphas)
    }
}