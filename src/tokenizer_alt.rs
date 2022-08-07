use std::fmt;


#[derive(Debug, Clone)]
pub enum TokenError {
    InvalidTermination
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
                    let mut s = format!("{}", output);
                    for (_,n) in v {
                        s.push_str(&format!("({})",n))
                    };
                    write!(f, "{}", s)
                } else {
                    write!(f, "")
                }
            },
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
                output: Some(output)
            }
        )
    }

    pub fn branch(c: char, output: Option<&'static str>, transitions: Vec<(char, Node)>) -> (char, Node) {
        (
            c,
            Node {
                transitions: Some(transitions),
                output
            }
        )
    }

    pub fn get<'a>(&self, chars: &'a [char]) -> (Option<&'static str>, usize) {
        let mut i = 0;
        let mut curr_node = self;
        for char in chars.iter() {
            if let Some(trans_node) = curr_node.find_transition_node(char.to_ascii_lowercase()) {
                curr_node = trans_node;
            } else {
                break;
            }
            i += 1;
        }

        if let Some(_output) = curr_node.output {
            (curr_node.output, i)
        } else {
            (None, 0)
        }
    }

    // pub fn get<'a>(&self, chars: &'a [char]) -> (&'static str, usize) {
    //     let mut i = 0;
    //     let mut curr_node = self;
    //     for char in chars.iter() {
    //         if let Some(trans_node) = curr_node.find_transition_node(*char) {
    //             curr_node = trans_node;
    //         } else {
    //             break;
    //         }
    //         i += 1;
    //     }
    //     (curr_node.output, i)
    // }
 
    pub fn find_transition_node(&self, char: char) -> Option<&Node> {
        if let Some(t) = &self.transitions {
            t.binary_search_by_key(&char, |t| t.0).ok().map(|index| &t[index].1)
        } else {
            None
        }
    }
 
    pub fn sort(&mut self) {
        if let Some(transitions) = &mut self.transitions {
            transitions.sort_by_key(|el| el.0);
            for el in transitions {
                el.1.sort();
            }
        }
    }
 
    pub fn count(&self) -> usize {
        match &self.transitions {
            Some(v) => {
                let mut sum = 1;
                for (_,n) in v {
                    sum += n.count()
                }
                sum
            }
            None => 1
        }
    }

    pub fn extract_tokens(&self, text: &str) -> Result<Vec<String>,TokenError> {
 
        let chars = text.chars().collect::<Vec<_>>();
        let mut ouput = Vec::new();
        let len = chars.len();
        // Position in the string that is being evaluated
        let mut curr_pos = 0;
     
        while curr_pos != len {
            let result = self.get(&chars[curr_pos..]);
            //nothing found, pass through
            if result.1 == 0 {
                ouput.push(chars[curr_pos].to_string());
                curr_pos += 1;
            } else {
                if let Some(text) = result.0 {
                    ouput.push(text.to_string());
                    curr_pos += result.1;
                } else {
                    return Err(TokenError::InvalidTermination)
                }
            }
        }
        Ok(ouput)
    }
}
 



#[test]
fn test_tokenizer() {
    
    let transitions = Some(vec![
        Node::branch(
            'a', Some("a"),
            vec![
                Node::branch('n', Some("an"), 
                    vec![
                        Node::leaf('d',"and"),
                ]),
                Node::leaf('r',"ar"),
                Node::leaf('t',"at"),
            ]
        ),
        Node::leaf('b',"b"),
        Node::branch('c', Some("c"),
            vec![
                Node::leaf('c',"ch"),
            ]
        ),
        Node::leaf('d',"d"),
        Node::branch('e', Some("e"),
            vec![
                Node::leaf('r',"er"),
                Node::leaf('s',"es"),
                Node::branch('n', Some("en"), 
                    vec![
                        Node::leaf('t',"ent"),
                    ]
                )
            ]
        ),
    ]);

    let mut tree = Node { transitions, output: Some("") };
    tree.sort();
    println!("{}",&tree);
    println!("{}",&tree.count());
    let sentence = "andaanerent";
    println!("{:?}",tree.extract_tokens(sentence));
}