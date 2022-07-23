use std::collections::HashMap;
use itertools::zip;
use std::hash::Hash;

use crate::errors::CipherError;

pub struct PlugboardGen<A: Hash + Eq + Copy> {
    wiring: HashMap<A,A>,
}

impl<A: Hash + Eq + Copy + Default> PlugboardGen<A> {
    pub fn build(a: &[A], b: &[A]) -> Result<PlugboardGen<A>,CipherError> {
        if a.len() != b.len() {
            return Err(CipherError::General("plugboard inputs and outputs must be the same length".into()))
        }
        let mut wiring = HashMap::with_capacity(a.len());
        for (l,r) in zip(a,b) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                return Err(CipherError::General("plugboard inputs cannot form chains or cycles".into()))
            }
            wiring.insert(*l, *r);
            wiring.insert(*r, *l);
        }
        Ok(PlugboardGen{ wiring })
    }

    pub fn swap(&self, a: A) -> A {
        *self.wiring.get(&a).unwrap_or_else(|| &a)
    }
}