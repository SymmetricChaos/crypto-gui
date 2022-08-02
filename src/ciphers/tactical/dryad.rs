use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{shuffled_str, PresetAlphabet},
};

pub struct Dryad {
    pub cipher_rows: [String; 25],
    pub message_key: u8, // easy conversion with char
    pub seed_string: String,
    pub seed: u64,
}

impl Default for Dryad {
    fn default() -> Self {
        Self {
            cipher_rows: [
                "MOFNIHJZCQELKVUBYARSPDGWXT".to_string(),
                "PNIFVSAMHQZDKGREOJTBWXYLUC".to_string(),
                "YQFIDTLSBXGKEUHOWPMZRNVACJ".to_string(),
                "KWPYZLSGQODHMBTICAVEURJFXN".to_string(),
                "OJRHLAMXIGWKYFDEPSCUBQTZNV".to_string(),
                "OYSBVFCXKELHPWNDRQUJIGAMZT".to_string(),
                "ICBTRYEKLZWSDFXGOHQUMANJPV".to_string(),
                "AFYVIBOSUPGZLWTEHQJKNCRDMX".to_string(),
                "BXTUVAYWIDEQMKPZNOSJRFCGHL".to_string(),
                "CNXAJIPOBYKTUEZVDQWFSHRGML".to_string(),
                "AXFTRPBSCVJUOKLWZENGYQHDMI".to_string(),
                "ZCKETUGYAILSOXBRVJFPQDMNHW".to_string(),
                "HPBNZTSQEGVCUWJIMORXDAFYKL".to_string(),
                "ZBSIKRNCTUDPEVLYWJMAGOHQFX".to_string(),
                "IZJLNUBVKQGCRMHDFEWTYASPXO".to_string(),
                "VKRZEWBPJYDOQFIHCSANLTMUGX".to_string(),
                "LUJMNSVROCPQEKHDBWTFIAYXZG".to_string(),
                "LKHXQZTVAOBJEDNFCRWYIPGSMU".to_string(),
                "KCWJUZRTOXQAGLVIHPMDBNSYEF".to_string(),
                "CIEMANZBHUFKRXVOJPSYQLGTDW".to_string(),
                "TVYNEKULZRMSQBJXCOAIGDPWFH".to_string(),
                "WJLITQRXVEZCHYDNBUKMAPOGSF".to_string(),
                "KPMBEYARSUHNIJQDFOLZWVGXCT".to_string(),
                "ZDUMSFOGKXNHACRTVWBLQIYJEP".to_string(),
                "WIVGMCAHPSRXEQODFKLUYJTNZB".to_string(),
            ],
            message_key: 0,
            seed_string: "0".to_string(),
            seed: 0,
        }
    }
}

impl Dryad {
    pub fn message_key_to_char(&self) -> char {
        (self.message_key + 65) as char
    }

    pub fn show_code_page(&self) -> String {
        let breaks = [0, 4, 7, 10, 12, 14, 17, 19, 21, 23, 25];
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXY";

        let mut s = "      0    1    2   3   4    5   6   7   8   9".to_string();
        for (i, c) in alphabet.chars().enumerate() {
            s.push('\n');
            s.push(c);
            s.push_str("  ");
            let r = &self.cipher_rows[i];
            for p in 0..10 {
                s.push_str(&r[breaks[p]..breaks[p + 1]]);
                s.push_str("  ");
            }
        }
        s
    }
}

impl Cipher for Dryad {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let breaks = [0, 4, 7, 10, 12, 14, 17, 19, 21, 23, 25];
        let alphabet = &self.cipher_rows[self.message_key as usize];

        let mut out = String::with_capacity(text.len());

        let mut rng = StdRng::from_entropy();
        for c in text.chars() {
            if !c.is_ascii_digit() {
                return Err(Error::input("DRYAD only encrypts digits"));
            }
            let n = c.to_digit(10).unwrap() as usize;
            let pos = rng.gen_range(breaks[n]..breaks[n + 1]);
            out.push(alphabet.chars().nth(pos).unwrap());
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let alphabet = &self.cipher_rows[self.message_key as usize];

        let mut out = String::with_capacity(text.len());
        for c in text.chars() {
            let pos = alphabet.chars().position(|x| x == c).unwrap();
            let d = match pos {
                0..=3 => 0,
                4..=6 => 1,
                5..=9 => 2,
                10..=11 => 3,
                12..=13 => 4,
                14..=16 => 5,
                17..=18 => 6,
                19..=20 => 7,
                21..=22 => 8,
                23..=24 => 9,
                _ => unreachable!("invalid position encountered"),
            };
            out.push(digits[d])
        }

        Ok(out)
    }

    fn randomize(&mut self) {
        let alpha = PresetAlphabet::BasicLatin.slice();
        for row in self.cipher_rows.iter_mut() {
            *row = shuffled_str(alpha, &mut get_global_rng())
        }
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
