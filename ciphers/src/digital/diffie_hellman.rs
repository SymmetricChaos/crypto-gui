use utils::math_functions::modular_pow;

use crate::Cipher;

pub struct DiffieHellman {
    a: u32,
    b: u32,
    g: u32,
    m: u32,
}

impl Default for DiffieHellman {
    fn default() -> Self {
        Self {
            a: 4,
            b: 3,
            g: 5,
            m: 23,
        }
    }
}

impl DiffieHellman {
    pub fn public_keys(&self) -> (u32, u32) {
        let pa = modular_pow(self.a, self.g, self.m);
        let pb = modular_pow(self.b, self.g, self.m);
        (pa, pb)
    }

    pub fn private_key(&self) -> u32 {
        let pa = modular_pow(self.a, self.g, self.m);
        modular_pow(self.b, pa, self.m)
    }
}

impl Cipher for DiffieHellman {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
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
        assert_eq!((4, 10), cipher.public_keys());
        assert_eq!(18, cipher.private_key());
    }
}
