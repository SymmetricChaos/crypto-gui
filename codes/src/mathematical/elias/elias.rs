use super::{
    delta::{recognize_delta, recognize_delta_single, DeltaGen},
    gamma::{recognize_gamma, recognize_gamma_single, GammaGen},
    omega::{recognize_omega, recognize_omega_single, OmegaGen},
    EliasVariant,
};
use crate::{
    mathematical::{string_to_u32s, swap_01},
    traits::Code,
};
use itertools::Itertools;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::BTreeMap,
};
use utils::errors::GeneralError;

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
        let mut out = Vec::new();
        for i in 1..=n {
            let mut code = match self.variant {
                EliasVariant::Delta => self.delta_cache.borrow().get(&n).unwrap().clone(),
                EliasVariant::Gamma => self.gamma_cache.borrow().get(&n).unwrap().clone(),
                EliasVariant::Omega => self.omega_cache.borrow().get(&n).unwrap().clone(),
            };
            if self.invert {
                code = swap_01(code)
            }
            out.push((i, code));
        }
        out
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
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
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

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let t = match self.invert {
            true => swap_01(text.to_string()),
            false => text.to_string(),
        };

        if self.spaced {
            let mut out: Vec<String> = Vec::new();
            for section in t.split(",").map(|s| s.trim()) {
                let c = match self.variant {
                    EliasVariant::Delta => recognize_delta_single(section),
                    EliasVariant::Gamma => recognize_gamma_single(section),
                    EliasVariant::Omega => recognize_omega_single(section),
                };
                if let Some(n) = c {
                    out.push(n.to_string());
                } else {
                    out.push(String::from("�"))
                }
            }
            Ok(out.join(", "))
        } else {
            let ns = match self.variant {
                EliasVariant::Delta => recognize_delta(&t),
                EliasVariant::Gamma => recognize_gamma(&t),
                EliasVariant::Omega => recognize_omega(&t),
            };
            let mut out = Vec::with_capacity(ns.len());
            for n in ns {
                if let Some(v) = n {
                    out.push(v.to_string());
                } else {
                    out.push(String::from("�"));
                }
            }
            Ok(out.join(", "))
        }
    }
}

#[cfg(test)]
mod elias_tests {

    use super::*;

    const PLAINTEXT: &'static str = "1, 2, 3, 4, 5";
    const ENCODEDTEXT_DELTA: &'static str = "1010001010110001101";
    const ENCODEDTEXT_DELTA_SP: &'static str = "1, 0100, 0101, 01100, 01101";
    const ENCODEDTEXT_DELTA_INV: &'static str = "0101110101001110010";
    const ENCODEDTEXT_DELTA_INV_SP: &'static str = "0, 1011, 1010, 10011, 10010";

    const ENCODEDTEXT_GAMMA: &'static str = "10100110010000101";
    const ENCODEDTEXT_GAMMA_SP: &'static str = "1, 010, 011, 00100, 00101";
    const ENCODEDTEXT_GAMMA_INV: &'static str = "01011001101111010";
    const ENCODEDTEXT_GAMMA_INV_SP: &'static str = "0, 101, 100, 11011, 11010";

    const ENCODEDTEXT_OMEGA: &'static str = "0100110101000101010";
    const ENCODEDTEXT_OMEGA_SP: &'static str = "0, 100, 110, 101000, 101010";
    const ENCODEDTEXT_OMEGA_INV: &'static str = "1011001010111010101";
    const ENCODEDTEXT_OMEGA_INV_SP: &'static str = "1, 011, 001, 010111, 010101";

    #[test]
    fn encode_delta() {
        let mut code = EliasCode::default();
        code.variant = EliasVariant::Delta;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA_SP);
        code.invert = true;
        code.spaced = false;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA_INV);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_DELTA_INV_SP);
    }

    #[test]
    fn encode_gamma() {
        let mut code = EliasCode::default();
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA_SP);
        code.invert = true;
        code.spaced = false;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA_INV);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_GAMMA_INV_SP);
    }

    #[test]
    fn encode_omega() {
        let mut code = EliasCode::default();
        code.variant = EliasVariant::Omega;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA_SP);
        code.invert = true;
        code.spaced = false;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA_INV);
        code.spaced = true;
        assert_eq!(code.encode(PLAINTEXT).unwrap(), ENCODEDTEXT_OMEGA_INV_SP);
    }

    #[test]
    fn decode_delta() {
        let mut code = EliasCode::default();
        code.variant = EliasVariant::Delta;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA_SP).unwrap(), PLAINTEXT);
        code.invert = true;
        code.spaced = false;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA_INV).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_DELTA_INV_SP).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_gamma() {
        let mut code = EliasCode::default();
        code.variant = EliasVariant::Gamma;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA_SP).unwrap(), PLAINTEXT);
        code.invert = true;
        code.spaced = false;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA_INV).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_GAMMA_INV_SP).unwrap(), PLAINTEXT);
    }

    #[test]
    fn decode_omega() {
        let mut code = EliasCode::default();
        code.variant = EliasVariant::Omega;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA_SP).unwrap(), PLAINTEXT);
        code.invert = true;
        code.spaced = false;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA_INV).unwrap(), PLAINTEXT);
        code.spaced = true;
        assert_eq!(code.decode(ENCODEDTEXT_OMEGA_INV_SP).unwrap(), PLAINTEXT);
    }
}
