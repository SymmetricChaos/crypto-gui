// use crypto_bigint::U256;
// use utils::byte_formatting::ByteFormat;
// use super::block_cipher::{BCMode, BCPadding, BlockCipher};

// pub struct Threefish256 {
//     pub input_format: ByteFormat,
//     pub output_format: ByteFormat,
//     pub iv: U256,
//     pub mode: BCMode,
//     pub padding: BCPadding,
// }

// impl Default for Threefish256 {
//     fn default() -> Self {
//         Self {
//             input_format: ByteFormat::Hex,
//             output_format: ByteFormat::Hex,
//             iv: U256::ZERO,
//             mode: Default::default(),
//             padding: Default::default(),
//         }
//     }
// }

// crate::block_cipher_builders! {Threefish256, U256}


// impl BlockCipher<32> for Threefish256 {
//     fn encrypt_block(&self, bytes: &mut [u8]) {

//     }

//     fn decrypt_block(&self, bytes: &mut [u8]) {

//     }
// }

// crate::impl_cipher_for_block_cipher!(Threefish256, 32);