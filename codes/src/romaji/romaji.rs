use crate::{errors::CodeError, traits::Code};

use super::{
    tokenizer::{Node, TransitionError},
    HEPBERN_SHIKI, HIRAGANA, KUNREI_SHIKI, NIHON_SHIKI, ROMAJI_TO_KANA,
};

pub fn to_romaji(orig: &str, tree: &Node) -> Result<String, TransitionError> {
    let tokens = tree.extract_tokens(orig)?;

    Ok(tokens.iter().cloned().collect())
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RomajiVariant {
    Nihon,
    Hepbern,
    Kunrei,
}

impl RomajiVariant {
    pub fn root(&self) -> &Node {
        match self {
            RomajiVariant::Nihon => &NIHON_SHIKI,
            RomajiVariant::Hepbern => &HEPBERN_SHIKI,
            RomajiVariant::Kunrei => &KUNREI_SHIKI,
        }
    }
}

pub struct Romaji {
    pub variant: RomajiVariant,
}

impl Romaji {
    pub fn hiragana_codes(&self) -> impl Iterator<Item = (&&str, String)> + '_ {
        HIRAGANA
            .iter()
            .map(|kana| (kana, self.encode(kana).unwrap()))
    }
}

impl Default for Romaji {
    fn default() -> Self {
        Self {
            variant: RomajiVariant::Kunrei,
        }
    }
}

impl Code for Romaji {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let tokens = self
            .variant
            .root()
            .extract_tokens(text)
            .map_err(|e| CodeError::General(e.to_string()))?;

        Ok(tokens.iter().cloned().collect())
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let tokens = ROMAJI_TO_KANA
            .extract_tokens(&text.to_lowercase())
            .map_err(|e| CodeError::General(e.to_string()))?;

        Ok(tokens.iter().cloned().collect())
    }
}

#[cfg(test)]
mod romaji_test {
    use super::*;

    const HIRAGANA_TEST: &'static str = "ちしつぢじづぢゃじゃこんにこんお";
    const LATIN_KUNREI: &'static str = "TISITUZIZIZUZYAZYAKONNIKON'O";
    const LATIN_HEBERN: &'static str = "CHISHITSUJIJIZUJAJAKONNIKON'O";
    const LATIN_NIHON: &'static str = "TISITUDIZIDUDYAZYAKONNIKON'O";

    #[test]
    fn kunrei_kana_to_latin() {
        let code = Romaji::default();
        assert_eq!(
            code.encode(HIRAGANA_TEST).unwrap().to_ascii_uppercase(),
            LATIN_KUNREI
        );
    }

    #[test]
    fn kunrei_latin_to_kana() {
        let code = Romaji::default();
        assert_eq!(
            code.decode(LATIN_KUNREI).unwrap().to_ascii_uppercase(),
            HIRAGANA_TEST
        );
    }

    #[test]
    fn nihon_kana_to_latin() {
        let mut code = Romaji::default();
        code.variant = RomajiVariant::Nihon;
        assert_eq!(
            code.encode(HIRAGANA_TEST).unwrap().to_ascii_uppercase(),
            LATIN_NIHON
        );
    }

    #[test]
    fn nihon_latin_to_kana() {
        let mut code = Romaji::default();
        code.variant = RomajiVariant::Nihon;
        assert_eq!(
            code.decode(LATIN_NIHON).unwrap().to_ascii_uppercase(),
            HIRAGANA_TEST
        );
    }

    #[test]
    fn hepbern_kana_to_latin() {
        let mut code = Romaji::default();
        code.variant = RomajiVariant::Hepbern;
        assert_eq!(
            code.encode(HIRAGANA_TEST).unwrap().to_ascii_uppercase(),
            LATIN_HEBERN
        );
    }

    #[test]
    fn hepbern_latin_to_kana() {
        let mut code = Romaji::default();
        code.variant = RomajiVariant::Hepbern;
        assert_eq!(
            code.decode(LATIN_HEBERN).unwrap().to_ascii_uppercase(),
            HIRAGANA_TEST
        );
    }
}
