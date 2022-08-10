use crate::tokenizer::{Node, TokenError};

pub fn to_romaji(orig: &str, tree: &Node) -> Result<String, TokenError> {
    let tokens = tree.extract_tokens(orig)?;

    Ok(tokens.iter().cloned().collect())
}

#[cfg(test)]
mod romaji_test {
    use super::*;

    use crate::codes::romaji::{HEPBERN_SHIKI, KUNREI_SHIKI, NIHON_SHIKI};
    // we check the differences between the methods
    const HIRAGANA: &'static str = "ち し つ ぢ じ づ ぢゃ じゃ こんに こんお";

    #[test]
    fn hepbern_text() {
        let latin_hs = "CHI SHI TSU JI JI ZU JA JA KONNI KON'O";
        assert_eq!(
            to_romaji(HIRAGANA, &HEPBERN_SHIKI).unwrap(),
            latin_hs
        );
    }

    #[test]
    fn nihon_text() {
        let latin_ns = "TI SI TU DI ZI DU DYA ZYA KONNI KON'O";
        assert_eq!(to_romaji(HIRAGANA, &NIHON_SHIKI).unwrap(), latin_ns);
    }

    #[test]
    fn kunrei_text() {
        let latin_ks = "TI SI TU ZI ZI ZU ZYA ZYA KONNI KON'O";
        assert_eq!(to_romaji(HIRAGANA, &KUNREI_SHIKI).unwrap(), latin_ks);
    }
}
