use crate::{polybius::StraddlingCheckerboard, transposition::Columnar, Cipher};
use itertools::Itertools;
use utils::{errors::GeneralError, preset_alphabet::Alphabet, text_functions::rank_str};

pub struct Vic {
    pub key_group: String,
    pub date: String,
    pub phrase: String,
    pub pin: u32,
    pub alphabet: String,
}

impl Default for Vic {
    fn default() -> Self {
        Self {
            key_group: String::from("72401"),
            date: String::from("13/9/1959"),
            phrase: String::from("TWASTHENIGHTBEFORECHRISTMAS"),
            pin: 6,
            alphabet: String::from(Alphabet::BasicLatin),
        }
    }
}

impl Vic {
    fn sequencing(&self, text: &str, alphabet: &str) -> Result<String, GeneralError> {
        Ok(rank_str(&text, alphabet)
            .map_err(|e| GeneralError::key(&format!("{:?}", e)))?
            .iter()
            .map(|n| char::from_digit(((n + 1) % 10).try_into().unwrap(), 10).unwrap())
            .join(""))
    }

    fn chain_addition(text: &str, n: usize) -> String {
        let mut v = text.chars().map(|c| Self::char_to_digit(c)).collect_vec();
        for i in 0..n {
            let t = (v[i] + v[i + 1]) % 10;
            v.push(t)
        }
        v.into_iter()
            .skip(text.chars().count())
            .map(|d| char::from_digit(d, 10).unwrap())
            .join("")
    }

    fn digital_addition(a: char, b: char) -> char {
        let t = (Self::char_to_digit(a) + Self::char_to_digit(b)) % 10;
        char::from_digit(t, 10).unwrap()
    }

    fn digital_subtraction(a: char, b: char) -> char {
        let t = (10 + Self::char_to_digit(a) - Self::char_to_digit(b)) % 10;
        char::from_digit(t, 10).unwrap()
    }

    fn digit_encoding(a: &str, b: &str) -> String {
        let mut out = String::new();
        for ch in a.chars() {
            let mut n = (u32::from(ch) - 48) as usize;
            n = (n + 9) % 10; // Equivalent to subtracting 1, without overflowing
            out.push(b.chars().nth(n).unwrap())
        }
        out
    }

    fn char_to_digit(c: char) -> u32 {
        u32::from(c) - 48
    }

    fn extract_date(&self) -> String {
        self.date.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    pub fn key_derivation_string(&self) -> Result<String, GeneralError> {
        let mut derivation = String::new();
        let date_digits = self.extract_date();

        // Line-A
        derivation.push_str(&format!(
            "A: {}                   The key group",
            &self.key_group[..5]
        ));

        // Line-B
        derivation.push_str(&format!(
            "\nB: {}                   First five digits of the date",
            &date_digits[..5]
        ));

        // Line-C
        let mut c = String::new();
        for (c1, c2) in self.key_group.chars().zip(date_digits.chars().take(5)) {
            c.push(Self::digital_subtraction(c1, c2))
        }
        derivation.push_str(&format!("\nC: {}                   A minus B", &c));

        // Line-D
        derivation.push_str(&format!(
            "\nD: {} {}   First ten letters of the phrase",
            &self.phrase[0..10],
            &self.phrase[10..20]
        ));

        // Line-E
        let e1 = self.sequencing(&self.phrase[0..10], &self.alphabet)?;
        let e2 = self.sequencing(&self.phrase[10..20], &self.alphabet)?;
        derivation.push_str(&format!(
            "\nE: {} {}   Letters of D1 and D2 sequenced seperately",
            &e1, &e2
        ));

        // Line-F
        let f = {
            let mut temp = c.clone();
            temp.push_str(&Self::chain_addition(&c, 5));
            temp.push_str("1234567890");
            temp
        };

        derivation.push_str(&format!(
            "\nF: {} {}   C extended by chain addition, followed by the digits ending with zero",
            &f[0..10],
            &f[10..20]
        ));

        // Line-G
        let g = {
            let mut temp = String::new();
            for (c1, c2) in e1.chars().zip(f[0..10].chars()) {
                temp.push(Self::digital_addition(c1, c2))
            }
            temp
        };
        derivation.push_str(&format!("\nG: {}              E1 added to F1", &g));

        // Line-H
        let h = Self::digit_encoding(&g, &e2);
        derivation.push_str(&format!("\nH: {}              G encoded using E2", &h));

        // Line-J (there is no Line-I)
        let j = self.sequencing(&h, "1234567890")?;
        derivation.push_str(&format!("\nJ: {}              Digits of H sequenced", &j));

        // Line-K through Line-P (there is no Line-O)
        let block = Self::chain_addition(&h, 50);
        derivation.push_str("\nK: ");
        derivation.push_str(&block[..10]);
        derivation.push_str(
            "              Lines K through P are the block, formed by chain addition from H",
        );
        derivation.push_str("\nL: ");
        derivation.push_str(&block[10..20]);
        derivation.push_str("\nM: ");
        derivation.push_str(&block[20..30]);
        derivation.push_str("\nN: ");
        derivation.push_str(&block[30..40]);
        derivation.push_str("\nP: ");
        derivation.push_str(&block[40..50]);

        // Derive key lengths
        let key_lengths = {
            let mut last_digits = block.chars().rev();
            let mut a = last_digits.next().unwrap();
            let mut b = last_digits.next().unwrap();
            while a == b {
                a = b;
                b = last_digits.next().unwrap();
            }
            (
                (Self::char_to_digit(b) + self.pin) as usize,
                (Self::char_to_digit(a) + self.pin) as usize,
            )
        };

        derivation.push_str(&format!(
            "\n\nThe last two unequal digits are {} and {}, since the personal number is {} the Q and R key lengths will be {} and {}\n",
            key_lengths.0 - self.pin as usize,
            key_lengths.1 - self.pin as usize,
            self.pin,
            key_lengths.0,
            key_lengths.1
        ));

        // Line-Q
        let mut columnar = Columnar::default();
        columnar.assign_key(&j, "1234567890").unwrap();
        let encrypted_block = columnar.encrypt(&block).unwrap();
        derivation.push_str(&format!(
            "\nQ: {:<23} First {} digits of the block read by columns in order given by J",
            &encrypted_block[..key_lengths.0],
            key_lengths.0
        ));
        derivation.push_str(&format!(
            "\nR: {:<23} Next {} digits of the block read the same way as Q",
            &encrypted_block[key_lengths.0..key_lengths.0 + key_lengths.1],
            key_lengths.1
        ));
        derivation.push_str(&format!(
            "\nS: {}              Digits of P sequenced",
            &self.sequencing(&block[40..50], "1234567890")?
        ));

        Ok(derivation)
    }

    pub fn key_derivation(&self) -> Result<(String, String, String), GeneralError> {
        let date_digits = self.extract_date();
        let a = &self.key_group[..5];
        let b = &date_digits[..5];

        let c = {
            let mut c = String::new();
            for (c1, c2) in a.chars().zip(b.chars()) {
                c.push(Self::digital_subtraction(c1, c2))
            }
            c
        };

        // Line-D is skipped

        let e1 = self.sequencing(&self.phrase[0..10], &self.alphabet)?;
        let e2 = self.sequencing(&self.phrase[10..20], &self.alphabet)?;

        let f = {
            let mut temp = c.clone();
            temp.push_str(&Self::chain_addition(&c, 5));
            temp.push_str("1234567890");
            temp
        };

        let g = {
            let mut temp = String::new();
            for (c1, c2) in e1.chars().zip(f[0..10].chars()) {
                temp.push(Self::digital_addition(c1, c2))
            }
            temp
        };

        let h = Self::digit_encoding(&g, &e2);

        let j = self.sequencing(&h, "1234567890")?;

        let block = Self::chain_addition(&h, 50);

        let key_lengths = {
            let mut last_digits = block.chars().rev();
            let mut a = last_digits.next().unwrap();
            let mut b = last_digits.next().unwrap();
            while a == b {
                a = b;
                b = last_digits.next().unwrap();
            }
            (
                (Self::char_to_digit(b) + self.pin) as usize,
                (Self::char_to_digit(a) + self.pin) as usize,
            )
        };

        // Line-Q
        let mut columnar = Columnar::default();
        columnar.assign_key(&j, "1234567890").unwrap();
        let encrypted_block = columnar.encrypt(&block)?;

        Ok((
            encrypted_block[..key_lengths.0].to_string(),
            encrypted_block[key_lengths.0..key_lengths.0 + key_lengths.1].to_string(),
            self.sequencing(&block[40..50], "1234567890")?.to_string(),
        ))
    }
}

impl Cipher for Vic {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let (q, r, s) = self.key_derivation()?;

        let mut checkerboard = StraddlingCheckerboard::default();
        checkerboard.assign_top_row(&s);
        checkerboard.assign_alphabet(&self.alphabet);
        let mut ctext = checkerboard.encrypt(text)?;

        let mut columnar = Columnar::default();
        columnar.assign_key(&q, "1234567890").unwrap();
        ctext = columnar.encrypt(&ctext)?;

        let mut diagonal_columnar = Columnar::default();
        diagonal_columnar.assign_key(&r, "1234567890").unwrap();
        ctext = diagonal_columnar.encrypt(&ctext)?;

        Ok(ctext)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let (q, r, s) = self.key_derivation()?;

        let mut diagonal_columnar = Columnar::default();
        diagonal_columnar.assign_key(&r, "1234567890").unwrap();
        let mut ptext = diagonal_columnar.decrypt(&text)?;

        let mut columnar = Columnar::default();
        columnar.assign_key(&q, "1234567890").unwrap();
        ptext = columnar.decrypt(&ptext)?;

        let mut checkerboard = StraddlingCheckerboard::default();
        checkerboard.assign_top_row(&s);
        checkerboard.assign_alphabet(&self.alphabet);
        ptext = checkerboard.decrypt(&ptext)?;

        Ok(ptext)
    }
}

#[cfg(test)]
mod vic_tests {

    use super::*;

    const PTEXT: &'static str = "WHENINTHECOURSEOFHUMANEVENTSITBECOMESNECESSARYFORONEPEOPLETODISSOLVETHEPOLITICALBANDSWHICHHAVECONNECTEDTHEMWITHANOTHERANDTOASSUMEAMONGTHEPOWERSOFTHEEARTHTHESEPARATEANDEQUALSTATIONTOWHICHTHELAWSOFNATUREANDOFNATURESGODENTITLETHEMADECENTRESPECTTOTHEOPINIONSOFMANKINDREQUIRESTHATTHEYSHOULDDECLARETHECAUSESWHICHIMPELTHEMTOTHESEPARATION";
    const CTEXT: &'static str = "8586132809363526687665008866982888298850266223382286868665466166203616868868086568968538586769952238982568006556248636598868260688555828852826563882556391218566168076288385060638455208156958912852806652936668104041946288608038328658856362095862294586859365585206248860823696664395586825988282856862062016659628988286502886228666846926208656095018586163622668462963006665968385468408256286125691058035910615656895486566591252276602628262060861582926668666508168928568062915829662958212322808622366918998451818262900253158269";

    #[test]
    #[ignore]
    fn derivation_test() {
        let cipher = Vic::default();
        assert_eq!(
        "A: 72401\nB: 13919\nC: 69592\nD: TWASTHENIG HTBEFORECH\nE: 8017942653 6013589427\nF: 6959254417 1234567890\nG: 4966196060\nH: 3288628787\nJ: 3178429506\nK: 5064805552\nL: 5602850077\nM: 1620350748\nN: 7823857125\nP: 5051328370\n\nThe last two unequal digits are 7 and 0, since the personal number is 6 the key lengths will be 13 and 6\n\nQ: 0668005552551\nR: 758838\nS: 5961328470",
        cipher
            .key_derivation_string()
            .unwrap()
    );
        println!("{}", cipher.key_derivation_string().unwrap());
        /* The key derivation page looks like this
        A: 72401
        B: 13919
        C: 69592
        D: TWASTHENIG HTBEFORECH
        E: 8017942653 6013589427
        F: 6959254417 1234567890
        G: 4966196060
        H: 3288628787
        J: 3178429506
        K: 5064805552
        L: 5602850077
        M: 1620350748
        N: 7823857125
        P: 5051328370

        The last two unequal digits are 7 and 0, since the personal number is 6 the key lengths will be 13 and 6

        Q: 0668005552551
        R: 758838
        S: 5961328470
        */
    }

    #[test]
    fn encrypt_test() {
        let cipher = Vic::default();
        assert_eq!(cipher.encrypt(PTEXT).unwrap(), CTEXT)
    }

    #[test]
    fn decrypt_test() {
        let cipher = Vic::default();
        assert_eq!(cipher.decrypt(CTEXT).unwrap(), PTEXT)
    }
}
