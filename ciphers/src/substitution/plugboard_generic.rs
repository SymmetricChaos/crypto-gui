use itertools::zip;
use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::CipherError;

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
            return Err(CipherError::General(
                "the lists of left and right entries must be the same length".into(),
            ));
        }
        let mut wiring = HashMap::with_capacity(left.len());
        for (l, r) in zip(left, right) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                return Err(CipherError::General(
                    "plugboard inputs cannot form chains or cycles".into(),
                ));
            }
            wiring.insert(l.clone(), r.clone());
            wiring.insert(r.clone(), l.clone());
        }
        Ok(Plugboard { wiring })
    }

    pub fn swap<'a>(&'a self, input: &A) -> Option<&A> {
        self.wiring.get(input)
    }
}
