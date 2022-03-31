use rand::prelude::StdRng;
use super::Cipher;
use crate::text_types::{PresetAlphabet};
use crate::errors::CipherError;
use crate::text_functions::{shuffled_str, validate_alphabet};

pub struct Bazeries {
    alphabet: String,
    wheels: Vec<String>,
    offset: usize,
}

impl Bazeries {
    pub fn control_wheels() {
        todo!("must be possible to enforce alphabet length, maybe just have function to 'set_wheels' that checks correctness")
    }
}

impl Default for Bazeries {
    fn default() -> Self {
        // 26 Random wheels
        // Maybe rotate these to be pseudo alphabetical
        let alphabet = String::from(PresetAlphabet::BasicLatin);
        let wheels = ["FDWCBAGJOEPKRSITUQLHMZNXYV",
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
                            "FXCEKVRMHLJNGUYPWBAODZSTQI"].iter().map(|x| x.to_string()).collect();
        let offset = 0;
        Self{ alphabet, wheels, offset }
    }
}

impl Cipher for Bazeries {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let alen = self.alphabet.chars().count();
        let mut out = String::with_capacity(text.chars().count());
        let key = self.wheels.iter();
        for (k, c) in key.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + self.offset) % alen;
            out.push(k.chars().nth(n).unwrap())
        }
        todo!("this needs to gracefully throw an error instead of panic");
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let alen = self.alphabet.chars().count();
        let mut out = String::with_capacity(text.chars().count());
        let rev_offset = alen - self.offset;
        let key = self.wheels.iter();
        for (k, c) in key.zip(text.chars()) {
            let n = (k.chars().position(|x| x == c).unwrap() + rev_offset) % alen;
            out.push(k.chars().nth(n).unwrap())
        }
        todo!("this needs to gracefully throw an error instead of panic");
        Ok(out)
    }
    
    fn reset(&mut self) {
        *self = Self::default();
    }
    
    fn randomize(&mut self, rng: &mut StdRng) {
        todo!("keep the current number of wheels and fill with permutations of alphabet")
    }
}