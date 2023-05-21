use crate::{errors::CodeError, global_rng::get_global_rng};
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use std::collections::HashMap;
use utils::{preset_alphabet::PresetAlphabet, vecstring::VecString};

use super::{CipherAttack, TextScorer};

pub struct SubstitutionAttack {
    pub num_trials: usize,
    pub quit_number: usize,
    pub text_scorer: TextScorer,
}

impl Default for SubstitutionAttack {
    fn default() -> Self {
        Self {
            num_trials: 200_000,
            quit_number: 2000,
            text_scorer: TextScorer::Bigram,
        }
    }
}

impl CipherAttack for SubstitutionAttack {
    fn attack_cipher(&self, text: &str) -> Result<String, CodeError> {
        let mut unique_chars = text.chars().unique().collect_vec();

        let alphabet = VecString::from(PresetAlphabet::BasicLatin);

        // Initialize output and score with raw input
        let mut top_output = text.to_string();
        let mut top_score = self.text_scorer.score(text);

        // Randomize the alphabet at the start of the round and set counter to 0
        let mut rng = get_global_rng();
        unique_chars.as_mut_slice().shuffle::<StdRng>(&mut rng);
        let mut trials_without_improvement = 0;

        for _trial in 0..self.num_trials {
            // Mutate the alphabet
            let a = rng.gen_range(0..unique_chars.len());
            let b = rng.gen_range(0..unique_chars.len());
            unique_chars.swap(a, b);

            // Build the mapping
            let map = {
                let mut hash_map = HashMap::new();
                for (a, b) in unique_chars.iter().zip(alphabet.chars()) {
                    hash_map.insert(*a, b);
                }
                hash_map
            };

            // Create a candidate decryption and score it
            let candidate: String = text.chars().map(|c| *map.get(&c).unwrap_or(&'�')).collect();
            let score = self.text_scorer.score(&candidate);

            // If the score is better than our best make it our new best and reset the counter
            if top_score < score {
                top_output = candidate;
                top_score = score;
                trials_without_improvement = 0;
            } else {
                // Otherwise undo the swap and increase the counter
                unique_chars.swap(a, b);
                trials_without_improvement += 1;
            }
            if trials_without_improvement == self.quit_number {
                break;
            }
        }

        Ok(top_output)
    }

    fn get_text_scorer(&mut self) -> &mut TextScorer {
        &mut self.text_scorer
    }
}

#[cfg(test)]
mod substitution_attack_tests {
    use super::*;

    #[test]
    fn attack() {
        let encrypted = "SOWFBRKAWFCZFSBSCSBQITBKOWLBFXTBKOWLSOXSOXFZWWIBICFWUQLRXINOCIJLWJFQUNWXLFBSZXFBTXAANTQIFBFSFQUFCZFSBSCSBIMWHWLNKAXBISWGSTOXLXTSWLUQLXJBUUWLWISTBKOWLSWGSTOXLXTSWLBSJBUUWLFULQRTXWFXLTBKOWLBISOXSSOWTBKOWLXAKOXZWSBFIQSFBRKANSOWXAKOXZWSFOBUSWJBSBFTQRKAWSWANECRZAWJ";
        let attacker = SubstitutionAttack::default();
        println!("{}", attacker.attack_cipher(encrypted).unwrap())
    }
}
