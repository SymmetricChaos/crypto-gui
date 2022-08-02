use itertools::zip;
use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::Error;

pub struct PlugboardGen<A> {
    wiring: HashMap<A, A>,
}

impl<A: Hash + Eq + Clone> Default for PlugboardGen<A> {
    fn default() -> Self {
        Self {
            wiring: HashMap::new(),
        }
    }
}

impl<A: Hash + Eq + Clone> PlugboardGen<A> {
    pub fn build(left: &[A], right: &[A]) -> Result<PlugboardGen<A>, Error> {
        if left.len() != right.len() {
            return Err(Error::General(
                "the lists of left and right plugs positions must be the same length".into(),
            ));
        }
        let mut wiring = HashMap::with_capacity(left.len());
        for (l, r) in zip(left, right) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                return Err(Error::General(
                    "plugboard inputs cannot form chains or cycles".into(),
                ));
            }
            wiring.insert(l.clone(), r.clone());
            wiring.insert(r.clone(), l.clone());
        }
        Ok(PlugboardGen { wiring })
    }

    // Infalliable build method
    pub fn build_silent(left: &[A], right: &[A]) -> PlugboardGen<A> {
        let mut wiring = HashMap::with_capacity(left.len());
        for (l, r) in zip(left, right) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                continue;
            }
            wiring.insert(l.clone(), r.clone());
            wiring.insert(r.clone(), l.clone());
        }
        PlugboardGen { wiring }
    }

    pub fn rebuild(&mut self, left: &[A], right: &[A]) -> Result<(), Error> {
        if left.len() != right.len() {
            return Err(Error::General(
                "the lists of left and right plugs positions must be the same length".into(),
            ));
        }
        let mut wiring = HashMap::with_capacity(left.len());
        for (l, r) in zip(left, right) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                return Err(Error::General(
                    "plugboard inputs cannot form chains or cycles".into(),
                ));
            }
            wiring.insert(l.clone(), r.clone());
            wiring.insert(r.clone(), l.clone());
        }
        self.wiring = wiring;
        Ok(())
    }

    pub fn rebuild_silent(&mut self, left: &[A], right: &[A]) {
        let mut wiring = HashMap::with_capacity(left.len());
        for (l, r) in zip(left, right) {
            if l == r || wiring.contains_key(l) || wiring.contains_key(r) {
                continue;
            }
            wiring.insert(l.clone(), r.clone());
            wiring.insert(r.clone(), l.clone());
        }
        self.wiring = wiring;
    }

    // Swap the input and catch if it is not covered
    pub fn swap_complete<'a>(&'a self, input: &'a A) -> Option<&A> {
        self.wiring.get(&input)
    }

    // Swamp the input, if it is not covered just return the input
    pub fn swap<'a>(&'a self, input: &'a A) -> &A {
        self.wiring.get(&input).unwrap_or_else(|| &input)
    }
}

impl<A: ToString> PlugboardGen<A> {
    // Vector of pairs to show state
    pub fn show_settings(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.wiring.len());
        for pair in self.wiring.iter() {
            out.push(format!("{} ⇒ {}", pair.0.to_string(), pair.1.to_string()))
        }
        out.sort();
        out
    }
}
