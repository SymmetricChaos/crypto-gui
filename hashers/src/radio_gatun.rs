use crate::traits::StatefulHasher;
use itertools::Itertools;

// https://radiogatun.noekeon.org/
// https://en.wikibooks.org/wiki/Cryptography/RadioGat%C3%BAn

macro_rules! radio_gatun {
    ($word_size: ty, $name: ident, $rotations: expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            hash_len: $word_size,
            belt: [$word_size; 39],
            mill: [$word_size; 19],
            buffer: Vec<u8>,
        }

        impl $name {
            const ROTATION: [u32; 19] = $rotations;
            const BYTES_PER_WORD: usize = (<$word_size>::BITS / 8) as usize;

            pub fn init(hash_len: $word_size) -> Self {
                Self {
                    hash_len,
                    belt: [0; 39],
                    mill: [0; 19],
                    buffer: Vec::new(),
                }
            }

            // XOR words from the mill into the belt
            fn belt_to_mill_feedforward(belt_words: &mut [$word_size], mill_words: &[$word_size]) {
                for i in 0..12 {
                    belt_words[i + ((i % 3) * 13)] ^= mill_words[i + 1]
                }
            }

            fn mill(mill_words: &mut [$word_size]) {
                // main mill operation
                let mut temp_arr = [0; 19];
                for a in 0..19 {
                    let r = Self::ROTATION[a];
                    let i = (a * 7) % 19;
                    let mut k = mill_words[i];
                    k = k
                        ^ ((mill_words[(i as usize + 1) % 19])
                            | (!mill_words[(i as usize + 2) % 19]));
                    temp_arr[a] = k.rotate_right(r)
                }
                // update the mill
                for i in 0..19 {
                    mill_words[i] = temp_arr[i] ^ temp_arr[(i + 1) % 19] ^ temp_arr[(i + 4) % 19]
                }
            }

            fn rotate_belt(belt_words: &mut [$word_size]) {
                belt_words[0..13].rotate_right(1);
                belt_words[13..26].rotate_right(1);
                belt_words[26..39].rotate_right(1);
            }

            fn iota(mill_words: &mut [$word_size]) {
                mill_words[0] ^= 1;
            }

            fn belt_to_mill(belt_words: &[$word_size], mill_words: &mut [$word_size]) {
                mill_words[13] ^= belt_words[0];
                mill_words[14] ^= belt_words[13];
                mill_words[15] ^= belt_words[26];
            }

            pub fn beltmill(belt_words: &mut [$word_size], mill_words: &mut [$word_size]) {
                Self::belt_to_mill_feedforward(belt_words, mill_words);
                Self::rotate_belt(belt_words);
                Self::mill(mill_words);
                Self::iota(mill_words);
                Self::belt_to_mill(belt_words, mill_words);
            }

            pub fn print_state(belt_words: &[$word_size], mill_words: &[$word_size]) {
                println!("{:08x?}", mill_words);
                println!("{:08x?}", &belt_words[0..13]);
                println!("{:08x?}", &belt_words[13..26]);
                println!("{:08x?}\n\n", &belt_words[26..39]);
            }
        }

        impl StatefulHasher for $name {
            fn update(&mut self, bytes: &[u8]) {
                self.buffer.extend_from_slice(bytes);
                let chunks = self.buffer.chunks_exact(Self::BYTES_PER_WORD * 3);
                let rem = chunks.remainder().to_vec();
                for chunk in chunks {
                    let words = chunk
                        .chunks_exact(Self::BYTES_PER_WORD)
                        .map(|c| <$word_size>::from_le_bytes(c.try_into().unwrap()))
                        .collect_vec();
                    for words in words.chunks_exact(3) {
                        self.belt[0] ^= words[0];
                        self.mill[16] ^= words[0];

                        self.belt[13] ^= words[1];
                        self.mill[17] ^= words[1];

                        self.belt[26] ^= words[2];
                        self.belt[18] ^= words[2];

                        for _ in 0..18 {
                            Self::beltmill(&mut self.belt, &mut self.mill);
                        }
                    }
                }
                self.buffer = rem;
            }

            fn finalize(mut self) -> Vec<u8> {
                self.buffer.push(0x01);
                while self.buffer.len() % (Self::BYTES_PER_WORD * 3) != 0 {
                    self.buffer.push(0x00)
                }

                let chunks = self.buffer.chunks_exact(Self::BYTES_PER_WORD * 3);
                for chunk in chunks {
                    let words = chunk
                        .chunks_exact(Self::BYTES_PER_WORD)
                        .map(|c| <$word_size>::from_le_bytes(c.try_into().unwrap()))
                        .collect_vec();
                    for words in words.chunks_exact(3) {
                        self.belt[0] ^= words[0];
                        self.mill[16] ^= words[0];

                        self.belt[13] ^= words[1];
                        self.mill[17] ^= words[1];

                        self.belt[26] ^= words[2];
                        self.belt[18] ^= words[2];

                        for _ in 0..18 {
                            Self::beltmill(&mut self.belt, &mut self.mill);
                        }
                    }
                }

                let mut out = Vec::new();
                while out.len() < self.hash_len as usize {
                    out.extend_from_slice(&self.mill[1].to_le_bytes());
                    out.extend_from_slice(&self.mill[2].to_le_bytes());
                    Self::beltmill(&mut self.belt, &mut self.mill);
                }
                out.truncate(self.hash_len as usize);
                out
            }

            crate::stateful_hash_helpers!();
        }

        // impl ClassicHasher for $name {
        //     fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        //         let mut input = bytes.to_vec();
        //         input.push(0x01);
        //         while input.len() % (Self::BYTES_PER_WORD * 3) != 0 {
        //             input.push(0x00)
        //         }

        //         let words = input
        //             .chunks_exact(Self::BYTES_PER_WORD)
        //             .map(|c| <$word_size>::from_le_bytes(c.try_into().unwrap()))
        //             .collect_vec();

        //         let mut belt: [$word_size; 39] = [0; 39]; // three rows of 13 words
        //         let mut mill: [$word_size; 19] = [0; 19]; // one row of 19 words

        //         for words in words.chunks_exact(3) {
        //             belt[0] ^= words[0];
        //             mill[16] ^= words[0];

        //             belt[13] ^= words[1];
        //             mill[17] ^= words[1];

        //             belt[26] ^= words[2];
        //             belt[18] ^= words[2];

        //             for _ in 0..18 {
        //                 Self::beltmill(&mut belt, &mut mill);
        //             }
        //         }

        //         let mut out = Vec::new();
        //         while out.len() < self.hash_len as usize {
        //             out.extend_from_slice(&mill[1].to_le_bytes());
        //             out.extend_from_slice(&mill[2].to_le_bytes());
        //             Self::beltmill(&mut belt, &mut mill);
        //         }
        //         out.truncate(self.hash_len as usize);
        //         out
        //     }

        //     crate::hash_bytes_from_string! {}
        // }
    };
}

radio_gatun!(
    u32,
    RadioGatun32,
    [0, 1, 3, 6, 10, 15, 21, 28, 4, 13, 23, 2, 14, 27, 9, 24, 8, 25, 11,]
);

radio_gatun!(
    u64,
    RadioGatun64,
    [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43,]
);

crate::stateful_hash_tests!(
    test1, RadioGatun32::init(32), b"1234", "9ebdd24f469993796c4aac6a821735a65a3cdef8a359944ce71f34e7a08e1182";
    test2, RadioGatun64::init(32), b"1234", "733e2b49a53fb166b6f3bd341919578b8c931880f8b8bd7c0fbbee1a538e7307";
);
