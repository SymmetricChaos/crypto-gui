pub trait StatefulHasher {
    /// Update the hasher's state with some bytes.
    fn update(&mut self, bytes: &[u8]);

    /// Finalize the hash with any padding and processing of final blocks then output bytes. Consumes the hasher so it cannot be reused.
    fn finalize(self) -> Vec<u8>;

    /// Update and then immediately finalize. Consumes the hasher so it cannot be reused.
    fn hash(mut self, bytes: &[u8]) -> Vec<u8>
    where
        Self: Sized,
    {
        self.update(bytes);
        self.finalize()
    }
}

pub trait ResettableHasher: StatefulHasher {
    /// Finalize the hash with any padding and processing of final blocks then output bytes. Resets the hasher to its starting state, allowing it to be reused.
    fn finalize_and_reset(&mut self) -> Vec<u8>;

    /// Update then immediately finalize. Resets the hasher to its starting state, allowing it to be reused.
    fn hash_and_reset(&mut self, bytes: &[u8]) -> Vec<u8> {
        self.update(bytes);
        self.finalize_and_reset()
    }
}

// Given a buffer, input bytes, a block length, and how to compress performs the most common routine
// Uses arithmetic to advance reading the input bytes into a buffer (avoids having to allocate all of the input at once)
#[macro_export]
macro_rules! compression_routine {
    ($buffer: expr, $bytes: expr, $block_len: expr, $compress: tt) => {
        // Check if there are more bytes
        while !$bytes.is_empty() {
            // If the buffer is full then compress and clear it
            if $buffer.len() == $block_len {
                $compress
                $buffer.clear();
            }
            // Take the next block or as much as possible
            let want = $block_len - $buffer.len();
            let take = std::cmp::min(want, $bytes.len());
            $buffer.extend(&$bytes[..take]);
            $bytes = &$bytes[take..]
        }
        // One last check before ending the routine.
        // It might be the case that the input was an example multiple of block_len and this will catch that.
        if $buffer.len() == $block_len {
            $compress
            $buffer.clear();
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
                let b = $hasher.hash($input);
                if a != b {
                    panic!("hash did not match test value\nexpected:   {:02x?}\ncalculated: {:02x?}", a,b)
                }
            }
        )+
        }
    };
    // Optional variant with module name for separation
    ($mod_name: ident; $($test_name: ident, $hasher: expr, $input: expr, $output: expr);+ $(;)?) => {
        #[cfg(test)]
        mod $mod_name {
        use super::*;
        $(
            #[test]
            fn $test_name() {
                let a = utils::byte_formatting::hex_to_bytes($output).unwrap();
                let b = $hasher.hash($input);
                if a != b {
                    panic!("hash did not match test value\nexpected:   {:02x?}\ncalculated: {:02x?}", a,b)
                }
            }
        )+
        }
    };
}
