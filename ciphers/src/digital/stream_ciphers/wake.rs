use crate::Cipher;

pub struct Wake {
    pub table: [u32; 256],
    pub regs: [u32; 6],
}

impl Default for Wake {
    fn default() -> Self {
        Self {
            table: [0; 256],
            regs: [0; 6],
        }
    }
}

impl Wake {
    fn mix(&self, x: u32, y: u32) -> u32 {
        let s = x.wrapping_add(y);
        (s >> 8) ^ self.table[s as u8 as usize]
    }

    pub fn encrypt_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(bytes.len());

        let mut r3 = 0;
        let mut r4 = 0;
        let mut r5 = 0;
        let mut r6 = 0;

        for block in bytes.chunks(4) {
            let word = u32::from_be_bytes(block.try_into().unwrap());
            let r2 = word ^ r6;
            r3 = self.mix(r3, r2);
            r4 = self.mix(r4, r3);
            r5 = self.mix(r5, r4);
            r6 = self.mix(r6, r5);
            out.extend_from_slice(&r2.to_be_bytes());
        }

        out
    }
}

impl Cipher for Wake {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}
