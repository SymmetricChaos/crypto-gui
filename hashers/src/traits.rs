use crate::errors::HasherError;

pub trait ClassicHasher {
    // const PUBLICATION_DATE: &'static str;
    // const AUTHORS: &'static str;
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError>;
}

pub trait KeyedHasher: ClassicHasher {
    fn set_salt(&mut self, bytes: &[u8]);
    fn set_key(&mut self, bytes: &[u8]);
}

#[macro_export]
macro_rules! hash_bytes_from_string {
    () => {
        fn hash_bytes_from_string(&self, text: &str) -> Result<String, crate::errors::HasherError> {
            let mut bytes = self
                .input_format
                .text_to_bytes(text)
                .map_err(|_| crate::errors::HasherError::general("byte format error"))?;
            let out = self.hash(&mut bytes);
            Ok(self.output_format.byte_slice_to_text(&out))
        }
    };
}
