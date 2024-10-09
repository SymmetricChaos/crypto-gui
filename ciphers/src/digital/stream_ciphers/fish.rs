use num::Integer;
use std::collections::VecDeque;
use utils::byte_formatting::{xor_into_bytes, ByteFormat};

#[derive(Debug, Clone)]
pub struct FishRng {
    lfg_a: VecDeque<u32>,
    lfg_b: VecDeque<u32>,
}

impl Default for FishRng {
    fn default() -> Self {
        Self {
            lfg_a: VecDeque::from_iter(0..55),
            lfg_b: VecDeque::from_iter(0..52),
        }
    }
}

impl FishRng {
    pub fn step(&mut self) -> Option<(u32, u32)> {
        // This is roughly correct but the exact taps and lengths are needed
        let a = self.lfg_a[0].wrapping_add(self.lfg_a[24]);
        self.lfg_a.pop_front();
        self.lfg_a.push_back(a);

        let b = self.lfg_b[0].wrapping_add(self.lfg_b[19]);
        self.lfg_b.pop_front();
        self.lfg_b.push_back(b);

        if b.is_even() {
            None
        } else {
            Some((a, b))
        }
    }

    pub fn next_pair(&mut self) -> (u32, u32) {
        loop {
            if let Some(pair) = self.step() {
                return pair;
            }
        }
    }

    pub fn next_output(&mut self) -> [u8; 8] {
        let (c1, d1) = self.next_pair();
        let (c2, d2) = self.next_pair();
        let e1 = c1 ^ (d1 & d2);
        let f1 = d2 & (e1 & c2);
        let k1 = e1 ^ f1;
        let k2 = c2 ^ f1;
        let mut n = k2 as u64;
        n |= (k1 as u64) << 32;
        n.to_be_bytes()
    }

    pub fn keystream(&mut self, n_bytes: usize) -> Vec<u8> {
        let mut bytes = vec![0; n_bytes];
        while bytes.len() < n_bytes {
            bytes.extend(self.next_output())
        }
        bytes.truncate(n_bytes);
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct FishCipher {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub rng: FishRng,
}

impl Default for FishCipher {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            rng: Default::default(),
        }
    }
}

impl FishCipher {
    pub fn encrypt_bytes(&self, bytes: &mut [u8]) {
        let mut rng = self.rng.clone();
        let keystream = rng.keystream(bytes.len());
        xor_into_bytes(bytes, &keystream);
    }
}

crate::impl_cipher_for_stream_cipher!(FishCipher);
