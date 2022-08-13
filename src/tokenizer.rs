
use std::{fmt, collections::HashMap};

use itertools::Itertools;

#[derive(Clone)]
pub struct TransitionError(String, Option<char>);

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            Some(c) => {
                if self.0 == "" {
                    write!(f, "invalid symbol `{}`", c)
                } else {
                    write!(f, "no transition `{}` -> `{}`", self.0, c)
                }
            },
            None => write!(f, "`{}` must transition", self.0),
        }
    }
}

impl fmt::Debug for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    pub output: Option<&'static str>,
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

    pub fn get<'a>(&self, chars: &'a [char]) -> Result<(&'static str, usize), TransitionError> {
        let mut i = 0;
        let mut curr_node = self;
        let mut maybe_char = None;
        for ch in chars.iter() {
            // find the transition to the next node or break if there is no
            // transition
            // a lack of transition could be a leaf node or a could mean that
            // the character has no transition from this node
            if let Some(trans_node) = curr_node.find_transition_node(*ch) {
                curr_node = trans_node;
            } else {
                maybe_char = Some(*ch);
                break;
            }
            i += 1;
        }

        // if an output exists then provide it and the index reached
        // otherwise the string being parsed is invalid
        if let Some(output) = curr_node.output {
            Ok((output, i))
        } else {
            Err(TransitionError(chars[0..i].iter().collect(), maybe_char))
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

    pub fn extract_tokens(&self, text: &str) -> Result<Vec<String>, TransitionError> {
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

    pub fn input_paths(&self) -> Vec<(String,&'static str)> {
        let mut paths: Vec<(String, &str)> = Vec::new();
        self.input_paths_inner(vec![], &mut paths);
        paths
    }

    fn input_paths_inner(&self, chars: Vec<char>, paths: &mut Vec<(String,&'static str)>) {
        if let Some(s) = self.output {
            paths.push((chars.iter().collect::<String>(),s))
        }
        if let Some(transitions) = &self.transitions {
            for (c, n) in transitions.iter() {
                let mut new_chars = chars.clone();
                new_chars.push(*c);
                n.input_paths_inner(new_chars,paths)
            }
        }
    }

    pub fn output_paths(&self) -> Vec<(&'static str, Vec<String>)> {
        let mut map = HashMap::new();
        self.output_paths_inner(vec![], &mut map);
        let mut paths = map.iter().map(|(k,v)| (*k,v.clone())).collect_vec();
        paths.sort_by_key(|a| a.0);
        paths
    }

    fn output_paths_inner(&self, chars: Vec<char>, paths: &mut HashMap<&'static str, Vec<String>>) {

        if let Some(s) = self.output {
            let input = chars.iter().collect::<String>();
            match paths.contains_key(s) {
                true => { paths.entry(s).and_modify(|e| e.push(input)); },
                false => { paths.insert(s, vec![input]); }
            };
        }
        if let Some(transitions) = &self.transitions {
            for (c, n) in transitions.iter() {
                let mut new_chars = chars.clone();
                new_chars.push(*c);
                n.output_paths_inner(new_chars,paths)
            }
        }
    }

}

#[test]
fn test() {

    let mut tree = Node::tree(
        vec![
            Node::branch(
                't', Some("letter"),
                vec![
                    Node::branch(
                        'h', None, 
                        vec![
                            Node::branch(
                                'e', Some("word"),
                                vec![
                                    Node::leaf('e', "word")
                                ]
                            )
                        ]
                    )
                ]
            ),
            Node::branch(
                'h', Some("letter"), 
                vec![
                    Node::leaf('e', "word")
                ]
            ),
            Node::leaf('e', "letter")
        ]
    );
    tree.sort();

    println!("\n\nInput Paths:");
    for (k,v) in &tree.input_paths() {
        println!("{k} => {v}")
    }

    println!("\n\nOutput Paths:");
    for (k,v) in &tree.output_paths() {
        println!("{k} <= {v:?}")
    }

    print!("\n\n");
    for sentence in ["t","the","thee","teh","ethehe","art","thj","th",] {
        println!("{}", sentence);
        println!("{:?}\n", tree.extract_tokens(sentence));
    }

}
