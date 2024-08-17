use utils::byte_formatting::{overwrite_bytes, u32_pair_to_u8_array, ByteFormat};

use crate::impl_cipher_for_block_cipher;

use super::block_cipher::{BCMode, BCPadding, BlockCipher};

const GOST_R_34_12_2015: [u64; 8] = [
    0xC462A5B9E8D703F1,
    0x68239A5C1E47BD0F,
    0xB3582FADE174C960,
    0xC821D4F670A53E9B,
    0x7F5A816D093EB42C,
    0x5DF692CAB78143E0,
    0x8E25691CF4B0DA37,
    0x17ED05834FA69CB2,
];

pub struct Gost {
    pub output_format: ByteFormat,
    pub input_format: ByteFormat,
    pub mode: BCMode,
    pub padding: BCPadding,
    pub iv: u64,
    pub sboxes: [u64; 8],
    pub key: [u32; 8],
}

impl Default for Gost {
    fn default() -> Self {
        Self {
            output_format: ByteFormat::Hex,
            input_format: ByteFormat::Hex,
            mode: BCMode::default(),
            padding: BCPadding::default(),
            iv: 0,
            sboxes: GOST_R_34_12_2015.clone(),
            key: [0; 8],
        }
    }
}

impl Gost {
    const ROUND_KEY_IDX: [usize; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 7, 6, 5, 4, 3, 2,
        1, 0,
    ];

    pub fn sbox(&self, n: u32) -> u32 {
        let mut out = 0;

        for i in 0..8 {
            let shift = 28 - (4 * i);
            let idx = (n >> shift) & 0x0f;
            let s = self.sboxes[i] >> (60 - idx * 4) & 0x0f;
            out |= (s as u32) << shift;
        }

        out
    }

    pub fn f(&self, n: u32, subkey: u32) -> u32 {
        let x = n.wrapping_add(subkey);
        let x = self.sbox(x);
        x.rotate_left(11)
    }
}

impl BlockCipher<8> for Gost {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        for idx in Gost::ROUND_KEY_IDX {
            let t = v[0];
            // L_i+1 = R_i
            v[0] = v[1];

            // R_i+1 = L_i xor f(R_i)
            v[1] = t ^ self.f(v[1], self.key[idx]);
        }
        v.swap(0, 1);

        overwrite_bytes(bytes, &u32_pair_to_u8_array(v));
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut v = [0u32; 2];
        for (elem, chunk) in v.iter_mut().zip(bytes.chunks_exact(4)) {
            *elem = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        for idx in Gost::ROUND_KEY_IDX.into_iter().rev() {
            let t = v[0];
            // L_i+1 = R_i
            v[0] = v[1];

            // R_i+1 = L_i xor f(R_i)
            v[1] = t ^ self.f(v[1], self.key[idx]);
        }
        v.swap(0, 1);

        overwrite_bytes(bytes, &u32_pair_to_u8_array(v));
    }
}

impl_cipher_for_block_cipher!(Gost, 8);

#[cfg(test)]
mod gost_tests {

    use rand::{thread_rng, Rng};

    use crate::Cipher;

    use super::*;

    const TEST_SBOX: [u64; 8] = [
        0x4a92d80e6b1c7f53,
        0xeb4c6dfa23810759,
        0x581da342efc7609b,
        0x7da1089fe46cb253,
        0x6c715fd84a9e03b2,
        0x4ba0721d36859cfe,
        0xdb413f590ae7682c,
        0x1fd057a4923e6b8c,
    ];

    #[test]
    fn gost_sboxes() {
        let cipher = Gost::default();
        assert_eq!(0xC6BC7581, cipher.sbox(0x00000000_u32));
    }

    #[test]
    fn encrypt_block() {
        let mut cipher = Gost::default();
        cipher.sboxes = TEST_SBOX;
        let mut input = [0, 0, 0, 0, 0, 0, 0, 0];
        let output = [0x0e, 0xca, 0x1a, 0x54, 0x4d, 0x33, 0x07, 0x0b];
        cipher.encrypt_block(&mut input);
        assert_eq!(input, output)
    }

    #[test]
    fn encrypt_decrypt_ecb() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Gost::default();
        thread_rng().fill(&mut cipher.key);
        cipher.mode = BCMode::Ecb;
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }

    #[test]
    fn encrypt_decrypt_ctr() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Gost::default();
        cipher.mode = BCMode::Ctr;
        let ctext = cipher.encrypt(ptext).unwrap();
        assert_eq!(cipher.decrypt(&ctext).unwrap(), ptext);
    }
}
