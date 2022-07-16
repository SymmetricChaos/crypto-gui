use wana_kana::to_romaji::to_romaji;


/*
This converter uses the Nihon-shiki romaji (日本式ローマ字, Japanese-style romanization)
to convert between Japanese kana and the Latin alphabet. This is no longer a common
romanization because it does not reflect actual pronunciation. However it is highly
regular and allows clean conversion between the two writing systems. Kana are always
and only written according to their position in the gojyuu-on. So こんにちは is
romanized as "kon'nitiha" rather than as it is pronounced "kon'nichiwa".
*/

pub fn to_romaji_ks(input: &str) -> String {
    let mut x = to_romaji(input);
    // Convert the consonant pairs to single letters
    x = x.replace("ch", "t");
    x = x.replace("sh", "s");
    x = x.replace("ts", "t");

    // Convert the f to h
    x = x.replace("f", "h");

    // The convert the j sounds to zs and always use the y
    x = x.replace("ji", "zi");
    x = x.replace("ja", "zya");
    x = x.replace("ju", "zyu");
    x = x.replace("jo", "zyo");
    x
}


#[test]
fn nihon_shiki_hiragana() {

    let hiragana = "こんにちは ひらがな きょうと おおさか とうきょ よこはま れんあい けん ふゆき みっつ ぼっち";
    let latin    = "konnichiha hiragana kyouto oosaka toukyo yokohama ren'ai ken fuyuki mittsu botchi";
    let latin_ks = "konnitiha hiragana kyouto oosaka toukyo yokohama ren'ai ken huyuki mittu botti";

    assert_eq!(to_romaji(hiragana), latin);
    assert_eq!(to_romaji_ks(hiragana), latin_ks);
}
