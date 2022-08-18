// https://support.apple.com/guide/japanese-input-method/roman-characters-and-corresponding-kana-jpim10277/mac

use lazy_static::lazy_static;

use crate::tokenizer::Node;


#[test]
fn test() {
    // fn check_kana(romaji: &str) {
    //     print!(
    //         "{}\n{}\n",
    //         romaji,
    //         ROMAJI_TO_KANA
    //             .extract_tokens(romaji)
    //             .unwrap()
    //             .iter()
    //             .cloned()
    //             .collect::<String>()
    //     );
    // }

    //let paths = ROMAJI_TO_KANA.input_paths();

    let paths = ROMAJI_TO_KANA.output_paths();
    
    for (k,v) in &paths {
        print!("{k} <= {v:?}\n")
    }

    // println!("{:?}", check_kana("“konokudehazenchishiwoshyouryakudenkiru!”"));
    // println!("{:?}", ROMAJI_TO_KANA.extract_tokens("x"));
    // println!("{:?}", ROMAJI_TO_KANA.extract_tokens("shm"));
    // println!("{:?}", ROMAJI_TO_KANA.extract_tokens("sh"));
}

lazy_static! {
    pub static ref ROMAJI_TO_KANA: Node = {
        let transitions = vec![
            // Symbols
            Node::leaf('!',"！"),
            Node::leaf('(',"（"),
            Node::leaf(')',"）"),
            Node::leaf(',',"、"),
            Node::leaf('-',"ー"),
            Node::leaf('.',"。"),
            Node::leaf('/',"・"),
            Node::leaf(':',"："),
            Node::leaf('?',"？"),
            Node::leaf('[',"［"),
            Node::leaf(']',"］"),
            Node::leaf(' ',"\u{3000}"),
            Node::leaf('{',"｛"),
            Node::leaf('}',"｝"),
            Node::leaf('~',"〜"),
            Node::leaf('‘',"「"),
            Node::leaf('’',"」"),
            Node::leaf('“',"『"),
            Node::leaf('”',"』"),

            // Simple vowels
            Node::leaf('a',"あ"),
            Node::leaf('e',"え"),
            Node::leaf('i',"い"),
            Node::leaf('o',"お"),
            Node::leaf('u',"う"),

            // Everything else
            Node::branch('b', None,
                vec![
                    Node::leaf('a', "ば"),
                    Node::leaf('e', "べ"),
                    Node::leaf('i', "び"),
                    Node::leaf('o', "ぼ"),
                    Node::leaf('u', "ぶ"),

                    Node::branch('b', None,
                        vec![
                            Node::leaf('a',"っば"),
                            Node::leaf('e',"っべ"),
                            Node::leaf('i',"っび"),
                            Node::leaf('o',"っぼ"),
                            Node::leaf('u',"っぶ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a',"っびゃ"),
                                    Node::leaf('e',"っびぇ"),
                                    Node::leaf('i',"っびぃ"),
                                    Node::leaf('o',"っびょ"),
                                    Node::leaf('u',"っびゅ"),
                                ]
                            )
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"びゃ"),
                            Node::leaf('e',"びぇ"),
                            Node::leaf('i',"びぃ"),
                            Node::leaf('o',"びょ"),
                            Node::leaf('u',"びゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('c', None,
                vec![
                    Node::branch('y', None,
                        vec![
                            Node::leaf('i', "ちぃ"),
                            Node::leaf('a', "ちゃ"),
                            Node::leaf('e', "ちぇ"),
                            Node::leaf('o', "ちょ"),
                            Node::leaf('u', "ちゅ"),
                        ]
                    ),

                    Node::branch('h', None,
                        vec![
                            Node::leaf('i', "ち"),
                            Node::leaf('a', "ちゃ"),
                            Node::leaf('e', "ちぇ"),
                            Node::leaf('o', "ちょ"),
                            Node::leaf('u', "ちゅ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a',"ちゃ"),
                                    Node::leaf('e',"ちぇ"),
                                    Node::leaf('i',"ちぃ"),
                                    Node::leaf('o',"ちょ"),
                                    Node::leaf('u',"ちゅ"),
                                ]
                            )
                        ]
                    ),
                    Node::branch('c', None,
                        vec![
                            Node::branch('h', None,
                                vec![
                                    Node::leaf('i', "っち"),
                                    Node::leaf('a', "っちゃ"),
                                    Node::leaf('e', "っちぇ"),
                                    Node::leaf('o', "っちょ"),
                                    Node::leaf('u', "っちゅ"),

                                    Node::branch('y', None,
                                        vec![
                                            Node::leaf('a',"っちゃ"),
                                            Node::leaf('e',"っちぇ"),
                                            Node::leaf('i',"っちぃ"),
                                            Node::leaf('o',"っちょ"),
                                            Node::leaf('u',"っちゅ"),
                                        ]
                                    )
                                ]
                            )
                        ]
                    )
                ]
            ),

            Node::branch('d', None,
                vec![
                    Node::leaf('a', "だ"),
                    Node::leaf('e', "で"),
                    Node::leaf('i', "ぢ"),
                    Node::leaf('o', "ど"),
                    Node::leaf('u', "づ"),

                    Node::branch('d', None,
                        vec![
                            Node::leaf('a', "っだ"),
                            Node::leaf('e', "っで"),
                            Node::leaf('i', "っぢ"),
                            Node::leaf('o', "っど"),
                            Node::leaf('u', "っづ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っぢゃ"),
                                    Node::leaf('e', "っぢぇ"),
                                    Node::leaf('i', "っぢぃ"),
                                    Node::leaf('o', "っぢょ"),
                                    Node::leaf('u', "っぢゅ"),
                               ]
                            ),

                            Node::branch('h', None,
                                vec![
                                    Node::leaf('a', "っでゃ"),
                                    Node::leaf('e', "っでぇ"),
                                    Node::leaf('i', "っでぃ"),
                                    Node::leaf('o', "っでょ"),
                                    Node::leaf('u', "っでゅ"),
                                ]
                            ),

                            Node::branch('w', None,
                                vec![
                                    Node::leaf('a', "っどぁ"),
                                    Node::leaf('e', "っどぇ"),
                                    Node::leaf('i', "っどぃ"),
                                    Node::leaf('o', "っどぉ"),
                                    Node::leaf('u', "っどぅ"),
                                ]
                            ),

                            Node::branch('z', None,
                                vec![
                                    Node::leaf('u', "っづ"),
                                ]
                            ),
                        ]
                    ),

                    Node::branch('h', None,
                        vec![
                            Node::leaf('a', "でゃ"),
                            Node::leaf('e', "でぇ"),
                            Node::leaf('i', "でぃ"),
                            Node::leaf('o', "でょ"),
                            Node::leaf('u', "でゅ"),
                       ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a', "どぁ"),
                            Node::leaf('e', "どぇ"),
                            Node::leaf('i', "どぃ"),
                            Node::leaf('o', "どぉ"),
                            Node::leaf('u', "どぅ"),
                       ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "ぢゃ"),
                            Node::leaf('e', "ぢぇ"),
                            Node::leaf('i', "ぢぃ"),
                            Node::leaf('o', "ぢょ"),
                            Node::leaf('u', "ぢゅ"),
                       ]
                    ),

                    Node::branch('z', None,
                        vec![
                            Node::leaf('u', "づ"),
                       ]
                    ),
                ]
            ),

            Node::branch('f', None,
                vec![
                    Node::leaf('u', "ふ"),
                    Node::leaf('a', "ふぁ"),
                    Node::leaf('e', "ふぇ"),
                    Node::leaf('i', "ふぃ"),
                    Node::leaf('o', "ふぉ"),

                    Node::branch('f', None,
                        vec![
                            Node::leaf('u', "っふ"),
                            Node::leaf('a', "っふぁ"),
                            Node::leaf('e', "っふぇ"),
                            Node::leaf('i', "っふぃ"),
                            Node::leaf('o', "っふぉ"),

                            Node::branch('w', None,
                                vec![
                                    Node::leaf('u', "っふぅ"),
                                    Node::leaf('a', "っふぁ"),
                                    Node::leaf('e', "っふぇ"),
                                    Node::leaf('i', "っふぃ"),
                                    Node::leaf('o', "っふぉ"),
                                ]
                            ),

                            Node::branch(
                                'y', None,
                                vec![
                                    Node::leaf('u', "っふゅ"),
                                    Node::leaf('a', "っふゃ"),
                                    Node::leaf('e', "っふぇ"),
                                    Node::leaf('i', "っふぃ"),
                                    Node::leaf('o', "っふょ"),
                                ]
                            )
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('u', "ふぅ"),
                            Node::leaf('a', "ふぁ"),
                            Node::leaf('e', "ふぇ"),
                            Node::leaf('i', "ふぃ"),
                            Node::leaf('o', "ふぉ"),
                        ]
                    ),

                    Node::branch(
                        'y', None,
                        vec![
                            Node::leaf('u', "ふゅ"),
                            Node::leaf('a', "ふゃ"),
                            Node::leaf('e', "ふぇ"),
                            Node::leaf('i', "ふぃ"),
                            Node::leaf('o', "ふょ"),
                        ]
                    )
                ]
            ),

            Node::branch('g', None,
                vec![
                    Node::leaf('a', "が"),
                    Node::leaf('e', "げ"),
                    Node::leaf('i', "ぎ"),
                    Node::leaf('o', "ご"),
                    Node::leaf('u', "ぐ"),

                    Node::branch('g', None,
                        vec![
                            Node::leaf('a', "っが"),
                            Node::leaf('e', "っげ"),
                            Node::leaf('i', "っぎ"),
                            Node::leaf('o', "っご"),
                            Node::leaf('u', "っぐ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っぎゃ"),
                                    Node::leaf('e', "っぎぇ"),
                                    Node::leaf('i', "っぎぃ"),
                                    Node::leaf('o', "っぎょ"),
                                    Node::leaf('u', "っぎゅ"),

                                    Node::branch('w', None,
                                        vec![
                                            Node::leaf('a', "っぐぁ"),
                                            Node::leaf('e', "っぐぇ"),
                                            Node::leaf('i', "っぐぃ"),
                                            Node::leaf('o', "っぐぉ"),
                                            Node::leaf('u', "っぐぅ"),
                                        ]
                                    ),
                               ]
                            )
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a', "ぐぁ"),
                            Node::leaf('e', "ぐぇ"),
                            Node::leaf('i', "ぐぃ"),
                            Node::leaf('o', "ぐぉ"),
                            Node::leaf('u', "ぐぅ"),
                       ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "ぎゃ"),
                            Node::leaf('e', "ぎぇ"),
                            Node::leaf('i', "ぎぃ"),
                            Node::leaf('o', "ぎょ"),
                            Node::leaf('u', "ぎゅ"),
                       ]
                    ),
                ]
            ),

            Node::branch('h', None,
                vec![
                    Node::leaf('a', "は"),
                    Node::leaf('e', "へ"),
                    Node::leaf('i', "ひ"),
                    Node::leaf('o', "ほ"),
                    Node::leaf('u', "ふ"),

                    Node::branch('h', None,
                        vec![
                            Node::leaf('a', "っは"),
                            Node::leaf('e', "っへ"),
                            Node::leaf('i', "っひ"),
                            Node::leaf('o', "っほ"),
                            Node::leaf('u', "っふ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っひゃ"),
                                    Node::leaf('e', "っひぇ"),
                                    Node::leaf('i', "っひぃ"),
                                    Node::leaf('o', "っひょ"),
                                    Node::leaf('u', "っひゅ"),
                                ]
                            ),

                            Node::branch('w', None,
                            vec![
                                Node::leaf('a',"っふぁ"),
                                Node::leaf('e',"っふぇ"),
                                Node::leaf('i',"っふぃ"),
                                Node::leaf('o',"っふぉ"),
                                Node::leaf('u',"っふぅ"),
                            ]
                        ),
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a',"ふぁ"),
                            Node::leaf('e',"ふぇ"),
                            Node::leaf('i',"ふぃ"),
                            Node::leaf('o',"ふぉ"),
                            Node::leaf('u',"ふぅ"),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"ひゃ"),
                            Node::leaf('e',"ひぇ"),
                            Node::leaf('i',"ひぃ"),
                            Node::leaf('o',"ひょ"),
                            Node::leaf('u',"ひゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('j', None,
                vec![
                    Node::leaf('a',"じゃ"),
                    Node::leaf('e',"じぇ"),
                    Node::leaf('i',"じ"),
                    Node::leaf('o',"じょ"),
                    Node::leaf('u',"じゅ"),

                    Node::branch('j', None,
                        vec![
                            Node::leaf('a',"っじゃ"),
                            Node::leaf('e',"っじぇ"),
                            Node::leaf('i',"っじ"),
                            Node::leaf('o',"っじょ"),
                            Node::leaf('u',"っじゅ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a',"っじゃ"),
                                    Node::leaf('e',"っじぇ"),
                                    Node::leaf('i',"っじぃ"),
                                    Node::leaf('o',"っじょ"),
                                    Node::leaf('u',"っじゅ"),
                                ]
                            )
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"じゃ"),
                            Node::leaf('e',"じぇ"),
                            Node::leaf('i',"じぃ"),
                            Node::leaf('o',"じょ"),
                            Node::leaf('u',"じゅ"),
                        ]
                    )
                ]
            ),

            Node::branch('k', None,
                vec![
                    Node::leaf('a', "か"),
                    Node::leaf('e', "け"),
                    Node::leaf('i', "き"),
                    Node::leaf('o', "こ"),
                    Node::leaf('u', "く"),

                    Node::branch('k', None,
                        vec![
                            Node::leaf('a', "っか"),
                            Node::leaf('e', "っけ"),
                            Node::leaf('i', "っき"),
                            Node::leaf('o', "っこ"),
                            Node::leaf('u', "っく"),

                            Node::branch('w', None,
                                vec![
                                    Node::leaf('a', "っくぁ"),
                                    Node::leaf('e', "っくぇ"),
                                    Node::leaf('i', "っくぃ"),
                                    Node::leaf('o', "っくぉ"),
                                    Node::leaf('u', "っくぅ"),
                                ]
                            ),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っきゃ"),
                                    Node::leaf('e', "っきぇ"),
                                    Node::leaf('i', "っきぃ"),
                                    Node::leaf('o', "っきょ"),
                                    Node::leaf('u', "っきゅ"),
                                ]
                            )
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a', "くぁ"),
                            Node::leaf('e', "くぇ"),
                            Node::leaf('i', "くぃ"),
                            Node::leaf('o', "くぉ"),
                            Node::leaf('u', "くぅ"),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"きゃ"),
                            Node::leaf('e',"きぇ"),
                            Node::leaf('i',"きぃ"),
                            Node::leaf('o',"きょ"),
                            Node::leaf('u',"きゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('l', None,
                vec![
                    Node::leaf('a',"ぁ"),
                    Node::leaf('e',"ぇ"),
                    Node::leaf('i',"ぃ"),
                    Node::leaf('o',"ぉ"),
                    Node::leaf('u',"ぅ"),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a',"ゎ"),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('e',"ぇ"),
                            Node::leaf('a',"ゃ"),
                            Node::leaf('u',"ゅ"),
                            Node::leaf('o',"ょ"),
                        ]
                    ),

                    Node::branch('t', None,
                        vec![
                            Node::leaf('u',"っ"),

                            Node::branch('s', None,
                                vec![
                                    Node::leaf('u',"っ")
                                ]
                            )
                        ]
                    ),
                ]
            ),

            Node::branch('m', None,
                vec![
                    Node::leaf('a', "ま"),
                    Node::leaf('e', "め"),
                    Node::leaf('i', "み"),
                    Node::leaf('o', "も"),
                    Node::leaf('u', "む"),

                    Node::branch('m', None,
                        vec![
                            Node::leaf('a', "っま"),
                            Node::leaf('e', "っめ"),
                            Node::leaf('i', "っみ"),
                            Node::leaf('o', "っも"),
                            Node::leaf('u', "っむ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っみゃ"),
                                    Node::leaf('e', "っみぇ"),
                                    Node::leaf('i', "っみぃ"),
                                    Node::leaf('o', "っみょ"),
                                    Node::leaf('u', "っみゅ"),
                                ])
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"みゃ"),
                            Node::leaf('e',"みぇ"),
                            Node::leaf('i',"みぃ"),
                            Node::leaf('o',"みょ"),
                            Node::leaf('u',"みゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('n', Some("ん"),
                vec![
                    Node::leaf('\'', "ん"),
                    Node::leaf('a', "な"),
                    Node::leaf('e', "ね"),
                    Node::leaf('i', "に"),
                    Node::leaf('o', "の"),
                    Node::leaf('u', "ぬ"),

                    Node::branch('n', Some("ん"),
                        vec![
                            Node::leaf('a', "っな"),
                            Node::leaf('e', "っね"),
                            Node::leaf('i', "っに"),
                            Node::leaf('o', "っの"),
                            Node::leaf('u', "っぬ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っにゃ"),
                                    Node::leaf('e', "っにぇ"),
                                    Node::leaf('i', "っにぃ"),
                                    Node::leaf('o', "っにょ"),
                                    Node::leaf('u', "っにゅ"),
                                ])
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"にゃ"),
                            Node::leaf('e',"にぇ"),
                            Node::leaf('i',"にぃ"),
                            Node::leaf('o',"にょ"),
                            Node::leaf('u',"にゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('p', None,
                vec![
                    Node::leaf('a', "ぱ"),
                    Node::leaf('e', "ぺ"),
                    Node::leaf('i', "ぴ"),
                    Node::leaf('o', "ぽ"),
                    Node::leaf('u', "ぷ"),

                    Node::branch('p', None,
                        vec![
                            Node::leaf('a', "っぱ"),
                            Node::leaf('e', "っぺ"),
                            Node::leaf('i', "っぴ"),
                            Node::leaf('o', "っぽ"),
                            Node::leaf('u', "っぷ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っぴゃ"),
                                    Node::leaf('e', "っぴぇ"),
                                    Node::leaf('i', "っぴぃ"),
                                    Node::leaf('o', "っぴょ"),
                                    Node::leaf('u', "っぴゅ"),
                                ])
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"ぴゃ"),
                            Node::leaf('e',"ぴぇ"),
                            Node::leaf('i',"ぴぃ"),
                            Node::leaf('o',"ぴょ"),
                            Node::leaf('u',"ぴゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('q', None,
                vec![
                    Node::leaf('a', "くぁ"),
                    Node::leaf('e', "くぇ"),
                    Node::leaf('i', "くぃ"),
                    Node::leaf('o', "くぉ"),
                    Node::leaf('u', "くぅ"),

                    Node::branch('q', None,
                        vec![
                            Node::leaf('a', "っくぁ"),
                            Node::leaf('e', "っくぇ"),
                            Node::leaf('i', "っくぃ"),
                            Node::leaf('o', "っくぉ"),
                            Node::leaf('u', "っくぅ"),

                            Node::branch('w', None,
                                vec![
                                    Node::leaf('a', "っくゎ"),
                                    Node::leaf('e', "っくぇ"),
                                    Node::leaf('i', "っくぃ"),
                                    Node::leaf('o', "っくぉ"),
                                    Node::leaf('u', "っくぅ"),
                                ]
                            ),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a',"っくゃ"),
                                    Node::leaf('e',"っくぇ"),
                                    Node::leaf('i',"っくぃ"),
                                    Node::leaf('o',"っくょ"),
                                    Node::leaf('u',"っくゅ"),
                                ]
                            )
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a', "くゎ"),
                            Node::leaf('e', "くぇ"),
                            Node::leaf('i', "くぃ"),
                            Node::leaf('o', "くぉ"),
                            Node::leaf('u', "くぅ"),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"くゃ"),
                            Node::leaf('e',"くぇ"),
                            Node::leaf('i',"くぃ"),
                            Node::leaf('o',"くょ"),
                            Node::leaf('u',"くゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('r', None,
                vec![
                    Node::leaf('a', "ら"),
                    Node::leaf('e', "れ"),
                    Node::leaf('i', "り"),
                    Node::leaf('o', "ろ"),
                    Node::leaf('u', "る"),

                    Node::branch('r', None,
                        vec![
                            Node::leaf('a', "っら"),
                            Node::leaf('e', "っれ"),
                            Node::leaf('i', "っり"),
                            Node::leaf('o', "っろ"),
                            Node::leaf('u', "っる"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っりゃ"),
                                    Node::leaf('e', "っりぇ"),
                                    Node::leaf('i', "っりぃ"),
                                    Node::leaf('o', "っりょ"),
                                    Node::leaf('u', "っりゅ"),
                                ])
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a',"りゃ"),
                            Node::leaf('e',"りぇ"),
                            Node::leaf('i',"りぃ"),
                            Node::leaf('o',"りょ"),
                            Node::leaf('u',"りゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('s', None,
                vec![
                    Node::leaf('a', "さ"),
                    Node::leaf('e', "せ"),
                    Node::leaf('i', "し"),
                    Node::leaf('o', "そ"),
                    Node::leaf('u', "す"),

                    Node::branch('s', None,
                        vec![
                            Node::leaf('a', "っさ"),
                            Node::leaf('e', "っせ"),
                            Node::leaf('i', "っし"),
                            Node::leaf('o', "っそ"),
                            Node::leaf('u', "っす"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っしゃ"),
                                    Node::leaf('e', "っしぇ"),
                                    Node::leaf('i', "っしぃ"),
                                    Node::leaf('o', "っしょ"),
                                    Node::leaf('u', "っしゅ"),
                               ]
                            ),

                            Node::branch('h', None,
                                vec![
                                    Node::leaf('i', "っし"),
                                    Node::leaf('a', "っしゃ"),
                                    Node::leaf('e', "っしぇ"),
                                    Node::leaf('o', "っしょ"),
                                    Node::leaf('u', "っしゅ"),

                                    Node::branch('y', None,
                                        vec![
                                            Node::leaf('i', "っし"),
                                            Node::leaf('a', "っしゃ"),
                                            Node::leaf('e', "っしぇ"),
                                            Node::leaf('o', "っしょ"),
                                            Node::leaf('u', "っしゅ"),
                                        ]
                                    ),
                                ]
                            )
                        ]
                    ),
                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "しゃ"),
                            Node::leaf('e', "しぇ"),
                            Node::leaf('i', "しぃ"),
                            Node::leaf('o', "しょ"),
                            Node::leaf('u', "しゅ"),
                       ]
                    ),

                    Node::branch('h', None,
                        vec![
                            Node::leaf('i', "し"),
                            Node::leaf('a', "しゃ"),
                            Node::leaf('e', "しぇ"),
                            Node::leaf('o', "しょ"),
                            Node::leaf('u', "しゅ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "しゃ"),
                                    Node::leaf('e', "しぇ"),
                                    Node::leaf('i', "しぃ"),
                                    Node::leaf('o', "しょ"),
                                    Node::leaf('u', "しゅ"),
                                ]
                            ),
                        ]
                    )
                ]
            ),

            Node::branch('t', None,
                vec![
                    Node::leaf('a', "た"),
                    Node::leaf('e', "て"),
                    Node::leaf('i', "ち"),
                    Node::leaf('o', "と"),
                    Node::leaf('u', "つ"),

                    Node::branch('t', None,
                        vec![
                            Node::leaf('a', "った"),
                            Node::leaf('e', "って"),
                            Node::leaf('i', "っち"),
                            Node::leaf('o', "っと"),
                            Node::leaf('u', "っつ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っちゃ"),
                                    Node::leaf('e', "っちぇ"),
                                    Node::leaf('i', "っちぃ"),
                                    Node::leaf('o', "っちょ"),
                                    Node::leaf('u', "っちゅ"),
                               ]
                            ),

                            Node::branch('s', None,
                                vec![
                                    Node::leaf('u', "っつ"),
                                ]
                            )
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "ちゃ"),
                            Node::leaf('e', "ちぇ"),
                            Node::leaf('i', "ちぃ"),
                            Node::leaf('o', "ちょ"),
                            Node::leaf('u', "ちゅ"),
                       ]
                    ),

                    Node::branch('s', None,
                        vec![
                            Node::leaf('u', "つ"),
                        ]
                    )
                ]
            ),

            Node::branch('v', None,
                vec![
                    Node::leaf('a', "ゔぁ"),
                    Node::leaf('e', "ゔぇ"),
                    Node::leaf('i', "ゔぃ"),
                    Node::leaf('o', "ゔぉ"),
                    Node::leaf('u', "ゔ"),

                    Node::branch('v', None,
                        vec![
                            Node::leaf('a', "っゔぁ"),
                            Node::leaf('e', "っゔぇ"),
                            Node::leaf('i', "っゔぃ"),
                            Node::leaf('o', "っゔぉ"),
                            Node::leaf('u', "っゔ"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っゔゃ"),
                                    Node::leaf('o', "っゔょ"),
                                    Node::leaf('u', "っゔゅ"),
                                ]
                            ),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "ゔゃ"),
                            Node::leaf('o', "ゔょ"),
                            Node::leaf('u', "ゔゅ"),
                        ]
                    ),
                ]
            ),

            Node::branch('w', None,
                vec![
                    Node::leaf('a', "わ"),
                    Node::leaf('o', "を"),
                    Node::leaf('i', "うぃ"),
                    Node::leaf('e', "うぇ"),
                    // If using nihon-shiki these would be preferred
                    // However both nihon-shiki and these kana are considered obsolete
                    // they are in the 'y' branch instead
                    // Node::leaf('i', "ゐ"),
                    // Node::leaf('e', "ゑ"),


                    Node::branch('h', None,
                        vec![
                            Node::leaf('a', "うぁ"),
                            Node::leaf('o', "うぉ"),
                            Node::leaf('i', "うぃ"),
                            Node::leaf('e', "うぇ"),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('i', "ゐ"),
                            Node::leaf('e', "ゑ"),
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a', "っわ"),
                            Node::leaf('o', "っを"),
                            Node::leaf('i', "っうぃ"),
                            Node::leaf('e', "っうぇ"),

                            Node::branch('h', None,
                                vec![
                                    Node::leaf('a', "っうぁ"),
                                    Node::leaf('o', "っうぉ"),
                                    Node::leaf('i', "っうぃ"),
                                    Node::leaf('e', "っうぇ"),
                                ]
                            ),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('i', "っゐ"),
                                    Node::leaf('e', "っゑ"),
                                ]
                            ),
                        ]
                    ),

                ]
            ),

            Node::branch('x', None,
                vec![
                    Node::leaf('a', "ぁ"),
                    Node::leaf('e', "ぇ"),
                    Node::leaf('i', "ぃ"),
                    Node::leaf('o', "ぉ"),
                    Node::leaf('u', "ぅ"),
                    Node::leaf('n', "ん"),

                    Node::branch('k', None,
                        vec![
                            Node::leaf('a', "ヵ"),
                            Node::leaf('e', "ヶ"),
                        ]
                    ),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "ゃ"),
                            Node::leaf('e', "ぇ"),
                            Node::leaf('o', "ょ"),
                            Node::leaf('u', "ゅ"),
                        ]
                    ),

                    Node::branch('t', None,
                        vec![
                            Node::leaf('u', "っ"),

                            Node::branch('s', None,
                                vec![
                                    Node::leaf('u', "っ"),
                                ]
                            ),
                        ]
                    ),

                    Node::branch('w', None,
                        vec![
                            Node::leaf('a', "ゎ"),
                        ]
                    ),
                ]
            ),

            Node::branch('y', None,
                vec![
                    Node::leaf('a', "や"),
                    Node::leaf('o', "よ"),
                    Node::leaf('u', "ゆ"),
                    Node::leaf('e', "いぇ"),
                ]
            ),

            Node::branch('z', None,
                vec![
                    Node::leaf('a', "ざ"),
                    Node::leaf('e', "ぜ"),
                    Node::leaf('i', "じ"),
                    Node::leaf('o', "ぞ"),
                    Node::leaf('u', "ず"),

                    Node::branch('y', None,
                        vec![
                            Node::leaf('a', "じゃ"),
                            Node::leaf('e', "じぇ"),
                            Node::leaf('i', "じぃ"),
                            Node::leaf('o', "じょ"),
                            Node::leaf('u', "じゅ"),
                        ]
                    ),

                    Node::branch('z', None,
                        vec![
                            Node::leaf('a', "っざ"),
                            Node::leaf('e', "っぜ"),
                            Node::leaf('i', "っじ"),
                            Node::leaf('o', "っぞ"),
                            Node::leaf('u', "っず"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っじゃ"),
                                    Node::leaf('e', "っじぇ"),
                                    Node::leaf('i', "っじぃ"),
                                    Node::leaf('o', "っじょ"),
                                    Node::leaf('u', "っじゅ"),
                                ]
                            )
                        ]
                    )
                ]
            ),
        ];

        let mut node = Node { transitions: Some(transitions), output: None };
        node.sort();
        node
    };
}
