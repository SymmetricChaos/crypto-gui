use num::integer::gcd;
use utils::math_functions::modular_pow;

use crate::Cipher;

pub struct DiffieHellmanTriple {
    pub private_keys: Vec<u32>,
    pub ephemeral_keys: Vec<u32>,
    pub generator: u32,
    pub modulus: u32,
}

impl Default for DiffieHellmanTriple {
    fn default() -> Self {
        Self {
            private_keys: vec![4, 3],
            ephemeral_keys: vec![7, 10],
            generator: 5,
            modulus: 23,
        }
    }
}

impl DiffieHellmanTriple {
    // Check if g is a generator in the multiplicative group
    pub fn g_is_valid(&self) -> bool {
        gcd(self.generator, self.modulus) == 1
    }

    pub fn public_keys(&self) -> Vec<(u32, u32)> {
        let mut out = Vec::with_capacity(self.private_keys.len());
        for i in 0..self.private_keys.len() {
            out.push((
                modular_pow(self.generator, self.private_keys[i], self.modulus),
                modular_pow(self.generator, self.ephemeral_keys[i], self.modulus),
            ))
        }
        out
    }

    pub fn shared_key(&self) -> u32 {
        let mut b = self.generator;
        for k in self.private_keys.iter() {
            b = modular_pow(b, *k, self.modulus);
        }
        b
    }
}

impl Cipher for DiffieHellmanTriple {
    fn encrypt(&self, _text: &str) -> Result<String, crate::CipherError> {
        Err(crate::CipherError::general(
            "Diffie-Hellman key exchange does not encrypt a message",
        ))
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        self.encrypt(text)
    }
}

#[cfg(test)]
mod diffie_hellman_triple_tests {

    use super::*;
    #[test]
    fn test_keys() {}
}
