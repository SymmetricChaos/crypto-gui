use itertools::Itertools;
use rand::{prelude::StdRng, thread_rng, Rng, SeedableRng};

use crate::{errors::CipherError, text_aux::PresetAlphabet};
use crate::text_aux::shuffled_str;
use super::Cipher;


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

const BATCO_DIGITS: [&'static str; 13] = ["0", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "CH", "."];

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

#[derive(Clone,Debug)]
pub struct Batco {
    cipher_rows: Vec<String>,
    key_cols: Vec<String>,
    message_key: (char,char),
    seed: Option<u64>
}


impl Default for Batco {
    fn default() -> Self {
        Batco { 
            cipher_rows: BATCO_ROWS_DEFAULT.iter().map(|x| x.to_string()).collect(),
            key_cols: BATCO_COLS_DEFAULT.iter().map(|x| x.to_string()).collect(),
            message_key: ('2','A'),
            seed: None,
        }
    }
}


impl Batco {

    fn randomize_seeded(&mut self) {

        // Setup an RNG, otherwise immediately stop
        let mut rng = if self.seed.is_some() {
            StdRng::seed_from_u64(self.seed.unwrap())
        } else {
            return ()
        };
    
        let alpha = PresetAlphabet::BasicLatin.slice();
        for idx in 0..26 {
            self.cipher_rows[idx] = shuffled_str(alpha, &mut rng)
        }

        for idx in 0..7 {
            self.key_cols[idx] = shuffled_str(alpha, &mut rng)
        }
    }

    pub fn show_code_page(&self) -> String {
        let mut s = "2 3 4 5 6 7   0  0  1  2  3  4  5  6  7  8  9 CH  .".to_string();
        for i in 0..26 {
            s.push('\n');
            for j in 0..6 {
                s.push( self.key_cols[j].chars().nth(i).unwrap() );
                s.push(' ')
            }

            s.push(' ');
            let r = &self.cipher_rows[i];
            let v = r.chars().collect_vec();
            let ch = v.chunks(2).map(|x| format!("{}{} ",x[0],x[1])).collect_vec();
            for pair in ch {
                s.push_str(&pair)
            }
        }
        s
    }

    pub fn show_key_rows(&self) -> String {
        let mut page = " 0  0  1  2  3  4  5  6  7  8  9 CH  .\n".to_string();
        for row in self.cipher_rows.iter() {
            for pair in row.chars().collect_vec().chunks(2).map(|x| format!("{}{} ",x[0],x[1])) {
                page.push_str(&pair);
            }
            page.push('\n');
        }
        page
    }

    // The key is usize but its defined by a digit from 2 to 7 (to select a column) and a letter (to select a row in that column)
    // fn key_to_row(&self) -> usize {
    //     let x = c.0.to_digit(10).unwrap() as usize;
    //     let alpha = &self.key_cols[x-2];
    //     alpha.chars().position(|x| x == c.1).unwrap()
    // }
}



impl Cipher for Batco {

    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        // if text.chars().count() > 22 {
        //     return Err(CipherError::input("BATCO messages are limited to 22 characters per key for security reasons"))
        // }
        // let mut rng = thread_rng();
        // let alphabet = &self.cipher_rows[self.message_key.get()];
        // let mut symbols = text.chars();
        // let breaks = [0,4,6,8,10,12,14,16,18,20,22,24,26];

        // let mut out = String::with_capacity(text.len());
        // // loop while c is Some(char)
        // while let Some(c) = symbols.next() {
        //     // H is ignored since it always follows C
        //     if c == 'H' { continue }
        //     // Convert the symbol to a number
        //     let v = match c {
        //         '0' => 0,
        //         '1' => 1,
        //         '2' => 2,
        //         '3' => 3,
        //         '4' => 4,
        //         '5' => 5,
        //         '6' => 6,
        //         '7' => 7,
        //         '8' => 8,
        //         '9' => 9,
        //         'C' => 10,
        //         '.' => 11,
        //         _ => return Err(CipherError::input("the only valid symbols are digits, CH, and the period"))
        //     };

        //     // Select a random symbol from the allowed range for that number
        //     let pos: usize = rng.gen_range(breaks[v]..breaks[v+1]);
        //     out.push( alphabet.chars().nth(pos).unwrap() );
        // }
        // Ok(out)
        todo!("encrypt")
    }

    fn decrypt(&self, text: &str) -> Result<String,CipherError> {
        // let alphabet = &self.cipher_rows[self.message_key.get()];
        // let symbols = text.chars();

        // let mut out = String::with_capacity(text.len());
        // for c in symbols {
        //     let pos = alphabet.chars().position(|x| x == c).unwrap()/2;
        //     out.push_str(BATCO_DIGITS[pos])
        // }
        // Ok(out)
        todo!("decrypt")
    }
    
    fn randomize(&mut self, rng: &mut StdRng) {
    
        let alpha = PresetAlphabet::BasicLatin.slice();
        for idx in 0..26 {
            self.cipher_rows[idx] = shuffled_str(alpha, rng)
        }

        for idx in 0..7 {
            self.key_cols[idx] = shuffled_str(alpha, rng)
        }
    }
    
    fn reset(&mut self) {
        *self = Self::default();
    }
}





#[cfg(test)]
mod batco_tests {
    use super::*;

    // #[test]
    // fn key_rows() {
    //     let cipher = Batco::default();
    //     println!("{}",cipher.show_key_rows());
    // }
}