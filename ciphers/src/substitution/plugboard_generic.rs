use crate::errors::CipherError;
use itertools::zip;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Plugboard<A> {
    wiring: HashMap<A, A>,
}

impl<A: Hash + Eq + Clone> Default for Plugboard<A> {
    fn default() -> Self {
        Self {
            wiring: HashMap::new(),
        }
    }
}

impl<A: Hash + Eq + Clone> Plugboard<A> {
    pub fn build(left: &[A], right: &[A]) -> Result<Plugboard<A>, CipherError> {
        if left.len() != right.len() {
            return Err(CipherError::general(
                "the lists of left and right entries must be the same length",
            ));
        }
        let mut wiring = HashMap::with_capacity(left.len());
        for (l, r) in zip(left, right) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                return Err(CipherError::general(
                    "plugboard inputs cannot form chains or cycles",
                ));
            }
            wiring.insert(l.clone(), r.clone());
            wiring.insert(r.clone(), l.clone());
        }
        Ok(Plugboard { wiring })
    }

    pub fn rebuild(&mut self, left: &[A], right: &[A]) -> Result<(), CipherError> {
        if left.len() != right.len() {
            return Err(CipherError::general(
                "the lists of left and right entries must be the same length",
            ));
        }
        self.wiring.clear();

        for (l, r) in zip(left, right) {
            if l == r || self.wiring.contains_key(l) || self.wiring.contains_key(r) {
                return Err(CipherError::general(
                    "plugboard inputs cannot form chains or cycles",
                ));
            }
            self.wiring.insert(l.clone(), r.clone());
            self.wiring.insert(r.clone(), l.clone());
        }
        Ok(())
    }

    pub fn swap<'a>(&'a self, input: &A) -> Option<&A> {
        self.wiring.get(input)
    }
}
