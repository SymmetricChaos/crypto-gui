use crate::{errors::CodeError, traits::Code};
use bimap::BiMap;
use lazy_static::lazy_static;
use regex::Regex;
use utils::text_functions::bimap_from_iter;

const SEMAPHORE_MEANING: [&'static str; 30] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "numeric", "cancel", "error", "ready",
];

const SEMAPHORE_POSITIONS: [&'static str; 30] = [
    "down/low",
    "down/out",
    "down/high",
    "down/up",
    "high/down",
    "out/down",
    "low/down",
    "across-low/out",
    "across-low/up",
    "out/up",
    "up/low",
    "high/low",
    "out/low",
    "low/low",
    "across-high/out",
    "up/out",
    "high/out",
    "out/out",
    "low/out",
    "up/high",
    "high/high",
    "low/up",
    "out/across-high",
    "low/across-high",
    "out/high",
    "out/across-low",
    "high/up",
    "low/high",
    "both-raised-and-lowered",
    "low/low",
];

lazy_static! {
    pub static ref SEMAPHORE_MAP: BiMap<&'static str, &'static str> = bimap_from_iter(
        SEMAPHORE_MEANING
            .into_iter()
            .zip(SEMAPHORE_POSITIONS.into_iter())
    );
    pub static ref SEMAPHORE_REGEX: Regex =
        Regex::new(r"[A-Z0-9 ]|numeric|cancel|error|ready|.").unwrap();
}

pub struct Semaphore {}

impl Semaphore {
    pub fn chars_codes(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        Box::new(
            SEMAPHORE_MEANING
                .into_iter()
                .zip(SEMAPHORE_POSITIONS.into_iter()),
        )
    }
}

impl Default for Semaphore {
    fn default() -> Self {
        Self {}
    }
}

impl Code for Semaphore {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut numeric_mode = false;
        let mut out = Vec::with_capacity(text.len());
        for symbol in SEMAPHORE_REGEX
            .captures_iter(text)
            .map(|cap| cap.get(0).unwrap().as_str())
        {
            // Ignore spaces
            if symbol == " " {
                continue;
            }

            // Before encoding digits set numeric mode and insert the numeric code
            if ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&symbol) {
                if !numeric_mode {
                    numeric_mode = true;
                    out.push("high/up")
                }
            }

            // When in numeric mode any non-numeric value turns it off and requires J be inserted
            if numeric_mode && !["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&symbol)
            {
                numeric_mode = false;
                out.push("out/up")
            }
            let symbol = match symbol {
                "1" => "A",
                "2" => "B",
                "3" => "C",
                "4" => "D",
                "5" => "E",
                "6" => "F",
                "7" => "G",
                "8" => "H",
                "9" => "I",
                "0" => "K",
                _ => symbol,
            };

            match SEMAPHORE_MAP.get_by_left(symbol) {
                Some(code) => out.push(*code),
                None => return Err(CodeError::invalid_input_group(symbol)),
            }
        }
        Ok(out.join(" "))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let mut numeric_mode = false;
        let mut output = String::new();
        for code in text.split(" ") {
            // These control numeric mode and print nothing
            if code == "high/up" {
                numeric_mode = true;
                continue;
            }
            if numeric_mode && code == "out/up" {
                numeric_mode = false;
                continue;
            }

            let symbol = if numeric_mode {
                let letter = *SEMAPHORE_MAP
                    .get_by_right(code)
                    .ok_or_else(|| CodeError::invalid_input_group(code))?;
                match letter {
                    "A" => "1",
                    "B" => "2",
                    "C" => "3",
                    "D" => "4",
                    "E" => "5",
                    "F" => "6",
                    "G" => "7",
                    "H" => "8",
                    "I" => "9",
                    "K" => "0",
                    _ => letter,
                }
            } else {
                *SEMAPHORE_MAP
                    .get_by_right(code)
                    .ok_or_else(|| CodeError::invalid_input_group(code))?
            };

            output.push_str(symbol)
        }
        Ok(output)
    }
}

#[cfg(test)]
mod semaphore_tests {
    use super::*;

    const PLAINTEXT: &'static str = "ABready123WORD";
    const CODETEXT: &'static str = "down/low down/out low/low high/up down/low down/out down/high out/up out/across-high across-high/out out/out down/up";

    #[test]
    fn encrypt_test() {
        let code = Semaphore::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), CODETEXT);
    }

    #[test]
    fn decrypt_test() {
        let code = Semaphore::default();
        assert_eq!(code.decode(CODETEXT).unwrap(), PLAINTEXT);
    }
}
