use crate::{
    auxiliary::des_functions::{expand_56_to_64, Des},
    traits::StatefulHasher,
};

// derived from the ASCII string "KGS!@#$%"
pub const LM_WORD: u64 = 0x4B47532140232425;

pub struct Lm {
    buffer: Vec<u8>,
}

impl Lm {
    pub fn init() -> Self {
        Self {
            buffer: Vec::with_capacity(14),
        }
    }
}

impl StatefulHasher for Lm {
    fn update(&mut self, bytes: &[u8]) {
        if !bytes.is_ascii() {
            panic!("LM accepts only ASCII characters")
        }
        self.buffer.extend_from_slice(bytes);
        self.buffer.truncate(14);
    }

    fn finalize(mut self) -> Vec<u8> {
        // Padding (really bad padding)
        while self.buffer.len() < 14 {
            self.buffer.push(0x00);
        }
        self.buffer.make_ascii_uppercase();
        let mut cipher = Des::default();
        let mut out = Vec::with_capacity(16);

        let k1 = expand_56_to_64(self.buffer[0..7].try_into().unwrap());
        let k2 = expand_56_to_64(self.buffer[7..14].try_into().unwrap());

        cipher.ksa(k1);
        out.extend(cipher.encrypt_block(LM_WORD).to_be_bytes());
        cipher.ksa(k2);
        out.extend(cipher.encrypt_block(LM_WORD).to_be_bytes());

        out
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    test1, Lm::init(), b"PassWord", "e52cac67419a9a224a3b108f3fa6cb6d";
);
