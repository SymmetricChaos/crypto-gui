pub mod aes;
pub mod chacha;
pub mod chacha20poly1305;
pub mod chacha_extended_nonce;
pub mod des;
pub mod elgamal;
pub mod rc4;
pub mod rc5;
pub mod rsa;
pub mod salsa20;
pub mod tea;
pub mod xtea;

#[derive(Debug, PartialEq, Eq)]
pub enum BlockCipherMode {
    Ecb,
    Ctr,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockCipherPadding {
    None,
    Bit, // add the byte 0x80, then add 0x00 bytes until the block size is reached
         // equivalently add a single 1 bit then append 0 bits until the block size is reached
}

pub fn bit_padding(bytes: &mut Vec<u8>, block_size: u32) {
    bytes.push(0x80);
    while bytes.len() % block_size as usize != 0 {
        bytes.push(0x00)
    }
}
