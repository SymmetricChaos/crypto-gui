use crate::{errors::HasherError, md2::Md2, md5::Md5, sha::Sha1, traits::ClassicHasher};
use strum::{Display, EnumIter, VariantNames};
use utils::byte_formatting::ByteFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, VariantNames)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Pbkdf1Variant {
    Md2,
    Md5,
    Sha1,
}

pub struct Pbkdf1 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub variant: Pbkdf1Variant,
    pub salt: [u8; 8],
    pub iterations: u32,
    pub hash_len: u32, // size of the output in bytes
}

impl Default for Pbkdf1 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            salt: [0; 8],
            variant: Pbkdf1Variant::Md5,
            iterations: 4096,
            hash_len: 32,
        }
    }
}

impl Pbkdf1 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn variant(mut self, variant: Pbkdf1Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn salt(mut self, salt: [u8; 8]) -> Self {
        self.salt = salt;
        self
    }

    pub fn iterations(mut self, iterations: u32) -> Self {
        assert!(self.iterations > 0);
        self.iterations = iterations;
        self
    }

    /// For MD2 and MD5 hash_len is limited to 16. For SHA-1 hash_len is limited to 20.
    pub fn hash_len(mut self, hash_len: u32) -> Self {
        assert!(hash_len > 0);
        self.hash_len = hash_len;
        self
    }

    pub fn salt_from_str(mut self, format: ByteFormat, salt_str: &str) -> Self {
        let bytes = format
            .text_to_bytes(salt_str)
            .expect("byte format error")
            .try_into()
            .expect("could not convert to [u8; 8]");
        self.salt = bytes;
        self
    }

    // For changing the key interactively
    pub fn set_salt(&mut self, salt: [u8; 8]) {
        self.salt = salt;
    }

    // Falliable method for changing the salt from a string interactively
    pub fn set_salt_from_str(
        &mut self,
        format: ByteFormat,
        salt_str: &str,
    ) -> Result<(), HasherError> {
        let bytes = format
            .text_to_bytes(salt_str)
            .map_err(|_| HasherError::general("byte format error"))?
            .try_into()
            .map_err(|_| HasherError::general("could not convert to [u8; 8]"))?;
        self.salt = bytes;
        Ok(())
    }

    pub fn inner_hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self.variant {
            Pbkdf1Variant::Sha1 => Sha1::default().hash(bytes),
            Pbkdf1Variant::Md2 => Md2::default().hash(bytes),
            Pbkdf1Variant::Md5 => Md5::default().hash(bytes),
        }
    }
}

impl ClassicHasher for Pbkdf1 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        assert!(self.iterations > 0);
        assert!(self.hash_len > 0);

        let h: Box<dyn ClassicHasher> = match self.variant {
            Pbkdf1Variant::Sha1 => Box::new(Sha1::default()),
            Pbkdf1Variant::Md2 => Box::new(Md2::default()),
            Pbkdf1Variant::Md5 => Box::new(Md5::default()),
        };

        let mut working_vector = bytes.to_vec();
        working_vector.extend(self.salt);

        for _ in 0..self.iterations {
            working_vector = h.hash(&working_vector);
        }

        working_vector.truncate(self.hash_len as usize);
        working_vector
    }

    crate::hash_bytes_from_string! {}
}

// Wasn't able to find any test vectors for PBKDF1
// crate::basic_hash_tests!(
//     Pbkdf1::default().variant(Pbkdf1Variant::Md5).iterations(1).hash_len(16).salt_from_str(ByteFormat::Utf8, "salt"), test1, "password", "";
// );
