use super::blowfish_arrays::{PARRAY, SBOX0, SBOX1, SBOX2, SBOX3};
use crate::{Cipher, CipherError};

pub fn u64_to_u32_pairs(n: u64) -> [u32; 2] {
    [(n >> 32) as u32, n as u32]
}

pub fn slice_to_u32_pair(s: &[u8]) -> [u32; 2] {
    let mut a = 0;
    let mut b = 0;
    for i in 0..4 {
        a <<= 8;
        a |= s[i] as u32;
        b <<= 8;
        b |= s[i + 4] as u32;
    }
    [a, b]
}

pub struct Blowfish {
    pub key: Vec<u8>,
    parray: [u32; 18],
    sboxes: [[u32; 256]; 4],
}

impl Default for Blowfish {
    fn default() -> Self {
        Self {
            key: vec![0; 4],
            parray: PARRAY,
            sboxes: [SBOX0, SBOX1, SBOX2, SBOX3],
        }
    }
}

impl Blowfish {
    // Derive the P-array and S-boxes from the key
    pub fn key_schedule(&mut self) {
        // Reset the P-array and sboxes to their IVs
        self.parray = PARRAY;
        self.sboxes = [SBOX0, SBOX1, SBOX2, SBOX3];

        // Endlessly repeat the key as needed
        let mut key_bytes = self.key.iter().cycle();

        // Xoring the key into the IV
        for word in self.parray.iter_mut() {
            let mut k = 0u32;
            for _ in 0..4 {
                k <<= 8;
                k |= (*key_bytes.next().unwrap()) as u32;
            }
            *word ^= k;
        }

        // Entries in the P-array and sboxes are replaced by encrypting a chain of values
        // This makes key generation relatively expensive.
        let mut lr = [0, 0];
        for i in 0..9 {
            self.encrypt_block(&mut lr);
            self.parray[i * 2] = lr[0];
            self.parray[i * 2 + 1] = lr[1];
        }
        for sbox in 0..4 {
            for i in 0..128 {
                self.encrypt_block(&mut lr);
                self.sboxes[sbox][i * 2] = lr[0];
                self.sboxes[sbox][i * 2 + 1] = lr[1];
            }
        }
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
        let a = self.sboxes[0][(x >> 24) as usize];
        let b = self.sboxes[1][((x >> 16) & 0xff) as usize];
        let c = self.sboxes[2][((x >> 8) & 0xff) as usize];
        let d = self.sboxes[3][(x & 0xff) as usize];
        (a.wrapping_add(b) ^ c).wrapping_add(d)
    }

    pub fn encrypt_block(&self, lr: &mut [u32; 2]) {
        for i in 0..16 {
            lr[0] ^= self.parray[i];
            lr[1] ^= self.f(lr[0]);
            lr.swap(0, 1);
        }
        lr.swap(0, 1);
        lr[1] ^= self.parray[16];
        lr[0] ^= self.parray[17];
    }

    pub fn decrypt_block(&self, lr: &mut [u32; 2]) {
        for i in (2..18).rev() {
            lr[0] ^= self.parray[i];
            lr[1] ^= self.f(lr[0]);
            lr.swap(0, 1);
        }
        lr.swap(0, 1);
        lr[1] ^= self.parray[1];
        lr[0] ^= self.parray[0];
    }

    pub fn encrypt_ecb(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    pub fn decrypt_ecb(&self, bytes: &[u8]) -> Vec<u8> {
        todo!()
    }

    pub fn encrypt_ctr(&self, bytes: &[u8]) -> Vec<u8> {
        let mut ctr = 0u64;
        let chunks = bytes.chunks(8);
        let mut out = Vec::with_capacity(bytes.len());

        for chunk in chunks {
            let mut c = u64_to_u32_pairs(ctr);
            self.encrypt_block(&mut c);
            for (byte, input) in c
                .iter()
                .map(|w| w.to_le_bytes())
                .flatten()
                .zip(chunk.iter())
            {
                out.push(byte ^ input)
            }
            ctr = ctr.wrapping_add(1);
        }

        out
    }

    // CTR mode is reciprocal
    pub fn decrypt_ctr(&self, bytes: &[u8]) -> Vec<u8> {
        self.encrypt_ctr(bytes)
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


    #[test]
    fn encrypt_decrypt_block() {
        let mut cipher = Blowfish::default();
        cipher.key = 0_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let mut lr = [0, 0];
        cipher.encrypt_block(&mut lr);
        assert_ne!(lr, [0, 0]);
        cipher.decrypt_block(&mut lr);
        assert_eq!(lr, [0, 0]);
    }

    #[test]
    fn test_vector() {
        let mut cipher = Blowfish::default();
        cipher.key = 0_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let mut lr = [0, 0];
        cipher.encrypt_block(&mut lr);
        let s = format!("{:08X} {:08X}", lr[0], lr[1]);
        assert_eq!(s, "4EF99745 6198DD78");

        // cipher.key = 0xffffffffffffffff_u64.to_be_bytes().to_vec();
        // cipher.key_schedule();
        // let mut l = 0xffffffff;
        // let mut r = 0xffffffff;
        // cipher.encrypt_block(&mut l, &mut r);
        // let s = format!("{:08X} {:08X}", l, r);
        // assert_eq!(s, "51866FD5 B85ECB8A");
    }
}
