use lazy_static::lazy_static;

use crate::tokenizer::Node;

lazy_static! {
    pub static ref SYLLABARY: Node = {
        let transitions = Some(vec![]);
        Node {
            transitions,
            output: None,
        }
    };
}
