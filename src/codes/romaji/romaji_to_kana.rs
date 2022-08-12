use lazy_static::lazy_static;

use crate::tokenizer::Node;

#[test]
fn test() {
    fn check_kana(romaji: &str) {
        println!(
            "\n{}\n{}",
            romaji,
            ROMAJI_TO_KANA
                .extract_tokens(romaji)
                .unwrap()
                .iter()
                .cloned()
                .collect::<String>()
        );
    }

    check_kana("ta ti tu te to tya tyi tyu tye tyo tta tti ttu tte tto");
    check_kana("da di du de do dya dyi dyu dye dyo dda ddi ddu dde ddo");
    check_kana("ka ki ku ke ko kya kyi kyu kye kyo kka kki kku kke kko");
    check_kana("ga gi gu ge go gya gyi gyu gye gyo gga ggi ggu gge ggo");
    check_kana("ra rya rra rrya");
    check_kana("la lya lla llya");
    check_kana("ha hya hha hhya");
    check_kana("na nya nna nnya n n'");
    check_kana("ma mya mma mmya");
    check_kana("chi chya cha cchi cchya ccha");
}

lazy_static! {
    pub static ref ROMAJI_TO_KANA: Node = {
        let transitions = vec![
            Node::leaf('!', "！"),
            Node::leaf('(', "（"),
            Node::leaf(')', "）"),
            Node::leaf(',', "、"),
            Node::leaf('-', "ー"),
            Node::leaf('.', "。"),
            Node::leaf('/', "・"),
            Node::leaf(':', "："),
            Node::leaf('?', "？"),
            Node::leaf('[', "［"),
            Node::leaf(']', "］"),
            Node::leaf('a', "あ"),
            Node::leaf('e', "え"),
            Node::leaf('i', "い"),
            Node::leaf('o', "お"),
            Node::leaf('u', "う"),
            Node::leaf(' ', "\u{3000}"),
            Node::branch(
                'b',
                None,
                vec![
                    Node::leaf('a', "ば"),
                    Node::leaf('e', "べ"),
                    Node::leaf('i', "び"),
                    Node::leaf('o', "ぼ"),
                    Node::leaf('u', "ぶ"),
                    Node::branch(
                        'b',
                        None,
                        vec![
                            Node::leaf('a', "っば"),
                            Node::leaf('e', "っべ"),
                            Node::leaf('i', "っび"),
                            Node::leaf('o', "っぼ"),
                            Node::leaf('u', "っぶ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っびゃ"),
                                    Node::leaf('e', "っびぇ"),
                                    Node::leaf('i', "っびぃ"),
                                    Node::leaf('o', "っびょ"),
                                    Node::leaf('u', "っびゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "びゃ"),
                            Node::leaf('e', "びぇ"),
                            Node::leaf('i', "びぃ"),
                            Node::leaf('o', "びょ"),
                            Node::leaf('u', "びゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'c',
                None,
                vec![
                    Node::branch(
                        'h',
                        None,
                        vec![
                            Node::leaf('i', "ち"),
                            Node::leaf('a', "ちゃ"),
                            Node::leaf('e', "ちぇ"),
                            Node::leaf('o', "ちょ"),
                            Node::leaf('a', "ちゅ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "ちゃ"),
                                    Node::leaf('e', "ちぇ"),
                                    Node::leaf('i', "ちぃ"),
                                    Node::leaf('o', "ちょ"),
                                    Node::leaf('u', "ちゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'c',
                        None,
                        vec![Node::branch(
                            'h',
                            None,
                            vec![
                                Node::leaf('i', "っち"),
                                Node::leaf('a', "っちゃ"),
                                Node::leaf('e', "っちぇ"),
                                Node::leaf('o', "っちょ"),
                                Node::leaf('a', "っちゅ"),
                                Node::branch(
                                    'y',
                                    None,
                                    vec![
                                        Node::leaf('a', "っちゃ"),
                                        Node::leaf('e', "っちぇ"),
                                        Node::leaf('i', "っちぃ"),
                                        Node::leaf('o', "っちょ"),
                                        Node::leaf('u', "っちゅ"),
                                    ],
                                ),
                            ],
                        )],
                    ),
                ],
            ),
            Node::branch(
                'd',
                None,
                vec![
                    Node::leaf('a', "だ"),
                    Node::leaf('e', "で"),
                    Node::leaf('i', "ぢ"),
                    Node::leaf('o', "ど"),
                    Node::leaf('u', "づ"),
                    Node::branch(
                        'd',
                        None,
                        vec![
                            Node::leaf('a', "っだ"),
                            Node::leaf('e', "っで"),
                            Node::leaf('i', "っぢ"),
                            Node::leaf('o', "っど"),
                            Node::leaf('u', "っづ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っぢゃ"),
                                    Node::leaf('e', "っぢぇ"),
                                    Node::leaf('i', "っぢぃ"),
                                    Node::leaf('o', "っぢょ"),
                                    Node::leaf('u', "っぢゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "ぢゃ"),
                            Node::leaf('e', "ぢぇ"),
                            Node::leaf('i', "ぢぃ"),
                            Node::leaf('o', "ぢょ"),
                            Node::leaf('u', "ぢゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'f',
                None,
                vec![
                    Node::leaf('u', "ふ"),
                    Node::branch('f', None, vec![Node::leaf('u', "っふ")]),
                ],
            ),
            Node::branch(
                'g',
                None,
                vec![
                    Node::leaf('a', "が"),
                    Node::leaf('e', "げ"),
                    Node::leaf('i', "ぎ"),
                    Node::leaf('o', "ど"),
                    Node::leaf('u', "ご"),
                    Node::branch(
                        'g',
                        None,
                        vec![
                            Node::leaf('a', "っが"),
                            Node::leaf('e', "っげ"),
                            Node::leaf('i', "っぎ"),
                            Node::leaf('o', "っど"),
                            Node::leaf('u', "っご"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っぎゃ"),
                                    Node::leaf('e', "っぎぇ"),
                                    Node::leaf('i', "っぎぃ"),
                                    Node::leaf('o', "っぎょ"),
                                    Node::leaf('u', "っぎゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "ぎゃ"),
                            Node::leaf('e', "ぎぇ"),
                            Node::leaf('i', "ぎぃ"),
                            Node::leaf('o', "ぎょ"),
                            Node::leaf('u', "ぎゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'h',
                None,
                vec![
                    Node::leaf('a', "は"),
                    Node::leaf('e', "へ"),
                    Node::leaf('i', "ひ"),
                    Node::leaf('o', "ほ"),
                    Node::leaf('u', "ふ"),
                    Node::branch(
                        'h',
                        None,
                        vec![
                            Node::leaf('a', "っは"),
                            Node::leaf('e', "っへ"),
                            Node::leaf('i', "っひ"),
                            Node::leaf('o', "っほ"),
                            Node::leaf('u', "っふ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っひゃ"),
                                    Node::leaf('e', "っひぇ"),
                                    Node::leaf('i', "っひぃ"),
                                    Node::leaf('o', "っひょ"),
                                    Node::leaf('u', "っひゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "ひゃ"),
                            Node::leaf('e', "ひぇ"),
                            Node::leaf('i', "ひぃ"),
                            Node::leaf('o', "ひょ"),
                            Node::leaf('u', "ひゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch('j', None, vec![]),
            Node::branch(
                'k',
                None,
                vec![
                    Node::leaf('a', "か"),
                    Node::leaf('e', "け"),
                    Node::leaf('i', "き"),
                    Node::leaf('o', "こ"),
                    Node::leaf('u', "く"),
                    Node::branch(
                        'k',
                        None,
                        vec![
                            Node::leaf('a', "っか"),
                            Node::leaf('e', "っけ"),
                            Node::leaf('i', "っき"),
                            Node::leaf('o', "っこ"),
                            Node::leaf('u', "っく"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っきゃ"),
                                    Node::leaf('e', "っきぇ"),
                                    Node::leaf('i', "っきぃ"),
                                    Node::leaf('o', "っきょ"),
                                    Node::leaf('u', "っきゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "きゃ"),
                            Node::leaf('e', "きぇ"),
                            Node::leaf('i', "きぃ"),
                            Node::leaf('o', "きょ"),
                            Node::leaf('u', "きゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'l',
                None,
                vec![
                    Node::leaf('a', "ら"),
                    Node::leaf('e', "れ"),
                    Node::leaf('i', "り"),
                    Node::leaf('o', "ろ"),
                    Node::leaf('u', "る"),
                    Node::branch(
                        'l',
                        None,
                        vec![
                            Node::leaf('a', "っら"),
                            Node::leaf('e', "っれ"),
                            Node::leaf('i', "っり"),
                            Node::leaf('o', "っろ"),
                            Node::leaf('u', "っる"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っりゃ"),
                                    Node::leaf('e', "っりぇ"),
                                    Node::leaf('i', "っりぃ"),
                                    Node::leaf('o', "っりょ"),
                                    Node::leaf('u', "っりゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "りゃ"),
                            Node::leaf('e', "りぇ"),
                            Node::leaf('i', "りぃ"),
                            Node::leaf('o', "りょ"),
                            Node::leaf('u', "りゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'm',
                None,
                vec![
                    Node::leaf('a', "ま"),
                    Node::leaf('e', "め"),
                    Node::leaf('i', "み"),
                    Node::leaf('o', "も"),
                    Node::leaf('u', "む"),
                    Node::branch(
                        'm',
                        None,
                        vec![
                            Node::leaf('a', "っま"),
                            Node::leaf('e', "っめ"),
                            Node::leaf('i', "っみ"),
                            Node::leaf('o', "っも"),
                            Node::leaf('u', "っむ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っみゃ"),
                                    Node::leaf('e', "っみぇ"),
                                    Node::leaf('i', "っみぃ"),
                                    Node::leaf('o', "っみょ"),
                                    Node::leaf('u', "っみゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "みゃ"),
                            Node::leaf('e', "みぇ"),
                            Node::leaf('i', "みぃ"),
                            Node::leaf('o', "みょ"),
                            Node::leaf('u', "みゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'n',
                Some("ん"),
                vec![
                    Node::leaf('\'', "ん"),
                    Node::leaf('a', "な"),
                    Node::leaf('e', "ね"),
                    Node::leaf('i', "に"),
                    Node::leaf('o', "の"),
                    Node::leaf('u', "ぬ"),
                    Node::branch(
                        'n',
                        None,
                        vec![
                            Node::leaf('a', "っな"),
                            Node::leaf('e', "っね"),
                            Node::leaf('i', "っに"),
                            Node::leaf('o', "っの"),
                            Node::leaf('u', "っぬ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っにゃ"),
                                    Node::leaf('e', "っにぇ"),
                                    Node::leaf('i', "っにぃ"),
                                    Node::leaf('o', "っにょ"),
                                    Node::leaf('u', "っにゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "にゃ"),
                            Node::leaf('e', "にぇ"),
                            Node::leaf('i', "にぃ"),
                            Node::leaf('o', "にょ"),
                            Node::leaf('u', "にゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'p',
                None,
                vec![
                    Node::leaf('a', "ぱ"),
                    Node::leaf('e', "ぺ"),
                    Node::leaf('i', "ぴ"),
                    Node::leaf('o', "ぽ"),
                    Node::leaf('u', "ぷ"),
                    Node::branch(
                        'p',
                        None,
                        vec![
                            Node::leaf('a', "っぱ"),
                            Node::leaf('e', "っぺ"),
                            Node::leaf('i', "っぴ"),
                            Node::leaf('o', "っぽ"),
                            Node::leaf('u', "っぷ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っぴゃ"),
                                    Node::leaf('e', "っぴぇ"),
                                    Node::leaf('i', "っぴぃ"),
                                    Node::leaf('o', "っぴょ"),
                                    Node::leaf('u', "っぴゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "ぴゃ"),
                            Node::leaf('e', "ぴぇ"),
                            Node::leaf('i', "ぴぃ"),
                            Node::leaf('o', "ぴょ"),
                            Node::leaf('u', "ぴゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'r',
                None,
                vec![
                    Node::leaf('a', "ら"),
                    Node::leaf('e', "れ"),
                    Node::leaf('i', "り"),
                    Node::leaf('o', "ろ"),
                    Node::leaf('u', "る"),
                    Node::branch(
                        'r',
                        None,
                        vec![
                            Node::leaf('a', "っら"),
                            Node::leaf('e', "っれ"),
                            Node::leaf('i', "っり"),
                            Node::leaf('o', "っろ"),
                            Node::leaf('u', "っる"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っりゃ"),
                                    Node::leaf('e', "っりぇ"),
                                    Node::leaf('i', "っりぃ"),
                                    Node::leaf('o', "っりょ"),
                                    Node::leaf('u', "っりゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "りゃ"),
                            Node::leaf('e', "りぇ"),
                            Node::leaf('i', "りぃ"),
                            Node::leaf('o', "りょ"),
                            Node::leaf('u', "りゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch('s', None, vec![]),
            Node::branch(
                't',
                None,
                vec![
                    Node::leaf('a', "た"),
                    Node::leaf('e', "て"),
                    Node::leaf('i', "ち"),
                    Node::leaf('o', "と"),
                    Node::leaf('u', "つ"),
                    Node::branch(
                        't',
                        None,
                        vec![
                            Node::leaf('a', "った"),
                            Node::leaf('e', "って"),
                            Node::leaf('i', "っち"),
                            Node::leaf('o', "っと"),
                            Node::leaf('u', "っつ"),
                            Node::branch(
                                'y',
                                None,
                                vec![
                                    Node::leaf('a', "っちゃ"),
                                    Node::leaf('e', "っちぇ"),
                                    Node::leaf('i', "っちぃ"),
                                    Node::leaf('o', "っちょ"),
                                    Node::leaf('u', "っちゅ"),
                                ],
                            ),
                        ],
                    ),
                    Node::branch(
                        'y',
                        None,
                        vec![
                            Node::leaf('a', "ちゃ"),
                            Node::leaf('e', "ちぇ"),
                            Node::leaf('i', "ちぃ"),
                            Node::leaf('o', "ちょ"),
                            Node::leaf('u', "ちゅ"),
                        ],
                    ),
                ],
            ),
            Node::branch(
                'w',
                None,
                vec![
                    Node::leaf('a', "わ"),
                    Node::leaf('o', "を"),
                    Node::branch(
                        'w',
                        None,
                        vec![Node::leaf('a', "っわ"), Node::leaf('o', "っを")],
                    ),
                ],
            ),
            Node::branch(
                'y',
                None,
                vec![
                    Node::leaf('a', "や"),
                    Node::leaf('o', "よ"),
                    Node::leaf('u', "ゆ"),
                ],
            ),
            Node::branch('z', None, vec![]),
        ];

        let mut node = Node {
            transitions: Some(transitions),
            output: None,
        };
        node.sort();
        node
    };
}
