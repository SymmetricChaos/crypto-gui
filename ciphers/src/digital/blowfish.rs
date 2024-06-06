use std::mem;

use super::blowfish_arrays::{PARRAY, SBOX0, SBOX1, SBOX2, SBOX3};
use crate::{Cipher, CipherError};

pub struct Blowfish {
    key: Vec<u8>,
}

impl Default for Blowfish {
    fn default() -> Self {
        Self { key: vec![0; 4] }
    }
}

impl Blowfish {
    // Derive the P-array and S-boxes from the key
    pub fn p_array_schedule(&self) -> [u32; 18] {
        // Endlessly repeat the key as needed
        let mut key_bytes = self.key.iter().cycle();
        let mut p = PARRAY.clone();
        for word in p.iter_mut() {
            let mut k = 0u32;
            for _ in 0..4 {
                k <<= 8;
                k |= (*key_bytes.next().unwrap()) as u32;
            }
            *word ^= k;
        }
        p
    }

    pub fn valid_key(&self) -> Result<(), CipherError> {
        if self.key.len() < 4 {
            return Err(CipherError::key("Blowfish key must be at least 4 bytes"));
        }
        if self.key.len() > 72 {
            return Err(CipherError::key("Blowfish key must be less than 72 bytes"));
        }
        Ok(())
    }

    pub fn f(&self, x: u32) -> u32 {
        let h = SBOX0[(x >> 24) as usize].wrapping_add(SBOX1[((x >> 24) & 0xff) as usize]);
        (h ^ SBOX2[((x >> 8) & 0xff) as usize]).wrapping_add(SBOX3[(x & 0xff) as usize])
    }

    pub fn encrypt_block(&self, l: &mut u32, r: &mut u32, parray: &[u32; 18]) {
        for i in 0..16 {
            *l ^= parray[i];
            *r = self.f(*l) ^ *r;
            mem::swap(l, r);
        }
        mem::swap(l, r);
        *r ^= parray[16];
        *l ^= parray[17];
    }

    pub fn decrypt_block(&self, l: &mut u32, r: &mut u32, parray: &[u32; 18]) {
        for i in (2..18).rev() {
            *l ^= parray[i];
            *r = self.f(*l) ^ *r;
            mem::swap(l, r);
        }
        mem::swap(l, r);
        *r ^= parray[1];
        *l ^= parray[0];
    }
}

impl Cipher for Blowfish {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        self.valid_key()?;
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.valid_key()?;
        todo!()
    }
}

#[cfg(test)]
mod blowfish_tests {

    use super::*;
}
