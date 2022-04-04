use rand::prelude::StdRng;
use super::Cipher;
use crate::preset_alphabet::PresetAlphabet;
use crate::errors::CipherError;
use crate::text_functions::shuffled_str;

pub struct Bazeries {
    pub alphabet: String,
    pub wheels: Vec<String>,
    pub offset: usize,
}

impl Bazeries {
    pub fn control_wheels() {
        todo!("must be possible to enforce alphabet length, maybe just have function to 'set_wheels' that checks correctness")
    }

    pub fn add_wheel(&mut self, rng: &mut StdRng) {
        self.wheels.push(shuffled_str(&self.alphabet, rng)) 
    }

    pub fn del_wheel(&mut self) {
        self.wheels.pop(); 
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
            let n = match k.chars().position(|x| x == c) {
                Some(n) => (n + self.offset) % alen,
                None => return Err(CipherError::invalid_alphabet_char(c)),
            };
            out.push(k.chars().nth(n).unwrap())
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let alen = self.alphabet.chars().count();
        let mut out = String::with_capacity(text.chars().count());
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
    
    fn reset(&mut self) {
        *self = Self::default();
    }
    
    fn randomize(&mut self, rng: &mut StdRng) {
        for wheel in self.wheels.iter_mut() {
            *wheel = shuffled_str(&self.alphabet, rng);
        }
    }
}