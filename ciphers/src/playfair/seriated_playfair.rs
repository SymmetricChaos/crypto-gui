use super::Playfair;
use crate::Cipher;
use itertools::Itertools;
use utils::{errors::GeneralError, text_functions::string_chunks};

pub struct SeriatedPlayfair {
    pub period: usize,
    pub playfair: Playfair,
}

impl Default for SeriatedPlayfair {
    fn default() -> Self {
        Self {
            period: 5,
            playfair: Playfair::default(),
        }
    }
}

impl SeriatedPlayfair {
    pub fn assign_key(&mut self, keyword: &str, alphabet: &str) {
        self.playfair.assign_key(keyword, alphabet);
    }

    pub fn groups(&self, text: &str) -> Vec<String> {
        let mut chunks = string_chunks(text, self.period);

        // if there are an even number of chunks fill out the last one with spacers
        if chunks.len() % 2 == 0 {
            let x = chunks.last_mut().unwrap();
            while x.len() != self.period {
                x.push(self.playfair.spacer)
            }
        // if there are an odd number of chunks split the last one in half and fill out the second half with spacers
        } else {
            let last = chunks.pop().unwrap();
            let len = last.len();
            let left: String = last.chars().take(len / 2).collect();
            let right: String = last.chars().skip(len / 2).collect();
            chunks.push(left);
            chunks.push(right);
            let x = chunks.last_mut().unwrap();
            while x.len() != len / 2 {
                x.push(self.playfair.spacer)
            }
        }
        chunks
    }
}

impl Cipher for SeriatedPlayfair {
    fn encrypt(&self, text: &str) -> Result<String, GeneralError> {
        let groups = self.groups(text);
        let mut out = String::new();
        for (l_str, r_str) in groups
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|c| c.collect_tuple().unwrap())
        {
            let mut left_out = String::new();
            let mut right_out = String::new();
            for (l, r) in l_str.chars().zip(r_str.chars()) {
                if l == r {
                    return Err(GeneralError::input(format!(
                        "found repeated character {l}, a spacer should be inserted",
                    )));
                }
                let lpos = self.playfair.char_to_position(l)?;
                let rpos = self.playfair.char_to_position(r)?;
                let pair = self.playfair.playfair_shift(lpos, rpos, true);
                left_out.push(pair.0);
                right_out.push(pair.1);
            }
            out.push_str(&left_out);
            out.push_str(&right_out);
        }

        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, GeneralError> {
        let groups = self.groups(text);
        let mut out = String::new();
        for (l_str, r_str) in groups
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|c| c.collect_tuple().unwrap())
        {
            let mut left_out = String::new();
            let mut right_out = String::new();
            for (l, r) in l_str.chars().zip(r_str.chars()) {
                if l == r {
                    return Err(GeneralError::input(format!(
                        "found repeated character {l}, a spacer should be inserted",
                    )));
                }
                let lpos = self.playfair.char_to_position(l)?;
                let rpos = self.playfair.char_to_position(r)?;
                let pair = self.playfair.playfair_shift(lpos, rpos, false);
                left_out.push(pair.0);
                right_out.push(pair.1);
            }
            out.push_str(&left_out);
            out.push_str(&right_out);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod seriated_playfair_tests {
    use utils::preset_alphabet::Alphabet;

    use super::*;

    #[test]
    fn chunks() {
        let mut cipher = SeriatedPlayfair::default();
        cipher.period = 5;
        assert_eq!(
            cipher.groups("AAAAAAAAAAAAAAAAAAAAAAA"),
            ["AAAAA", "AAAAA", "AAAAA", "AAAAA", "AA", "AX"]
        );
        cipher.period = 6;
        assert_eq!(
            cipher.groups("AAAAAAAAAAAAAAAAAAAAAAA"),
            ["AAAAAA", "AAAAAA", "AAAAAA", "AAAAAX"]
        );
    }

    #[test]
    fn encrypt_test_example() {
        let mut cipher = SeriatedPlayfair::default();
        cipher.assign_key("SERIATEDPLAYFAIR", Alphabet::BasicLatinNoJ.slice());
        cipher.period = 7;
        assert_eq!(cipher.encrypt("BABBAGESRULENOMANSCIPHERISWORTHLOOKINGATUNLESSTHEINVENTORHASHIMSELFSOLVEDAVERYDIFFICULTCIPHERXTHECODEBREAKERSBYKAHNX").unwrap(), "FSFGSCIEIVDROMQSWEFRLBRPARXNIPFYKKMAKHILXOGREEPFILMURKYMIBITFLOEAYKAXDZDLSURPDBEHBANVPLFADFSIUPGRGQBRFEASMDIECDQRGQZ");
    }

    #[test]
    fn decrypt_test_example() {
        let mut cipher = SeriatedPlayfair::default();
        cipher.assign_key("SERIATEDPLAYFAIR", Alphabet::BasicLatinNoJ.slice());
        cipher.period = 7;
        assert_eq!(cipher.decrypt("FSFGSCIEIVDROMQSWEFRLBRPARXNIPFYKKMAKHILXOGREEPFILMURKYMIBITFLOEAYKAXDZDLSURPDBEHBANVPLFADFSIUPGRGQBRFEASMDIECDQRGQZ").unwrap(), "BABBAGESRULENOMANSCIPHERISWORTHLOOKINGATUNLESSTHEINVENTORHASHIMSELFSOLVEDAVERYDIFFICULTCIPHERXTHECODEBREAKERSBYKAHNX");
    }
}
