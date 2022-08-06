use lazy_static::lazy_static;

use crate::tokenizer::Node;

lazy_static! {
    pub static ref SYLLABARY: Node = {

        let transitions = Some(vec![
            Node::branch(
                'a', "a",
                vec![
                    Node::leaf('r',"ar"),
                    Node::leaf('t',"at"),
                    Node::branch(
                        'n', "an",
                        vec![
                            Node::leaf('d', "and")
                        ]
                    )

                ]
            ),
            Node::leaf('b',"b"),
            (
                'c',
                Node {
                    transitions: Some(vec![
                        Node::leaf('c',"ch"),
                    ]),
                    output: "c"
                }
            ),
            Node::leaf('d',"d"),
            (
                'e',
                Node {
                    transitions: Some(vec![
                    Node::leaf('r',"er"),
                    Node::leaf('s',"es"),
                    (
                        'n',
                        Node {
                            transitions: Some(vec![
                                Node::leaf('t',"ent"),
                            ]),
                            output: "en"
                        }
                    ),
                    ]),
                    output: "e"
                }
            ),
            Node::leaf('f',"f"),
            Node::leaf('g',"g"),
            Node::leaf('h',"h"),
            (
                'i',
                Node {
                    transitions: Some(vec![
                        Node::leaf('t',"it"),
                        (
                            'n',
                            Node {
                                transitions: Some(vec![
                                Node::leaf('g',"ing"),
                            ]),
                            output: "in"
                        }
                        )
                    ]),
                    output: "i"
                }
            ),
            Node::leaf('j',"j"),
            Node::leaf('k',"k"),
            Node::leaf('l',"l"),
            Node::leaf('m',"m"),
            Node::leaf('n',"n"),
            (
                'o',
                Node {
                    transitions: Some(vec![
                        Node::leaf('f',"of"),
                        Node::leaf('r',"or"),
                    ]),
                    output: "o"
                }
            ),
            Node::leaf('o',"o"),
            (
                'q',
                Node {
                    transitions: Some(vec![
                        Node::leaf('u',"qu"),
                    ]),
                    output: "q"
                }
            ),
            Node::leaf('r',"r"),
            Node::leaf('s',"s"),
            (
                't',
                Node {
                    transitions: Some(vec![
                        Node::leaf('i',"ti"),
                        Node::leaf('o',"to"),
                        (
                            'h',
                            Node {
                                transitions: Some(vec![
                                    Node::leaf('e',"the"),
                                    Node::leaf('a',"tha"),
                                    Node::leaf('i',"thi"),
                                ]),
                                output: "th"
                            })
                        ]),
                    output: "t"
                }
            ),
            Node::leaf('u',"u"),
            Node::leaf('v',"v"),
            (
                'w',
                Node {
                    transitions: Some(vec![
                        Node::leaf('a',"wa"),
                    ]),
                    output: "w"
                }
            ),
            Node::leaf('x',"x"),
            Node::leaf('y',"y"),
            Node::leaf('z',"z"),
        ]);
        Node { transitions, output: "" }
    };
}