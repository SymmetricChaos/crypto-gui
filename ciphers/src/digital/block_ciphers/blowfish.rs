use utils::byte_formatting::ByteFormat;

use super::{
    bit_padding,
    blowfish_arrays::{PARRAY, SBOX0, SBOX1, SBOX2, SBOX3},
    strip_bit_padding, BlockCipher, BlockCipherMode,
};
use crate::{Cipher, CipherError};

pub fn u64_to_u32_pairs(n: u64) -> [u32; 2] {
    [(n >> 32) as u32, n as u32]
}

pub fn u8_slice_to_u32_pair(s: &[u8]) -> [u32; 2] {
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

pub fn u32_pair_to_u8_array(s: [u32; 2]) -> [u8; 8] {
    let a = s[0].to_be_bytes();
    let b = s[1].to_be_bytes();
    let mut out = [0; 8];
    for i in 0..4 {
        out[i] = a[i];
        out[i + 4] = b[i];
    }

    out
}

pub struct Blowfish {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: Vec<u8>,
    parray: [u32; 18],
    sboxes: [[u32; 256]; 4],
    pub ctr: u64,
    pub iv: u64,
    pub mode: BlockCipherMode,
}

impl Default for Blowfish {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: vec![0; 4],
            parray: PARRAY,
            sboxes: [SBOX0, SBOX1, SBOX2, SBOX3],
            ctr: 0,
            iv: 0,
            mode: BlockCipherMode::Ecb,
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

    pub fn encrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % 8 == 0);

        for plaintext in bytes.chunks_mut(8) {
            // Encrypt the plaintext chunk
            let mut c = u8_slice_to_u32_pair(plaintext);
            self.encrypt_block(&mut c);
            // Overwrite the plaintext bytes with the ciphertext
            for (ctext, source) in c
                .iter()
                .map(|w| w.to_be_bytes())
                .flatten()
                .zip(plaintext.iter_mut())
            {
                *source = ctext
            }
        }
    }

    pub fn decrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % 8 == 0);

        for ciphertext in bytes.chunks_mut(8) {
            // Decrypt the ciphertext chunk
            let mut c = u8_slice_to_u32_pair(ciphertext);
            self.decrypt_block(&mut c);
            // Overwrite the ciphertext bytes with the plaintext
            for (ptext, source) in c
                .iter()
                .map(|w| w.to_be_bytes())
                .flatten()
                .zip(ciphertext.iter_mut())
            {
                *source = ptext
            }
        }
    }

    pub fn encrypt_ctr(&self, bytes: &mut [u8]) {
        let mut ctr = self.ctr;

        for plaintext in bytes.chunks_mut(8) {
            // Encrypt the counter to create a mask
            let mut mask = u64_to_u32_pairs(ctr);
            self.encrypt_block(&mut mask);
            // XOR the mask into the plaintext at the source, creating ciphertext
            for (key_byte, ptext) in mask
                .iter()
                .map(|w| w.to_be_bytes())
                .flatten()
                .zip(plaintext.iter_mut())
            {
                *ptext ^= key_byte
            }
            ctr = ctr.wrapping_add(1);
        }
    }

    // CTR mode is reciprocal
    pub fn decrypt_ctr(&self, bytes: &mut [u8]) {
        self.encrypt_ctr(bytes)
    }

    pub fn encrypt_cbc(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % 8 == 0);
        println!("bytes {:?}", bytes);
        // Start chain with an IV
        let mut chain = self.iv.to_le_bytes();

        for source in bytes.chunks_mut(8) {
            println!("\n\nencrypt");
            println!("chain {:?}", chain);
            // XOR the plaintext into the chain, creating a mixed array
            for (c, b) in chain.iter_mut().zip(source.iter()) {
                *c ^= b;
            }

            println!("mixed {:?}", chain);

            // Encrypt the mixed value, producing ciphertext
            let mut ciphertext: [u32; 2] = u8_slice_to_u32_pair(&chain);
            self.encrypt_block(&mut ciphertext);

            // Store the ciphertext as the next chain value
            chain = u32_pair_to_u8_array(ciphertext);

            println!("ctext {:?}", chain);

            // Overwrite plaintext at source with the ciphertext
            for (ctext, source) in ciphertext
                .iter()
                .map(|w| w.to_be_bytes())
                .flatten()
                .zip(source.iter_mut())
            {
                *source = ctext
            }
        }
    }

    pub fn decrypt_cbc(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % 8 == 0);

        // Start chain with an IV
        let mut chain = self.iv.to_le_bytes();

        for source in bytes.chunks_mut(8) {
            println!("\n\ndecrypt");
            println!("chain {:?}", chain);

            // Decrypt the ciphertext at the source to get the plaintext XORed with the previous chain value
            let mut temp = u8_slice_to_u32_pair(&source);
            self.decrypt_block(&mut temp);
            let mut mixed = u32_pair_to_u8_array(temp);
            println!("mixed {:?}", mixed);

            println!("ciphertext {:?}", source);

            // XOR the current chain value into the mixed text
            for (c, b) in mixed.iter_mut().zip(chain.iter()) {
                *c ^= b;
            }

            // The overwrite ciphertext at source with the plaintext
            for (ptext, source) in mixed.into_iter().zip(source.iter_mut()) {
                *source = ptext
            }
            // Store the ciphertext as the next chain value
            chain = source.try_into().unwrap();
        }
    }
}

// impl BlockCipher for Blowfish {
//     fn encrypt_block(&self, lr: &mut [u32; 2]) {
//         for i in 0..16 {
//             lr[0] ^= self.parray[i];
//             lr[1] ^= self.f(lr[0]);
//             lr.swap(0, 1);
//         }
//         lr.swap(0, 1);
//         lr[1] ^= self.parray[16];
//         lr[0] ^= self.parray[17];
//     }

//     fn decrypt_block(&self, lr: &mut [u32; 2]) {
//         for i in (2..18).rev() {
//             lr[0] ^= self.parray[i];
//             lr[1] ^= self.f(lr[0]);
//             lr.swap(0, 1);
//         }
//         lr.swap(0, 1);
//         lr[1] ^= self.parray[1];
//         lr[0] ^= self.parray[0];
//     }
// }

impl Cipher for Blowfish {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        // ECB and CBC need padding
        if [BlockCipherMode::Ecb, BlockCipherMode::Cbc].contains(&self.mode) {
            bit_padding(&mut bytes, 8)
        }

        match self.mode {
            BlockCipherMode::Ecb => self.encrypt_ecb(&mut bytes),
            BlockCipherMode::Ctr => self.encrypt_ctr(&mut bytes),
            BlockCipherMode::Cbc => self.encrypt_cbc(&mut bytes),
        };
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;
        match self.mode {
            BlockCipherMode::Ecb => self.decrypt_ecb(&mut bytes),
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes),
            BlockCipherMode::Cbc => self.decrypt_cbc(&mut bytes),
        };

        // ECB and CBC need padding removed
        if [BlockCipherMode::Ecb, BlockCipherMode::Cbc].contains(&self.mode) {
            strip_bit_padding(&mut bytes)
        }
        Ok(self.output_format.byte_slice_to_text(&bytes))
    }
}

#[cfg(test)]
mod blowfish_tests {

    use super::*;

    #[test]
    fn encrypt_decrypt_block() {
        let mut cipher = Blowfish::default();
        cipher.key = 123_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let mut lr = [0, 0];
        cipher.encrypt_block(&mut lr);
        assert_ne!(lr, [0, 0]);
        cipher.decrypt_block(&mut lr);
        assert_eq!(lr, [0, 0]);
    }

    #[test]
    fn encrypt_decrypt_ctr() {
        let mut cipher = Blowfish::default();
        cipher.mode = BlockCipherMode::Ctr;
        cipher.ctr = 0xAB12CD34;
        cipher.key = 0x9078563412_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let ptext = "abcdef123456abcdef123456abcdef123456abcdef123456";
        let ctext = cipher.encrypt(ptext).unwrap();
        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, dtext);
    }

    #[test]
    fn encrypt_decrypt_ecb() {
        let mut cipher = Blowfish::default();
        cipher.mode = BlockCipherMode::Ecb;
        cipher.key = 0x9078563412_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let ptext = "abcdef123456abcdef123456abcdef123456abcdef123456";
        let ctext = cipher.encrypt(ptext).unwrap();
        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, dtext);
    }

    #[test]
    fn encrypt_decrypt_cbc() {
        let mut cipher = Blowfish::default();
        cipher.mode = BlockCipherMode::Cbc;
        cipher.iv = 0x0123456789;
        cipher.key = 0x9078563412_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let ptext = "abcdef123456abcdef123456abcdef123456abcdef123456";
        let ctext = cipher.encrypt(ptext).unwrap();
        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, dtext);
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
