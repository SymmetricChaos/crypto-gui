use crate::{
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{PresetAlphabet, VecString},
};
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use std::collections::HashMap;

use super::{CipherAttack, TextScore};

pub struct Substitution {
    pub alphabet: VecString,
    pub alphabet_string: String,
    pub num_trials: usize,
    pub num_cadidates: usize,
    pub quit_number: usize,
    pub text_scorer: TextScore,
}

impl Default for Substitution {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            num_trials: 200_000,
            num_cadidates: 5,
            quit_number: 2000,
            text_scorer: TextScore::Bigram,
        }
    }
}

impl CipherAttack for Substitution {
    fn attack_cipher(&self, text: &str) -> Result<String, Error> {
        let mut unique_chars = text.chars().unique().collect_vec();

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
                for (a, b) in unique_chars.iter().zip(self.alphabet.chars()) {
                    hash_map.insert(*a, b);
                }
                hash_map
            };

            // Create a candidate decryption and score it
            let candidate: String = text.chars().map(|c| *map.get(&c).unwrap_or(&'�')).collect();
            let score = self.text_scorer.score(&candidate);

            // If the score is better than out best insert it at the head of the list and reset the counter
            if top_score < score {
                top_output = candidate;
                top_score = score;
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
}

#[cfg(test)]
mod substitution_attack_tests {
    use super::*;

    #[test]
    fn attack() {
        let encrypted = "SOWFBRKAWFCZFSBSCSBQITBKOWLBFXTBKOWLSOXSOXFZWWIBICFWUQLRXINOCIJLWJFQUNWXLFBSZXFBTXAANTQIFBFSFQUFCZFSBSCSBIMWHWLNKAXBISWGSTOXLXTSWLUQLXJBUUWLWISTBKOWLSWGSTOXLXTSWLBSJBUUWLFULQRTXWFXLTBKOWLBISOXSSOWTBKOWLXAKOXZWSBFIQSFBRKANSOWXAKOXZWSFOBUSWJBSBFTQRKAWSWANECRZAWJ";
        let attacker = Substitution::default();
        println!("{}", attacker.attack_cipher(encrypted).unwrap())
    }
}
