use crate::{
    errors::CodeError,
    mathematical::{string_to_u32s, swap_01},
    traits::Code,
};
use itertools::Itertools;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::BTreeMap,
};
use utils::bits::bits_from_str;

use super::{
    delta::{delta_to_u32, DeltaGen},
    gamma::{gamma_to_u32, GammaGen},
    omega::{omega_to_u32, OmegaGen},
    EliasVariant,
};

pub struct EliasCode {
    pub variant: EliasVariant,
    pub spaced: bool,
    pub invert: bool,
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
            spaced: false,
            invert: false,
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

        for n in string_to_u32s(text, ",")? {
            self.extend_all(n);
            match self.variant {
                EliasVariant::Delta => out.push(self.delta_cache.borrow().get(&n).unwrap().clone()),
                EliasVariant::Gamma => out.push(self.gamma_cache.borrow().get(&n).unwrap().clone()),
                EliasVariant::Omega => out.push(self.omega_cache.borrow().get(&n).unwrap().clone()),
            }
        }
        let sep = if self.spaced { ", " } else { "" };

        if self.invert {
            Ok(swap_01(out.join(sep)))
        } else {
            Ok(out.join(sep))
        }
    }

    fn decode(&self, text: &str) -> Result<String, CodeError> {
        let t = match self.invert {
            true => swap_01(text.to_string()),
            false => text.to_string(),
        };
        let mut text = bits_from_str(&t).unwrap();
        Ok(match self.variant {
            EliasVariant::Delta => delta_to_u32(&mut text)?,
            EliasVariant::Gamma => gamma_to_u32(&mut text)?,
            EliasVariant::Omega => omega_to_u32(&mut text)?,
        }
        .into_iter()
        .map(|f| f.to_string())
        .join(", "))
    }
}

#[cfg(test)]
mod elias_tests {

    use super::*;

    const PLAINTEXT: &'static str = "1, 2, 3, 4, 5";
    const ENCODEDTEXT_DELTA: &'static str = "1010001010110001101";
    const ENCODEDTEXT_DELTA_INV: &'static str = "0101110101001110010";
    const ENCODEDTEXT_GAMMA: &'static str = "10100110010000101";
    const ENCODEDTEXT_GAMMA_INV: &'static str = "01011001101111010";
    const ENCODEDTEXT_OMEGA: &'static str = "0100110101000101010";
    const ENCODEDTEXT_OMEGA_INV: &'static str = "1011001010111010101";

    #[test]
    fn encode_test() {
        let mut code = EliasCode::default();
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA);
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA);
        code.variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA);
    }

    #[test]
    fn encode_test_inv() {
        let mut code = EliasCode::default();
        code.invert = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA_INV);
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA_INV);
        code.variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA_INV);
    }

    #[test]
    fn decode_test() {
        let mut code = EliasCode::default();
        assert_eq!(code.decode(ENCODEDTEXT_DELTA).unwrap(), PLAINTEXT);
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA).unwrap(), PLAINTEXT);
        code.variant = EliasVariant::Omega;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_test_inv() {
        let mut code = EliasCode::default();
        code.invert = true;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA_INV).unwrap(), PLAINTEXT);
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA_INV).unwrap(), PLAINTEXT);
        code.variant = EliasVariant::Omega;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA_INV).unwrap(), PLAINTEXT);
    }
}
