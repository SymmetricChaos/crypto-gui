use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be, ByteFormat};

const DEBUG: bool = false;
macro_rules! debug_state {
    ($s:literal, $v:ident) => {
        if DEBUG {
            print!($s);
            println!(" {:08x?}", $v);
        }
    };
}

const N: usize = 32;

fn s0(a: u8, b: u8) -> u8 {
    a.wrapping_add(b).rotate_left(2)
}

fn s1(a: u8, b: u8) -> u8 {
    a.wrapping_add(b).wrapping_add(1).rotate_left(2)
}

fn f(a: u32, b: u16) -> u32 {
    let a = a.to_be_bytes();
    let b = b.to_be_bytes();

    let mut k = [0, a[1] ^ b[0] ^ a[0], a[2] ^ b[1] ^ a[3], 0];

    k[1] = s1(k[1], k[2]);
    k[2] = s0(k[2], k[1]);
    k[0] = s0(a[0], k[1]);
    k[3] = s1(a[3], k[2]);

    u32::from_be_bytes(k)
}

fn fk(a: u32, b: u32) -> u32 {
    let a = a.to_be_bytes();
    let b = b.to_be_bytes();

    let mut k = [0, a[1] ^ a[0], a[2] ^ a[3], 0];

    k[1] = s1(k[1], k[2] ^ b[0]);
    k[2] = s0(k[2], k[1] ^ b[1]);
    k[0] = s0(a[0], k[1] ^ b[2]);
    k[3] = s1(a[3], k[2] ^ b[3]);

    u32::from_be_bytes(k)
}

pub struct FealNx {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub subkeys: [u16; N + 8],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for FealNx {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            subkeys: [0; N + 8],
            iv: 0,
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

impl FealNx {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        let mut key = [0; 4];
        fill_u32s_be(&mut key, &bytes);
        let q = [key[2] ^ key[3], key[2], key[3]];

        let mut d = 0;
        let mut a = key[0];
        let mut b = key[1];

        for i in 0..(N / 2 + 4) {
            let t = d;
            d = a;
            a = b;
            b = fk(d, a ^ t ^ q[i % 3]);

            self.subkeys[2 * i] = (b >> 16) as u16;
            self.subkeys[2 * i + 1] = b as u16;
        }
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<8> for FealNx {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        fill_u32s_be(&mut v, bytes);
        debug_state!("e input:", v);

        // Preprocessing stage
        v[0] ^= (self.subkeys[N] as u32) << 16 | self.subkeys[N + 1] as u32;
        v[1] ^= (self.subkeys[N + 2] as u32) << 16 | self.subkeys[N + 3] as u32;
        v[1] ^= v[0];
        debug_state!("e pre:", v);

        // Feistel network
        for subkey in self.subkeys.into_iter().take(32) {
            let t = v[0];
            // L_i+1 = R_i
            v[0] = v[1];

            // R_i+1 = L_i xor f(R_i)
            v[1] = t ^ f(v[1], subkey);
            debug_state!("e med:", v);
        }

        // Postprocessing
        v.swap(0, 1);
        v[1] ^= v[0];
        v[0] ^= (self.subkeys[N + 4] as u32) << 16 | self.subkeys[N + 5] as u32;
        v[1] ^= (self.subkeys[N + 6] as u32) << 16 | self.subkeys[N + 7] as u32;
        debug_state!("e post:", v);

        u32s_to_bytes_be(bytes, &v);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        fill_u32s_be(&mut v, bytes);
        debug_state!("d input:", v);

        // Preprocessing stage
        v[0] ^= (self.subkeys[N + 4] as u32) << 16 | self.subkeys[N + 5] as u32;
        v[1] ^= (self.subkeys[N + 6] as u32) << 16 | self.subkeys[N + 7] as u32;
        v[1] ^= v[0];
        v.swap(0, 1);
        debug_state!("d pre:", v);

        // Feistel network
        for subkey in self.subkeys.into_iter().take(32).rev() {
            let t = v[1];
            // L_i+1 = R_i
            v[1] = v[0];

            // R_i+1 = L_i xor f(R_i)
            v[0] = t ^ f(v[0], subkey);
            debug_state!("d med:", v);
        }

        // Postprocessing
        v[1] ^= v[0];
        v[0] ^= (self.subkeys[N] as u32) << 16 | self.subkeys[N + 1] as u32;
        v[1] ^= (self.subkeys[N + 2] as u32) << 16 | self.subkeys[N + 3] as u32;
        debug_state!("d post:", v);

        u32s_to_bytes_be(bytes, &v);
    }
}

crate::impl_cipher_for_block_cipher!(FealNx, 8);

#[cfg(test)]
mod feal_tests {

    use super::*;

    #[test]
    fn ksa() {
        let cipher = FealNx::default().with_key([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef,
        ]);
        assert_eq!(
            [
                0x7519, 0x71f9, 0x84e9, 0x4886, 0x88e5, 0x523b, 0x4ea4, 0x7ade, 0xfe40, 0x5e76,
                0x9819, 0xeeac, 0x1bd4, 0x2455, 0xdca0, 0x653b, 0x3e32, 0x4652, 0x1cc1, 0x34df,
                0x778b, 0x771d, 0xd324, 0x8410, 0x1ca8, 0xbc64, 0xa0db, 0xbdd2, 0x1f5f, 0x8f1c,
                0x6b81, 0xb560, 0x196a, 0x9ab1, 0xe015, 0x8190, 0x9f72, 0x6643, 0xad32, 0x683a
            ],
            cipher.subkeys
        );
    }

    #[test]
    fn f_function() {
        let x = f(0x00ffff00, 0xffff);
        assert_eq!(0x10041044, x);
    }
}

crate::test_block_cipher!(
    FealNx::default().with_key([
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xcd, 0xef,
    ]), test_1,
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0x9c, 0x9b, 0x54, 0x97, 0x3d, 0xf6, 0x85, 0xf8];

    FealNx::default().with_key([
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10,
    ]), test_2,
    [0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04],
    [0x64, 0xdc, 0xeb, 0xc2, 0x86, 0xd4, 0x6c, 0xe1];
);
