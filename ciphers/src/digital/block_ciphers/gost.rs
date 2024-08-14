use utils::byte_formatting::ByteFormat;

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
            sboxes: GOST_R_34_12_2015,
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
}

impl BlockCipher<8> for Gost {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        for round in 0..32 {
            let subkey = self.key[Gost::ROUND_KEY_IDX[round]];
        }

        todo!()
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        todo!()
    }
}

impl_cipher_for_block_cipher!(Gost, 8);

#[cfg(test)]
mod gost_tests {

    use crate::Cipher;

    use super::*;

    #[test]
    fn gost_sboxes() {
        let mut cipher = Gost::default();
        assert_eq!(0xC6BC7581, cipher.sbox(0x00000000_u32));
    }

    #[test]
    fn encrypt_decrypt_ecb() {
        let ptext = "01020304050607080102030405060708";
        let mut cipher = Gost::default();
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
