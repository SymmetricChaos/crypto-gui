pub mod aes;
pub mod ascon;
pub mod block_cipher;
pub mod blowfish;
pub mod camellia;
pub mod des;
pub mod fealnx;
pub mod gost;
pub mod idea;
pub mod lea;
pub mod rc5;
pub mod rc6;
pub mod seed;
pub mod simon;
pub mod sm4;
pub mod speck;
pub mod tea;
pub mod twofish;

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
                    } // crate::digital::block_ciphers::block_cipher::BCMode::Gcm => {
                      //     self.encrypt_gcm(&mut bytes, self.iv.to_be_bytes())
                      // }
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
                    } // crate::digital::block_ciphers::block_cipher::BCMode::Gcm => {
                      //     self.decrypt_gcm(&mut bytes, self.iv.to_be_bytes())
                      // }
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
    ($($cipher: expr, $name: ident, $ptext: expr, $ctext: expr);+ $(;)?) => {
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
