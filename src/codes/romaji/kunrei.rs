use lazy_static::lazy_static;

use crate::tokenizer::Node;

lazy_static! {
    pub static ref KUNREI_SHIKI: Node = {
        let transitions = Some(vec![
            (
                '\u{3000}',
                Node {
                    transitions: None,
                    output: Some(" "),
                },
            ),
            (
                '、',
                Node {
                    transitions: None,
                    output: Some(","),
                },
            ),
            (
                '。',
                Node {
                    transitions: None,
                    output: Some("."),
                },
            ),
            (
                '「',
                Node {
                    transitions: None,
                    output: Some("‘"),
                },
            ),
            (
                '」',
                Node {
                    transitions: None,
                    output: Some("’"),
                },
            ),
            (
                '『',
                Node {
                    transitions: None,
                    output: Some("“"),
                },
            ),
            (
                '』',
                Node {
                    transitions: None,
                    output: Some("”"),
                },
            ),
            (
                '〜',
                Node {
                    transitions: None,
                    output: Some("~"),
                },
            ),
            (
                'ぁ',
                Node {
                    transitions: None,
                    output: Some("a"),
                },
            ),
            (
                'あ',
                Node {
                    transitions: None,
                    output: Some("a"),
                },
            ),
            (
                'ぃ',
                Node {
                    transitions: None,
                    output: Some("i"),
                },
            ),
            (
                'い',
                Node {
                    transitions: None,
                    output: Some("i"),
                },
            ),
            (
                'ぅ',
                Node {
                    transitions: None,
                    output: Some("u"),
                },
            ),
            (
                'う',
                Node {
                    transitions: None,
                    output: Some("u"),
                },
            ),
            (
                'ぇ',
                Node {
                    transitions: None,
                    output: Some("e"),
                },
            ),
            (
                'え',
                Node {
                    transitions: None,
                    output: Some("e"),
                },
            ),
            (
                'ぉ',
                Node {
                    transitions: None,
                    output: Some("o"),
                },
            ),
            (
                'お',
                Node {
                    transitions: None,
                    output: Some("o"),
                },
            ),
            (
                'か',
                Node {
                    transitions: None,
                    output: Some("ka"),
                },
            ),
            (
                'が',
                Node {
                    transitions: None,
                    output: Some("ga"),
                },
            ),
            (
                'き',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "kyi"),
                        Node::leaf('ぇ', "kye"),
                        Node::leaf('ゃ', "kya"),
                        Node::leaf('ゅ', "kyu"),
                        Node::leaf('ょ', "kyo"),
                    ]),
                    output: Some("ki"),
                },
            ),
            (
                'ぎ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "gyi"),
                        Node::leaf('ぇ', "gye"),
                        Node::leaf('ゃ', "gya"),
                        Node::leaf('ゅ', "gyu"),
                        Node::leaf('ょ', "gyo"),
                    ]),
                    output: Some("gi"),
                },
            ),
            (
                'く',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "kyi"),
                        Node::leaf('ぇ', "kye"),
                        Node::leaf('ゃ', "kya"),
                        Node::leaf('ゅ', "kyu"),
                        Node::leaf('ょ', "kyo"),
                    ]),
                    output: Some("ku"),
                },
            ),
            (
                'ぐ',
                Node {
                    transitions: None,
                    output: Some("gu"),
                },
            ),
            (
                'け',
                Node {
                    transitions: None,
                    output: Some("ke"),
                },
            ),
            (
                'げ',
                Node {
                    transitions: None,
                    output: Some("ge"),
                },
            ),
            (
                'こ',
                Node {
                    transitions: None,
                    output: Some("ko"),
                },
            ),
            (
                'ご',
                Node {
                    transitions: None,
                    output: Some("go"),
                },
            ),
            (
                'さ',
                Node {
                    transitions: None,
                    output: Some("sa"),
                },
            ),
            (
                'ざ',
                Node {
                    transitions: None,
                    output: Some("za"),
                },
            ),
            (
                'し',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "syi"),
                        Node::leaf('ぇ', "sye"),
                        Node::leaf('ゃ', "sya"),
                        Node::leaf('ゅ', "syu"),
                        Node::leaf('ょ', "syo"),
                    ]),
                    output: Some("si"),
                },
            ),
            (
                'じ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "zyi"),
                        Node::leaf('ぇ', "zye"),
                        Node::leaf('ゃ', "zya"),
                        Node::leaf('ゅ', "zyu"),
                        Node::leaf('ょ', "zyo"),
                    ]),
                    output: Some("zi"),
                },
            ),
            (
                'す',
                Node {
                    transitions: None,
                    output: Some("su"),
                },
            ),
            (
                'ず',
                Node {
                    transitions: None,
                    output: Some("zu"),
                },
            ),
            (
                'せ',
                Node {
                    transitions: None,
                    output: Some("se"),
                },
            ),
            (
                'ぜ',
                Node {
                    transitions: None,
                    output: Some("ze"),
                },
            ),
            (
                'そ',
                Node {
                    transitions: None,
                    output: Some("so"),
                },
            ),
            (
                'ぞ',
                Node {
                    transitions: None,
                    output: Some("zo"),
                },
            ),
            (
                'た',
                Node {
                    transitions: None,
                    output: Some("ta"),
                },
            ),
            (
                'だ',
                Node {
                    transitions: None,
                    output: Some("da"),
                },
            ),
            (
                'ち',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "tyi"),
                        Node::leaf('ぇ', "tye"),
                        Node::leaf('ゃ', "tya"),
                        Node::leaf('ゅ', "tyu"),
                        Node::leaf('ょ', "tyo"),
                    ]),
                    output: Some("ti"),
                },
            ),
            (
                'ぢ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ', "zyi"),
                        Node::leaf('ぇ', "zye"),
                        Node::leaf('ゃ', "zya"),
                        Node::leaf('ゅ', "zyu"),
                        Node::leaf('ょ', "zyo"),
                    ]),
                    output: Some("zi"),
                },
            ),
            (
                'っ',
                Node {
                    transitions: Some(vec![
                        (
                            '\u{3000}',
                            Node {
                                transitions: None,
                                output: Some(" "),
                            },
                        ),
                        (
                            '、',
                            Node {
                                transitions: None,
                                output: Some(","),
                            },
                        ),
                        (
                            '。',
                            Node {
                                transitions: None,
                                output: Some("."),
                            },
                        ),
                        (
                            '「',
                            Node {
                                transitions: None,
                                output: Some("‘"),
                            },
                        ),
                        (
                            '」',
                            Node {
                                transitions: None,
                                output: Some("’"),
                            },
                        ),
                        (
                            '『',
                            Node {
                                transitions: None,
                                output: Some("“"),
                            },
                        ),
                        (
                            '』',
                            Node {
                                transitions: None,
                                output: Some("”"),
                            },
                        ),
                        (
                            '〜',
                            Node {
                                transitions: None,
                                output: Some("~"),
                            },
                        ),
                        (
                            'ぁ',
                            Node {
                                transitions: None,
                                output: Some("a"),
                            },
                        ),
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: Some("a"),
                            },
                        ),
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("i"),
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: Some("i"),
                            },
                        ),
                        (
                            'ぅ',
                            Node {
                                transitions: None,
                                output: Some("u"),
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: Some("u"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("e"),
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: Some("e"),
                            },
                        ),
                        (
                            'ぉ',
                            Node {
                                transitions: None,
                                output: Some("o"),
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: Some("o"),
                            },
                        ),
                        (
                            'か',
                            Node {
                                transitions: None,
                                output: Some("kka"),
                            },
                        ),
                        (
                            'が',
                            Node {
                                transitions: None,
                                output: Some("gga"),
                            },
                        ),
                        (
                            'き',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "kkyi"),
                                    Node::leaf('ぇ', "kkye"),
                                    Node::leaf('ゃ', "kkya"),
                                    Node::leaf('ゅ', "kkyu"),
                                    Node::leaf('ょ', "kkyo"),
                                ]),
                                output: Some("kki"),
                            },
                        ),
                        (
                            'ぎ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "ggyi"),
                                    Node::leaf('ぇ', "ggye"),
                                    Node::leaf('ゃ', "ggya"),
                                    Node::leaf('ゅ', "ggyu"),
                                    Node::leaf('ょ', "ggyo"),
                                ]),
                                output: Some("ggi"),
                            },
                        ),
                        (
                            'く',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "kkyi"),
                                    Node::leaf('ぇ', "kkye"),
                                    Node::leaf('ゃ', "kkya"),
                                    Node::leaf('ゅ', "kkyu"),
                                    Node::leaf('ょ', "kkyo"),
                                ]),
                                output: Some("kku"),
                            },
                        ),
                        (
                            'ぐ',
                            Node {
                                transitions: None,
                                output: Some("ggu"),
                            },
                        ),
                        (
                            'け',
                            Node {
                                transitions: None,
                                output: Some("kke"),
                            },
                        ),
                        (
                            'げ',
                            Node {
                                transitions: None,
                                output: Some("gge"),
                            },
                        ),
                        (
                            'こ',
                            Node {
                                transitions: None,
                                output: Some("kko"),
                            },
                        ),
                        (
                            'ご',
                            Node {
                                transitions: None,
                                output: Some("ggo"),
                            },
                        ),
                        (
                            'さ',
                            Node {
                                transitions: None,
                                output: Some("ssa"),
                            },
                        ),
                        (
                            'ざ',
                            Node {
                                transitions: None,
                                output: Some("zza"),
                            },
                        ),
                        (
                            'し',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "ssyi"),
                                    Node::leaf('ぇ', "ssye"),
                                    Node::leaf('ゃ', "ssya"),
                                    Node::leaf('ゅ', "ssyu"),
                                    Node::leaf('ょ', "ssyo"),
                                ]),
                                output: Some("ssi"),
                            },
                        ),
                        (
                            'じ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "zzyi"),
                                    Node::leaf('ぇ', "zzye"),
                                    Node::leaf('ゃ', "zzya"),
                                    Node::leaf('ゅ', "zzyu"),
                                    Node::leaf('ょ', "zzyo"),
                                ]),
                                output: Some("zzi"),
                            },
                        ),
                        (
                            'す',
                            Node {
                                transitions: None,
                                output: Some("ssu"),
                            },
                        ),
                        (
                            'ず',
                            Node {
                                transitions: None,
                                output: Some("zzu"),
                            },
                        ),
                        (
                            'せ',
                            Node {
                                transitions: None,
                                output: Some("sse"),
                            },
                        ),
                        (
                            'ぜ',
                            Node {
                                transitions: None,
                                output: Some("zze"),
                            },
                        ),
                        (
                            'そ',
                            Node {
                                transitions: None,
                                output: Some("sso"),
                            },
                        ),
                        (
                            'ぞ',
                            Node {
                                transitions: None,
                                output: Some("zzo"),
                            },
                        ),
                        (
                            'た',
                            Node {
                                transitions: None,
                                output: Some("tta"),
                            },
                        ),
                        (
                            'だ',
                            Node {
                                transitions: None,
                                output: Some("dda"),
                            },
                        ),
                        (
                            'ち',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "ttyi"),
                                    Node::leaf('ぇ', "ttye"),
                                    Node::leaf('ゃ', "ttya"),
                                    Node::leaf('ゅ', "ttyu"),
                                    Node::leaf('ょ', "ttyo"),
                                ]),
                                output: Some("tti"),
                            },
                        ),
                        (
                            'ぢ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "zzyi"),
                                    Node::leaf('ぇ', "zzye"),
                                    Node::leaf('ゃ', "zzya"),
                                    Node::leaf('ゅ', "zzyu"),
                                    Node::leaf('ょ', "zzyo"),
                                ]),
                                output: Some("zzi"),
                            },
                        ),
                        (
                            'つ',
                            Node {
                                transitions: None,
                                output: Some("ttu"),
                            },
                        ),
                        (
                            'づ',
                            Node {
                                transitions: None,
                                output: Some("zzu"),
                            },
                        ),
                        (
                            'て',
                            Node {
                                transitions: None,
                                output: Some("tte"),
                            },
                        ),
                        (
                            'で',
                            Node {
                                transitions: None,
                                output: Some("dde"),
                            },
                        ),
                        (
                            'と',
                            Node {
                                transitions: None,
                                output: Some("tto"),
                            },
                        ),
                        (
                            'ど',
                            Node {
                                transitions: None,
                                output: Some("ddo"),
                            },
                        ),
                        (
                            'な',
                            Node {
                                transitions: None,
                                output: Some("na"),
                            },
                        ),
                        (
                            'に',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ', "nyi"),
                                    Node::leaf('ぇ', "nye"),
                                    Node::leaf('ゃ', "nya"),
                                    Node::leaf('ゅ', "nyu"),
                                    Node::leaf('ょ', "nyo"),
                                ]),
                                output: Some("ni"),
                            },
                        ),
                        (
                            'ぬ',
                            Node {
                                transitions: None,
                                output: Some("nu"),
                            },
                        ),
                        (
                            'ね',
                            Node {
                                transitions: None,
                                output: Some("ne"),
                            },
                        ),
                        (
                            'の',
                            Node {
                                transitions: None,
                                output: Some("no"),
                            },
                        ),
                        (
                            'は',
                            Node {
                                transitions: None,
                                output: Some("hha"),
                            },
                        ),
                        (
                            'ば',
                            Node {
                                transitions: None,
                                output: Some("bba"),
                            },
                        ),
                        (
                            'ぱ',
                            Node {
                                transitions: None,
                                output: Some("ppa"),
                            },
                        ),
                        (
                            'ひ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhyi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhyu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhyo"),
                                        },
                                    ),
                                ]),
                                output: Some("hhi"),
                            },
                        ),
                        (
                            'び',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("bbyi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("bbye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("bbya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("bbyu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("bbyo"),
                                        },
                                    ),
                                ]),
                                output: Some("bbi"),
                            },
                        ),
                        (
                            'ぴ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("ppyi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("ppye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("ppya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("ppyu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("ppyo"),
                                        },
                                    ),
                                ]),
                                output: Some("ppi"),
                            },
                        ),
                        (
                            'ふ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhyi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhyu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("hhyo"),
                                        },
                                    ),
                                ]),
                                output: Some("hhu"),
                            },
                        ),
                        (
                            'ぶ',
                            Node {
                                transitions: None,
                                output: Some("bbu"),
                            },
                        ),
                        (
                            'ぷ',
                            Node {
                                transitions: None,
                                output: Some("ppu"),
                            },
                        ),
                        (
                            'へ',
                            Node {
                                transitions: None,
                                output: Some("hhe"),
                            },
                        ),
                        (
                            'べ',
                            Node {
                                transitions: None,
                                output: Some("bbe"),
                            },
                        ),
                        (
                            'ぺ',
                            Node {
                                transitions: None,
                                output: Some("ppe"),
                            },
                        ),
                        (
                            'ほ',
                            Node {
                                transitions: None,
                                output: Some("hho"),
                            },
                        ),
                        (
                            'ぼ',
                            Node {
                                transitions: None,
                                output: Some("bbo"),
                            },
                        ),
                        (
                            'ぽ',
                            Node {
                                transitions: None,
                                output: Some("ppo"),
                            },
                        ),
                        (
                            'ま',
                            Node {
                                transitions: None,
                                output: Some("mma"),
                            },
                        ),
                        (
                            'み',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("mmyi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("mmye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("mmya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("mmyu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("mmyo"),
                                        },
                                    ),
                                ]),
                                output: Some("mmi"),
                            },
                        ),
                        (
                            'む',
                            Node {
                                transitions: None,
                                output: Some("mmu"),
                            },
                        ),
                        (
                            'め',
                            Node {
                                transitions: None,
                                output: Some("mme"),
                            },
                        ),
                        (
                            'も',
                            Node {
                                transitions: None,
                                output: Some("mmo"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("ya"),
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: Some("ya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("yu"),
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: Some("yu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("yo"),
                            },
                        ),
                        (
                            'よ',
                            Node {
                                transitions: None,
                                output: Some("yo"),
                            },
                        ),
                        (
                            'ら',
                            Node {
                                transitions: None,
                                output: Some("rra"),
                            },
                        ),
                        (
                            'り',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("rryi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("rrye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("rrya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("rryu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("rryo"),
                                        },
                                    ),
                                ]),
                                output: Some("rri"),
                            },
                        ),
                        (
                            'る',
                            Node {
                                transitions: None,
                                output: Some("rru"),
                            },
                        ),
                        (
                            'れ',
                            Node {
                                transitions: None,
                                output: Some("rre"),
                            },
                        ),
                        (
                            'ろ',
                            Node {
                                transitions: None,
                                output: Some("rro"),
                            },
                        ),
                        (
                            'わ',
                            Node {
                                transitions: None,
                                output: Some("wwa"),
                            },
                        ),
                        (
                            'ゐ',
                            Node {
                                transitions: None,
                                output: Some("wwi"),
                            },
                        ),
                        (
                            'ゑ',
                            Node {
                                transitions: None,
                                output: Some("wwe"),
                            },
                        ),
                        (
                            'を',
                            Node {
                                transitions: None,
                                output: Some("wwo"),
                            },
                        ),
                        (
                            'ん',
                            Node {
                                transitions: None,
                                output: Some("n"),
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: Some("vvyi"),
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: Some("vvye"),
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: Some("vvya"),
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: Some("vvyu"),
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: Some("vvyo"),
                                        },
                                    ),
                                ]),
                                output: Some("vvu"),
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: Some("vva"),
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: Some("vvi"),
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: Some("vve"),
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: Some("vvo"),
                            },
                        ),
                        (
                            '・',
                            Node {
                                transitions: None,
                                output: Some("/"),
                            },
                        ),
                        (
                            'ー',
                            Node {
                                transitions: None,
                                output: Some("-"),
                            },
                        ),
                        (
                            '！',
                            Node {
                                transitions: None,
                                output: Some("!"),
                            },
                        ),
                        (
                            '（',
                            Node {
                                transitions: None,
                                output: Some("("),
                            },
                        ),
                        (
                            '）',
                            Node {
                                transitions: None,
                                output: Some(")"),
                            },
                        ),
                        (
                            '：',
                            Node {
                                transitions: None,
                                output: Some(":"),
                            },
                        ),
                        (
                            '？',
                            Node {
                                transitions: None,
                                output: Some("?"),
                            },
                        ),
                        (
                            '［',
                            Node {
                                transitions: None,
                                output: Some("["),
                            },
                        ),
                        (
                            '］',
                            Node {
                                transitions: None,
                                output: Some("]"),
                            },
                        ),
                        (
                            '｛',
                            Node {
                                transitions: None,
                                output: Some("{"),
                            },
                        ),
                        (
                            '｝',
                            Node {
                                transitions: None,
                                output: Some("}"),
                            },
                        ),
                    ]),
                    output: Some(""),
                },
            ),
            (
                'つ',
                Node {
                    transitions: None,
                    output: Some("tu"),
                },
            ),
            (
                'づ',
                Node {
                    transitions: None,
                    output: Some("zu"),
                },
            ),
            (
                'て',
                Node {
                    transitions: None,
                    output: Some("te"),
                },
            ),
            (
                'で',
                Node {
                    transitions: None,
                    output: Some("de"),
                },
            ),
            (
                'と',
                Node {
                    transitions: None,
                    output: Some("to"),
                },
            ),
            (
                'ど',
                Node {
                    transitions: None,
                    output: Some("do"),
                },
            ),
            (
                'な',
                Node {
                    transitions: None,
                    output: Some("na"),
                },
            ),
            (
                'に',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("nyi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("nye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("nya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("nyu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("nyo"),
                            },
                        ),
                    ]),
                    output: Some("ni"),
                },
            ),
            (
                'ぬ',
                Node {
                    transitions: None,
                    output: Some("nu"),
                },
            ),
            (
                'ね',
                Node {
                    transitions: None,
                    output: Some("ne"),
                },
            ),
            (
                'の',
                Node {
                    transitions: None,
                    output: Some("no"),
                },
            ),
            (
                'は',
                Node {
                    transitions: None,
                    output: Some("ha"),
                },
            ),
            (
                'ば',
                Node {
                    transitions: None,
                    output: Some("ba"),
                },
            ),
            (
                'ぱ',
                Node {
                    transitions: None,
                    output: Some("pa"),
                },
            ),
            (
                'ひ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("hyi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("hye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("hya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("hyu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("hyo"),
                            },
                        ),
                    ]),
                    output: Some("hi"),
                },
            ),
            (
                'び',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("byi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("bye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("bya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("byu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("byo"),
                            },
                        ),
                    ]),
                    output: Some("bi"),
                },
            ),
            (
                'ぴ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("pyi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("pye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("pya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("pyu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("pyo"),
                            },
                        ),
                    ]),
                    output: Some("pi"),
                },
            ),
            (
                'ふ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("hyi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("hye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("hya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("hyu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("hyo"),
                            },
                        ),
                    ]),
                    output: Some("hu"),
                },
            ),
            (
                'ぶ',
                Node {
                    transitions: None,
                    output: Some("bu"),
                },
            ),
            (
                'ぷ',
                Node {
                    transitions: None,
                    output: Some("pu"),
                },
            ),
            (
                'へ',
                Node {
                    transitions: None,
                    output: Some("he"),
                },
            ),
            (
                'べ',
                Node {
                    transitions: None,
                    output: Some("be"),
                },
            ),
            (
                'ぺ',
                Node {
                    transitions: None,
                    output: Some("pe"),
                },
            ),
            (
                'ほ',
                Node {
                    transitions: None,
                    output: Some("ho"),
                },
            ),
            (
                'ぼ',
                Node {
                    transitions: None,
                    output: Some("bo"),
                },
            ),
            (
                'ぽ',
                Node {
                    transitions: None,
                    output: Some("po"),
                },
            ),
            (
                'ま',
                Node {
                    transitions: None,
                    output: Some("ma"),
                },
            ),
            (
                'み',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("myi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("mye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("mya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("myu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("myo"),
                            },
                        ),
                    ]),
                    output: Some("mi"),
                },
            ),
            (
                'む',
                Node {
                    transitions: None,
                    output: Some("mu"),
                },
            ),
            (
                'め',
                Node {
                    transitions: None,
                    output: Some("me"),
                },
            ),
            (
                'も',
                Node {
                    transitions: None,
                    output: Some("mo"),
                },
            ),
            (
                'ゃ',
                Node {
                    transitions: None,
                    output: Some("ya"),
                },
            ),
            (
                'や',
                Node {
                    transitions: None,
                    output: Some("ya"),
                },
            ),
            (
                'ゅ',
                Node {
                    transitions: None,
                    output: Some("yu"),
                },
            ),
            (
                'ゆ',
                Node {
                    transitions: None,
                    output: Some("yu"),
                },
            ),
            (
                'ょ',
                Node {
                    transitions: None,
                    output: Some("yo"),
                },
            ),
            (
                'よ',
                Node {
                    transitions: None,
                    output: Some("yo"),
                },
            ),
            (
                'ら',
                Node {
                    transitions: None,
                    output: Some("ra"),
                },
            ),
            (
                'り',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("ryi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("rye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("rya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("ryu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("ryo"),
                            },
                        ),
                    ]),
                    output: Some("ri"),
                },
            ),
            (
                'る',
                Node {
                    transitions: None,
                    output: Some("ru"),
                },
            ),
            (
                'れ',
                Node {
                    transitions: None,
                    output: Some("re"),
                },
            ),
            (
                'ろ',
                Node {
                    transitions: None,
                    output: Some("ro"),
                },
            ),
            (
                'わ',
                Node {
                    transitions: None,
                    output: Some("wa"),
                },
            ),
            (
                'ゐ',
                Node {
                    transitions: None,
                    output: Some("wi"),
                },
            ),
            (
                'ゑ',
                Node {
                    transitions: None,
                    output: Some("we"),
                },
            ),
            (
                'を',
                Node {
                    transitions: None,
                    output: Some("wo"),
                },
            ),
            (
                'ん',
                Node {
                    transitions: Some(vec![
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: Some("n\'a"),
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: Some("n\'i"),
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: Some("n\'u"),
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: Some("n\'e"),
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: Some("n\'o"),
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: Some("n\'ya"),
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: Some("n\'yu"),
                            },
                        ),
                        (
                            'よ',
                            Node {
                                transitions: None,
                                output: Some("n\'yo"),
                            },
                        ),
                    ]),
                    output: Some("n"),
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: Some("vyi"),
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: Some("vye"),
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: Some("vya"),
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: Some("vyu"),
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: Some("vyo"),
                            },
                        ),
                    ]),
                    output: Some("vu"),
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: Some("va"),
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: Some("vi"),
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: Some("ve"),
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: Some("vo"),
                },
            ),
            (
                '・',
                Node {
                    transitions: None,
                    output: Some("/"),
                },
            ),
            (
                'ー',
                Node {
                    transitions: None,
                    output: Some("-"),
                },
            ),
            (
                '！',
                Node {
                    transitions: None,
                    output: Some("!"),
                },
            ),
            (
                '（',
                Node {
                    transitions: None,
                    output: Some("("),
                },
            ),
            (
                '）',
                Node {
                    transitions: None,
                    output: Some(")"),
                },
            ),
            (
                '：',
                Node {
                    transitions: None,
                    output: Some(":"),
                },
            ),
            (
                '？',
                Node {
                    transitions: None,
                    output: Some("?"),
                },
            ),
            (
                '［',
                Node {
                    transitions: None,
                    output: Some("["),
                },
            ),
            (
                '］',
                Node {
                    transitions: None,
                    output: Some("]"),
                },
            ),
            (
                '｛',
                Node {
                    transitions: None,
                    output: Some("{"),
                },
            ),
            (
                '｝',
                Node {
                    transitions: None,
                    output: Some("}"),
                },
            ),
        ]);

        let mut node = Node {
            transitions,
            output: Some(""),
        };
        node.sort();
        node
    };
}
