use crate::errors::HasherError;

pub trait ClassicHasher {
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
    fn hash_bytes_from_string(&self, text: &str) -> Result<String, HasherError>;
}

pub trait StatefulHasher {
    // Update the hasher's state with some bytes.
    fn update(&mut self, bytes: &[u8]);

    // Finalize the hash with any padding and processing of final blocks then output bytes. Consumes the hasher so it cannot be reused.
    fn finalize(self) -> Vec<u8>;

    // Simultaneously update and finalize.
    fn update_multiple(&mut self, bytes: &[&[u8]]);

    // Simultaneously update and finalize.
    fn hash(self, bytes: &[u8]) -> Vec<u8>;

    // Hash multiple inputs
    fn hash_multiple(self, bytes: &[&[u8]]) -> Vec<u8>;
}

#[macro_export]
macro_rules! stateful_hash_helpers {
    () => {
        fn update_multiple(&mut self, bytes: &[&[u8]]) {
            for b in bytes {
                self.update(b);
            }
        }

        fn hash(mut self, bytes: &[u8]) -> Vec<u8> {
            self.update(bytes);
            self.finalize()
        }

        fn hash_multiple(mut self, bytes: &[&[u8]]) -> Vec<u8> {
            for b in bytes {
                self.update(b)
            }
            self.finalize()
        }
    };
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

#[macro_export]
macro_rules! stateful_hash_tests {
    ($($test_name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
        #[cfg(test)]
        mod stateful_tests {
        use super::*;
        $(
            #[test]
            fn $test_name() {
                let a = utils::byte_formatting::hex_to_bytes_ltr($output).unwrap();
                let b = $hasher.hash($input);
                if a != b {
                    panic!("hash did not match test value\nexpected:   {:02x?}\ncalculated  {:02x?}", a,b)
                }
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
                let a = utils::byte_formatting::hex_to_bytes_ltr($output).unwrap();
                let b = $hasher.hash($input);
                if a != b {
                    panic!("hash did not match test value\nexpected:   {:02x?}\ncalculated  {:02x?}", a,b)
                }
            }
        )+
        }
    };
}

#[macro_export]
macro_rules! incremental_hash_tests {
    ($($test_name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
        #[cfg(test)]
        mod stateful_incremental_tests {
        use super::*;
        $(
            #[test]
            fn $test_name() {
                assert_eq!(utils::byte_formatting::hex_to_bytes_ltr($output).unwrap(), $hasher.hash_multiple($input));
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
            fn $test_name() {
                $hasher.hash_multiple($input);
                assert_eq!(utils::byte_formatting::hex_to_bytes_ltr($output).unwrap(), $hasher.hash_multiple($input));
            }
        )+
        }
    };
}
