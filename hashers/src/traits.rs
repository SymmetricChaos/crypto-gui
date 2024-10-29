use crate::errors::HasherError;

pub trait ClassicHasher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError>;
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

#[macro_export]
macro_rules! basic_hash_tests {
    ($($test_name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
        #[cfg(test)]
        mod basic_tests {
        use super::*;
        $(
            #[test]
            fn $test_name() {
                assert_eq!($output, $hasher.hash_bytes_from_string($input).unwrap());
            }
        )+
        }
    };
    // Optional variant with module name for separation
    (($mod_name: ident)?; $($name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
        #[cfg(test)]
        mod $mod_name {
        use super::*;
        $(
            #[test]
            fn $name() {
                assert_eq!($output, $hasher.hash_bytes_from_string($input).unwrap());
            }
        )+
        }
    };
}
