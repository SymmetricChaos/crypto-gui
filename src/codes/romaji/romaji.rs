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

#[test]
fn nihon_shiki_hiragana() {
    
    use super::KUNREI_SHIKI;
    let hiragana = "こんにちは ひらがな きょうと おおさか とうきょ よこはま れんあい けん ふゆき みっつ ぼっち";
    let latin_ks =
        "KONNITIHA HIRAGANA KYOUTO OOSAKA TOUKYO YOKOHAMA REN'AI KEN HUYUKI MITTU BOTTI";
    assert_eq!(to_romaji(hiragana,KUNREI_SHIKI.clone()), latin_ks);
}
