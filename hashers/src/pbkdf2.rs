use crate::{
    hmac::{Hmac, HmacVariant},
    traits::{ResettableHasher, StatefulHasher},
};
use itertools::Itertools;

pub struct Pbkdf2 {
    buffer: Vec<u8>,
    variant: HmacVariant,
    salt: Vec<u8>,
    iterations: u32,
    hash_len: u32, // size of the output in bytes
}

impl Pbkdf2 {
    pub fn init(variant: HmacVariant, iterations: u32, hash_len: u32, salt: &[u8]) -> Self {
        assert!(iterations >= 1);
        assert!(hash_len >= 4);
        Self {
            buffer: Vec::new(),
            salt: salt.to_vec(),
            variant,
            iterations,
            hash_len,
        }
    }

    pub fn hash_block(&self, hmac: &mut Hmac, s: &[u8]) -> Vec<u8> {
        // Create the first output block in the chain by hashing the salt (the password has already been set for the hmac)
        let mut block = hmac.hash_and_reset(&s);
        let mut temp: Vec<u8> = Vec::new();

        // The second iteration uses the block as input as temp is still empty.
        if self.iterations > 1 {
            temp = hmac.hash_and_reset(&block);
            for (target, new) in block.iter_mut().zip_eq(temp.iter()) {
                *target ^= new
            }
        }

        // After the second iteration the previous temp is hashed, saved, and xored into the block
        for _ in 2..self.iterations {
            temp = hmac.hash_and_reset(&temp);
            for (target, new) in block.iter_mut().zip_eq(temp.iter()) {
                *target ^= new
            }
        }

        block
    }
}

impl StatefulHasher for Pbkdf2 {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    fn finalize(mut self) -> Vec<u8> {
        let mut hmac = Hmac::init(self.variant, &self.buffer);
        let mut out = Vec::new();
        self.salt.extend([0; 4]);
        let l = self.salt.len();

        let mut block_num: u32 = 0;
        while out.len() < self.hash_len as usize {
            block_num += 1;
            self.salt[l - 4..].copy_from_slice(&block_num.to_be_bytes());
            out.extend(self.hash_block(&mut hmac, &self.salt));
        }

        out.truncate(self.hash_len as usize);
        out
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1, Pbkdf2::init(HmacVariant::Sha1, 1, 20, b"salt"), b"password", "0c60c80f961f0e71f3a9b524af6012062fe037a6";
    test2, Pbkdf2::init(HmacVariant::Sha1, 2, 20, b"salt"), b"password", "ea6c014dc72d6f8ccd1ed92ace1d41f0d8de8957";
    test3, Pbkdf2::init(HmacVariant::Sha1, 4096, 20, b"salt"), b"password", "4b007901b765489abead49d926f721d065a429c1";
    test4, Pbkdf2::init(HmacVariant::Sha1, 4096, 25, b"saltSALTsaltSALTsaltSALTsaltSALTsalt"), b"passwordPASSWORDpassword", "3d2eec4fe41c849b80c8d83662c0e44a8b291a964cf2f07038";
    test5, Pbkdf2::init(HmacVariant::Sha1, 4096, 16, b"sa\0lt"), b"pass\0word", "56fa6aa75548099dcc37d7f03425e0c3";
);
