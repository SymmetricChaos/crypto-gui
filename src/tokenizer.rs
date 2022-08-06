use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    pub output: &'static str,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.transitions {
            Some(v) => {
                let mut s = format!("{}", self.output.to_string());
                for (_,n) in v {
                    s.push_str(&format!("({})",n))
                }
                write!(f, "{}", s)
            },
            None => write!(f, "{}", self.output)
        }
    }
}

impl Node {
    pub fn leaf(c: char, output: &'static str) -> (char, Node) {
        (
            c,
            Node {
                transitions: None,
                output
            }
        )
    }

    pub fn branch(c: char, output: &'static str, transitions: Vec<(char, Node)>) -> (char, Node) {
        (
            c,
            Node {
                transitions: Some(transitions),
                output
            }
        )
    }

    pub fn get<'a>(&self, chars: &'a [char]) -> (&'static str, usize) {
        let mut i = 0;
        let mut curr_node = self;
        for char in chars.iter() {
            if let Some(trans_node) = curr_node.find_transition_node(*char) {
                curr_node = trans_node;
            } else {
                break;
            }
            i += 1;
        }
        (curr_node.output, i)
    }
 
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

    pub fn extract_tokens(&self, text: &str) -> Vec<String> {
 
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
                ouput.push(result.0.to_string());
                curr_pos += result.1;
            }
        }
        ouput
    }
}
 



#[test]
fn test_tokenizer() {
    
    let transitions = Some(vec![
    (
        'a',
        Node {
            transitions: Some(vec![
                (
                'n',
                Node {
                    transitions: Some(vec![
                        Node::leaf('d',"and"),
                    ]),
                    output: "an"
                }),
                Node::leaf('r',"ar"),
                Node::leaf('t',"at"),
            ]),
            output: "a"
        }
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
    )
    ]);

    let mut tree = Node { transitions, output: "" };
    tree.sort();
    println!("{}",&tree);
    println!("{}",&tree.count());
    let sentence = "andaanerent";
    println!("{:?}",tree.extract_tokens(sentence));
}