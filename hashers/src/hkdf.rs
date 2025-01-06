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
    info: Vec<u8>,
}

impl Hkdf {
    pub fn init(length: usize, salt: &[u8], ikm: &[u8]) -> Self {
        Self {
            prk: hkdf_extract(salt, ikm),
            length,
            info: Vec::new(),
        }
    }
}

impl StatefulHasher for Hkdf {
    fn update(&mut self, bytes: &[u8]) {
        self.info.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        hkdf_expand(&self.prk, &self.info, self.length)
    }

    crate::stateful_hash_helpers!();
}

// https://datatracker.ietf.org/doc/html/rfc4231
crate::stateful_hash_tests!(
    test1_sha256, Hkdf::init(42,
        &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c],
        &[0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b]
    ),
    &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9],
    "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865";

);
