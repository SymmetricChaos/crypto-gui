use super::TextScore;
use crate::{
    errors::Error,
    text_aux::{text_functions::validate_text, PresetAlphabet, VecString},
};
use itertools::Itertools;

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
    pub fn attack_cipher(&self, text: &str) -> Result<Vec<String>, Error> {
        validate_text(text, &self.alphabet)?;
        let n_trials = self.alphabet.len() as i32;
        let mut candidates = Vec::with_capacity(self.depth + 1);
        candidates.push((text.to_string(), self.text_scorer.score(text)));
        for offset in 1..n_trials {
            let candidate: String = text
                .chars()
                .map(|c| self.alphabet.get_shifted_char(c, offset).unwrap())
                .collect();
            let score = self.text_scorer.score(&candidate);
            if candidates[0].1 < score {
                candidates.insert(0, (candidate, score));
            }
            if candidates.len() > self.depth {
                candidates.pop();
            }
        }
        Ok(candidates.into_iter().map(|(c, _)| c).collect_vec())
    }
}

#[cfg(test)]
mod caesar_attack_tests {
    use super::*;

    #[test]
    fn attack() {
        let encrypted = "WKHTXLFNEURZQIRAMXPSVRYHUWKHODCBGRJ";
        let attacker = CaesarAttack::default();
        println!("{:?}", attacker.attack_cipher(encrypted))
    }
}
