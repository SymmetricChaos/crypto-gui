pub mod a51;
pub mod a52;
pub mod aes_gcm;
pub mod chacha;
pub mod fish;
pub mod isaac;
pub mod lfsr_copy;
pub mod rc4;
pub mod salsa20;
pub mod seal;
pub mod snow;

#[macro_export]
macro_rules! impl_cipher_for_stream_cipher {
    ($name:ident) => {
        impl crate::Cipher for $name {
            fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|e| crate::CipherError::Input(e.to_string()))?;
                self.encrypt_bytes(&mut bytes);
                Ok(self.output_format.byte_slice_to_text(&bytes))
            }

            fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
                self.encrypt(text)
            }
        }
    };
}
