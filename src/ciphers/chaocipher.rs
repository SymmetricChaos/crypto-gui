use std::cell::RefCell;

use crate::errors::CipherError;
use crate::text_types::{VecString, PresetAlphabet};

use super::Cipher;
 
pub struct Chaocipher {
    left: RefCell<VecString>,
    right: RefCell<VecString>,
}

impl Chaocipher {
    fn left_permute(&self, n: usize) {
        self.left.borrow_mut().rotate_left(n);
        let t = self.left.borrow_mut().remove(1).unwrap();
        self.left.borrow_mut().insert(13, t);
    }

    fn right_permute(&self, n: usize) {
        self.right.borrow_mut().rotate_left(n+1);
        let t = self.right.borrow_mut().remove(2).unwrap();
        self.right.borrow_mut().insert(13, t);
    }

    pub fn set_left(&mut self, s: &str) {
        self.left = RefCell::new(VecString::from(s))
    }

    pub fn set_right(&mut self, s: &str) {
        self.right = RefCell::new(VecString::from(s))
    }
}

impl Default for Chaocipher {
    fn default() -> Self {
        Chaocipher { 
            left: RefCell::new(VecString::from(PresetAlphabet::BasicLatin)), 
            right: RefCell::new(VecString::from("AZDNBUHYFWJLVGRCQMPSOEXTKI")),  
        }
    }
}

impl Cipher for Chaocipher {

    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = self.right.borrow().pos(c).unwrap();
            out.push(self.left.borrow()[n]);
            self.left_permute(n);
            self.right_permute(n);
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let symbols = text.chars();
        let mut out = String::new();
        for c in symbols {
            let n = self.left.borrow().pos(c).unwrap();
            out.push(self.right.borrow()[n]);
            self.left_permute(n);
            self.right_permute(n);
        }
        Ok(out)
 
    }

    fn randomize(&mut self, rng: &mut rand::prelude::StdRng) {
        todo!()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}