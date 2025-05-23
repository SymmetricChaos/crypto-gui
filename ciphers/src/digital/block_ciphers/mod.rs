pub mod aes;
pub mod aria;
pub mod ascon;
pub mod block_cipher;
pub mod blowfish;
pub mod camellia;
pub mod des;
pub mod e2;
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
pub mod skipjack;
pub mod sm4;
pub mod speck;
pub mod tea;
pub mod threefish;
pub mod twofish;

// This Big Scary Macro is just avoiding a lot of boilerplate since all block ciphers have essentially the same
// implementation for the Cipher trait. Fully qualified names are used to avoid import conflicts.
#[macro_export]
macro_rules! impl_cipher_for_block_cipher {
    ($cipher: ty, $blocksize: literal) => {
        impl crate::traits::Cipher for $cipher {
            fn encrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
                use crate::digital::block_ciphers::block_cipher::BlockCipher;
                // Interpret the input
                let mut bytes = self
                    .input_format
                    .text_to_bytes(text)
                    .map_err(|e| crate::errors::CipherError::Input(e.to_string()))?;

                self.encrypt_bytes(&mut bytes);

                Ok(self.output_format.byte_slice_to_text(&bytes))
            }

            fn decrypt(&self, text: &str) -> Result<String, crate::errors::CipherError> {
                use crate::digital::block_ciphers::block_cipher::BlockCipher;
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
                self.decrypt_bytes(&mut bytes);

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
            use crate::digital::block_ciphers::block_cipher::BlockCipher;
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

#[macro_export]
macro_rules! test_block_cipher_str {
    ($( $name: ident, $cipher: expr, $ptext: expr, $ctext: expr);+ $(;)?) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::digital::block_ciphers::block_cipher::BlockCipher;
            $(
                #[test]
                fn $name() {
                    let mut msg = hex_literal::hex!($ptext);
                    $cipher.encrypt_block(&mut msg);
                    assert!(hex_literal::hex!($ctext) == msg, "encrypt failed:\n correct: {:02x?}\n   ctext: {:02x?}", hex_literal::hex!($ctext), msg);
                    $cipher.decrypt_block(&mut msg);
                    assert!(hex_literal::hex!($ptext) == msg, "decrypt failed:\n correct: {:02x?}\n   ptext: {:02x?}", hex_literal::hex!($ptext), msg);
                }
            )+
        }
    }
}
