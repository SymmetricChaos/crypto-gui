use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    errors::Error,
    text_aux::{PresetAlphabet, VecString},
};

use super::TextScore;

pub struct SubstitutionAttack {
    pub alphabet: VecString,
    pub alphabet_string: String,
    pub depth: usize,
    pub text_scorer: TextScore,
}

impl Default for SubstitutionAttack {
    fn default() -> Self {
        Self {
            alphabet: VecString::from(PresetAlphabet::BasicLatin),
            alphabet_string: String::from(PresetAlphabet::BasicLatin),
            depth: 5,
            text_scorer: TextScore::Bigram,
        }
    }
}

impl SubstitutionAttack {
    pub fn attack_cipher(&self, text: &str) -> Result<Vec<String>, Error> {
        let unique_chars = text.chars().unique().collect_vec();

        let mut candidates = Vec::with_capacity(self.depth + 1);
        candidates.push((text.to_string(), self.text_scorer.score(text)));

        for _ in 0..10_000 {
            let new_alphabet: Vec<char> = todo!("generate a possible alternative alphabet somehow");

            // let map = HashMap::from_iter(new_alphabet.iter().zip(self.alphabet.chars()));
            // let candidate: String = text.chars().map(|c| *map.get(&c).unwrap_or(&'�')).collect();

            // let score = self.text_scorer.score(&candidate);
            // if candidates[0].1 < score {
            //     candidates.insert(0, (candidate, score));
            // }
            // if candidates.len() > self.depth {
            //     candidates.pop();
            // }
        }

        Ok(candidates.into_iter().map(|(c, _)| c).collect_vec())
    }
}

#[cfg(test)]
mod substitution_attack_tests {
    use super::*;

    #[test]
    fn attack() {
        let encrypted = "JEEBMKHLVFSQQLTJLKOLLISDSCLCIEISINSUHMTTLPSUHMTTLPOMKHNQSOPAQECKHLOSQSINPLINPHMPCEKHLQKHLKMPKESPBREGLKEHLTXKHLKQERSIPPULILJLKOLLIREGLSINRFIEEIETYCXFPPMIDEDENNLPPKHLSIDLQEASUHMTTLPPEIEAXLTLFPKHSKJQEFDHKUEFIKTLPPMTTPFXEIKHLSUHSLSIPCSIYSJQSGLPEFTNMNMKPLINHFQQYMIDNEOIKEHSNLPSINCSIYSHLQENMNMKYMLTNSXQLYKENEDPSINGFTKFQLPAEQPEOLQLKHLUEFIPLTPEAREGLAFTAMTTLNAQECKHLNSYEIOHMUHKHLPEIEASKQLFPBMIDEACLISINDQLSKSUHMTTLPAMQPKALTTEFKOMKHEILSIEKHLQSINOHMUHEAKHLDENPOSPMKKHSKPLKKHLCEIKEVFSQQLTMKOSPKHLPEIEAREGLSINTLKEAEQHLOSPSIDQYOMKHKHLBMIDSINPLIKSXLPKMTLIULFXEIKHLHEPKKEXTSDFLKHLXLEXTLJLUSFPLKHLPEIEASKQLFPHSNNMPHEIEFQLNUHQYPLPHMPXQMLPKIEOUHQYPLPHSNUECLKEKHLPHMXPEAKHLSUHSLSIPKEAQLLHMPNSFDHKLQSINHSNJQEFDHKOMKHHMCSDQLSKQSIPECCEQLEGLQHLJEQLMIHMPHSINKHLPULXKQLEASXETTEOQLSKHLNOMKHSPFXXTMSIKPOQLSKHSINHLJLPEFDHKKHLSUHSLSIPJFKCEPKEASTTKHLKOEPEIPEASKQLFPOHEOLQLKHLMQUHMLAP";
        let attacker = SubstitutionAttack::default();
        println!("{:?}", attacker.attack_cipher(encrypted))
    }
}
