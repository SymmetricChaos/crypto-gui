use utils::{preset_alphabet::PresetAlphabet, vecstring::VecString};

use super::{CipherAttack, TextScorer};
use crate::errors::Error;

pub struct CaesarAttack {
    pub text_scorer: TextScorer,
}

impl Default for CaesarAttack {
    fn default() -> Self {
        Self {
            text_scorer: TextScorer::Bigram,
        }
    }
}

impl CipherAttack for CaesarAttack {
    fn attack_cipher(&self, text: &str) -> Result<String, Error> {
        let alphabet = VecString::from(PresetAlphabet::BasicLatin);

        let n_trials = alphabet.len() as i32;

        // Initialize output and score with raw input
        let mut top_output = text.to_string();
        let mut top_score = self.text_scorer.score(text);

        for offset in 1..n_trials {
            let candidate: String = text
                .chars()
                .map(|c| alphabet.get_shifted_char(c, offset).unwrap())
                .collect();
            let score = self.text_scorer.score(&candidate);
            if top_score < score {
                top_output = candidate;
                top_score = score;
            }
        }
        Ok(top_output)
    }

    fn get_text_scorer(&mut self) -> &mut TextScorer {
        &mut self.text_scorer
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
