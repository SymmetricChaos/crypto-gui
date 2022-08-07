use crate::tokenizer::Node;

pub fn to_romaji(orig: &str, tree: Node) -> String {
    //let kana = katakana_to_hiragana_with_opt(orig, true);
    let chars = orig.chars().collect::<Vec<_>>();
    let mut ouput = String::with_capacity(orig.len());
    let len = chars.len();
    // Position in the string that is being evaluated
    let mut curr_pos = 0;

    while curr_pos != len {
        let result = tree.get(&chars[curr_pos..]);
        //nothing found, pass through
        if result.1 == 0 {
            ouput.push(chars[curr_pos]);
            curr_pos += 1;
        } else {
            ouput.push_str(&result.0.to_uppercase());
            curr_pos += result.1;
        }
    }

    ouput
}

#[cfg(test)]
mod romaji_test {
    use super::*;
    
    use crate::codes::romaji::{NIHON_SHIKI,KUNREI_SHIKI,HEPBERN_SHIKI};
    // we check the differences between the methods
    const HIRAGANA: &'static str = "ち し つ ぢ じ づ ぢゃ じゃ こんに こんお";

    
    #[test]
    fn hepbern_text() {
        let latin_hs =
            "CHI SHI TSU JI JI ZU JA JA KONNI KON'O";
        assert_eq!(to_romaji(HIRAGANA,HEPBERN_SHIKI.clone()), latin_hs);
    }

    #[test]
    fn nihon_text() {
        let latin_ns =
            "TI SI TU DI ZI DU DYA ZYA KONNI KON'O";
        assert_eq!(to_romaji(HIRAGANA,NIHON_SHIKI.clone()), latin_ns);
    }
    
    #[test]
    fn kunrei_text() {
        let latin_ks =
            "TI SI TU ZI ZI ZU ZYA ZYA KONNI KON'O";
        assert_eq!(to_romaji(HIRAGANA,KUNREI_SHIKI.clone()), latin_ks);
    
    }

    
}
