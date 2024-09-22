use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be, ByteFormat};

// const DEBUG: bool = false;

fn round(v: u64, subkey: u64) -> u64 {
    todo!()
}

fn s0(a: u8, b: u8) -> u8 {
    a.wrapping_add(b).rotate_left(2)
}

fn s1(a: u8, b: u8) -> u8 {
    a.wrapping_add(b).wrapping_add(1).rotate_left(2)
}

fn f(a: [u8; 4], b: [u8; 2]) {
    todo!()
}

fn fk(a: u32, b: u32) -> u32 {
    let bytes_a = a.to_be_bytes();
    let bytes_b = b.to_be_bytes();

    let mut k = [0, bytes_a[1] ^ bytes_a[0], bytes_a[2] ^ bytes_a[3], 0];

    k[1] = s1(k[1], k[2] ^ bytes_b[0]);
    k[2] = s0(k[2], k[1] ^ bytes_b[1]);
    k[0] = s0(bytes_a[0], k[1] ^ bytes_b[2]);
    k[3] = s1(bytes_a[3], k[2] ^ bytes_b[3]);
    u32::from_be_bytes(k)
}

pub struct FealNx {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub subkeys: [u16; 40],
    pub iv: u64,
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for FealNx {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            subkeys: [0; 40],
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

        for i in 0..20 {
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

        // Feistel network
        for subkey in self.subkeys {
            // let t = v[0];
            // // L_i+1 = R_i
            // v[0] = v[1];

            // // R_i+1 = L_i xor f(R_i)
            // v[1] = t ^ round(v[1], subkey);
        }
        v.swap(0, 1);

        u32s_to_bytes_be(bytes, &v);
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        fill_u32s_be(&mut v, bytes);

        // Feistel network
        for subkey in self.subkeys.into_iter().rev() {
            // let t = v[0];
            // // L_i+1 = R_i
            // v[0] = v[1];

            // // R_i+1 = L_i xor f(R_i)
            // v[1] = t ^ round(v[1], subkey);
        }
        v.swap(0, 1);

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
        println!("{:04x?}", cipher.subkeys)
    }
}
