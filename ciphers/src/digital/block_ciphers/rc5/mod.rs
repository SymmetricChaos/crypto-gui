pub mod rc5_16;
pub mod rc5_32;
pub mod rc5_64;

#[macro_export]
macro_rules! impl_rc5 {
    ($name: ident, $word: ty, $bytes_in_word: literal, $bits_in_word: literal, $bytes_in_block: literal, $p: literal, $q: literal, $iv_word: ty, $rounds: literal) => {
        use std::ops::{BitXor, Shl};

        pub struct $name {
            pub input_format: utils::byte_formatting::ByteFormat,
            pub output_format: utils::byte_formatting::ByteFormat,
            pub rounds: usize,
            pub state: Vec<$word>,
            pub iv: $iv_word,
            pub mode: crate::digital::block_ciphers::block_cipher::BCMode,
            pub padding: crate::digital::block_ciphers::block_cipher::BCPadding,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    rounds: $rounds,
                    state: Vec::new(),
                    input_format: utils::byte_formatting::ByteFormat::Hex,
                    output_format: utils::byte_formatting::ByteFormat::Hex,
                    iv: 0,
                    mode: crate::digital::block_ciphers::block_cipher::BCMode::default(),
                    padding: crate::digital::block_ciphers::block_cipher::BCPadding::default(),
                }
            }
        }

        crate::block_cipher_builders! {$name, $iv_word}

        impl $name {
            pub fn bytes_to_words(s: &[u8]) -> [$word; 2] {
                [
                    <$word>::from_le_bytes(s[..$bytes_in_word].try_into().unwrap()),
                    <$word>::from_le_bytes(s[$bytes_in_word..$bytes_in_block].try_into().unwrap()),
                ]
            }

            pub fn words_to_bytes(s: &[$word]) -> [u8; $bytes_in_block] {
                let mut out = [0; $bytes_in_block];
                let (left, right) = out.split_at_mut($bytes_in_word);
                left.copy_from_slice(&s[0].to_le_bytes());
                right.copy_from_slice(&s[1].to_le_bytes());
                out
            }

            pub fn state_size(&self) -> usize {
                2 * (self.rounds + 1)
            }

            pub fn ksa(&mut self, key: &[u8]) {
                assert!(
                    key.len() < 256,
                    "RC5 key is limited to 255 bytes, which is enough for anybody"
                );

                let u = $bytes_in_word; // bytes in a word
                let b = key.len(); // bytes in the key
                let c = std::cmp::max(b.div_ceil(u), 1); // words in the key
                let t = self.state_size(); // words in the state
                let mut l = vec![0 as $word; c];
                for i in (0..b).rev() {
                    l[i / u] = (l[i / u].shl(8_u32)).wrapping_add(key[i] as $word)
                }

                let mut s = vec![0; t];
                s[0] = $p as $word;
                for i in 1..t {
                    s[i] = s[i - 1].wrapping_add($q)
                }

                let mut i = 0;
                let mut j = 0;
                let mut a = 0;
                let mut b = 0;
                for _ in 0..(3 * std::cmp::max(t, c)) {
                    s[i] = (s[i].wrapping_add(a).wrapping_add(b)).rotate_left(3);
                    a = s[i];
                    l[j] = (l[j].wrapping_add(a).wrapping_add(b))
                        .rotate_left(a.wrapping_add(b) as u32 % $bits_in_word);
                    b = l[j];
                    i = (i + 1) % t;
                    j = (j + 1) % c;
                }

                self.state = s;
            }
        }

        impl crate::digital::block_ciphers::block_cipher::BlockCipher<$bytes_in_block> for $name {
            fn encrypt_block(&self, bytes: &mut [u8]) {
                let mut block = Self::bytes_to_words(bytes);
                block[0] = block[0].wrapping_add(self.state[0]);
                block[1] = block[1].wrapping_add(self.state[1]);

                for i in 1..=self.rounds {
                    block[0] = block[0]
                        .bitxor(block[1])
                        .rotate_left(block[1] as u32 % $bits_in_word)
                        .wrapping_add(self.state[2 * i]);
                    block[1] = block[1]
                        .bitxor(block[0])
                        .rotate_left(block[0] as u32 % $bits_in_word)
                        .wrapping_add(self.state[(2 * i) + 1])
                }
                utils::byte_formatting::overwrite_bytes(bytes, &Self::words_to_bytes(&block));
            }

            fn decrypt_block(&self, bytes: &mut [u8]) {
                let mut block = Self::bytes_to_words(bytes);
                for i in (1..=self.rounds).rev() {
                    block[1] = block[1]
                        .wrapping_sub(self.state[(2 * i) + 1])
                        .rotate_right(block[0] as u32 % $bits_in_word)
                        .bitxor(block[0]);
                    block[0] = block[0]
                        .wrapping_sub(self.state[2 * i])
                        .rotate_right(block[1] as u32 % $bits_in_word)
                        .bitxor(block[1]);
                }

                block[0] = block[0].wrapping_sub(self.state[0]);
                block[1] = block[1].wrapping_sub(self.state[1]);
                utils::byte_formatting::overwrite_bytes(bytes, &Self::words_to_bytes(&block));
            }

            crate::block_cipher_getters!();
        }

        crate::impl_cipher_for_block_cipher!($name, $bytes_in_block);
    };
}
