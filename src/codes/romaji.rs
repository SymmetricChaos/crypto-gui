use wana_kana::{to_romaji::to_romaji_with_opt, Options};

/*
This converter uses the Nihon-shiki romaji (日本式ローマ字, Japanese-style romanization)
to convert between Japanese kana and the Latin alphabet. This is no longer a common
romanization because it does not reflect actual pronunciation. However it is highly
regular and allows clean conversion between the two writing systems. Kana are always
and only written according to their position in the gojyuu-on. So こんにちは is
romanized as "kon'nitiha" rather than as it is pronounced "kon'nichiwa".
*/

// Turns kana into uppercase romaji, passing through any romaji
pub fn to_romaji_ks(input: &str) -> String {
    let mut x = to_romaji_with_opt(
        input,
        Options {
            pass_romaji: true,
            ..Default::default()
        },
    );
    x = x.to_ascii_uppercase();
    // Convert the consonant pairs to single letters
    x = x.replace("CH", "T");
    x = x.replace("SH", "S");
    x = x.replace("TS", "T");

    // Convert the f to h
    x = x.replace("F", "H");

    // The convert the j sounds to zs and always use the y
    x = x.replace("JI", "ZI");
    x = x.replace("JA", "ZYA");
    x = x.replace("JU", "ZYU");
    x = x.replace("JO", "ZYO");
    x
}

#[test]
fn nihon_shiki_hiragana() {
    let hiragana = "こんにちは ひらがな きょうと おおさか とうきょ よこはま れんあい けん ふゆき みっつ ぼっち romaji";
    let latin_ks =
        "KONNITIHA HIRAGANA KYOUTO OOSAKA TOUKYO YOKOHAMA REN'AI KEN HUYUKI MITTU BOTTI ROMAJI";
    assert_eq!(to_romaji_ks(hiragana), latin_ks);
}
