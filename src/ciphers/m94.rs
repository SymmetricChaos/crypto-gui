use super::Cipher;
use crate::errors::CipherError;
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng,
};

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
    pub wheels: [&'static str; 25], //wheels can be reordered but not changed
}

impl Default for M94 {
    fn default() -> M94 {
        let wheels = M94_WHEELS.clone();
        M94 {
            offset: 0,
            wheels,
        }
    }
}

impl Cipher for M94 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        if text.len() != self.wheels.len() {
            return Err(CipherError::Input(
                "M94 messages must have exactly 25 characters".to_string(),
            ));
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
        if text.len() != self.wheels.len() {
            return Err(CipherError::Input(
                "M94 messages must have exactly 25 characters".to_string(),
            ));
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

    fn randomize(&mut self, rng: &mut StdRng) {
        self.wheels.shuffle(rng);
        self.offset = rng.gen_range(1..self.wheels.len());
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}


#[cfg(test)]
mod m94_tests {
    use super::*;

    const PLAINTEXT: &'static str =  "THEQUICKBROWNFOXJUMPSOVER";
    const CIPHERTEXT: &'static str = "WVYAHWENQCKCGUAYKNZFTISYK";

    #[test]
    fn encrypt_test() {
        let mut cipher = M94::default();
        cipher.offset = 10;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = M94::default();
        cipher.offset = 10;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
