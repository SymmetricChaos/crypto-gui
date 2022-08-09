use std::fmt;

#[derive(Clone, Debug)]
pub enum TokenError {
    NoTransition(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    pub output: Option<&'static str>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.transitions {
            Some(v) => {
                if let Some(output) = self.output {
                    let mut s = output.to_string();
                    for (_, n) in v {
                        s.push_str(&format!("({})", n))
                    }
                    write!(f, "{}", s)
                } else {
                    let mut s = String::new();
                    for (_, n) in v {
                        s.push_str(&format!("({})", n))
                    }
                    write!(f, "{}", s)
                }
            }
            None => {
                if let Some(output) = self.output {
                    write!(f, "{}", output)
                } else {
                    write!(f, "")
                }
            }
        }
    }
}

impl Node {
    // It is invalid for a leaf to have no output
    pub fn leaf(c: char, output: &'static str) -> (char, Node) {
        (
            c,
            Node {
                transitions: None,
                output: Some(output),
            },
        )
    }

    pub fn branch(
        c: char,
        output: Option<&'static str>,
        transitions: Vec<(char, Node)>,
    ) -> (char, Node) {
        (
            c,
            Node {
                transitions: Some(transitions),
                output,
            },
        )
    }

    pub fn tree(transitions: Vec<(char, Node)>) -> Node {
        Node {
            transitions: Some(transitions),
            output: None,
        }
    }

    pub fn get<'a>(&self, chars: &'a [char]) -> Result<(&'static str, usize), TokenError> {
        let mut i = 0;
        let mut curr_node = self;
        for char in chars.iter() {
            // find the transition to the next node or break if there is no
            // transition
            // a lack of transition could be a leaf node or a could mean that
            // the character has no transition from this node
            if let Some(trans_node) = curr_node.find_transition_node(*char) {
                curr_node = trans_node;
            } else {
                break;
            }
            i += 1;
        }

        // if an output exists then provide it and the index reached
        // otherwise the string being parsed is invalid
        if let Some(output) = curr_node.output {
            Ok((output, i))
        } else {
            let chunk: String = match i {
                0 => chars[0].to_string(),
                _ => chars[0..i].iter().collect(),
            };
            Err(TokenError::NoTransition(chunk))
        }
    }

    pub fn find_transition_node(&self, char: char) -> Option<&Node> {
        // If transitions exist find one that acts on 'char' and return it, if
        // there is no such node return none. At a leaf return none.
        if let Some(t) = &self.transitions {
            t.binary_search_by_key(&char, |t| t.0)
                .ok()
                .map(|index| &t[index].1)
        } else {
            None
        }
    }

    pub fn extract_tokens(&self, text: &str) -> Result<Vec<String>, TokenError> {
        let chars = text.chars().collect::<Vec<_>>();
        let mut ouput = Vec::new();
        let len = chars.len();
        let mut curr_pos = 0;

        while curr_pos != len {
            let result = self.get(&chars[curr_pos..])?;
            ouput.push(result.0.to_string());
            curr_pos += result.1;
        }
        Ok(ouput)
    }

    pub fn sort(&mut self) {
        if let Some(transitions) = &mut self.transitions {
            transitions.sort_by_key(|el| el.0);
            for el in transitions {
                el.1.sort();
            }
        }
    }

    pub fn num_output_paths(&self) -> usize {
        match &self.transitions {
            Some(v) => {
                let mut sum = match self.output {
                    Some(_) => 1,
                    None => 0,
                };
                for (_, n) in v {
                    sum += n.num_output_paths()
                }
                sum
            }
            None => 1,
        }
    }
}

#[test]
fn test() {
    let transitions = vec![
        Node::branch(
            'a',
            Some("a"),
            vec![
                Node::branch('n', Some("an"), vec![Node::leaf('d', "and")]),
                Node::leaf('r', "ar"),
                Node::leaf('t', "at"),
            ],
        ),
        Node::leaf('b', "b"),
        Node::branch('c', Some("c"), vec![Node::leaf('c', "ch")]),
        Node::leaf('d', "d"),
        Node::branch(
            'e',
            Some("e"),
            vec![
                Node::leaf('r', "er"),
                Node::leaf('s', "es"),
                Node::branch('n', Some("en"), vec![Node::leaf('t', "ent")]),
            ],
        ),
        Node::branch(
            'f',
            None,
            vec![
                Node::leaf('a', "fr"),
                Node::leaf('a', "fs"),
                Node::branch('n', Some("fn"), vec![Node::leaf('t', "fnt")]),
                Node::branch('x', None, vec![Node::leaf('t', "fnx")]),
            ],
        ),
    ];

    let mut tree = Node::tree(transitions);
    tree.sort();
    println!("{}", &tree);
    println!("{}", &tree.num_output_paths());
    let sentence = "andaanerent";
    println!("{:?}", tree.extract_tokens(sentence));
    let sentence = "anfnxdaanerent";
    println!("{:?}", tree.extract_tokens(sentence));
}
