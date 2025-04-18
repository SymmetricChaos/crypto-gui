use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use crypto_bigint::U256;
use utils::byte_formatting::fill_u64s_le;
use utils::byte_formatting::ByteFormat;

pub const PERM_256: [usize; 4] = [0, 3, 2, 1];
pub const PERM_512: [usize; 8] = [2, 1, 4, 7, 6, 5, 0, 3];
pub const PERM_1024: [usize; 16] = [0, 9, 2, 13, 6, 11, 4, 15, 10, 7, 12, 3, 14, 5, 8, 1];

macro_rules! skein_mix {
    ($a: ident, $b: ident, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r);
        $b ^= $a;
    };
}

pub fn four_rounds(
    mut a: u64,
    mut b: u64,
    mut c: u64,
    mut d: u64,
    keys: [u64; 4],
) -> (u64, u64, u64, u64) {
    skein_mix!(a, b, 14);
    skein_mix!(c, d, 16);
    (b, d) = (d, b);

    skein_mix!(a, b, 52);
    skein_mix!(c, d, 57);
    (b, d) = (d, b);

    skein_mix!(a, b, 23);
    skein_mix!(c, d, 40);
    (b, d) = (d, b);

    skein_mix!(a, b, 5);
    skein_mix!(c, d, 37);
    (b, d) = (d, b);

    a = a.wrapping_add(keys[0]);
    b = b.wrapping_add(keys[1]);
    c = c.wrapping_add(keys[2]);
    d = d.wrapping_add(keys[3]);

    (a, b, c, d)
}

pub struct Threefish256 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: U256,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Threefish256 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: U256::ZERO,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl Threefish256 {
    pub fn with_key(mut self, bytes: [u8; 32]) -> Self {
        todo!()
    }

    fn subkeys(&self) {
        todo!()
    }
}

crate::block_cipher_builders! {Threefish256, U256}

impl BlockCipher<32> for Threefish256 {
    fn encrypt_block(&self, bytes: &mut [u8]) {}

    fn decrypt_block(&self, bytes: &mut [u8]) {}
}

crate::impl_cipher_for_block_cipher!(Threefish256, 32);
