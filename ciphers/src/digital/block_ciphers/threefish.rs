use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crypto_bigint::U256;
use hex_literal::hex;
use std::ops::{Index, IndexMut};
use utils::byte_formatting::{fill_u64s_le, make_u64s_le, u64s_to_bytes_le, ByteFormat};

const BLOCK_WORDS: usize = 4;
const BLOCK_BYTES: usize = BLOCK_WORDS * 8;

const KEY_WORDS: usize = 4;
const KEY_BYTES: usize = KEY_WORDS * 8;

const TWEAK_BYTES: usize = 16;

const ROUNDS: usize = 72;
const SUBKEYS: usize = ROUNDS / 4 + 1;

const C240: u64 = 0x1BD11BDAA9FC1A22;

#[derive(Debug, Copy, Clone)]
pub struct Tweak([u64; 2]);

impl Tweak {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == 16);
        Tweak([
            u64::from_le_bytes(bytes[..8].try_into().unwrap()),
            u64::from_le_bytes(bytes[8..].try_into().unwrap()),
        ])
    }

    // Created the extended three word array
    pub fn extended(&self) -> [u64; 3] {
        [self[0], self[1], self[0] ^ self[1]]
    }
}

impl Index<usize> for Tweak {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tweak {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! threefish_subkey_add {
    ($a: expr, $b: expr, $c: expr, $d: expr, $k: expr) => {
        $a = $a.wrapping_add($k[0]);
        $b = $b.wrapping_add($k[1]);
        $c = $c.wrapping_add($k[2]);
        $d = $d.wrapping_add($k[3]);
    };
}

macro_rules! threefish_subkey_sub {
    ($a: expr, $b: expr, $c: expr, $d: expr, $k: expr) => {
        $a = $a.wrapping_sub($k[0]);
        $b = $b.wrapping_sub($k[1]);
        $c = $c.wrapping_sub($k[2]);
        $d = $d.wrapping_sub($k[3]);
    };
}

macro_rules! threefish_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r) ^ $a;
    };
}

macro_rules! threefish_unmix {
    ($a: expr, $b: expr, $r: literal) => {
        $b = ($a ^ $b).rotate_right($r);
        $a = $a.wrapping_sub($b);
    };
}

pub fn octo_round_256(w: &mut [u64; 4], subkey: &[[u64; 4]]) {
    threefish_subkey_add!(w[0], w[1], w[2], w[3], subkey[0]);

    threefish_mix!(w[0], w[1], 14);
    threefish_mix!(w[2], w[3], 16);

    threefish_mix!(w[0], w[3], 52);
    threefish_mix!(w[2], w[1], 57);

    threefish_mix!(w[0], w[1], 23);
    threefish_mix!(w[2], w[3], 40);

    threefish_mix!(w[0], w[3], 5);
    threefish_mix!(w[2], w[1], 37);

    threefish_subkey_add!(w[0], w[1], w[2], w[3], subkey[1]);

    threefish_mix!(w[0], w[1], 25);
    threefish_mix!(w[2], w[3], 33);

    threefish_mix!(w[0], w[3], 46);
    threefish_mix!(w[2], w[1], 12);

    threefish_mix!(w[0], w[1], 58);
    threefish_mix!(w[2], w[3], 22);

    threefish_mix!(w[0], w[3], 32);
    threefish_mix!(w[2], w[1], 32);
}

pub fn octo_round_256_inv(w: &mut [u64; 4], subkey: &[[u64; 4]]) {
    threefish_unmix!(w[2], w[1], 32);
    threefish_unmix!(w[0], w[3], 32);

    threefish_unmix!(w[2], w[3], 22);
    threefish_unmix!(w[0], w[1], 58);

    threefish_unmix!(w[2], w[1], 12);
    threefish_unmix!(w[0], w[3], 46);

    threefish_unmix!(w[2], w[3], 33);
    threefish_unmix!(w[0], w[1], 25);

    threefish_subkey_sub!(w[0], w[1], w[2], w[3], subkey[1]);

    threefish_unmix!(w[2], w[1], 37);
    threefish_unmix!(w[0], w[3], 5);

    threefish_unmix!(w[2], w[3], 40);
    threefish_unmix!(w[0], w[1], 23);

    threefish_unmix!(w[2], w[1], 57);
    threefish_unmix!(w[0], w[3], 52);

    threefish_unmix!(w[2], w[3], 16);
    threefish_unmix!(w[0], w[1], 14);

    threefish_subkey_sub!(w[0], w[1], w[2], w[3], subkey[0]);
}

pub fn create_subkeys(
    key: &[u8; KEY_BYTES],
    tweak: &[u8; TWEAK_BYTES],
) -> [[u64; KEY_WORDS]; SUBKEYS] {
    let ex_tweak = Tweak::from_bytes(tweak).extended();
    let mut ex_key = [0_u64; KEY_WORDS + 1];
    fill_u64s_le(&mut ex_key[0..KEY_WORDS], key);
    ex_key[KEY_WORDS] = ex_key.iter().fold(C240, core::ops::BitXor::bitxor);

    let mut subkeys = [[0u64; KEY_WORDS]; SUBKEYS];

    // The inner loop allows this to be reused for other key sizes
    for k in 0..SUBKEYS {
        for i in 0..KEY_WORDS {
            subkeys[k][i] = ex_key[(k + i) % (KEY_WORDS + 1)];
            if i == KEY_WORDS - 3 {
                subkeys[k][i] = subkeys[k][i].wrapping_add(ex_tweak[k % 3]);
            } else if i == KEY_WORDS - 2 {
                subkeys[k][i] = subkeys[k][i].wrapping_add(ex_tweak[(k + 1) % 3]);
            } else if i == KEY_WORDS - 1 {
                subkeys[k][i] = subkeys[k][i].wrapping_add(k as u64);
            }
        }
    }
    subkeys
}

pub struct Threefish256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: U256,
    pub subkeys: [[u64; KEY_WORDS]; SUBKEYS],
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Threefish256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: U256::ZERO,
            subkeys: [[0; KEY_WORDS]; SUBKEYS],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Threefish256 {
    pub fn with_key_and_tweak(key: &[u8; KEY_BYTES], tweak: &[u8; 16]) -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: U256::ZERO,
            subkeys: create_subkeys(key, tweak),
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

crate::block_cipher_builders! {Threefish256, U256}

impl BlockCipher<BLOCK_BYTES> for Threefish256 {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block: [u64; BLOCK_WORDS] = make_u64s_le(bytes);

        for r in 0..((SUBKEYS - 1) / 2) {
            octo_round_256(&mut block, &self.subkeys[(2 * r)..][..2]);
        }

        for i in 0..4 {
            block[i] = block[i].wrapping_add(self.subkeys[SUBKEYS - 1][i])
        }

        u64s_to_bytes_le(bytes, &block.map(|n| n));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block: [u64; BLOCK_WORDS] = make_u64s_le(bytes);

        for i in 0..4 {
            block[i] = block[i].wrapping_sub(self.subkeys[SUBKEYS - 1][i])
        }

        for r in (0..((SUBKEYS - 1) / 2)).rev() {
            octo_round_256_inv(&mut block, &self.subkeys[(2 * r)..][..2]);
        }

        u64s_to_bytes_le(bytes, &block.map(|n| n));
    }
}

crate::impl_cipher_for_block_cipher!(Threefish256, 32);

crate::test_block_cipher!(
    test_1, Threefish256::with_key_and_tweak(&hex!(
        "1011121314151617 18191A1B1C1D1E1F 2021222324252627 28292A2B2C2D2E2F"
    ), &hex!(
        "0001020304050607 08090A0B0C0D0E0F"
    )),
    hex!(
        "FFFEFDFCFBFAF9F8 F7F6F5F4F3F2F1F0 EFEEEDECEBEAE9E8 E7E6E5E4E3E2E1E0"
    ),
    hex!(
        "E0D091FF0EEA8FDF C98192E62ED80AD5 9D865D08588DF476 657056B5955E97DF"
    );


);
