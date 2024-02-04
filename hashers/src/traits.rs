use crate::errors::HasherError;

pub trait ClassicHasher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError>;
}
