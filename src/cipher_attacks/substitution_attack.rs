use crate::{
    errors::Error,
    global_rng::get_global_rng,
    text_aux::{PresetAlphabet, VecString},
};
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use std::collections::HashMap;

use super::TextScore;

pub struct SubstitutionAttack {
    pub alphabet: VecString,
    pub alphabet_string: String,
    pub num_trials: usize,
    pub num_cadidates: usize,
    pub text_scorer: TextScore,
}

impl Default for SubstitutionAttack {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            num_trials: 200_000,
            num_cadidates: 5,
            text_scorer: TextScore::Trigram,
        }
    }
}

impl SubstitutionAttack {
    pub fn attack_cipher(&self, text: &str) -> Result<Vec<String>, Error> {
        let mut unique_chars = text.chars().unique().collect_vec();

        let mut candidates = Vec::with_capacity(self.num_cadidates + 1);
        candidates.push((text.to_string(), self.text_scorer.score(text)));

        let mut rng = get_global_rng();
        // Randomize the alphabet at the start of the round and set counter to 0
        unique_chars.as_mut_slice().shuffle::<StdRng>(&mut rng);
        let mut trials_without_improvement = 0;
        for _trial in 0..self.num_trials {
            // Mutate the alphabet
            let a = rng.gen_range(0..unique_chars.len());
            let b = rng.gen_range(0..unique_chars.len());
            unique_chars.swap(a, b);

            // Build
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
            if candidates[0].1 < score {
                candidates.insert(0, (candidate, score));
                println!("{}", unique_chars.iter().collect::<String>());
                trials_without_improvement = 0;
                // If we have too many candidates remove one
                if candidates.len() > self.num_cadidates {
                    candidates.pop();
                }
            } else {
                // Otherwise undo the swap and increase the counter
                unique_chars.swap(a, b);
                trials_without_improvement += 1;
            }
            if trials_without_improvement == 5_000 {
                break;
            }
        }

        Ok(candidates.into_iter().map(|(c, _)| c).collect_vec())
    }
}

#[cfg(test)]
mod substitution_attack_tests {
    use super::*;

    #[test]
    fn attack() {
        let encrypted = "SOWFBRKAWFCZFSBSCSBQITBKOWLBFXTBKOWLSOXSOXFZWWIBICFWUQLRXINOCIJLWJFQUNWXLFBSZXFBTXAANTQIFBFSFQUFCZFSBSCSBIMWHWLNKAXBISWGSTOXLXTSWLUQLXJBUUWLWISTBKOWLSWGSTOXLXTSWLBSJBUUWLFULQRTXWFXLTBKOWLBISOXSSOWTBKOWLXAKOXZWSBFIQSFBRKANSOWXAKOXZWSFOBUSWJBSBFTQRKAWSWANECRZAWJ";
        let attacker = SubstitutionAttack::default();
        println!("{:#?}", attacker.attack_cipher(encrypted).unwrap())
    }
}
