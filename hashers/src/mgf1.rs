use crate::{
    sha::{sha2::Variant, Sha2_224, Sha2_256, Sha2_384, Sha2_512, Sha2_512_224, Sha2_512_256},
    traits::StatefulHasher,
};

#[derive(Debug, Clone)]
pub struct Mgf1 {
    buffer: Vec<u8>,
    hash_len: u32,
    variant: Variant,
}

impl Default for Mgf1 {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            hash_len: 32,
            variant: Variant::Sha256,
        }
    }
}

impl Mgf1 {
    pub fn init(hash_len: u32, variant: Variant) -> Self {
        Self {
            buffer: Vec::new(),
            hash_len,
            variant,
        }
    }

    fn inner_hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.variant {
            Variant::Sha224 => Sha2_224::init().hash(&bytes),
            Variant::Sha256 => Sha2_256::init().hash(&bytes),
            Variant::Sha384 => Sha2_384::init().hash(&bytes),
            Variant::Sha512 => Sha2_512::init().hash(&bytes),
            Variant::Sha512_224 => Sha2_512_224::init().hash(&bytes),
            Variant::Sha512_256 => Sha2_512_256::init().hash(&bytes),
        }
    }
}

impl StatefulHasher for Mgf1 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.hash_len as usize);
        let mut ctr = 0_u32;

        while out.len() < self.hash_len as usize {
            let mut seed = self.buffer.clone();
            seed.extend_from_slice(&ctr.to_be_bytes());
            out.extend_from_slice(&self.inner_hash(&seed));
            ctr += 1;
        }

        out[0..self.hash_len as usize].to_vec()
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1,  Mgf1::init(50, Variant::Sha256), b"bar", "382576a7841021cc28fc4c0948753fb8312090cea942ea4c4e735d10dc724b155f9f6069f289d61daca0cb814502ef04eae1";
);
