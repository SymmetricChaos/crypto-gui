use crate::CipherError;
use std::fmt::Display;
use strum::EnumIter;
use utils::{
    byte_formatting::{overwrite_bytes, xor_into_bytes},
    math_functions::incr_array_ctr,
    padding::{
        ansi923_padding, bit_padding, none_padding, pkcs5_padding, strip_ansi923_padding,
        strip_bit_padding, strip_none_padding, strip_pkcs5_padding,
    },
};

#[macro_export]
macro_rules! block_cipher_builders {
    ($name: ident) => {
        impl $name {
            pub fn input(mut self, input: utils::byte_formatting::ByteFormat) -> Self {
                self.input_format = input;
                self
            }

            pub fn output(mut self, output: utils::byte_formatting::ByteFormat) -> Self {
                self.output_format = output;
                self
            }

            pub fn padding(
                mut self,
                padding: crate::digital::block_ciphers::block_cipher::BCPadding,
            ) -> Self {
                self.padding = padding;
                self
            }

            pub fn mode(
                mut self,
                mode: crate::digital::block_ciphers::block_cipher::BCMode,
            ) -> Self {
                self.mode = mode;
                self
            }
        }
    };
}

pub trait BlockCipher<const N: usize> {
    fn encrypt_block(&self, bytes: &mut [u8]);
    fn decrypt_block(&self, bytes: &mut [u8]);

    fn encrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % N == 0);

        for ptext in bytes.chunks_mut(N) {
            self.encrypt_block(ptext);
        }
    }
    fn decrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % N == 0);

        for ctext in bytes.chunks_mut(N) {
            self.decrypt_block(ctext);
        }
    }

    fn encrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; N]) {
        let mut ctr = ctr;

        for ptext in bytes.chunks_mut(N) {
            // Encrypt the counter to create a mask
            let mut mask = ctr;
            self.encrypt_block(&mut mask);

            // XOR the mask into the plaintext at the source, creating ciphertext
            xor_into_bytes(ptext, &mask);

            incr_array_ctr(&mut ctr);
        }
    }

    // CTR mode is reciprocal
    fn decrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; N]) {
        self.encrypt_ctr(bytes, ctr)
    }

    fn encrypt_cbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for ptext in bytes.chunks_mut(N) {
            // XOR the plaintext into the previous ciphertext (or the IV), creating a mixed array
            xor_into_bytes(&mut chain, &ptext);

            // Encrypt the mixed value, producing ciphertext
            self.encrypt_block(&mut chain);

            // Overwrite plaintext at source with the ciphertext
            overwrite_bytes(ptext, &chain);
        }
    }

    fn decrypt_cbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for ctext in bytes.chunks_mut(N) {
            // Decrypt the ciphertext at the source to get the plaintext XORed with the previous chain value
            let mut mixed = ctext.to_vec();
            self.decrypt_block(&mut mixed);

            // XOR the current chain value into the mixed text
            xor_into_bytes(&mut mixed, &chain);

            // Store the ciphertext as the next chain value before it gets overwritten
            chain = ctext.try_into().unwrap();

            // The overwrite ciphertext at source with the plaintext
            overwrite_bytes(ctext, &mixed);
        }
    }

    fn encrypt_pcbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for ptext in bytes.chunks_mut(N) {
            // Save the plaintext
            let saved_ptext = ptext.to_vec();

            // XOR the plaintext into the previous ciphertext (or the IV), creating a mixed array
            xor_into_bytes(&mut chain, &saved_ptext);

            // Encrypt the mixed value, producing ciphertext
            self.encrypt_block(&mut chain);

            // Overwrite plaintext at source with the ciphertext
            overwrite_bytes(ptext, &chain);

            // XOR the plaintext into the chain (yes, again)
            xor_into_bytes(&mut chain, &saved_ptext);
        }
    }

    fn decrypt_pcbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for ctext in bytes.chunks_mut(N) {
            // Save the ciphertext
            let saved_ctext = ctext.to_vec();

            // Decrypt the ciphertext at the source to get the plaintext XORed with the previous chain value
            let mut mixed = ctext.to_vec();
            self.decrypt_block(&mut mixed);

            // XOR the current chain value into the mixed text, making it plaintext
            xor_into_bytes(&mut mixed, &chain);

            // XOR the mixed text (now plaintext) with the chain
            xor_into_bytes(&mut chain, &mixed);

            // The overwrite ciphertext at source with the plaintext
            overwrite_bytes(ctext, &mixed);

            // XOR the plaintext into the chain
            xor_into_bytes(&mut chain, &saved_ctext);
        }
    }

    fn encrypt_ofb(&self, bytes: &mut [u8], iv: [u8; N]) {
        let mut chain = iv;

        for ptext in bytes.chunks_mut(N) {
            // Encrypt the chain to create a mask
            self.encrypt_block(&mut chain);

            // XOR the mask into the plaintext at the source, creating ciphertext
            xor_into_bytes(ptext, &chain);
        }
    }

    // OFB is reciprocal
    fn decrypt_ofb(&self, bytes: &mut [u8], iv: [u8; N]) {
        self.encrypt_ofb(bytes, iv)
    }

    fn encrypt_cfb(&self, bytes: &mut [u8], iv: [u8; N]) {
        let mut chain = iv;

        for ptext in bytes.chunks_mut(N) {
            // Encrypt the chain to create a mask
            self.encrypt_block(&mut chain);

            // XOR the mask into the plaintext at the source, creating ciphertext
            xor_into_bytes(ptext, &chain);

            // The ptext has had the keystream XORed into it and is now the ciphertext
            overwrite_bytes(&mut chain, &ptext)
        }
    }

    // CFB is reciprocal
    fn decrypt_cfb(&self, bytes: &mut [u8], iv: [u8; N]) {
        self.encrypt_cfb(bytes, iv)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum BCMode {
    Cbc,
    Ctr,
    Ecb,
    Pcbc,
    Ofb,
    Cfb,
}

impl BCMode {
    /// Is a padding rule needed?
    pub fn padded(&self) -> bool {
        match self {
            BCMode::Ecb => true,
            BCMode::Ctr => false,
            BCMode::Cbc => true,
            BCMode::Pcbc => true,
            BCMode::Ofb => false,
            BCMode::Cfb => false,
        }
    }

    pub fn iv_needed(&self) -> bool {
        match self {
            BCMode::Ecb => false,
            BCMode::Ctr => true,
            BCMode::Cbc => true,
            BCMode::Pcbc => true,
            BCMode::Ofb => true,
            BCMode::Cfb => true,
        }
    }

    pub fn info(&self) -> &'static str {
        match self {
            BCMode::Cbc => "Cipher Block Chaining XORs information from the ciphertext into the plaintext of the block that comes after it before encryption with the block function. This ensures that even identical blocks of plaintext are encrypted differently. The first block requires an initialization vector that should not be repeated for different messages with the same key. Encryption in inherently sequential but decryption can be performed parallel.",
            BCMode::Ctr => "Counter mode operates the block cipher as if it were a stream cipher or secure PRNG. Rather than encrypting the plaintext directly the cipher is used to encrypt a sequence of numbers and the result is XORed with the plaintext. The it is important that the counter never repeat for two messages with the same key so steps must be taken to carefully select its initial value. Encryption and decryption can be performed in parallel.",
            BCMode::Ecb => "Eelectronic Code Book mode encrypts each block of plaintext directly with the cipher. This is the simplest but least secure way to operate a block cipher and not recommended for use in any circumstance. If two blocks are the same they will be encrypted exactly the same way, exposing information about the plaintext. Encryption and decryption can be performed in parallel.",
            BCMode::Pcbc => "Propogating Cipher Block Chaining is similar to CBC but XORs the plaintext into the chain value both before and after encryption. This means that both encryption and decryption are inherently serial and that corruption in any block corrupts all following blocks.",
            BCMode::Ofb => "Output Feedback mode iteratively encrypts the initialization vector and XORs the chain of blocks created into the plaintext. This is similar to CTR mode but cannot be encrypted or decrypted in parallel.",
            BCMode::Cfb => "Cipher Feedback mode encrypts the previous ciphertext block and XORs that into the plaintext. Encryption cannot be parallelized but decryption can be.",
        }
    }
}

impl Default for BCMode {
    fn default() -> Self {
        BCMode::Ecb
    }
}

impl Display for BCMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BCMode::Cbc => write!(f, "CBC"),
            BCMode::Ctr => write!(f, "CTR"),
            BCMode::Ecb => write!(f, "ECB"),
            BCMode::Pcbc => write!(f, "PCBC"),
            BCMode::Ofb => write!(f, "OFB"),
            BCMode::Cfb => write!(f, "CFB"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum BCPadding {
    None,
    Bit,
    Pkcs,
    Ansi923,
}

impl Default for BCPadding {
    fn default() -> Self {
        BCPadding::Bit
    }
}

impl BCPadding {
    pub fn add_padding(&self, bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
        match self {
            BCPadding::None => none_padding(bytes, block_size),
            BCPadding::Bit => bit_padding(bytes, block_size),
            BCPadding::Pkcs => pkcs5_padding(bytes, block_size),
            BCPadding::Ansi923 => ansi923_padding(bytes, block_size),
        }
        .map_err(|e| CipherError::General(e.to_string()))
    }

    pub fn strip_padding(&self, bytes: &mut Vec<u8>, block_size: u32) -> Result<(), CipherError> {
        match self {
            BCPadding::None => strip_none_padding(bytes, block_size),
            BCPadding::Bit => strip_bit_padding(bytes),
            BCPadding::Pkcs => strip_pkcs5_padding(bytes),
            BCPadding::Ansi923 => strip_ansi923_padding(bytes),
        }
        .map_err(|e| CipherError::General(e.to_string()))
    }

    pub fn info(&self) -> &'static str {
        match self {
            BCPadding::None => "If no padding is used the length of the input to the cipher must be a multiple of the block size.",
            BCPadding::Bit => "Bit padding adds the byte 0b10000000 (or 0x80) to the end of the input and then fills the rest with null bytes to reach a multiple of the block size.",
            BCPadding::Pkcs => "PKCS5 padding adds n bytes each with value n to reach a multiple of the block size.",
            BCPadding::Ansi923 => "ANSI X9.23 padding adds n-1 null bytes and then a final byte with a value of n to reach a multiple of the block size.",
        }
    }
}

impl Display for BCPadding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Bit => write!(f, "Bit"),
            Self::Pkcs => write!(f, "PKCS5"),
            Self::Ansi923 => write!(f, "ANSI X9.23"),
        }
    }
}
