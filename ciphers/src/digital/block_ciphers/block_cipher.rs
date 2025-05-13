use crate::CipherError;
use std::fmt::Display;
use strum::EnumIter;
use utils::{
    byte_formatting::{overwrite_bytes, xor_into_bytes},
    math_functions::incr_array_ctr_be,
    padding::{
        ansi923_padding, bit_padding, none_padding, pkcs5_padding, strip_ansi923_padding,
        strip_bit_padding, strip_none_padding, strip_pkcs5_padding,
    },
};

#[macro_export]
macro_rules! block_cipher_builders {
    ($name: ident, $iv_type: ty) => {
        impl $name {
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

            pub fn iv(mut self, iv: $iv_type) -> Self {
                self.iv = iv;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! block_cipher_getters {
    () => {
        fn get_padding(&self) -> crate::digital::block_ciphers::block_cipher::BCPadding {
            self.padding
        }

        fn get_mode(&self) -> crate::digital::block_ciphers::block_cipher::BCMode {
            self.mode
        }

        fn get_iv_be(&self) -> Vec<u8> {
            self.iv.to_be_bytes().to_vec()
        }

        fn get_iv_le(&self) -> Vec<u8> {
            self.iv.to_le_bytes().to_vec()
        }
    };
}

pub trait BlockCipher<const N: usize> {
    /// Use the block function to encrypt a single block of bytes.
    fn encrypt_block(&self, bytes: &mut [u8]);

    /// Use the block function to decrypt a single block of bytes.
    fn decrypt_block(&self, bytes: &mut [u8]);

    fn get_padding(&self) -> BCPadding;
    fn get_mode(&self) -> BCMode;
    fn get_iv_be(&self) -> Vec<u8>;
    fn get_iv_le(&self) -> Vec<u8>;

    /// Given some bytes apply padding and encrypt
    fn encrypt_bytes(&self, bytes: &mut Vec<u8>) {
        if self.get_mode().padded() {
            self.get_padding()
                .add_padding(bytes, N as u32)
                .expect("error applying padding");
        }
        match self.get_mode() {
            crate::digital::block_ciphers::block_cipher::BCMode::Ecb => self.encrypt_ecb(bytes),
            crate::digital::block_ciphers::block_cipher::BCMode::Ctr => {
                self.encrypt_ctr(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Cbc => {
                self.encrypt_cbc(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Pcbc => {
                self.encrypt_pcbc(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Ofb => {
                self.encrypt_ofb(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Cfb => {
                self.encrypt_cfb(bytes, self.get_iv_be().try_into().unwrap())
            }
        };
    }

    /// Given bytes decrypt them in-place, removing padding
    fn decrypt_bytes(&self, bytes: &mut Vec<u8>) {
        match self.get_mode() {
            crate::digital::block_ciphers::block_cipher::BCMode::Ecb => self.decrypt_ecb(bytes),
            crate::digital::block_ciphers::block_cipher::BCMode::Ctr => {
                self.decrypt_ctr(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Cbc => {
                self.decrypt_cbc(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Pcbc => {
                self.decrypt_pcbc(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Ofb => {
                self.decrypt_ofb(bytes, self.get_iv_be().try_into().unwrap())
            }
            crate::digital::block_ciphers::block_cipher::BCMode::Cfb => {
                self.decrypt_cfb(bytes, self.get_iv_be().try_into().unwrap())
            }
        };
        if self.get_mode().padded() {
            self.get_padding()
                .strip_padding(bytes, N as u32)
                .expect("error removing padding");
        }
    }

    /// Encrypt in Electronic Code Book Mode
    fn encrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % N == 0);

        for ptext in bytes.chunks_mut(N) {
            self.encrypt_block(ptext);
        }
    }
    /// Decrypt in Electronic Code Book Mode
    fn decrypt_ecb(&self, bytes: &mut [u8]) {
        assert!(bytes.len() % N == 0);

        for ctext in bytes.chunks_mut(N) {
            self.decrypt_block(ctext);
        }
    }

    /// Encrypt in Counter Mode
    fn encrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; N]) {
        let mut ctr = ctr;

        for ptext in bytes.chunks_mut(N) {
            // Encrypt the counter to create a mask
            let mut mask = ctr;
            self.encrypt_block(&mut mask);

            // XOR the mask into the plaintext at the source, creating ciphertext
            xor_into_bytes(ptext, &mask);

            // Step the counter
            incr_array_ctr_be(&mut ctr);
        }
    }

    /// Decrypt in Counter Mode (equivalent to encrypt)
    fn decrypt_ctr(&self, bytes: &mut [u8], ctr: [u8; N]) {
        self.encrypt_ctr(bytes, ctr)
    }

    /// Encrypt in Cipher Block Chaining Mode
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

    /// Decrypt in Cipher Block Chaining Mode
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

    /// Encrypt in Propogating Cipher Block Chaining Mode
    fn encrypt_pcbc(&self, bytes: &mut [u8], iv: [u8; N]) {
        assert!(bytes.len() % N == 0);

        // Start chain with an IV
        let mut chain = iv;

        for ptext in bytes.chunks_mut(N) {
            // Save the plaintext
            let saved_ptext = ptext.to_vec();

            // XOR the plaintext into the chain, creating a mixed array
            xor_into_bytes(&mut chain, &ptext);

            // Encrypt the mixed value, producing ciphertext
            self.encrypt_block(&mut chain);

            // Overwrite plaintext at source with the ciphertext
            overwrite_bytes(ptext, &chain);

            // XOR the plaintext into the chain (yes, again)
            xor_into_bytes(&mut chain, &saved_ptext);
        }
    }

    /// Decrypt in Propogating Cipher Block Chaining Mode
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

    /// Encrypt in Output Feedback Mode
    fn encrypt_ofb(&self, bytes: &mut [u8], iv: [u8; N]) {
        let mut chain = iv;

        for ptext in bytes.chunks_mut(N) {
            // Encrypt the chain to create a mask
            self.encrypt_block(&mut chain);

            // XOR the mask into the plaintext at the source, creating ciphertext
            xor_into_bytes(ptext, &chain);
        }
    }

    /// Decrypt in Output Feedback Mode (equvalent to encrypt)
    fn decrypt_ofb(&self, bytes: &mut [u8], iv: [u8; N]) {
        self.encrypt_ofb(bytes, iv)
    }

    /// Encrypt in Cipher Feedback Mode
    fn encrypt_cfb(&self, bytes: &mut [u8], iv: [u8; N]) {
        let mut chain = iv;

        for mut ptext in bytes.chunks_mut(N) {
            // Encrypt the chain to create a mask
            self.encrypt_block(&mut chain);

            // XOR the mask into the plaintext at the source, creating ciphertext
            xor_into_bytes(&mut ptext, &chain);

            // The ptext has had the keystream XORed into it and is now the ciphertext
            overwrite_bytes(&mut chain, &ptext)
        }
    }

    /// Decrypt in Cipher Feedback Mode (equivalent to encrypt)
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
            BCMode::Cbc => "Cipher Block Chaining Mode works by XORing information from the ciphertext into the plaintext of the block that comes after it, before encryption with the block function. This ensures that even identical blocks of plaintext are encrypted differently. The first block requires an initialization vector that should not be repeated for different messages with the same key. Encryption in inherently sequential but decryption can be performed parallel.",
            BCMode::Ctr => "Counter Mode operates the block cipher as if it were a stream cipher or secure PRNG. Rather than encrypting the plaintext directly the cipher is used to encrypt a sequence of numbers and the result is XORed with the plaintext. The it is important that the counter never repeat for two messages with the same key so steps must be taken to carefully select its initial value. Encryption and decryption can be performed in parallel.",
            BCMode::Ecb => "Eelectronic Code Book Mode encrypts each block of plaintext directly with the cipher. This is the simplest but least secure way to operate a block cipher and not recommended for use in any circumstance. If two blocks are the same they will be encrypted exactly the same way, exposing information about the plaintext. Encryption and decryption can be performed in parallel.",
            BCMode::Pcbc => "Propogating Cipher Block Chaining Mode is similar to CBC but XORs the plaintext into the chain value both before and after encryption. This means that both encryption and decryption are inherently serial and that corruption in any block corrupts all following blocks.",
            BCMode::Ofb => "Output Feedback Mode iteratively encrypts the initialization vector and XORs the chain of blocks created into the plaintext. This is similar to CTR mode but cannot be encrypted or decrypted in parallel.",
            BCMode::Cfb => "Cipher Feedback Mode encrypts the previous ciphertext block and XORs that into the plaintext. Encryption cannot be parallelized but decryption can be.",
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
            BCPadding::Bit => "Bit padding adds the byte 0x80 (0b10000000) to the end of the input and then fills the rest with null bytes to reach a multiple of the block size.",
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

#[cfg(test)]
#[test]
fn aes_test_modes() {
    use crate::digital::block_ciphers::aes::aes::Aes256;
    use hex_literal::hex;

    // Multiblock ECB test
    let cipher = Aes256::default().with_key(hex!(
        "ba42b760bb5a5de21acb9aba214c9783cd71ea841ada018580abc4e1be3b76dd"
    ));
    let mut ptext = hex!("4b4b12d6ee6fc0bf987eaafe2634aad464781ff4c83d3f8a61a6af7c0a6d51f0e3855d0e02feb307652a6f562bfebe4604baf1b4e7cdd01603f231bcf7a0c95645a141b704008cd8d62979201a4c84e2");
    let ctext = hex!("fa18d25e37ea0ce94f0949efc0edecc6a40fada8f007fd8e760afed0a83ebb350c82b03baaa6ee19f791bb9bd1b44d27a76fc6eb0e1c0017d68776ed69a541851a732e46ef328def064baf6a0a755588");
    cipher.encrypt_ecb(&mut ptext);
    assert_eq!(ctext, ptext);

    // Multiblock OFB test
    let cipher = Aes256::default()
        .iv(0x0e28bd0603b31c26250345a118408ffc)
        .with_key(hex!(
            "318aa7c73006ff95840f17f2b9cf01fe7f031105ff01daa66ff95834e47b6f5c"
        ));
    let mut ptext = hex!("257f3fc84537158b68c8af111b1e9eb41f8841686ab1e94c6fd13a7f9f24d535309c340a1dd3d4966e439a41b9b97058e9072f613ef9c1ac958b872bea59f8831b578b63eec2d7155657f953f2c2375b");
    let ctext = hex!("ba4ebcdc894e6de54f8f1d7ccbb19e13d2ae0ca66c05c10e2f90bad2e9b8db94ee7770c3557927029d49fd2b3f80a01025af0e7a343237fb625dbdee85367ddfbd7f6664b511cdc7e832b2c4d91f1c0e");
    cipher.encrypt_ofb(&mut ptext, cipher.iv.to_be_bytes());
    assert_eq!(ctext, ptext);

    // Multiblock CBC test
    let cipher = Aes256::default()
        .iv(0x11958dc6ab81e1c7f01631e9944e620f)
        .with_key(hex!(
            "9adc8fbd506e032af7fa20cf5343719de6d1288c158c63d6878aaf64ce26ca85"
        ));
    let mut ptext = hex!("c7917f84f747cd8c4b4fedc2219bdbc5f4d07588389d8248854cf2c2f89667a2d7bcf53e73d32684535f42318e24cd45793950b3825e5d5c5c8fcd3e5dda4ce9246d18337ef3052d8b21c5561c8b660e");
    let ctext = hex!("9c99e68236bb2e929db1089c7750f1b356d39ab9d0c40c3e2f05108ae9d0c30b04832ccdbdc08ebfa426b7f5efde986ed05784ce368193bb3699bc691065ac62e258b9aa4cc557e2b45b49ce05511e65");
    cipher.encrypt_cbc(&mut ptext, cipher.iv.to_be_bytes());
    assert_eq!(ctext, ptext);

    // Multiblock CTR test
    let cipher = Aes256::default()
        .iv(0xf0f1f2f3f4f5f6f7f8f9fafbfcfdfeff)
        .with_key(hex!(
            "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4"
        ));
    let mut ptext = hex!("6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710");
    let ctext = hex!("601ec313775789a5b7a7f504bbf3d228f443e3ca4d62b59aca84e990cacaf5c52b0930daa23de94ce87017ba2d84988ddfc9c58db67aada613c2dd08457941a6");
    cipher.encrypt_ctr(&mut ptext, cipher.iv.to_be_bytes());
    assert_eq!(ctext, ptext);

    // todo!("need to find test vectors for PCBC and CFB but all others are passing")
}
