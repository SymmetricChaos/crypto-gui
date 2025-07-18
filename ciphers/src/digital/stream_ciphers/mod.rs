pub mod a51;
pub mod a52;
pub mod aes_gcm;
pub mod chacha;
pub mod e0;
pub mod fish;
pub mod hc128;
pub mod hc256;
pub mod isaac;
pub mod lfsr;
pub mod rabbit;
pub mod rc4;
pub mod salsa20;
pub mod seal;
pub mod snow;
pub mod wake;

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
