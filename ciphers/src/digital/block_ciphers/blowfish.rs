use utils::byte_formatting::{
    u32_pair_to_u8_array, u64_to_u32_pair, u8_slice_to_u32_pair, ByteFormat,
};

use super::{
    bit_padding,
    blowfish_arrays::{PARRAY, SBOXES},
    ecb_decrypt, ecb_encrypt, none_padding, strip_bit_padding, BlockCipher, BlockCipherMode,
    BlockCipherPadding,
};
use crate::{Cipher, CipherError};

pub struct Blowfish {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub key: Vec<u8>,
    parray: [u32; 18],
    sboxes: [[u32; 256]; 4],
    pub ctr: u64,
    pub iv: u64,
    pub mode: BlockCipherMode,
    pub padding: BlockCipherPadding,
}

impl Default for Blowfish {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            key: vec![0; 4],
            parray: PARRAY,
            sboxes: SBOXES,
            ctr: 0,
            iv: 0,
            mode: BlockCipherMode::default(),
            padding: BlockCipherPadding::default(),
        }
    }
}

impl Blowfish {
    pub fn parray_string(&self) -> String {
        format!("{:08x?}", self.parray)
    }

    pub fn parray(&self) -> &[u32; 18] {
        &self.parray
    }

    pub fn sboxes_string(&self) -> String {
        format!(
            "{:08x?}\n{:08x?}\n{:08x?}\n{:08x?}\n",
            self.sboxes[0], self.sboxes[1], self.sboxes[2], self.sboxes[3]
        )
    }

    pub fn sboxes(&self) -> &[[u32; 256]; 4] {
        &self.sboxes
    }

    // Derive the P-array and S-boxes from the key
    pub fn key_schedule(&mut self) {
        // Reset the P-array and sboxes to their IVs
        self.parray = PARRAY;
        self.sboxes = SBOXES;

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
            self.encrypt_u32_pair(&mut lr);
            self.parray[i * 2] = lr[0];
            self.parray[i * 2 + 1] = lr[1];
        }
        for sbox in 0..4 {
            for i in 0..128 {
                self.encrypt_u32_pair(&mut lr);
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

    pub fn encrypt_u32_pair(&self, lr: &mut [u32; 2]) {
        for i in 0..16 {
            lr[0] ^= self.parray[i];
            lr[1] ^= self.f(lr[0]);
            lr.swap(0, 1);
        }
        lr.swap(0, 1);
        lr[1] ^= self.parray[16];
        lr[0] ^= self.parray[17];
    }

    pub fn decrypt_u32_pair(&self, lr: &mut [u32; 2]) {
        for i in (2..18).rev() {
            lr[0] ^= self.parray[i];
            lr[1] ^= self.f(lr[0]);
            lr.swap(0, 1);
        }
        lr.swap(0, 1);
        lr[1] ^= self.parray[1];
        lr[0] ^= self.parray[0];
    }

    pub fn encrypt_ctr(&self, bytes: &mut [u8]) {
        let mut ctr = self.ctr;

        for plaintext in bytes.chunks_mut(8) {
            // Encrypt the counter to create a mask
            let mut mask = u64_to_u32_pair(ctr);
            self.encrypt_u32_pair(&mut mask);
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
            self.encrypt_u32_pair(&mut ciphertext);

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
            self.decrypt_u32_pair(&mut temp);
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

impl BlockCipher for Blowfish {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut lr = u8_slice_to_u32_pair(&bytes);
        for i in 0..16 {
            lr[0] ^= self.parray[i];
            lr[1] ^= self.f(lr[0]);
            lr.swap(0, 1);
        }
        lr.swap(0, 1);
        lr[1] ^= self.parray[16];
        lr[0] ^= self.parray[17];
        for (plaintext, ciphertext) in bytes.iter_mut().zip(u32_pair_to_u8_array(lr).iter()) {
            *plaintext = *ciphertext
        }
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut lr = u8_slice_to_u32_pair(&bytes);
        for i in (2..18).rev() {
            lr[0] ^= self.parray[i];
            lr[1] ^= self.f(lr[0]);
            lr.swap(0, 1);
        }
        lr.swap(0, 1);
        lr[1] ^= self.parray[1];
        lr[0] ^= self.parray[0];
        for (plaintext, ciphertext) in bytes.iter_mut().zip(u32_pair_to_u8_array(lr).iter()) {
            *plaintext = *ciphertext
        }
    }
}

impl Cipher for Blowfish {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| CipherError::input("byte format error"))?;

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut bytes, 8)?,
            BlockCipherPadding::Bit => bit_padding(&mut bytes, 8),
        };

        match self.mode {
            BlockCipherMode::Ecb => ecb_encrypt(self, &mut bytes, 8),
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

        if self.padding == BlockCipherPadding::None {
            none_padding(&mut bytes, 8)?
        };

        match self.mode {
            BlockCipherMode::Ecb => ecb_decrypt(self, &mut bytes, 8),
            BlockCipherMode::Ctr => self.decrypt_ctr(&mut bytes),
            BlockCipherMode::Cbc => self.decrypt_cbc(&mut bytes),
        };

        match self.padding {
            BlockCipherPadding::None => none_padding(&mut bytes, 8)?,
            BlockCipherPadding::Bit => strip_bit_padding(&mut bytes)?,
        };

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
        cipher.encrypt_u32_pair(&mut lr);
        assert_ne!(lr, [0, 0]);
        cipher.decrypt_u32_pair(&mut lr);
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
        cipher.encrypt_u32_pair(&mut lr);
        let s = format!("{:08X} {:08X}", lr[0], lr[1]);
        assert_eq!(s, "4EF99745 6198DD78");

        cipher.key = 0_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let mut lr = [0_u8; 8];
        cipher.encrypt_block(&mut lr);
        let s = format!(
            "{:02X}{:02X}{:02X}{:02X} {:02X}{:02X}{:02X}{:02X}",
            lr[0], lr[1], lr[2], lr[3], lr[4], lr[5], lr[6], lr[7]
        );
        assert_eq!(s, "4EF99745 6198DD78");

        cipher.key = 0xffffffffffffffff_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let mut lr = [0xffffffff, 0xffffffff];
        cipher.encrypt_u32_pair(&mut lr);
        let s = format!("{:08X} {:08X}", lr[0], lr[1]);
        assert_eq!(s, "51866FD5 B85ECB8A");

        cipher.key = 0xffffffffffffffff_u64.to_be_bytes().to_vec();
        cipher.key_schedule();
        let mut lr = [0xff_u8; 8];
        cipher.encrypt_block(&mut lr);
        let s = format!(
            "{:02X}{:02X}{:02X}{:02X} {:02X}{:02X}{:02X}{:02X}",
            lr[0], lr[1], lr[2], lr[3], lr[4], lr[5], lr[6], lr[7]
        );
        assert_eq!(s, "51866FD5 B85ECB8A");
    }
}
