use crate::{errors::CipherError, traits::Cipher};
use rand::{thread_rng, Rng};

pub struct Dryad {
    pub cipher_rows: [String; 25],
    pub message_key: usize, // easy conversion with char
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
        }
    }
}

impl Dryad {
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
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let breaks = [0, 4, 7, 10, 12, 14, 17, 19, 21, 23, 25];
        let alphabet = &self.cipher_rows[self.message_key];

        let mut out = String::with_capacity(text.len());

        let mut rng = thread_rng();
        for c in text.chars() {
            if !c.is_ascii_digit() {
                return Err(CipherError::input("DRYAD only encrypts digits"));
            }
            let n = c.to_digit(10).unwrap() as usize;
            let pos = rng.gen_range(breaks[n]..breaks[n + 1]);
            out.push(alphabet.chars().nth(pos).unwrap());
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let alphabet = &self.cipher_rows[self.message_key];

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
}
