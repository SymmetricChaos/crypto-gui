use lazy_static::lazy_static;

use crate::tokenizer::Node;

lazy_static! {
    pub static ref NIHON_SHIKI: Node = {
        let transitions = Some(vec![
            (
                '\u{3000}',
                Node {
                    transitions: None,
                    output: " ",
                },
            ),
            (
                '、',
                Node {
                    transitions: None,
                    output: ",",
                },
            ),
            (
                '。',
                Node {
                    transitions: None,
                    output: ".",
                },
            ),
            (
                '「',
                Node {
                    transitions: None,
                    output: "‘",
                },
            ),
            (
                '」',
                Node {
                    transitions: None,
                    output: "’",
                },
            ),
            (
                '『',
                Node {
                    transitions: None,
                    output: "“",
                },
            ),
            (
                '』',
                Node {
                    transitions: None,
                    output: "”",
                },
            ),
            (
                '〜',
                Node {
                    transitions: None,
                    output: "~",
                },
            ),
            (
                'ぁ',
                Node {
                    transitions: None,
                    output: "a",
                },
            ),
            (
                'あ',
                Node {
                    transitions: None,
                    output: "a",
                },
            ),
            (
                'ぃ',
                Node {
                    transitions: None,
                    output: "i",
                },
            ),
            (
                'い',
                Node {
                    transitions: None,
                    output: "i",
                },
            ),
            (
                'ぅ',
                Node {
                    transitions: None,
                    output: "u",
                },
            ),
            (
                'う',
                Node {
                    transitions: None,
                    output: "u",
                },
            ),
            (
                'ぇ',
                Node {
                    transitions: None,
                    output: "e",
                },
            ),
            (
                'え',
                Node {
                    transitions: None,
                    output: "e",
                },
            ),
            (
                'ぉ',
                Node {
                    transitions: None,
                    output: "o",
                },
            ),
            (
                'お',
                Node {
                    transitions: None,
                    output: "o",
                },
            ),
            (
                'か',
                Node {
                    transitions: None,
                    output: "ka",
                },
            ),
            (
                'が',
                Node {
                    transitions: None,
                    output: "ga",
                },
            ),
            (
                'き',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"kyi"),
                        Node::leaf('ぇ',"kye"),
                        Node::leaf('ゃ',"kya"),
                        Node::leaf('ゅ',"kyu"),
                        Node::leaf('ょ',"kyo"),
                    ]),
                    output: "ki",
                },
            ),
            (
                'ぎ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"gyi"),
                        Node::leaf('ぇ',"gye"),
                        Node::leaf('ゃ',"gya"),
                        Node::leaf('ゅ',"gyu"),
                        Node::leaf('ょ',"gyo"),
                    ]),
                    output: "gi",
                },
            ),
            (
                'く',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"kyi"),
                        Node::leaf('ぇ',"kye"),
                        Node::leaf('ゃ',"kya"),
                        Node::leaf('ゅ',"kyu"),
                        Node::leaf('ょ',"kyo"),
                    ]),
                    output: "ku",
                },
            ),
            (
                'ぐ',
                Node {
                    transitions: None,
                    output: "gu",
                },
            ),
            (
                'け',
                Node {
                    transitions: None,
                    output: "ke",
                },
            ),
            (
                'げ',
                Node {
                    transitions: None,
                    output: "ge",
                },
            ),
            (
                'こ',
                Node {
                    transitions: None,
                    output: "ko",
                },
            ),
            (
                'ご',
                Node {
                    transitions: None,
                    output: "go",
                },
            ),
            (
                'さ',
                Node {
                    transitions: None,
                    output: "sa",
                },
            ),
            (
                'ざ',
                Node {
                    transitions: None,
                    output: "za",
                },
            ),
            (
                'し',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"syi"),
                        Node::leaf('ぇ',"sye"),
                        Node::leaf('ゃ',"sya"),
                        Node::leaf('ゅ',"syu"),
                        Node::leaf('ょ',"syo"),
                    ]),
                    output: "si",
                },
            ),
            (
                'じ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"zyi"),
                        Node::leaf('ぇ',"zye"),
                        Node::leaf('ゃ',"zya"),
                        Node::leaf('ゅ',"zyu"),
                        Node::leaf('ょ',"zyo"),
                    ]),
                    output: "zi",
                },
            ),
            (
                'す',
                Node {
                    transitions: None,
                    output: "su",
                },
            ),
            (
                'ず',
                Node {
                    transitions: None,
                    output: "zu",
                },
            ),
            (
                'せ',
                Node {
                    transitions: None,
                    output: "se",
                },
            ),
            (
                'ぜ',
                Node {
                    transitions: None,
                    output: "ze",
                },
            ),
            (
                'そ',
                Node {
                    transitions: None,
                    output: "so",
                },
            ),
            (
                'ぞ',
                Node {
                    transitions: None,
                    output: "zo",
                },
            ),
            (
                'た',
                Node {
                    transitions: None,
                    output: "ta",
                },
            ),
            (
                'だ',
                Node {
                    transitions: None,
                    output: "da",
                },
            ),
            (
                'ち',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"tyi"),
                        Node::leaf('ぇ',"tye"),
                        Node::leaf('ゃ',"tya"),
                        Node::leaf('ゅ',"tyu"),
                        Node::leaf('ょ',"tyo"),
                    ]),
                    output: "ti",
                },
            ),
            (
                'ぢ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"dyi"),
                        Node::leaf('ぇ',"dye"),
                        Node::leaf('ゃ',"dya"),
                        Node::leaf('ゅ',"dyu"),
                        Node::leaf('ょ',"dyo"),
                    ]),
                    output: "di",
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
                                output: " ",
                            },
                        ),
                        (
                            '、',
                            Node {
                                transitions: None,
                                output: ",",
                            },
                        ),
                        (
                            '。',
                            Node {
                                transitions: None,
                                output: ".",
                            },
                        ),
                        (
                            '「',
                            Node {
                                transitions: None,
                                output: "‘",
                            },
                        ),
                        (
                            '」',
                            Node {
                                transitions: None,
                                output: "’",
                            },
                        ),
                        (
                            '『',
                            Node {
                                transitions: None,
                                output: "“",
                            },
                        ),
                        (
                            '』',
                            Node {
                                transitions: None,
                                output: "”",
                            },
                        ),
                        (
                            '〜',
                            Node {
                                transitions: None,
                                output: "~",
                            },
                        ),
                        (
                            'ぁ',
                            Node {
                                transitions: None,
                                output: "a",
                            },
                        ),
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: "a",
                            },
                        ),
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "i",
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "i",
                            },
                        ),
                        (
                            'ぅ',
                            Node {
                                transitions: None,
                                output: "u",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "u",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "e",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "e",
                            },
                        ),
                        (
                            'ぉ',
                            Node {
                                transitions: None,
                                output: "o",
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: "o",
                            },
                        ),
                        (
                            'か',
                            Node {
                                transitions: None,
                                output: "kka",
                            },
                        ),
                        (
                            'が',
                            Node {
                                transitions: None,
                                output: "gga",
                            },
                        ),
                        (
                            'き',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"kkyi"),
                                    Node::leaf('ぇ',"kkye"),
                                    Node::leaf('ゃ',"kkya"),
                                    Node::leaf('ゅ',"kkyu"),
                                    Node::leaf('ょ',"kkyo"),
                                ]),
                                output: "kki",
                            },
                        ),
                        (
                            'ぎ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"ggyi"),
                                    Node::leaf('ぇ',"ggye"),
                                    Node::leaf('ゃ',"ggya"),
                                    Node::leaf('ゅ',"ggyu"),
                                    Node::leaf('ょ',"ggyo"),
                                ]),
                                output: "ggi",
                            },
                        ),
                        (
                            'く',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"kkyi"),
                                    Node::leaf('ぇ',"kkye"),
                                    Node::leaf('ゃ',"kkya"),
                                    Node::leaf('ゅ',"kkyu"),
                                    Node::leaf('ょ',"kkyo"),
                                ]),
                                output: "kku",
                            },
                        ),
                        (
                            'ぐ',
                            Node {
                                transitions: None,
                                output: "ggu",
                            },
                        ),
                        (
                            'け',
                            Node {
                                transitions: None,
                                output: "kke",
                            },
                        ),
                        (
                            'げ',
                            Node {
                                transitions: None,
                                output: "gge",
                            },
                        ),
                        (
                            'こ',
                            Node {
                                transitions: None,
                                output: "kko",
                            },
                        ),
                        (
                            'ご',
                            Node {
                                transitions: None,
                                output: "ggo",
                            },
                        ),
                        (
                            'さ',
                            Node {
                                transitions: None,
                                output: "ssa",
                            },
                        ),
                        (
                            'ざ',
                            Node {
                                transitions: None,
                                output: "zza",
                            },
                        ),
                        (
                            'し',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"ssyi"),
                                    Node::leaf('ぇ',"ssye"),
                                    Node::leaf('ゃ',"ssya"),
                                    Node::leaf('ゅ',"ssyu"),
                                    Node::leaf('ょ',"ssyo"),
                                ]),
                                output: "ssi",
                            },
                        ),
                        (
                            'じ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"zzyi"),
                                    Node::leaf('ぇ',"zzye"),
                                    Node::leaf('ゃ',"zzya"),
                                    Node::leaf('ゅ',"zzyu"),
                                    Node::leaf('ょ',"zzyo"),
                                ]),
                                output: "zzi",
                            },
                        ),
                        (
                            'す',
                            Node {
                                transitions: None,
                                output: "ssu",
                            },
                        ),
                        (
                            'ず',
                            Node {
                                transitions: None,
                                output: "zzu",
                            },
                        ),
                        (
                            'せ',
                            Node {
                                transitions: None,
                                output: "sse",
                            },
                        ),
                        (
                            'ぜ',
                            Node {
                                transitions: None,
                                output: "zze",
                            },
                        ),
                        (
                            'そ',
                            Node {
                                transitions: None,
                                output: "sso",
                            },
                        ),
                        (
                            'ぞ',
                            Node {
                                transitions: None,
                                output: "zzo",
                            },
                        ),
                        (
                            'た',
                            Node {
                                transitions: None,
                                output: "tta",
                            },
                        ),
                        (
                            'だ',
                            Node {
                                transitions: None,
                                output: "dda",
                            },
                        ),
                        (
                            'ち',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"ttyi"),
                                    Node::leaf('ぇ',"ttye"),
                                    Node::leaf('ゃ',"ttya"),
                                    Node::leaf('ゅ',"ttyu"),
                                    Node::leaf('ょ',"ttyo"),
                                ]),
                                output: "tti",
                            },
                        ),
                        (
                            'ぢ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"ddyi"),
                                    Node::leaf('ぇ',"ddye"),
                                    Node::leaf('ゃ',"ddya"),
                                    Node::leaf('ゅ',"ddyu"),
                                    Node::leaf('ょ',"ddyo"),
                                ]),
                                output: "ddi",
                            },
                        ),
                        (
                            'つ',
                            Node {
                                transitions: None,
                                output: "ttu",
                            },
                        ),
                        (
                            'づ',
                            Node {
                                transitions: None,
                                output: "ddu",
                            },
                        ),
                        (
                            'て',
                            Node {
                                transitions: None,
                                output: "tte",
                            },
                        ),
                        (
                            'で',
                            Node {
                                transitions: None,
                                output: "dde",
                            },
                        ),
                        (
                            'と',
                            Node {
                                transitions: None,
                                output: "tto",
                            },
                        ),
                        (
                            'ど',
                            Node {
                                transitions: None,
                                output: "ddo",
                            },
                        ),
                        (
                            'な',
                            Node {
                                transitions: None,
                                output: "na",
                            },
                        ),
                        (
                            'に',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"nyi"),
                                    Node::leaf('ぇ',"nye"),
                                    Node::leaf('ゃ',"nya"),
                                    Node::leaf('ゅ',"nyu"),
                                    Node::leaf('ょ',"nyo"),
                                ]),
                                output: "ni",
                            },
                        ),
                        (
                            'ぬ',
                            Node {
                                transitions: None,
                                output: "nu",
                            },
                        ),
                        (
                            'ね',
                            Node {
                                transitions: None,
                                output: "ne",
                            },
                        ),
                        (
                            'の',
                            Node {
                                transitions: None,
                                output: "no",
                            },
                        ),
                        (
                            'は',
                            Node {
                                transitions: None,
                                output: "hha",
                            },
                        ),
                        (
                            'ば',
                            Node {
                                transitions: None,
                                output: "bba",
                            },
                        ),
                        (
                            'ぱ',
                            Node {
                                transitions: None,
                                output: "ppa",
                            },
                        ),
                        (
                            'ひ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"hhyi"),
                                    Node::leaf('ぇ',"hhye"),
                                    Node::leaf('ゃ',"hhya"),
                                    Node::leaf('ゅ',"hhyu"),
                                    Node::leaf('ょ',"hhyo"),
                                ]),
                                output: "hhi",
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
                                            output: "ppyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ppye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ppya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ppyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "ppyo",
                                        },
                                    ),
                                ]),
                                output: "ppi",
                            },
                        ),
                        (
                            'ぴ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"ppyi"),
                                    Node::leaf('ぇ',"ppye"),
                                    Node::leaf('ゃ',"ppya"),
                                    Node::leaf('ゅ',"ppyu"),
                                    Node::leaf('ょ',"ppyo"),
                                ]),
                                output: "ppi",
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
                                            output: "hhyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "hhye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "hhya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "hhyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "hhyo",
                                        },
                                    ),
                                ]),
                                output: "hhu",
                            },
                        ),
                        (
                            'ぶ',
                            Node {
                                transitions: None,
                                output: "bbu",
                            },
                        ),
                        (
                            'ぷ',
                            Node {
                                transitions: None,
                                output: "ppu",
                            },
                        ),
                        (
                            'へ',
                            Node {
                                transitions: None,
                                output: "hhe",
                            },
                        ),
                        (
                            'べ',
                            Node {
                                transitions: None,
                                output: "bbe",
                            },
                        ),
                        (
                            'ぺ',
                            Node {
                                transitions: None,
                                output: "ppe",
                            },
                        ),
                        (
                            'ほ',
                            Node {
                                transitions: None,
                                output: "hho",
                            },
                        ),
                        (
                            'ぼ',
                            Node {
                                transitions: None,
                                output: "bbo",
                            },
                        ),
                        (
                            'ぽ',
                            Node {
                                transitions: None,
                                output: "ppo",
                            },
                        ),
                        (
                            'ま',
                            Node {
                                transitions: None,
                                output: "mma",
                            },
                        ),
                        (
                            'み',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"mmyi"),
                                    Node::leaf('ぇ',"mmye"),
                                    Node::leaf('ゃ',"mmya"),
                                    Node::leaf('ゅ',"mmyu"),
                                    Node::leaf('ょ',"mmyo"),
                                ]),
                                output: "mmi",
                            },
                        ),
                        (
                            'む',
                            Node {
                                transitions: None,
                                output: "mmu",
                            },
                        ),
                        (
                            'め',
                            Node {
                                transitions: None,
                                output: "mme",
                            },
                        ),
                        (
                            'も',
                            Node {
                                transitions: None,
                                output: "mmo",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "ya",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "ya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "yu",
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: "yu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "yo",
                            },
                        ),
                        (
                            'よ',
                            Node {
                                transitions: None,
                                output: "yo",
                            },
                        ),
                        (
                            'ら',
                            Node {
                                transitions: None,
                                output: "rra",
                            },
                        ),
                        (
                            'り',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"rryi"),
                                    Node::leaf('ぇ',"rrye"),
                                    Node::leaf('ゃ',"rrya"),
                                    Node::leaf('ゅ',"rryu"),
                                    Node::leaf('ょ',"rryo"),
                                ]),
                                output: "rri",
                            },
                        ),
                        (
                            'る',
                            Node {
                                transitions: None,
                                output: "rru",
                            },
                        ),
                        (
                            'れ',
                            Node {
                                transitions: None,
                                output: "rre",
                            },
                        ),
                        (
                            'ろ',
                            Node {
                                transitions: None,
                                output: "rro",
                            },
                        ),
                        (
                            'わ',
                            Node {
                                transitions: None,
                                output: "wwa",
                            },
                        ),
                        (
                            'ゐ',
                            Node {
                                transitions: None,
                                output: "wwi",
                            },
                        ),
                        (
                            'ゑ',
                            Node {
                                transitions: None,
                                output: "wwe",
                            },
                        ),
                        (
                            'を',
                            Node {
                                transitions: None,
                                output: "wwo",
                            },
                        ),
                        (
                            'ん',
                            Node {
                                transitions: None,
                                output: "n",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('ぃ',"vvyi"),
                                    Node::leaf('ぇ',"vvye"),
                                    Node::leaf('ゃ',"vvya"),
                                    Node::leaf('ゅ',"vvyu"),
                                    Node::leaf('ょ',"vvyo"),
                                ]),
                                output: "vvu",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vva",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vvi",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vve",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vvo",
                            },
                        ),
                        (
                            '・',
                            Node {
                                transitions: None,
                                output: "/",
                            },
                        ),
                        (
                            'ー',
                            Node {
                                transitions: None,
                                output: "-",
                            },
                        ),
                        (
                            '！',
                            Node {
                                transitions: None,
                                output: "!",
                            },
                        ),
                        (
                            '（',
                            Node {
                                transitions: None,
                                output: "(",
                            },
                        ),
                        (
                            '）',
                            Node {
                                transitions: None,
                                output: ")",
                            },
                        ),
                        (
                            '：',
                            Node {
                                transitions: None,
                                output: ":",
                            },
                        ),
                        (
                            '？',
                            Node {
                                transitions: None,
                                output: "?",
                            },
                        ),
                        (
                            '［',
                            Node {
                                transitions: None,
                                output: "[",
                            },
                        ),
                        (
                            '］',
                            Node {
                                transitions: None,
                                output: "]",
                            },
                        ),
                        (
                            '｛',
                            Node {
                                transitions: None,
                                output: "{",
                            },
                        ),
                        (
                            '｝',
                            Node {
                                transitions: None,
                                output: "}",
                            },
                        ),
                    ]),
                    output: "",
                },
            ),
            (
                'つ',
                Node {
                    transitions: None,
                    output: "tu",
                },
            ),
            (
                'づ',
                Node {
                    transitions: None,
                    output: "du",
                },
            ),
            (
                'て',
                Node {
                    transitions: None,
                    output: "te",
                },
            ),
            (
                'で',
                Node {
                    transitions: None,
                    output: "de",
                },
            ),
            (
                'と',
                Node {
                    transitions: None,
                    output: "to",
                },
            ),
            (
                'ど',
                Node {
                    transitions: None,
                    output: "do",
                },
            ),
            (
                'な',
                Node {
                    transitions: None,
                    output: "na",
                },
            ),
            (
                'に',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"nnyi"),
                        Node::leaf('ぇ',"nnye"),
                        Node::leaf('ゃ',"nnya"),
                        Node::leaf('ゅ',"nnyu"),
                        Node::leaf('ょ',"nnyo"),
                    ]),
                    output: "ni",
                },
            ),
            (
                'ぬ',
                Node {
                    transitions: None,
                    output: "nu",
                },
            ),
            (
                'ね',
                Node {
                    transitions: None,
                    output: "ne",
                },
            ),
            (
                'の',
                Node {
                    transitions: None,
                    output: "no",
                },
            ),
            (
                'は',
                Node {
                    transitions: None,
                    output: "ha",
                },
            ),
            (
                'ば',
                Node {
                    transitions: None,
                    output: "ba",
                },
            ),
            (
                'ぱ',
                Node {
                    transitions: None,
                    output: "pa",
                },
            ),
            (
                'ひ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"hyi"),
                        Node::leaf('ぇ',"hye"),
                        Node::leaf('ゃ',"hya"),
                        Node::leaf('ゅ',"hyu"),
                        Node::leaf('ょ',"hyo"),
                    ]),
                    output: "hi",
                },
            ),
            (
                'び',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"byi"),
                        Node::leaf('ぇ',"bye"),
                        Node::leaf('ゃ',"bya"),
                        Node::leaf('ゅ',"byu"),
                        Node::leaf('ょ',"byo"),
                    ]),
                    output: "bi",
                },
            ),
            (
                'ぴ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"pyi"),
                        Node::leaf('ぇ',"pye"),
                        Node::leaf('ゃ',"pya"),
                        Node::leaf('ゅ',"pyu"),
                        Node::leaf('ょ',"pyo"),
                    ]),
                    output: "pi",
                },
            ),
            (
                'ふ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"hyi"),
                        Node::leaf('ぇ',"hye"),
                        Node::leaf('ゃ',"hya"),
                        Node::leaf('ゅ',"hyu"),
                        Node::leaf('ょ',"hyo"),
                    ]),
                    output: "hu",
                },
            ),
            (
                'ぶ',
                Node {
                    transitions: None,
                    output: "bu",
                },
            ),
            (
                'ぷ',
                Node {
                    transitions: None,
                    output: "pu",
                },
            ),
            (
                'へ',
                Node {
                    transitions: None,
                    output: "he",
                },
            ),
            (
                'べ',
                Node {
                    transitions: None,
                    output: "be",
                },
            ),
            (
                'ぺ',
                Node {
                    transitions: None,
                    output: "pe",
                },
            ),
            (
                'ほ',
                Node {
                    transitions: None,
                    output: "ho",
                },
            ),
            (
                'ぼ',
                Node {
                    transitions: None,
                    output: "bo",
                },
            ),
            (
                'ぽ',
                Node {
                    transitions: None,
                    output: "po",
                },
            ),
            (
                'ま',
                Node {
                    transitions: None,
                    output: "ma",
                },
            ),
            (
                'み',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"myi"),
                        Node::leaf('ぇ',"mye"),
                        Node::leaf('ゃ',"mya"),
                        Node::leaf('ゅ',"myu"),
                        Node::leaf('ょ',"myo"),
                    ]),
                    output: "mi",
                },
            ),
            (
                'む',
                Node {
                    transitions: None,
                    output: "mu",
                },
            ),
            (
                'め',
                Node {
                    transitions: None,
                    output: "me",
                },
            ),
            (
                'も',
                Node {
                    transitions: None,
                    output: "mo",
                },
            ),
            (
                'ゃ',
                Node {
                    transitions: None,
                    output: "ya",
                },
            ),
            (
                'や',
                Node {
                    transitions: None,
                    output: "ya",
                },
            ),
            (
                'ゅ',
                Node {
                    transitions: None,
                    output: "yu",
                },
            ),
            (
                'ゆ',
                Node {
                    transitions: None,
                    output: "yu",
                },
            ),
            (
                'ょ',
                Node {
                    transitions: None,
                    output: "yo",
                },
            ),
            (
                'よ',
                Node {
                    transitions: None,
                    output: "yo",
                },
            ),
            (
                'ら',
                Node {
                    transitions: None,
                    output: "ra",
                },
            ),
            (
                'り',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"ryi"),
                        Node::leaf('ぇ',"rye"),
                        Node::leaf('ゃ',"rya"),
                        Node::leaf('ゅ',"ryu"),
                        Node::leaf('ょ',"ryo"),
                    ]),
                    output: "ri",
                },
            ),
            (
                'る',
                Node {
                    transitions: None,
                    output: "ru",
                },
            ),
            (
                'れ',
                Node {
                    transitions: None,
                    output: "re",
                },
            ),
            (
                'ろ',
                Node {
                    transitions: None,
                    output: "ro",
                },
            ),
            (
                'わ',
                Node {
                    transitions: None,
                    output: "wa",
                },
            ),
            (
                'ゐ',
                Node {
                    transitions: None,
                    output: "wi",
                },
            ),
            (
                'ゑ',
                Node {
                    transitions: None,
                    output: "we",
                },
            ),
            (
                'を',
                Node {
                    transitions: None,
                    output: "wo",
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
                                output: "n\'a",
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "n\'i",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "n\'u",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "n\'e",
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: "n\'o",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "n\'ya",
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: "n\'yu",
                            },
                        ),
                        (
                            'よ',
                            Node {
                                transitions: None,
                                output: "n\'yo",
                            },
                        ),
                    ]),
                    output: "n",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: Some(vec![
                        Node::leaf('ぃ',"vyi"),
                        Node::leaf('ぇ',"vye"),
                        Node::leaf('ゃ',"vya"),
                        Node::leaf('ゅ',"vyu"),
                        Node::leaf('ょ',"vyo"),
                    ]),
                    output: "vu",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "va",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "vi",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "ve",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "vo",
                },
            ),
            (
                '・',
                Node {
                    transitions: None,
                    output: "/",
                },
            ),
            (
                'ー',
                Node {
                    transitions: None,
                    output: "-",
                },
            ),
            (
                '！',
                Node {
                    transitions: None,
                    output: "!",
                },
            ),
            (
                '（',
                Node {
                    transitions: None,
                    output: "(",
                },
            ),
            (
                '）',
                Node {
                    transitions: None,
                    output: ")",
                },
            ),
            (
                '：',
                Node {
                    transitions: None,
                    output: ":",
                },
            ),
            (
                '？',
                Node {
                    transitions: None,
                    output: "?",
                },
            ),
            (
                '［',
                Node {
                    transitions: None,
                    output: "[",
                },
            ),
            (
                '］',
                Node {
                    transitions: None,
                    output: "]",
                },
            ),
            (
                '｛',
                Node {
                    transitions: None,
                    output: "{",
                },
            ),
            (
                '｝',
                Node {
                    transitions: None,
                    output: "}",
                },
            ),
        ]);

        let mut node = Node { transitions, output: "" };
        node.sort();
        node
    };
}