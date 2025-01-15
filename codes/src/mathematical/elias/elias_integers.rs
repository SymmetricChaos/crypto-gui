use crate::errors::CodeError;
use num::Zero;
use std::collections::BTreeMap;
use utils::bits::{bits_from_str, bits_to_u32, Bit};

pub struct EliasCodeIntegers {
    pub variant: EliasVariant,
    current_max: u32,
    delta: DeltaGen,
    pub delta_cache: BTreeMap<u32, String>,
    gamma: GammaGen,
    pub gamma_cache: BTreeMap<u32, String>,
    omega: OmegaGen,
    pub omega_cache: BTreeMap<u32, String>,
}

impl EliasCodeIntegers {
    pub fn extend_all(&mut self, value: u32) {
        while value > self.current_max {
            let (n, c) = self.delta.next().unwrap();
            self.delta_cache.insert(n, c);

            let (n, c) = self.gamma.next().unwrap();
            self.gamma_cache.insert(n, c);

            let (n, c) = self.omega.next().unwrap();
            self.omega_cache.insert(n, c);

            self.current_max += 1;
        }
    }

    pub fn encode_u32(&self, n: u32) -> Option<&String> {
        match self.variant {
            EliasVariant::Delta => self.delta_cache.get(&n),
            EliasVariant::Gamma => self.gamma_cache.get(&n),
            EliasVariant::Omega => self.omega_cache.get(&n),
        }
    }

    // Operates on a single codegroup
    pub fn decode_u32(&self, text: &str) -> Result<Vec<u32>, CodeError> {
        let mut filtered = bits_from_str(text).map_err(|e| CodeError::input(&e.to_string()))?;
        match self.variant {
            EliasVariant::Delta => self.decode_to_u32_delta(&mut filtered),
            EliasVariant::Gamma => self.decode_to_u32_gamma(&mut filtered),
            EliasVariant::Omega => self.decode_to_u32_omega(&mut filtered),
        }
    }
}

impl Default for EliasCodeIntegers {
    fn default() -> Self {
        Self {
            variant: EliasVariant::Delta,
            current_max: 0,
            delta: DeltaGen::new(),
            delta_cache: Default::default(),
            gamma: GammaGen::new(),
            gamma_cache: Default::default(),
            omega: OmegaGen::new(),
            omega_cache: Default::default(),
        }
    }
}
