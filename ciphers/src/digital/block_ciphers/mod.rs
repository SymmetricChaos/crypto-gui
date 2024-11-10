pub mod aes;
pub mod aria;
pub mod ascon;
pub mod block_cipher;
pub mod blowfish;
pub mod camellia;
pub mod des;
pub mod fealnx;
pub mod gift;
pub mod gost;
pub mod idea;
pub mod kasumi;
pub mod khufu_khafre;
pub mod lea;
pub mod lucifer;
pub mod misty1;
pub mod present;
pub mod rc2;
pub mod rc5;
pub mod rc6;
pub mod seed;
pub mod serpent;
pub mod simon;
pub mod sm4;
pub mod speck;
pub mod tea;
pub mod threefish;
pub mod twofish;

// TEMPLATE
// use super::block_cipher::{BCMode, BCPadding, BlockCipher};
// use utils::byte_formatting::ByteFormat;
// pub struct CIPHERNAME {
//     pub input_format: ByteFormat,
//     pub output_format: ByteFormat,
//     pub iv: IV_TYPE,
//     pub mode: BCMode,
//     pub padding: BCPadding,
// }
// impl Default for CIPHERNAME {
//     fn default() -> Self {
//         Self {
//             input_format: ByteFormat::Hex,
//             output_format: ByteFormat::Hex,
//             iv: 0,
//             mode: Default::default(),
//             padding: Default::default(),
//         }
//     }
// }
// crate::block_cipher_builders! {CIPHERNAME, IV_TYPE}
// impl Misty1 {
//     pub fn ksa(&mut self, bytes: [u8; KEYBYTES]) {}
//     pub fn with_key(mut self, bytes: [u8; KEYBYTES]) -> Self {
//         self.ksa(bytes);
//         self
//     }
// }
// impl BlockCipher<BLOCKSIZE> for CIPHERNAME {
//     fn encrypt_block(&self, bytes: &mut [u8]) {
//         todo!()
//     }
//     fn decrypt_block(&self, bytes: &mut [u8]) {
//         todo!()
//     }
// }
// crate::impl_cipher_for_block_cipher!(CIPHERNAME, BLOCKSIZE);
// crate::test_block_cipher!(
//     test_1, CIPHERNAME::default().with_key([0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]),
//     [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
//     [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
// );

// This Big Scary Macro is just avoiding a lot of boilerplate since all block ciphers have essentially the same
// implementation for the Cipher trait. Fully qualified names are used to avoid import conflicts.
#[macro_export]
macro_rules! impl_cipher_for_block_cipher {
    ($cipher: ty, $blocksize: literal) => {
        impl crate::traits::Cipher for $cipher {
            fn encrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
                // Interpret the input
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|e| crate::errors::CipherError::Input(e.to_string()))?;

                // Provide the necessary kind and amount of padding
                if self.mode.padded() {
                    self.padding.add_padding(&mut bytes, $blocksize)?;
                }

                // Select the correct mode. Since block ciphers all implement the BlockCipher
                // trait these are available for free. The fully qualified names for each of
                // the encrypt and decrypt functions are too messy and avoiding them is a pain
                // so when this macro is called the file must have the BlockCipher trait
                // imported.
                match self.mode {
                    crate::digital::block_ciphers::block_cipher::BCMode::Ecb => {
                        self.encrypt_ecb(&mut bytes)
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Ctr => {
                        self.encrypt_ctr(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Cbc => {
                        self.encrypt_cbc(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Pcbc => {
                        self.encrypt_pcbc(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Ofb => {
                        self.encrypt_ofb(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Cfb => {
                        self.encrypt_cfb(&mut bytes, self.iv.to_be_bytes())
                    }
                };

                Ok(self.output_format.byte_slice_to_text(&bytes))
            }

            fn decrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
                // Interpret the input
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|e| crate::errors::CipherError::Input(e.to_string()))?;

                // If padding is needed return an error if the input for decryption is the wrong size
                if self.mode.padded() {
                    if bytes.len() % $blocksize != 0 {
                        return Err(crate::errors::CipherError::General(format!(
                            "decryption requires blocks of exactly {} bytes",
                            $blocksize
                        )));
                    }
                }

                // Select the correct mode. Since block ciphers all implement the BlockCipher
                // trait these are available for free. The fully qualified names for each of
                // the encrypt and decrypt functions are too messy and avoiding them is a pain
                // so when this macro is called the file must have the BlockCipher trait
                // imported.
                match self.mode {
                    crate::digital::block_ciphers::block_cipher::BCMode::Ecb => {
                        self.decrypt_ecb(&mut bytes)
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Ctr => {
                        self.decrypt_ctr(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Cbc => {
                        self.decrypt_cbc(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Pcbc => {
                        self.decrypt_pcbc(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Ofb => {
                        self.decrypt_ofb(&mut bytes, self.iv.to_be_bytes())
                    }
                    crate::digital::block_ciphers::block_cipher::BCMode::Cfb => {
                        self.decrypt_cfb(&mut bytes, self.iv.to_be_bytes())
                    }
                };

                // Remove the appropriate kind and amount of padding
                if self.mode.padded() {
                    self.padding.strip_padding(&mut bytes, $blocksize)?;
                }

                Ok(self.output_format.byte_slice_to_text(&bytes))
            }
        }
    };
}

#[macro_export]
macro_rules! test_block_cipher {
    ($( $name: ident, $cipher: expr, $ptext: expr, $ctext: expr);+ $(;)?) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            $(
                #[test]
                fn $name() {
                    let mut msg = $ptext;
                    $cipher.encrypt_block(&mut msg);
                    assert!($ctext == msg, "encrypt failed:\n correct: {:02x?}\n   ctext: {:02x?}", $ctext, msg);
                    $cipher.decrypt_block(&mut msg);
                    assert!($ptext == msg, "decrypt failed:\n correct: {:02x?}\n   ptext: {:02x?}", $ptext, msg);
                }
            )+
        }
    }
}
