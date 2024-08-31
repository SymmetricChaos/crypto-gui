pub mod aes;
pub mod block_cipher;
pub mod blowfish;
pub mod des;
pub mod feal;
pub mod gost;
pub mod idea;
pub mod lea;
pub mod rc5;
pub mod rc6;
pub mod seed;
pub mod tea;

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
                    .map_err(|_| crate::errors::CipherError::input("byte format error"))?;

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
                    .map_err(|_| crate::errors::CipherError::input("byte format error"))?;

                // If no padding is used check for an error
                if self.mode.padded() {
                    if self.padding == crate::digital::block_ciphers::block_cipher::BCPadding::None
                    {
                        utils::padding::none_padding(&mut bytes, $blocksize)
                            .map_err(|e| crate::errors::CipherError::General(e.to_string()))?
                    };
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
    ($($cipher: expr, $name: ident, $ptext: expr, $ctext: expr);+ $(;)?) => {
        #[cfg(test)]
        mod block_cipher_tests {
            use super::*;
            $(
                #[test]
                fn $name() {
                    let mut msg = $ptext;
                    $cipher.encrypt_block(&mut msg);
                    assert_eq!($ctext, msg, "encrypt failed");
                    $cipher.decrypt_block(&mut msg);
                    assert_eq!($ptext, msg, "decrypt failed");
                }
            )+
        }
    }
}
