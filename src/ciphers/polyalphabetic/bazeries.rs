use crate::ciphers::Cipher;
use crate::errors::Error;
use crate::global_rng::get_global_rng;
use crate::text_aux::{shuffled_str, PresetAlphabet, VecString};
use itertools::Itertools;

pub struct Bazeries {
    pub alphabet_string: String,
    alphabet: VecString,
    pub wheels: Vec<String>,
    pub offset: usize,
}

impl Bazeries {
    pub fn control_wheels() {
        todo!("must be possible to enforce alphabet length, maybe just have function to 'set_wheels' that checks correctness")
    }

    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }

    pub fn add_wheel(&mut self) {
        self.wheels
            .push(shuffled_str(&self.alphabet_string, &mut get_global_rng()))
    }

    pub fn del_wheel(&mut self) {
        self.wheels.pop();
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    pub fn validate(&self, text: &str) -> Result<(), Error> {
        if text.chars().count() > self.alphabet.len() {
            return Err(Error::input("the text cannot be longer the the number of wheels, for longer messages send each part with a different key"));
        }

        let sorted = self.alphabet_string.chars().sorted().collect_vec();
        for wheel in self.wheels.iter() {
            if wheel.chars().sorted().collect_vec() != sorted {
                return Err(Error::input(
                    "the wheels must have exactly the same letters as the alphabet",
                ));
            }
        }

        Ok(())
    }
}

impl Default for Bazeries {
    fn default() -> Self {
        // 26 Random wheels
        // Maybe rotate these to be pseudo alphabetical
        let alphabet = VecString::from(PresetAlphabet::BasicLatin);
        let alphabet_string = String::from(PresetAlphabet::BasicLatin);
        let wheels = [
            "FDWCBAGJOEPKRSITUQLHMZNXYV",
            "YGALXKDFEPCTSOHVWMIRZNJBUQ",
            "JVORFDLAZTIHBWXMYPQNECGKSU",
            "CYHNKSBRTOPXMEIDGLVZWAFJUQ",
            "USGCLXBRDHIAJTZOFQEYVPWMNK",
            "EDHPQRTFWIMOVNJBCGUYXLZAKS",
            "JQRAWFLYMCHGBSOZXDKUVINEPT",
            "VWSPHDXGOBUTIEZANKFMJQRCYL",
            "SXWIEPOZFQBJMVTAKRUGCYHDLN",
            "YKFUJONPTWCVGDLBIAREMHSXQZ",
            "VHKJAUZGYDPONLSTWMIBCQRFXE",
            "OJNBKZRYLQMWPHASCVEIUFGXTD",
            "KLTYVFQBMUOGNWRCEDZJSAXIHP",
            "IJHCMOFXPSEAZKWGRDUTLVNYBQ",
            "MYZHDCOJGBSEWFRPLTUXKNAVIQ",
            "EPDXSQZAYBVUOJRCGTIMKNHWFL",
            "LTKFQAMGCOVDZYRPUBISJEHWNX",
            "SITKUREOMNBZXFVHLGYJCPDQAW",
            "IKCSGFUNBMHZEYXVRALDJTQWOP",
            "GPQLWSMCBRYZKNDVJFIXEOUHAT",
            "ERWLOSYJDGCBKXAMTHZUPVFNQI",
            "KRFWOVZHNPXTDEUYQICBMAJGSL",
            "PWGRTACFDHBXJVOSKELMUIYNZQ",
            "JGAXRKSTLPQBUOICVDNYFZMWHE",
            "OUGTAHPWXQZYSJVDMNRCIEBFKL",
            "LHYZSUMCKDIVRQAPWXBOETFJNG",
            "FXCEKVRMHLJNGUYPWBAODZSTQI",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let offset = 0;
        Self {
            alphabet,
            wheels,
            offset,
            alphabet_string,
        }
    }
}

impl Cipher for Bazeries {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        self.validate(text)?;

        let mut out = String::with_capacity(text.len());

        let key = self.wheels.iter();
        for (k, c) in key.zip(text.chars()) {
            let n = match k.chars().position(|x| x == c) {
                Some(n) => (n + self.offset) % self.alphabet.len(),
                None => return Err(Error::invalid_alphabet_char(c)),
            };
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.validate(text)?;

        let alen = self.alphabet.len();
        let mut out = String::with_capacity(text.len());
        let rev_offset = alen - self.offset;
        let key = self.wheels.iter();

        for (k, c) in key.zip(text.chars()) {
            let n = match k.chars().position(|x| x == c) {
                Some(n) => (n + rev_offset) % alen,
                None => return Err(Error::invalid_alphabet_char(c)),
            };
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn randomize(&mut self) {
        for wheel in self.wheels.iter_mut() {
            *wheel = shuffled_str(&self.alphabet_string, &mut get_global_rng());
        }
    }
}

#[cfg(test)]
mod bazeries_tests {
    use super::*;

    const PLAINTEXT: &'static str = "THEQUICKBROWNFOXJUMPSOVERTH";
    const CIPHERTEXT: &'static str = "LMKHCVBJVHSACSBZWOEWDHKAENN";

    #[test]
    fn encrypt_test() {
        let mut cipher = Bazeries::default();
        cipher.offset = 3;
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = Bazeries::default();
        cipher.offset = 3;
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
