use crate::tokenizer::{Node, TransitionError};

pub fn to_romaji(orig: &str, tree: &Node) -> Result<String, TransitionError> {
    let tokens = tree.extract_tokens(orig)?;

    Ok(tokens.iter().cloned().collect())
}

#[cfg(test)]
mod romaji_test {
    use super::*;

    use crate::codes::romaji::{HEPBERN_SHIKI, KUNREI_SHIKI, NIHON_SHIKI};
    // we check the differences between the methods
    const HIRAGANA: &'static str = "ちしつぢじづぢゃじゃこんにこんお";

    #[test]
    fn hepbern_text() {
        let latin_hs = "CHISHITSUJIJIZUJAJAKONNIKON'O";
        assert_eq!(
            to_romaji(HIRAGANA, &HEPBERN_SHIKI)
                .unwrap()
                .to_ascii_uppercase(),
            latin_hs
        );
    }

    #[test]
    fn nihon_text() {
        let latin_ns = "TISITUDIZIDUDYAZYAKONNIKON'O";
        assert_eq!(
            to_romaji(HIRAGANA, &NIHON_SHIKI)
                .unwrap()
                .to_ascii_uppercase(),
            latin_ns
        );
    }

    #[test]
    fn kunrei_text() {
        let latin_ks = "TISITUZIZIZUZYAZYAKONNIKON'O";
        assert_eq!(
            to_romaji(HIRAGANA, &KUNREI_SHIKI)
                .unwrap()
                .to_ascii_uppercase(),
            latin_ks
        );
    }
}
