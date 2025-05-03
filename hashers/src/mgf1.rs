use crate::{sha::sha2::Sha2Variant, traits::StatefulHasher};

#[derive(Debug, Clone)]
pub struct Mgf1 {
    buffer: Vec<u8>,
    hash_len: u32,
    variant: Sha2Variant,
}

impl Default for Mgf1 {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            hash_len: 32,
            variant: Sha2Variant::Sha256,
        }
    }
}

impl Mgf1 {
    pub fn init(hash_len: u32, variant: Sha2Variant) -> Self {
        Self {
            buffer: Vec::new(),
            hash_len,
            variant,
        }
    }
}

impl StatefulHasher for Mgf1 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(mut self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.hash_len as usize);
        let mut ctr = 0_u32;
        self.buffer.extend([0; 4]);
        let l = self.buffer.len() - 4;

        while out.len() < self.hash_len as usize {
            // Overwrite the last few bytes rather than reallocated the vector and appending new ones
            self.buffer[l..].copy_from_slice(&ctr.to_be_bytes());
            out.extend_from_slice(&self.variant.hash(&self.buffer));
            ctr += 1;
        }

        out[0..self.hash_len as usize].to_vec()
    }
}

crate::stateful_hash_tests!(
    test1,  Mgf1::init(50, Sha2Variant::Sha256), b"bar", "382576a7841021cc28fc4c0948753fb8312090cea942ea4c4e735d10dc724b155f9f6069f289d61daca0cb814502ef04eae1";
);
