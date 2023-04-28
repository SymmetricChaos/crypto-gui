use crate::text_aux::{PresetAlphabet, VecString};

use super::score_bigrams;

pub struct CaesarAttack {
    pub alphabet: VecString,
    pub alphabet_string: String,
}

impl Default for CaesarAttack {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
        }
    }
}

impl CaesarAttack {
    pub fn attack_cipher(&self, text: &str) -> Vec<String> {
        let n_trials = self.alphabet.len() as i32;
        let mut out = Vec::new();
        for offset in 0..n_trials {
            out.push(
                text.chars()
                    .map(|c| {
                        self.alphabet
                            .get_shifted_char(c, offset)
                            .expect("attack doesn't handle unknown characters")
                    })
                    .collect::<String>(),
            )
        }
        out.sort_by(|s, t| score_bigrams(t).cmp(&score_bigrams(s)));
        out
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
