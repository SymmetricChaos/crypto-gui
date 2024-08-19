pub mod aes;
pub mod block_cipher;
pub mod blowfish;
pub mod des;
pub mod feal;
pub mod gost;
pub mod idea;
pub mod rc5;
pub mod rc6;
pub mod tea;

#[macro_export]
macro_rules! impl_cipher_for_block_cipher {
    ($cipher: ty, $blocksize: literal) => {
        impl crate::traits::Cipher for $cipher {
            fn encrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|_| crate::errors::CipherError::input("byte format error"))?;

                if self.mode.padded() {
                    self.padding.add_padding(&mut bytes, $blocksize)?;
                }

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
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|_| crate::errors::CipherError::input("byte format error"))?;

                if self.mode.padded() {
                    if self.padding == crate::digital::block_ciphers::block_cipher::BCPadding::None
                    {
                        utils::padding::none_padding(&mut bytes, $blocksize)
                            .map_err(|e| crate::errors::CipherError::General(e.to_string()))?
                    };
                }

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

                if self.mode.padded() {
                    self.padding.strip_padding(&mut bytes, $blocksize)?;
                }

                Ok(self.output_format.byte_slice_to_text(&bytes))
            }
        }
    };
}
