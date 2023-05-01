use super::{CipherAttack, TextScore};
use crate::{
    errors::Error,
    text_aux::{text_functions::validate_text, PresetAlphabet, VecString},
};

pub struct CaesarAttack {
    pub alphabet: VecString,
    pub alphabet_string: String,
    pub depth: usize,
    pub text_scorer: TextScore,
}

impl Default for CaesarAttack {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            depth: 5,
            text_scorer: TextScore::Bigram,
        }
    }
}

impl CaesarAttack {
    pub fn set_alphabet(&mut self) {
        self.alphabet = VecString::unique_from(&self.alphabet_string);
    }
}

impl CipherAttack for CaesarAttack {
    fn attack_cipher(&self, text: &str) -> Result<String, Error> {
        validate_text(text, &self.alphabet)?;
        let n_trials = self.alphabet.len() as i32;

        // Initialize output and score with raw input
        let mut top_output = text.to_string();
        let mut top_score = self.text_scorer.score(text);

        for offset in 1..n_trials {
            let candidate: String = text
                .chars()
                .map(|c| self.alphabet.get_shifted_char(c, offset).unwrap())
                .collect();
            let score = self.text_scorer.score(&candidate);
            if top_score < score {
                top_output = candidate;
                top_score = score;
            }
        }
        Ok(top_output)
    }
}

#[cfg(test)]
mod caesar_attack_tests {
    use super::*;

    #[test]
    fn attack() {
        let encrypted = "WKHTXLFNEURZQIRAMXPSVRYHUWKHODCBGRJ";
        let attacker = CaesarAttack::default();
        println!("{}", attacker.attack_cipher(encrypted).unwrap())
    }
}
