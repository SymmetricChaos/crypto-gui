pub trait SimpleHasher {
    /// Hash some sequence of bytes
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
}

pub trait StatefulHasher {
    /// Update the hasher's state with some bytes.
    fn update(&mut self, bytes: &[u8]);

    /// Finalize the hash with any padding and processing of final blocks then output bytes. Consumes the hasher so it cannot be reused.
    fn finalize(self) -> Vec<u8>;

    /// Update with multiple inputs in the given order.
    fn update_multiple(&mut self, bytes: &[&[u8]]);

    /// Update and then immediately finalize. Consumes the hasher so it cannot be reused.
    fn update_and_finalize(self, bytes: &[u8]) -> Vec<u8>;

    /// Update with multiple inputs in the given order and then finalize.  Consumes the hasher so it cannot be reused.
    fn update_multiple_and_finalize(self, bytes: &[&[u8]]) -> Vec<u8>;
}

pub trait ResettableHasher: StatefulHasher {
    /// Finalize the hash with any padding and processing of final blocks then output bytes. Resets the hasher to its starting state, allowing it to be reused.
    fn finalize_and_reset(&mut self) -> Vec<u8>;

    /// Update, then finalize and reset.
    fn hash_and_reset(&mut self, bytes: &[u8]) -> Vec<u8> {
        self.update(bytes);
        self.finalize_and_reset()
    }

    /// Update with multiple inputs in the given order, then finalize and reset.
    fn hash_multiple_and_reset(&mut self, bytes: &[&[u8]]) -> Vec<u8> {
        self.update_multiple(bytes);
        self.finalize_and_reset()
    }
}

// Use arithmetic to advance reading the input bytes into a buffer
#[macro_export]
macro_rules! take_bytes {
    ($buffer: expr, $bytes: expr, $block_len: expr) => {
        let want = $block_len - $buffer.len();
        let take = std::cmp::min(want, $bytes.len());
        $buffer.extend(&$bytes[..take]);
        $bytes = &$bytes[take..]
    };
}

// Given a buffer, input bytes, a block length, and how to compress
#[macro_export]
macro_rules! compression_routine {
    ($buffer: expr, $bytes: expr, $block_len: expr, $compress: tt) => {
        while !$bytes.is_empty() {
            if $buffer.len() == $block_len {
                $compress
                $buffer.clear();
            }
            let want = $block_len - $buffer.len();
            let take = std::cmp::min(want, $bytes.len());
            $buffer.extend(&$bytes[..take]);
            $bytes = &$bytes[take..]
        }
    };
}

#[macro_export]
macro_rules! stateful_hash_helpers {
    () => {
        fn update_multiple(&mut self, bytes: &[&[u8]]) {
            for b in bytes {
                self.update(b);
            }
        }

        fn update_and_finalize(mut self, bytes: &[u8]) -> Vec<u8> {
            self.update(bytes);
            self.finalize()
        }

        fn update_multiple_and_finalize(mut self, bytes: &[&[u8]]) -> Vec<u8> {
            self.update_multiple(bytes);
            self.finalize()
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
                let a = utils::byte_formatting::hex_to_bytes($output).unwrap();
                let b = $hasher.update_and_finalize($input);
                println!("update_and_finalize");
                if a != b {
                    panic!("hash did not match test value\nexpected:   {:02x?}\ncalculated  {:02x?}", a,b)
                }
            }
        )+
        }
    };
    // Optional variant with module name for separation
    (($mod_name: ident)?; $($test_name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
        #[cfg(test)]
        mod $mod_name {
        use super::*;
        $(
            #[test]
            fn $test_name() {
                let a = utils::byte_formatting::hex_to_bytes($output).unwrap();
                let b = $hasher.update_and_finalize($input);
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
        mod incremental_tests {
        use super::*;
        $(
            #[test]
            fn $test_name() {
                let a = utils::byte_formatting::hex_to_bytes($output).unwrap();
                let b = $hasher.update_multiple_and_finalize($input);
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
            fn $test_name() {
                let a = utils::byte_formatting::hex_to_bytes($output).unwrap();
                let b = $hasher.update_multiple_and_finalize($input);
                if a != b {
                    panic!("hash did not match test value\nexpected:   {:02x?}\ncalculated  {:02x?}", a,b)
                }
            }
        )+
        }
    };
}
