use super::block_cipher::{none_padding, BCMode, BCPadding, BlockCipher};
use crate::{impl_block_cipher, Cipher, CipherError};
use std::{cmp::max, ops::Shl};
use utils::byte_formatting::{u8_slice_to_u32_4, ByteFormat};

const P32: u32 = 0xb7e15163;
const Q32: u32 = 0x9e3779b9;
const BLOCKSIZE: u32 = 16;
struct Rc6 {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub rounds: usize,
    pub state: Vec<u32>,
    pub iv: u128,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Rc6 {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            rounds: 20,
            state: Default::default(),
            iv: 0,
            mode: BCMode::default(),
            padding: BCPadding::default(),
        }
    }
}

impl Rc6 {
    pub fn state_size(&self) -> usize {
        (2 * self.rounds) + 4
    }

    pub fn ksa_32(&mut self, key: &[u8]) {
        assert!(
            key.len() < 256,
            "RC5 key is limited to 255 bytes, which is enough for anybody"
        );

        let u = 4; // Bytes in a word
        let b = key.len(); // Bytes in the key
        let c = max(b.div_ceil(u), 1); // number of words in the key
        let mut l = vec![0_u32; c];
        for i in (0..b).rev() {
            l[i / u] = (l[i / u].shl(8_u32)).wrapping_add(key[i] as u32)
        }

        let t = self.state_size();
        let mut s = vec![0; t];
        s[0] = P32;
        for i in 1..t {
            s[i] = s[i - 1].wrapping_add(Q32)
        }

        let mut i = 0;
        let mut j = 0;
        let mut a = 0;
        let mut b = 0;
        for _ in 0..(3 * max(t, c)) {
            s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
            a = s[i];
            l[j] = (l[j].wrapping_add(a).wrapping_add(b)).rotate_left(a.wrapping_add(b));
            b = l[j];
            i = (i + 1) % t;
            j = (j + 1) % c;
        }

        self.state = s;
    }
}

impl BlockCipher<16> for Rc6 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = u8_slice_to_u32_4(bytes);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = u8_slice_to_u32_4(bytes);
    }

    fn set_mode(&mut self, mode: BCMode) {
        self.mode = mode
    }

    fn set_padding(&mut self, padding: BCPadding) {
        self.padding = padding
    }
}

impl_block_cipher!(Rc6);
