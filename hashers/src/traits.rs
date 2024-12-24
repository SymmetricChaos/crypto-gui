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
    fn hash(self, bytes: &[u8]) -> Vec<u8>;
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
                assert_eq!(utils::byte_formatting::hex_to_bytes_ltr($output).unwrap(), $hasher.hash($input));
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
                assert_eq!(utils::byte_formatting::hex_to_bytes_ltr($output).unwrap(), $hasher.hash($input));
            }
        )+
        }
    };
}

// The update doesn't work for some reason
// #[macro_export]
// macro_rules! incremental_hash_tests {
//     ($($test_name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
//         #[cfg(test)]
//         mod stateful_incremental_tests {
//         use super::*;
//         $(
//             #[test]
//             fn $test_name() {
//                 for partial in $input {
//                     $hasher.update(&partial);
//                     println!("state {:02x?}", $hasher.state_bytes());
//                 }
//                 assert_eq!(utils::byte_formatting::hex_to_bytes_ltr($output).unwrap(), $hasher.finalize());
//             }
//         )+
//         }
//     };
//     // Optional variant with module name for separation
//     (($mod_name: ident)?; $($name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
//         #[cfg(test)]
//         mod $mod_name {
//         use super::*;
//         $(
//             #[test]
//             fn $test_name() {
//                 for partial in input {
//                     $hasher.update(partial)
//                 }
//                 assert_eq!(utils::byte_formatting::hex_to_bytes_ltr($output).unwrap(), $hasher.finalize());
//             }
//         )+
//         }
//     };
// }
