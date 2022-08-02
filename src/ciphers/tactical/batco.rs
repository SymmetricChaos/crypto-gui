use std::num::ParseIntError;

use itertools::Itertools;
use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{
    ciphers::Cipher,
    errors::Error,
    global_rng::{get_global_rng, seed_global_rng},
    text_aux::{shuffled_str, PresetAlphabet},
};

/*
BATCO is an example of a tactical cipher, one meant to be used quickly to send
simple tactical messages. Its security is not in the algorithm itself but rather
comes from three sources. First the BATCO message is meant to be a code looked
up in a set of vocabulary cards, without these cards even a deciphered message
is nearly useless. Second the messages are required to be short, no more than 22
digits, which limits the amount of cipher text available to an attacker. Finally,
tactical information is relevant only for minutes or hours making serious
cryptanalysis a waste of resources for the attacker.

Tactical ciphers are still taught in some armed forces but in practice have been
replaced by secure voice channels. These are radios that uses modern digital
encryption to transmit data.
*/

const BATCO_DIGITS: [&'static str; 13] = [
    "0", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "CH", ".",
];

const BATCO_ROWS_DEFAULT: [&'static str; 26] = [
    "ITLHCFRUMGSQKYXPVBODNEJAWZ",
    "SDEMFRTOVULJBAWZGIPYHKNQCX",
    "HZRCVTLSGEJMDYAFQNBIOPXUWK",
    "SOZWBFPAHIELNCGMJVUDRYTKXQ",
    "BLUSTQYRWCHNGIJAKFOVXMEZPD",
    "GPDQOBUYFHRZJSNLIWKATVMEXC",
    "PYLJRQUBIMWHEVFCZSODKXTANG",
    "TCOXFKSVLDZIPAGRHJYEUWMNBQ",
    "KMDXZGYLUEOAPIFQHTNSWRVJBC",
    "PQSZVBEWCITKOUDAMYLJRGFNXH",
    "BYHJTIGCUREKQXPFLNWOZVMSAD",
    "SJMBPITVKWNACEGLZOQRXUHYFD",
    "ZCTBEAHYKQFVRUDWNLXJIPGOSM",
    "LSUAQVYKINOMBJTGFRDWHXCPEZ",
    "ILJSUPFWTRQZMCOHYBKNDXGEAV",
    "KQLFBHMVDIOPEYZTSGUAWCJRXN",
    "JYVXFKQELPIZSBMHDGAOUNWTCR",
    "EPMLZITKCVBOGSRWAUJDXQHFNY",
    "GJCTODPKUBMQZAVINXFSLYRHEW",
    "CMPSUEANDLGOKTIYRBXFWQVZJH",
    "EQLMHKUNYTDIZOXBCRAJFGSPWV",
    "EGYAPINBDOURTWMQZCHXFJKSVL",
    "DLKOPSXTIJEBWCGAMUZQNYRHFV",
    "UPXBTHERCFIYLGWSZNMQAJKOVD",
    "NPRLWDXOEHSJQUMGBTCFKIYAVZ",
    "QVJWTBUFZENKMSRGDAIYPOCLXH",
];

const BATCO_COLS_DEFAULT: [&'static str; 6] = [
    "VHKOXNAMRUSYEPFLTWCJIBDZGQ",
    "YBSMRVKDQWEPXCUZTHNGAOIJFL",
    "SNJTQUDLHRXYGBVKIEPFMZACWO",
    "ZTPHROFMLYQJEXADSINVKWCUGB",
    "RZWXHJNGIOFUDVBLSTPACQYMEK",
    "QIUBPGXMYNOFRLAVEWCHDZKJST",
];

#[derive(Clone, Debug)]
pub struct Batco {
    cipher_rows: Vec<String>,
    key_cols: Vec<String>,
    pub message_number: u8, // easy conversion with char
    pub message_letter: u8, // easy conversion with char
    pub seed_string: String,
    pub seed: u64,
}

impl Default for Batco {
    fn default() -> Self {
        Batco {
            cipher_rows: BATCO_ROWS_DEFAULT.iter().map(|x| x.to_string()).collect(),
            key_cols: BATCO_COLS_DEFAULT.iter().map(|x| x.to_string()).collect(),
            message_number: 0,
            message_letter: 0,
            seed_string: String::from("0"),
            seed: 0,
        }
    }
}

impl Batco {
    pub fn message_letter_to_char(&self) -> char {
        (self.message_letter + 65) as char
    }

    pub fn message_number_to_char(&self) -> char {
        (self.message_number + 50) as char
    }

    pub fn randomize_seeded(&mut self) -> Result<(), ParseIntError> {
        self.seed = self.seed_string.parse::<u64>()?;

        seed_global_rng(self.seed);

        let alpha = PresetAlphabet::BasicLatin.slice();
        for row in self.cipher_rows.iter_mut() {
            *row = shuffled_str(alpha, &mut get_global_rng())
        }
        for col in self.key_cols.iter_mut() {
            *col = shuffled_str(alpha, &mut get_global_rng())
        }
        Ok(())
    }

    pub fn show_code_page(&self) -> String {
        let mut s = "2 3 4 5 6 7   0  0  1  2  3  4  5  6  7  8  9 CH  .".to_string();
        for i in 0..26 {
            s.push('\n');
            for j in 0..6 {
                s.push(self.key_cols[j].chars().nth(i).unwrap());
                s.push(' ')
            }

            s.push(' ');
            let r = &self.cipher_rows[i];
            let v = r.chars().collect_vec();
            let ch = v
                .chunks(2)
                .map(|x| format!("{}{} ", x[0], x[1]))
                .collect_vec();
            for pair in ch {
                s.push_str(&pair)
            }
        }
        s
    }

    pub fn show_key_rows(&self) -> String {
        let mut page = " 0  0  1  2  3  4  5  6  7  8  9 CH  .\n".to_string();
        for row in self.cipher_rows.iter() {
            for pair in row
                .chars()
                .collect_vec()
                .chunks(2)
                .map(|x| format!("{}{} ", x[0], x[1]))
            {
                page.push_str(&pair);
            }
            page.push('\n');
        }
        page
    }

    // The keys is u8 but are defined as being a digit from 2 to 7 (to select a column) and an uppercase Latin letter (to select a row in that column)
    fn key_to_row(&self) -> Result<usize, Error> {
        if self.message_number > 6 {
            return Err(Error::key("the key number must be between 2 and 7"));
        }
        if self.message_letter > 26 {
            return Err(Error::key(
                "the key letter must be an uppercase basic Latin letter",
            ));
        }
        let column = self.message_number as usize;
        let alpha = &self.key_cols[column - 2];
        Ok(alpha
            .chars()
            .position(|x| x == self.message_letter_to_char())
            .unwrap())
    }

    fn symbol_to_number(&self, c: char) -> Result<usize, Error> {
        let v = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'C' => 10,
            '.' => 11,
            _ => {
                return Err(Error::input(
                    "the only valid symbols are digits, CH, and the period",
                ))
            }
        };
        Ok(v)
    }
}

impl Cipher for Batco {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        if text.chars().count() > 22 {
            return Err(Error::input(
                "BATCO messages are limited to 22 characters per key for security reasons",
            ));
        }

        let mut rng = StdRng::from_entropy();

        let alphabet = &self.cipher_rows[self.key_to_row()?];
        let mut symbols = text.chars();

        let breaks = [0, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26];

        let mut out = String::with_capacity(text.len());

        // loop while c is Some(char)
        while let Some(c) = symbols.next() {
            // H is ignored since it always follows C
            if c == 'H' {
                continue;
            }
            let v = self.symbol_to_number(c)?;
            // Select a random symbol from the allowed range for that number
            let pos: usize = rng.gen_range(breaks[v]..breaks[v + 1]);
            out.push(alphabet.chars().nth(pos).unwrap());
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        let alphabet = &self.cipher_rows[self.key_to_row()?];
        let symbols = text.chars();

        let mut out = String::with_capacity(text.len());
        for c in symbols {
            let pos = alphabet.chars().position(|x| x == c).unwrap() / 2;
            out.push_str(BATCO_DIGITS[pos])
        }
        Ok(out)
    }

    fn randomize(&mut self) {
        let alpha = PresetAlphabet::BasicLatin.slice();
        for row in self.cipher_rows.iter_mut() {
            *row = shuffled_str(alpha, &mut get_global_rng())
        }
        for col in self.key_cols.iter_mut() {
            *col = shuffled_str(alpha, &mut get_global_rng())
        }
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod batco_tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_test() {
        let cipher = Batco::default();
        let ptext = "THEQUICKBROWNFOX";
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(ptext, cipher.decrypt(&ctext).unwrap());
    }
}
