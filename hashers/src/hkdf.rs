use crate::{
    hmac::{Hmac, HmacVariant},
    traits::{ResettableHasher, StatefulHasher},
};

fn hkdf_extract(salt: &[u8], ikm: &[u8]) -> Vec<u8> {
    if salt.is_empty() {
        Hmac::init(
            HmacVariant::Sha256,
            &vec![0; HmacVariant::Sha256.block_size() as usize],
        )
        .hash(ikm)
    } else {
        Hmac::init(HmacVariant::Sha256, salt).hash(ikm)
    }
}

fn hkdf_expand(prk: &[u8], info: &[u8], length: usize) -> Vec<u8> {
    let mut t = Vec::new();
    let mut okm = Vec::new();
    let mut i: u8 = 0;
    let mut hmac = Hmac::init(HmacVariant::Sha256, &prk);
    while okm.len() < length {
        i = i.wrapping_add(1);
        hmac.update_multiple(&[prk, &t, info, &[i]]);
        t = hmac.finalize_and_reset();
        okm.extend_from_slice(&t);
    }
    okm[..length].to_vec()
}

pub struct Hkdf {
    prk: Vec<u8>,
    length: usize,
    buffer: Vec<u8>,
}

impl Hkdf {
    pub fn init(length: usize, salt: &[u8], ikm: &[u8]) -> Self {
        Self {
            prk: hkdf_extract(salt, ikm),
            length,
            buffer: Vec::new(),
        }
    }
}

impl StatefulHasher for Hkdf {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        hkdf_expand(&self.prk, &self.buffer, self.length)
    }

    crate::stateful_hash_helpers!();
}
