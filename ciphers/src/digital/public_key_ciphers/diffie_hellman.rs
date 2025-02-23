use num::integer::gcd;
use utils::math_functions::mod_pow_32;

use crate::Cipher;

pub struct DiffieHellman {
    pub private_keys: Vec<u32>,
    pub generator: u32,
    pub modulus: u32,
}

impl Default for DiffieHellman {
    fn default() -> Self {
        Self {
            private_keys: vec![4, 3],
            generator: 5,
            modulus: 23,
        }
    }
}

impl DiffieHellman {
    // Check if g is a generator in the multiplicative group
    pub fn g_is_valid(&self) -> bool {
        gcd(self.generator, self.modulus) == 1
    }

    pub fn public_keys(&self) -> Vec<u32> {
        Vec::from_iter(
            self.private_keys
                .iter()
                .map(|p| mod_pow_32(self.generator, *p, self.modulus)),
        )
    }

    pub fn shared_key(&self) -> u32 {
        let mut b = self.generator;
        for k in self.private_keys.iter() {
            b = mod_pow_32(b, *k, self.modulus);
        }
        b
    }
}

impl Cipher for DiffieHellman {
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
mod diffie_hellman_tests {

    use super::*;
    #[test]
    fn test_keys() {
        let cipher = DiffieHellman::default();
        assert_eq!(vec![4, 10], cipher.public_keys());
        assert_eq!(18, cipher.shared_key());
    }
}
