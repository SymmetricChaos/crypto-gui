use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crypto_bigint::U256;
use std::num::Wrapping as W;
use std::ops::Index;
use std::ops::IndexMut;
use utils::byte_formatting::make_u64s_le;
use utils::byte_formatting::u64s_to_bytes_le;
use utils::byte_formatting::ByteFormat;

const WORDS: usize = 4;
const ROUNDS: usize = 72;
const N_OCTO_ROUNDS: usize = ROUNDS / 4;

#[derive(Debug, Copy, Clone)]
pub struct Tweak([W<u64>; 2]);

impl Tweak {
    pub fn new() -> Self {
        Tweak([W(0); 2])
    }

    /// Increment the 96-bit counter
    pub fn increment(&mut self, n: u64) {
        match self[0].0.overflowing_add(n) {
            (x, false) => self[0] = W(x),
            (x, true) => {
                self[0] = W(x);
                self[1] = W((self[1].0 + 1) & 0x00000000FFFFFFFF);
            }
        }
    }

    // Created the extended three word array
    pub fn extended(&self) -> [W<u64>; 3] {
        [self[0], self[1], W(self[0].0 ^ self[1].0)]
    }

    /// Set for the first block of a UBI compression
    pub fn first_block(&mut self) {
        self[1] |= 1 << 62;
    }

    /// Set for the last block of a UBI compression
    pub fn final_block(&mut self) {
        self[1] |= 1 << 63;
    }

    // Unused in this implementation.
    ///  Level in the hash tree, zero for non-tree computations.
    pub fn tree_level(&mut self, n: u64) {
        assert!(n < 64);
        self[1] &= 0xFFC0FFFFFFFFFFFF;
        self[1] |= n << 48;
    }

    // Unused in this implementation. All inputs are assumed to be full bytes.
    /// Set if this block contains the last byte of an input whose length was not an integral number of bytes. 0 otherwise.
    pub fn bit_pad(&mut self) {
        self[1] |= 1 << 55;
    }

    /// Key (for MAC and KDF)
    pub fn key(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 0 << 56;
    }

    /// Configuration block
    pub fn cfg(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 4 << 56;
    }

    /// Personalization string
    pub fn prs(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 8 << 56;
    }

    /// Public key (for digital signature hashing)
    pub fn pk(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 12 << 56;
    }

    /// Key identifier (for KDF)
    pub fn kdf(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 16 << 56;
    }

    /// Nonce (for stream cipher or randomized hashing)
    pub fn non(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 20 << 56;
    }

    /// Message
    pub fn msg(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 48 << 56;
    }

    /// Output
    pub fn out(&mut self) {
        self[1] &= 0xC0FFFFFFFFFFFFFF;
        self[1] |= 63 << 56;
    }
}

impl Index<usize> for Tweak {
    type Output = W<u64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tweak {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! skein_subkey_add {
    ($a: expr, $b: expr, $c: expr, $d: expr, $k: expr, $t: expr, $r: expr) => {
        $a = $a + $k[($r + 0) as usize % 5];
        $b = $b + $k[($r + 1) as usize % 5] + $t[($r + 0) as usize % 3];
        $c = $c + $k[($r + 2) as usize % 5] + $t[($r + 1) as usize % 3];
        $d = $d + $k[($r + 3) as usize % 5] + W($r);
    };
}

macro_rules! skein_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a + $b;
        $b = W($b.0.rotate_left($r) ^ $a.0);
    };
}

pub fn octo_round_256(
    w: &mut [W<u64>; 4],
    ex_key: &[W<u64>; WORDS + 1],
    ex_tweak: &[W<u64>; 3],
    round: u64,
) {
    skein_subkey_add!(w[0], w[1], w[2], w[3], ex_key, ex_tweak, round);

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    skein_subkey_add!(w[0], w[1], w[2], w[3], ex_key, ex_tweak, round + 1);

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);
}

pub struct Threefish256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: U256,
    pub extended_key: [W<u64>; WORDS + 1],
    pub extended_tweak: [W<u64>; 3],
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Threefish256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: U256::ZERO,
            extended_key: [W(0); WORDS + 1],
            extended_tweak: [W(0); 3],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Threefish256 {}

crate::block_cipher_builders! {Threefish256, U256}

impl BlockCipher<32> for Threefish256 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = make_u64s_le::<4>(bytes).map(|n| W(n));

        for r in 0..(N_OCTO_ROUNDS) {
            octo_round_256(
                &mut block,
                &self.extended_key,
                &self.extended_tweak,
                r as u64,
            );
        }

        block[0] = block[0] + self.extended_key[3];
        block[1] = block[1] + (self.extended_key[4]) + (self.extended_tweak[0]);
        block[2] = block[2] + (self.extended_key[0]) + (self.extended_tweak[1]);
        block[3] = block[3] + (self.extended_key[1]) + W(N_OCTO_ROUNDS as u64);

        u64s_to_bytes_le(bytes, &block.map(|n| n.0));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {}
}

crate::impl_cipher_for_block_cipher!(Threefish256, 32);
