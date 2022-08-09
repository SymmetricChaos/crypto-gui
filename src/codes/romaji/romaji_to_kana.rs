use lazy_static::lazy_static;

use crate::tokenizer::Node;

lazy_static! {
    pub static ref ROMAJI_TO_KANA: Node = {
        let transitions = vec![
            Node::leaf('1',"！"),
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
            Node::leaf('a',"あ"),
            Node::leaf('e',""),
            Node::leaf('i',""),
            Node::leaf('o',""),
            Node::leaf('u',""),

            Node::branch(
                'b', None,
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
                                ])
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
            Node::branch(
                'c', None,
                vec![
                    Node::leaf('a', "か"),
                    Node::leaf('e', "け"),
                    Node::leaf('i', "き"),
                    Node::leaf('o', "こ"),
                    Node::leaf('u', "く"),
                    Node::branch('c', None,
                        vec![
                            Node::leaf('a', "っか"),
                            Node::leaf('e', "っけ"),
                            Node::leaf('i', "っき"),
                            Node::leaf('o', "っこ"),
                            Node::leaf('u', "っく"),

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っきゃ"),
                                    Node::leaf('e', "っきぇ"),
                                    Node::leaf('i', "っきぃ"),
                                    Node::leaf('o', "っきょ"),
                                    Node::leaf('u', "っきゅ"),
                                ])
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
                    // chi branch
                    Node::branch('h', None,
                        vec![
                            Node::leaf('i', "ち")
                        ]
                    ),
                ]
            ),
            Node::branch(
                'd', None,
                vec![

                ]
            ),
            Node::branch(
                'f', None,
                vec![

                ]
            ),
            Node::branch(
                'g', None,
                vec![

                ]
            ),
            Node::branch(
                'h', None,
                vec![

                ]
            ),
            Node::branch(
                'j', None,
                vec![

                ]
            ),
            Node::branch(
                'k', None,
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

                            Node::branch('y', None,
                                vec![
                                    Node::leaf('a', "っきゃ"),
                                    Node::leaf('e', "っきぇ"),
                                    Node::leaf('i', "っきぃ"),
                                    Node::leaf('o', "っきょ"),
                                    Node::leaf('u', "っきゅ"),
                                ])
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
            Node::branch(
                'l', None,
                vec![
                    Node::leaf('a', "ら"),
                    Node::leaf('e', "れ"),
                    Node::leaf('i', "り"),
                    Node::leaf('o', "ろ"),
                    Node::leaf('u', "る"),

                    Node::branch('l', None,
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
            Node::branch(
                'm', None,
                vec![

                ]
            ),
            Node::branch(
                'n', None,
                vec![

                ]
            ),
            Node::branch(
                'p', None,
                vec![

                ]
            ),
            Node::branch(
                'r', None,
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
            Node::branch(
                's', None,
                vec![

                ]
            ),
            Node::branch(
                't', None,
                vec![

                ]
            ),
            Node::branch(
                'w', None,
                vec![

                ]
            ),
            Node::branch(
                'y', None,
                vec![

                ]
            ),
            Node::branch(
                'z', None,
                vec![

                ]
            ),
        ];

        let mut node = Node { transitions: Some(transitions), output: None };
        node.sort();
        node
    };
}

//             (
//                 'c',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("か"),
//                             },
//                         ),
//                         (
//                             'c',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っか"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っけ"),
//                                         },
//                                     ),
//                                     (
//                                         'h',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っち"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちゅ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'y',
//                                                     Node {
//                                                         transitions: vec![
//                                                             (
//                                                                 'a',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っちゃ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'e',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っちぇ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'i',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っちぃ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'o',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っちょ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'u',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っちゅ"),
//                                                                 },
//                                                             ),
//                                                         ],
//                                                         output: None,
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っき"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っこ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っく"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("け"),
//                             },
//                         ),
//                         (
//                             'h',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ち"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちゅ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ちゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ちぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ちぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ちょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ちゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("き"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("こ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("く"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'd',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("だ"),
//                             },
//                         ),
//                         (
//                             'd',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っだ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っで"),
//                                         },
//                                     ),
//                                     (
//                                         'h',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っでゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っでぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っでぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っでょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っでゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぢ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っど"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っづ"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っどぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っどぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っどぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っどぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っどぅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぢゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぢぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぢぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぢょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぢゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("で"),
//                             },
//                         ),
//                         (
//                             'h',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("でゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("でぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("でぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("でょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("でゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぢ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ど"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("づ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("どぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("どぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("どぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("どぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("どぅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぢゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぢぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぢぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぢょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぢゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'e',
//                 Node {
//                     transitions: vec![],
//                     output: Some("え"),
//                 },
//             ),
//             (
//                 'f',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ふぁ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ふぇ"),
//                             },
//                         ),
//                         (
//                             'f',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っふぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っふぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っふぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っふぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っふ"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っふゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ふぃ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ふぉ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ふ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ふゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'g',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("が"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("げ"),
//                             },
//                         ),
//                         (
//                             'g',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っが"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っげ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぎ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っご"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぐ"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぐぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぐぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぐぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぐぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぐぅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぎゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぎぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぎぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぎょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぎゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぎ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ご"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぐ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぐぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぐぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぐぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぐぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぐぅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぎゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぎぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぎぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぎょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぎゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'h',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("は"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("へ"),
//                             },
//                         ),
//                         (
//                             'h',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っは"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っへ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っひ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っほ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っふ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っひゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っひぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っひぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っひょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っひゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ひ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ほ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ふ"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ひゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ひぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ひぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ひょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ひゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'i',
//                 Node {
//                     transitions: vec![],
//                     output: Some("い"),
//                 },
//             ),
//             (
//                 'j',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("じゃ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("じぇ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("じ"),
//                             },
//                         ),
//                         (
//                             'j',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っじゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っじぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っじ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っじょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っじゅ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("じょ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("じゅ"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'k',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("か"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("け"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("き"),
//                             },
//                         ),
//                         (
//                             'k',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っか"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っけ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っき"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っこ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っく"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![(
//                                                 'a',
//                                                 Node {
//                                                     transitions: vec![],
//                                                     output: Some("っくぁ"),
//                                                 },
//                                             )],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っきゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っきぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っきぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っきょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っきゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("こ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("く"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![(
//                                     'a',
//                                     Node {
//                                         transitions: vec![],
//                                         output: Some("くぁ"),
//                                     },
//                                 )],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("きゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("きぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("きぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("きょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("きゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'l',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぁ"),
//                             },
//                         ),
//                         (
//                             'c',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヵ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヶ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぇ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぃ"),
//                             },
//                         ),
//                         (
//                             'k',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヵ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヶ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぉ"),
//                             },
//                         ),
//                         (
//                             't',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         's',
//                                         Node {
//                                             transitions: vec![(
//                                                 'u',
//                                                 Node {
//                                                     transitions: vec![],
//                                                     output: Some("っ"),
//                                                 },
//                                             )],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぅ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![(
//                                     'a',
//                                     Node {
//                                         transitions: vec![],
//                                         output: Some("ゎ"),
//                                     },
//                                 )],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'm',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ま"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("め"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("み"),
//                             },
//                         ),
//                         (
//                             'm',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っま"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っめ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っみ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っも"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っむ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っみゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っみぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っみぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っみょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っみゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("も"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("む"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("みゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("みぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("みぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("みょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("みゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'n',
//                 Node {
//                     transitions: vec![
//                         (
//                             '\'',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ん"),
//                             },
//                         ),
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("な"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ね"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("に"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("の"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぬ"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("にゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("にぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("にぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("にょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("にゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: Some("ん"),
//                 },
//             ),
//             (
//                 'o',
//                 Node {
//                     transitions: vec![],
//                     output: Some("お"),
//                 },
//             ),
//             (
//                 'p',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぱ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぺ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぴ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぽ"),
//                             },
//                         ),
//                         (
//                             'p',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぱ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぺ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぴ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぽ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぷ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぴゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぴぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぴぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぴょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っぴゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぷ"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぴゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぴぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぴぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぴょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぴゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'q',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("くぁ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("くぇ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("くぃ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("くぉ"),
//                             },
//                         ),
//                         (
//                             'q',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っくぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っくぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っくぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っくぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っくぅ"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っくゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("くぅ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("くゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'r',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ら"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("れ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("り"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ろ"),
//                             },
//                         ),
//                         (
//                             'r',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っら"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っれ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っり"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っろ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っる"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っりゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っりぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っりぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っりょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っりゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("る"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("りゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("りぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("りぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("りょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("りゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 's',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("さ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("せ"),
//                             },
//                         ),
//                         (
//                             'h',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("し"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しゅ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("しゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("しぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("しぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("しょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("しゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("し"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("そ"),
//                             },
//                         ),
//                         (
//                             's',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っさ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っせ"),
//                                         },
//                                     ),
//                                     (
//                                         'h',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っし"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしゅ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'y',
//                                                     Node {
//                                                         transitions: vec![
//                                                             (
//                                                                 'a',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っしゃ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'e',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っしぇ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'i',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っしぃ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'o',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っしょ"),
//                                                                 },
//                                                             ),
//                                                             (
//                                                                 'u',
//                                                                 Node {
//                                                                     transitions: vec![],
//                                                                     output: Some("っしゅ"),
//                                                                 },
//                                                             ),
//                                                         ],
//                                                         output: None,
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っし"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っそ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っす"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っすぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っすぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っすぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っすぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っすぅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っしゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("す"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("すぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("すぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("すぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("すぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("すぅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("しゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 't',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("た"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("て"),
//                             },
//                         ),
//                         (
//                             'h',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("てゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("てぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("てぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("てょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("てゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ち"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("と"),
//                             },
//                         ),
//                         (
//                             's',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("つぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("つぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("つぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("つぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("つ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             't',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("った"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("って"),
//                                         },
//                                     ),
//                                     (
//                                         'h',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ってゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ってぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ってぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ってょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("ってゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っち"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っと"),
//                                         },
//                                     ),
//                                     (
//                                         's',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っつぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っつぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っつぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っつぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っつ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っつ"),
//                                         },
//                                     ),
//                                     (
//                                         'w',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っとぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っとぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っとぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っとぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っとぅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っちゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("つ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("とぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("とぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("とぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("とぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("とぅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ちゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'u',
//                 Node {
//                     transitions: vec![],
//                     output: Some("う"),
//                 },
//             ),
//             (
//                 'v',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ゔぁ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ゔぇ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ゔぃ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ゔぉ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ゔ"),
//                             },
//                         ),
//                         (
//                             'v',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っゔぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っゔぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っゔぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っゔぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っゔ"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っゔゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っゔぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っゔぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っゔょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っゔゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゔゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゔぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゔぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゔょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゔゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'w',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("わ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("うぇ"),
//                             },
//                         ),
//                         (
//                             'h',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("うぁ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("うぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("うぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("うぉ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("う"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("うぃ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("を"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("う"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っわ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っうぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'h',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っうぁ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っうぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っうぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っうぉ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っう"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っうぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っを"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っう"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'x',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぁ"),
//                             },
//                         ),
//                         (
//                             'c',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヵ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヶ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぇ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぃ"),
//                             },
//                         ),
//                         (
//                             'k',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヵ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ヶ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'n',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ん"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぉ"),
//                             },
//                         ),
//                         (
//                             't',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         's',
//                                         Node {
//                                             transitions: vec![(
//                                                 'u',
//                                                 Node {
//                                                     transitions: vec![],
//                                                     output: Some("っ"),
//                                                 },
//                                             )],
//                                             output: None,
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぅ"),
//                             },
//                         ),
//                         (
//                             'w',
//                             Node {
//                                 transitions: vec![(
//                                     'a',
//                                     Node {
//                                         transitions: vec![],
//                                         output: Some("ゎ"),
//                                     },
//                                 )],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("ゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'y',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("や"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("いぇ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("い"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("よ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ゆ"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っや"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っいぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っい"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っよ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っゆ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 'z',
//                 Node {
//                     transitions: vec![
//                         (
//                             'a',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ざ"),
//                             },
//                         ),
//                         (
//                             'e',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぜ"),
//                             },
//                         ),
//                         (
//                             'i',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("じ"),
//                             },
//                         ),
//                         (
//                             'o',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ぞ"),
//                             },
//                         ),
//                         (
//                             'u',
//                             Node {
//                                 transitions: vec![],
//                                 output: Some("ず"),
//                             },
//                         ),
//                         (
//                             'y',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じゃ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じぇ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じぃ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じょ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("じゅ"),
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                         (
//                             'z',
//                             Node {
//                                 transitions: vec![
//                                     (
//                                         'a',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っざ"),
//                                         },
//                                     ),
//                                     (
//                                         'e',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぜ"),
//                                         },
//                                     ),
//                                     (
//                                         'i',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っじ"),
//                                         },
//                                     ),
//                                     (
//                                         'o',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っぞ"),
//                                         },
//                                     ),
//                                     (
//                                         'u',
//                                         Node {
//                                             transitions: vec![],
//                                             output: Some("っず"),
//                                         },
//                                     ),
//                                     (
//                                         'y',
//                                         Node {
//                                             transitions: vec![
//                                                 (
//                                                     'a',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじゃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'e',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじぇ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'i',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじぃ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'o',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじょ"),
//                                                     },
//                                                 ),
//                                                 (
//                                                     'u',
//                                                     Node {
//                                                         transitions: vec![],
//                                                         output: Some("っじゅ"),
//                                                     },
//                                                 ),
//                                             ],
//                                             output: None,
//                                         },
//                                     ),
//                                 ],
//                                 output: None,
//                             },
//                         ),
//                     ],
//                     output: None,
//                 },
//             ),
//             (
//                 '{',
//                 Node {
//                     transitions: vec![],
//                     output: Some("｛"),
//                 },
//             ),
//             (
//                 '}',
//                 Node {
//                     transitions: vec![],
//                     output: Some("｝"),
//                 },
//             ),
//             (
//                 '~',
//                 Node {
//                     transitions: vec![],
//                     output: Some("〜"),
//                 },
//             ),
//             (
//                 '‘',
//                 Node {
//                     transitions: vec![],
//                     output: Some("「"),
//                 },
//             ),
//             (
//                 '’',
//                 Node {
//                     transitions: vec![],
//                     output: Some("」"),
//                 },
//             ),
//             (
//                 '“',
//                 Node {
//                     transitions: vec![],
//                     output: Some("『"),
//                 },
//             ),
//             (
//                 '”',
//                 Node {
//                     transitions: vec![],
//                     output: Some("』"),
//                 },
//             ),
//         ];
//         let mut tree = Node { transitions, output: None };
//         tree.sort();
//         tree
//     };
// }
