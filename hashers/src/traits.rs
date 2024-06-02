use crate::errors::HasherError;

pub trait ClassicHasher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError>;
}

pub trait KeyedHasher: ClassicHasher {
    fn set_salt(&mut self, bytes: &[u8]);
    fn set_key(&mut self, bytes: &[u8]);
}
