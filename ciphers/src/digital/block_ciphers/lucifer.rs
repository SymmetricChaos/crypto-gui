use super::block_cipher::{BCMode, BCPadding, BlockCipher};
use utils::byte_formatting::ByteFormat;

// Based on
// https://github.com/robwaddell/lucifer-go/blob/master/lucifer-go.go

const SBOX0: [u8; 16] = [12, 15, 7, 10, 14, 13, 11, 0, 2, 6, 3, 1, 9, 4, 5, 8];
const SBOX1: [u8; 16] = [7, 2, 14, 9, 3, 11, 0, 4, 12, 13, 1, 10, 6, 15, 8, 5];

pub fn sbox(x: u8, control: bool) -> u8 {
    // Extract each 4-bit nibble and reverse the bits
    let l = (x >> 4).reverse_bits() >> 4;
    let r = (x & 0xf).reverse_bits() >> 4;

    if control {
        (SBOX1[r as usize] << 4 | SBOX0[l as usize]).reverse_bits()
    } else {
        (SBOX1[l as usize] << 4 | SBOX0[r as usize]).reverse_bits()
    }
}

// Lookup table for the bitwise Lucifer permutation on a byte
// The permutation is 35042176 (starting with the most significant bit)
const PTABLE: [u8; 256] = [
    0, 2, 1, 3, 64, 66, 65, 67, 32, 34, 33, 35, 96, 98, 97, 99, 8, 10, 9, 11, 72, 74, 73, 75, 40,
    42, 41, 43, 104, 106, 105, 107, 128, 130, 129, 131, 192, 194, 193, 195, 160, 162, 161, 163,
    224, 226, 225, 227, 136, 138, 137, 139, 200, 202, 201, 203, 168, 170, 169, 171, 232, 234, 233,
    235, 4, 6, 5, 7, 68, 70, 69, 71, 36, 38, 37, 39, 100, 102, 101, 103, 12, 14, 13, 15, 76, 78,
    77, 79, 44, 46, 45, 47, 108, 110, 109, 111, 132, 134, 133, 135, 196, 198, 197, 199, 164, 166,
    165, 167, 228, 230, 229, 231, 140, 142, 141, 143, 204, 206, 205, 207, 172, 174, 173, 175, 236,
    238, 237, 239, 16, 18, 17, 19, 80, 82, 81, 83, 48, 50, 49, 51, 112, 114, 113, 115, 24, 26, 25,
    27, 88, 90, 89, 91, 56, 58, 57, 59, 120, 122, 121, 123, 144, 146, 145, 147, 208, 210, 209, 211,
    176, 178, 177, 179, 240, 242, 241, 243, 152, 154, 153, 155, 216, 218, 217, 219, 184, 186, 185,
    187, 248, 250, 249, 251, 20, 22, 21, 23, 84, 86, 85, 87, 52, 54, 53, 55, 116, 118, 117, 119,
    28, 30, 29, 31, 92, 94, 93, 95, 60, 62, 61, 63, 124, 126, 125, 127, 148, 150, 149, 151, 212,
    214, 213, 215, 180, 182, 181, 183, 244, 246, 245, 247, 156, 158, 157, 159, 220, 222, 221, 223,
    188, 190, 189, 191, 252, 254, 253, 255,
];

fn diffusion(byte: u8, step: usize, half_block: &mut [u8]) {
    if ((byte >> 7) & 0x01) == 0x01 {
        half_block[(7 + step) % 8] ^= 128
    }
    if ((byte >> 6) & 0x01) == 0x01 {
        half_block[(6 + step) % 8] ^= 64
    }
    if ((byte >> 5) & 0x01) == 0x01 {
        half_block[(2 + step) % 8] ^= 32
    }
    if ((byte >> 4) & 0x01) == 0x01 {
        half_block[(1 + step) % 8] ^= 16
    }
    if ((byte >> 3) & 0x01) == 0x01 {
        half_block[(5 + step) % 8] ^= 8
    }
    if ((byte >> 2) & 0x01) == 0x01 {
        half_block[(0 + step) % 8] ^= 4
    }
    if ((byte >> 1) & 0x01) == 0x01 {
        half_block[(3 + step) % 8] ^= 2
    }
    if ((byte >> 0) & 0x01) == 0x01 {
        half_block[(4 + step) % 8] ^= 1
    }
}

fn f(key_byte: u8, upper_byte: u8, control: bool, step: usize, half_block: &mut [u8]) {
    let confused = sbox(upper_byte, control);
    let interrupted = confused & key_byte;
    let permuted = PTABLE[interrupted as usize];
    diffusion(permuted, step, half_block);
}

pub struct Lucifer {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: u128,
    pub key_bytes: [u8; 16],
    pub mode: BCMode,
    pub padding: BCPadding,
}

impl Default for Lucifer {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            iv: 0,
            key_bytes: [0; 16],
            mode: Default::default(),
            padding: Default::default(),
        }
    }
}

crate::block_cipher_builders! {Lucifer, u128}

impl Lucifer {
    pub fn ksa(&mut self, bytes: [u8; 16]) {
        self.key_bytes = bytes
    }

    pub fn with_key(mut self, bytes: [u8; 16]) -> Self {
        self.ksa(bytes);
        self
    }
}

impl BlockCipher<16> for Lucifer {
    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes.split_at_mut(8);

        for i in 0..16 {
            // Feistel round
            let control_byte = self.key_bytes[(i * 7) & 0xf];
            for step in 0..8 {
                let key_byte = self.key_bytes[(i * 7 + step) & 0xf];
                let upper_byte = block.1[step];
                let control = ((control_byte >> (7 - step)) & 0x1) == 0x1;
                f(key_byte, upper_byte, control, step, block.0);
            }
            // Swap
            (block.0, block.1) = (block.1, block.0)
        }
        // Final unswap
        (block.0, block.1) = (block.1, block.0)
    }

    fn decrypt_block(&self, bytes: &mut [u8]) {
        let mut block = bytes.split_at_mut(8);

        for i in 0..16 {
            let control_byte = self.key_bytes[((i + 1) * 9) & 0x0f];
            for step in 0..8 {
                let key_byte = self.key_bytes[((i + 1) * 9 + step) & 0xf];
                let upper_byte = block.1[step];
                let control = ((control_byte >> (7 - step)) & 0x1) == 0x1;
                f(key_byte, upper_byte, control, step, block.0);
            }
            // Swap
            (block.0, block.1) = (block.1, block.0)
        }
        // Final unswap
        (block.0, block.1) = (block.1, block.0)
    }
    crate::block_cipher_getters!();
}

crate::impl_cipher_for_block_cipher!(Lucifer, 16);

#[cfg(test)]
mod idea_tests {

    use crate::Cipher;

    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let cipher = Lucifer::default();
        let ptext = "0123456789abcdeffedcba9876543210";
        let ctext = cipher.encrypt(ptext).unwrap();
        let dtext = cipher.decrypt(&ctext).unwrap();
        assert_eq!(ptext, dtext)
    }
}

fn ttb(s: &str) -> Vec<u8> {
    ByteFormat::Hex.text_to_bytes(s).unwrap()
}

// https://github.com/scorpiochn/Applied-Cryptography/blob/master/lucifer-outerbridge-5.0/tests
crate::test_block_cipher!(
    test_1,
    Lucifer::default().with_key(ttb("0123456789abcdeffedcba9876543210").try_into().unwrap()),
    ttb("00000000000000000000000000000000"),
    ttb("a201fc18d62c85ef5965a58295bbf609");

    test_2,
    Lucifer::default().with_key(ttb("00000000000000000000000000000000").try_into().unwrap()),
    ttb("0123456789abcdeffedcba9876543210"),
    ttb("9d14fe4377aa87dd07cc8a14522c21ed");

    test_3,
    Lucifer::default().with_key(ttb("0123456789abcdeffedcba9876543210").try_into().unwrap()),
    ttb("ffffffffffffffffffffffffffffffff"),
    ttb("97f1c104b0f120d194c07024f14815ed");

    test_4,
    Lucifer::default().with_key(ttb("ffffffffffffffffffffffffffffffff").try_into().unwrap()),
    ttb("0123456789abcdeffedcba9876543210"),
    ttb("d442a34dd70e2b4156eb0f2a8aded1a7");

    test_5,
    Lucifer::default().with_key(ttb("0123456789abcdeffedcba9876543210").try_into().unwrap()),
    ttb("0123456789abcdeffedcba9876543210"),
    ttb("cf46622fa98546bb9a5bc00239eb0c92");

    test_6,
    Lucifer::default().with_key(ttb("fedcba98765432100123456789abcdef").try_into().unwrap()),
    ttb("0123456789abcdeffedcba9876543210"),
    ttb("7faf65bfc5458fd2dc9cc2266012ef44");
);
