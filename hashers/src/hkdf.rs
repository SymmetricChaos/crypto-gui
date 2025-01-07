use crate::{
    hmac::{Hmac, HmacVariant},
    traits::{ResettableHasher, StatefulHasher},
};

fn hkdf_extract(variant: HmacVariant, salt: &[u8], ikm: &[u8]) -> Vec<u8> {
    if salt.is_empty() {
        Hmac::init(variant, &vec![0; variant.block_size() as usize]).hash(ikm)
    } else {
        Hmac::init(variant, salt).hash(ikm)
    }
}

fn hkdf_expand(variant: HmacVariant, prk: &[u8], info: &[u8], length: usize) -> Vec<u8> {
    let mut t = Vec::new();
    let mut okm = Vec::new();
    let mut i: u8 = 0;
    let mut hmac = Hmac::init(variant, &prk);
    while okm.len() < length {
        i = i.wrapping_add(1);
        t = hmac.hash_multiple_and_reset(&[&t, info, &[i]]);
        okm.extend_from_slice(&t);
    }
    okm[..length].to_vec()
}

pub struct Hkdf {
    variant: HmacVariant,
    prk: Vec<u8>,
    length: usize,
    info: Vec<u8>,
}

impl Hkdf {
    pub fn init(variant: HmacVariant, length: usize, ikm: &[u8], salt: &[u8]) -> Self {
        Self {
            variant,
            prk: hkdf_extract(variant, salt, ikm),
            length,
            info: Vec::new(),
        }
    }

    // This is the better way to use HKDF as info input (the context string) is used otherwise the hash input which is confusing
    pub fn derive_key(
        variant: HmacVariant,
        length: usize,
        ikm: &[u8],
        salt: &[u8],
        info: &[u8],
    ) -> Vec<u8> {
        hkdf_expand(variant, &hkdf_extract(variant, salt, ikm), info, length)
    }
}

impl StatefulHasher for Hkdf {
    fn update(&mut self, bytes: &[u8]) {
        self.info.extend_from_slice(bytes);
    }

    fn finalize(self) -> Vec<u8> {
        hkdf_expand(self.variant, &self.prk, &self.info, self.length)
    }

    crate::stateful_hash_helpers!();
}

// https://datatracker.ietf.org/doc/html/rfc4231
crate::stateful_hash_tests!(
    test1_sha256, Hkdf::init(HmacVariant::Sha256, 42,
        &[0x0b; 22],
        &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]
    ),
    &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9],
    "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865";

    test2_sha256, Hkdf::init(HmacVariant::Sha256, 42,
        &[0x0b; 22],
        &[],
    ),
    &[],
    "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8";

    test3_sha256, Hkdf::init(HmacVariant::Sha256, 82,
        &utils::byte_formatting::hex_to_bytes("
          000102030405060708090a0b0c0d0e0f
          101112131415161718191a1b1c1d1e1f
          202122232425262728292a2b2c2d2e2f
          303132333435363738393a3b3c3d3e3f
          404142434445464748494a4b4c4d4e4f").unwrap(),
        &utils::byte_formatting::hex_to_bytes("
          606162636465666768696a6b6c6d6e6f
          707172737475767778797a7b7c7d7e7f
          808182838485868788898a8b8c8d8e8f
          909192939495969798999a9b9c9d9e9f
          a0a1a2a3a4a5a6a7a8a9aaabacadaeaf").unwrap(),
    ),
    &utils::byte_formatting::hex_to_bytes("
          b0b1b2b3b4b5b6b7b8b9babbbcbdbebf
          c0c1c2c3c4c5c6c7c8c9cacbcccdcecf
          d0d1d2d3d4d5d6d7d8d9dadbdcdddedf
          e0e1e2e3e4e5e6e7e8e9eaebecedeeef
          f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap(),
    "b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71cc30c58179ec3e87c14c01d5c1f3434f1d87";


    test1_sha1, Hkdf::init(HmacVariant::Sha1, 42,
        &[0x0b; 11],
        &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c],

    ),
    &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9],
    "085a01ea1b10f36933068b56efa5ad81a4f14b822f5b091568a9cdd4f155fda2c22e422478d305f3f896";

    test2_sha1, Hkdf::init(HmacVariant::Sha1, 42,
        &[0x0b; 22],
        &[],

    ),
    &[],
    "0ac1af7002b3d761d1e55298da9d0506b9ae52057220a306e07b6b87e8df21d0ea00033de03984d34918";

    test3_sha1, Hkdf::init(HmacVariant::Sha1, 82,
        &utils::byte_formatting::hex_to_bytes("
          000102030405060708090a0b0c0d0e0f
          101112131415161718191a1b1c1d1e1f
          202122232425262728292a2b2c2d2e2f
          303132333435363738393a3b3c3d3e3f
          404142434445464748494a4b4c4d4e4f").unwrap(),
        &utils::byte_formatting::hex_to_bytes("
          606162636465666768696a6b6c6d6e6f
          707172737475767778797a7b7c7d7e7f
          808182838485868788898a8b8c8d8e8f
          909192939495969798999a9b9c9d9e9f
          a0a1a2a3a4a5a6a7a8a9aaabacadaeaf").unwrap(),

    ),
    &utils::byte_formatting::hex_to_bytes("
          b0b1b2b3b4b5b6b7b8b9babbbcbdbebf
          c0c1c2c3c4c5c6c7c8c9cacbcccdcecf
          d0d1d2d3d4d5d6d7d8d9dadbdcdddedf
          e0e1e2e3e4e5e6e7e8e9eaebecedeeef
          f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap(),
    "0bd770a74d1160f7c9f12cd5912a06ebff6adcae899d92191fe4305673ba2ffe8fa3f1a4e5ad79f3f334b3b202b2173c486ea37ce3d397ed034c7f9dfeb15c5e927336d0441f4c4300e2cff0d0900b52d3b4";

);
