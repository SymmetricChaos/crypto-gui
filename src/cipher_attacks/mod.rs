pub mod caesar_attack;

use csv;
use lazy_static::lazy_static;
use std::collections::HashMap;

const TWO_GRAM_DATA: &'static str = include_str!("2_gram_scores.csv");
const THREE_GRAM_DATA: &'static str = include_str!("3_gram_scores.csv");

lazy_static! {
    pub static ref BIGRAM_LOGPROB: HashMap<String, i64> = {
        let mut reader = csv::ReaderBuilder::new().from_reader(TWO_GRAM_DATA.as_bytes());
        let mut map = HashMap::new();
        for record in reader.records() {
            let fields = record.expect("failure reading row of bigrama data");
            map.insert(
                fields[0].to_string(),
                i64::from_str_radix(&fields[2], 10)
                    .expect("failure to convert log probability to i64"),
            );
        }
        map
    };
    pub static ref TRIGRAM_LOGPROB: HashMap<String, i64> = {
        let mut reader = csv::ReaderBuilder::new().from_reader(THREE_GRAM_DATA.as_bytes());
        let mut map = HashMap::new();
        for record in reader.records() {
            let fields = record.expect("failure reading row of trigrama data");
            map.insert(
                fields[0].to_string(),
                i64::from_str_radix(&fields[2], 10)
                    .expect("failure to convert log probability to i64"),
            );
        }
        map
    };
}

// From StackOverflow
fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

// Log probability score for a text, higher (closer to zero) is better. The number has no real meaning on its own, it is only useful for comparison.
// Overflow happens at a text size of several quadrillion characters
pub fn score_bigrams(text: &str) -> i64 {
    char_windows(text, 2)
        .map(|s| BIGRAM_LOGPROB.get(s).unwrap())
        .sum()
}

pub fn score_trigrams(text: &str) -> i64 {
    char_windows(text, 3)
        .map(|s| TRIGRAM_LOGPROB.get(s).unwrap_or(&-5000))
        .sum()
}

pub enum TextScore {
    Bigram,
    Trigram,
}

impl TextScore {
    pub fn score(&self, text: &str) -> i64 {
        match self {
            TextScore::Bigram => score_bigrams(text),
            TextScore::Trigram => score_trigrams(text),
        }
    }
}
#[cfg(test)]
mod cipher_attack_tests {
    use super::*;

    #[test]
    fn pairs() {
        let x = "SOMETEXTIWROTE";
        println!("{:?}", score_bigrams(x));
    }
}
