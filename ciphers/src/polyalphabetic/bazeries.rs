use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use utils::{functions::keyed_alphabet, preset_alphabet::Alphabet, vecstring::VecString};

pub struct Bazeries {
    pub alphabet: VecString,
    pub wheels: Vec<String>,
    pub offset: usize,
}

impl Bazeries {
    pub fn assign_alphabet(&mut self, alphabet: &str) {
        self.alphabet = VecString::unique_from(alphabet);
        for wheel in self.wheels.iter_mut() {
            *wheel = keyed_alphabet(&wheel, alphabet)
        }
    }

    pub fn add_wheel(&mut self) {
        self.wheels.push(String::from(self.alphabet.clone()))
    }

    pub fn del_wheel(&mut self) {
        self.wheels.pop();
    }

    pub fn alphabet_len(&self) -> usize {
        self.alphabet.len()
    }

    pub fn validate(&self, text: &str) -> Result<(), CipherError> {
        if text.chars().count() > self.alphabet.len() {
            return Err(CipherError::input("the text cannot be longer the the number of wheels, for longer messages send each part with a different key"));
        }

        let sorted = self.alphabet.chars().sorted().collect_vec();
        for wheel in self.wheels.iter() {
            if wheel.chars().sorted().collect_vec() != sorted {
                return Err(CipherError::input(
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
        let alphabet = VecString::from(Alphabet::BasicLatin);
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
        }
    }
}

impl Cipher for Bazeries {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate(text)?;

        let mut out = String::with_capacity(text.len());

        let key = self.wheels.iter();
        for (k, c) in key.zip(text.chars()) {
            let n = match k.chars().position(|x| x == c) {
                Some(n) => (n + self.offset) % self.alphabet.len(),
                None => return Err(CipherError::invalid_alphabet_char(c)),
            };
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.validate(text)?;

        let alen = self.alphabet.len();
        let mut out = String::with_capacity(text.len());
        let rev_offset = alen - self.offset;
        let key = self.wheels.iter();

        for (k, c) in key.zip(text.chars()) {
            let n = match k.chars().position(|x| x == c) {
                Some(n) => (n + rev_offset) % alen,
                None => return Err(CipherError::invalid_alphabet_char(c)),
            };
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
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
