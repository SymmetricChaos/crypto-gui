use crate::{errors::CodeError, mathematical::string_to_u32s, traits::Code};
use itertools::Itertools;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::BTreeMap,
};
use utils::bits::bits_from_str;

use super::{
    delta::{decode_to_u32_delta, DeltaGen},
    gamma::{decode_to_u32_gamma, GammaGen},
    omega::{decode_to_u32_omega, OmegaGen},
    EliasVariant,
};

pub struct EliasCode {
    pub variant: EliasVariant,
    current_max: RefCell<u32>,
    delta: RefCell<DeltaGen>,
    delta_cache: RefCell<BTreeMap<u32, String>>,
    gamma: RefCell<GammaGen>,
    gamma_cache: RefCell<BTreeMap<u32, String>>,
    omega: RefCell<OmegaGen>,
    omega_cache: RefCell<BTreeMap<u32, String>>,
}

impl Default for EliasCode {
    fn default() -> Self {
        Self {
            variant: EliasVariant::Delta,
            current_max: RefCell::new(0),
            delta: RefCell::new(DeltaGen::new()),
            delta_cache: Default::default(),
            gamma: RefCell::new(GammaGen::new()),
            gamma_cache: Default::default(),
            omega: RefCell::new(OmegaGen::new()),
            omega_cache: Default::default(),
        }
    }
}

impl EliasCode {
    pub fn cache(&self) -> Ref<'_, BTreeMap<u32, std::string::String>> {
        match self.variant {
            EliasVariant::Delta => self.delta_cache.borrow(),
            EliasVariant::Gamma => self.gamma_cache.borrow(),
            EliasVariant::Omega => self.omega_cache.borrow(),
        }
    }

    pub fn cache_mut(&self) -> RefMut<'_, BTreeMap<u32, std::string::String>> {
        match self.variant {
            EliasVariant::Delta => self.delta_cache.borrow_mut(),
            EliasVariant::Gamma => self.gamma_cache.borrow_mut(),
            EliasVariant::Omega => self.omega_cache.borrow_mut(),
        }
    }

    pub fn values(&self) -> Vec<String> {
        self.cache().values().cloned().collect_vec()
    }

    pub fn n_pairs(&self, n: u32) -> Vec<(u32, String)> {
        self.extend_all(n);
        self.cache()
            .iter()
            .take(n as usize)
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    pub fn extend_all(&self, value: u32) {
        while value > *self.current_max.borrow() {
            let (n, c) = self.delta.borrow_mut().next().unwrap();
            self.delta_cache.borrow_mut().insert(n, c);

            let (n, c) = self.gamma.borrow_mut().next().unwrap();
            self.gamma_cache.borrow_mut().insert(n, c);

            let (n, c) = self.omega.borrow_mut().next().unwrap();
            self.omega_cache.borrow_mut().insert(n, c);

            *self.current_max.borrow_mut() += 1;
        }
    }
}

impl Code for EliasCode {
    fn encode(&self, text: &str) -> Result<String, CodeError> {
        let mut out = Vec::new();

        for n in string_to_u32s(text, " ")? {
            self.extend_all(n);
            match self.variant {
                EliasVariant::Delta => out.push(self.delta_cache.borrow().get(&n).unwrap().clone()),
                EliasVariant::Gamma => out.push(self.gamma_cache.borrow().get(&n).unwrap().clone()),
                EliasVariant::Omega => out.push(self.omega_cache.borrow().get(&n).unwrap().clone()),
            }
        }

        Ok(out.into_iter().join(""))
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let t = text.replace(" ", "");
        let mut text = bits_from_str(&t).unwrap();
        Ok(match self.variant {
            EliasVariant::Delta => decode_to_u32_delta(&mut text)?,
            EliasVariant::Gamma => decode_to_u32_gamma(&mut text)?,
            EliasVariant::Omega => decode_to_u32_omega(&mut text)?,
        }
        .into_iter()
        .map(|f| f.to_string())
        .join(" "))
    }
}

#[cfg(test)]
mod elias_tests {

    use super::*;

    const PLAINTEXT_INT: &'static str = "1, 2, 3";
    const ENCODEDTEXT_DELTA: &'static str = "101000101";
    const ENCODEDTEXT_GAMMA: &'static str = "1010011";
    const ENCODEDTEXT_OMEGA: &'static str = "0100110";

    #[test]
    fn encode_test_int() {
        let mut code = EliasCode::default();
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_DELTA);
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_GAMMA);
        code.variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT_INT).unwrap(), ENCODEDTEXT_OMEGA);
    }

    #[test]
    fn decode_test_int() {
        let mut code = EliasCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_DELTA).unwrap(), PLAINTEXT_INT);
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA).unwrap(), PLAINTEXT_INT);
        code.variant = EliasVariant::Omega;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA).unwrap(), PLAINTEXT_INT);
    }
}
