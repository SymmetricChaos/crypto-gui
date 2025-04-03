use crate::traits::StatefulHasher;

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
            const BYTES_PER_INPUT: usize = Self::BYTES_PER_WORD * 3;

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

            fn beltmill(belt_words: &mut [$word_size], mill_words: &mut [$word_size]) {
                Self::belt_to_mill_feedforward(belt_words, mill_words);
                Self::rotate_belt(belt_words);
                Self::mill(mill_words);
                Self::iota(mill_words);
                Self::belt_to_mill(belt_words, mill_words);
            }

            fn compress(&mut self, input: [$word_size; 3]) {
                self.belt[0] ^= input[0];
                self.belt[13] ^= input[1];
                self.belt[26] ^= input[2];

                self.mill[16] ^= input[0];
                self.mill[17] ^= input[1];
                self.mill[18] ^= input[2];

                for _ in 0..18 {
                    Self::beltmill(&mut self.belt, &mut self.mill);
                }
            }

            // This will panic if self.buffer is too small but we're not invoking it in that case
            fn make_input(&self) -> [$word_size; 3] {
                let bpw: usize = Self::BYTES_PER_WORD;
                println!("{:02x?}", self.buffer);
                [
                    <$word_size>::from_le_bytes(self.buffer[0..bpw].try_into().unwrap()),
                    <$word_size>::from_le_bytes(self.buffer[bpw..(bpw * 2)].try_into().unwrap()),
                    <$word_size>::from_le_bytes(
                        self.buffer[(bpw * 2)..(bpw * 3)].try_into().unwrap(),
                    ),
                ]
            }

            pub fn print_state(belt_words: &[$word_size], mill_words: &[$word_size]) {
                println!("{:08x?}", mill_words);
                println!("{:08x?}", &belt_words[0..13]);
                println!("{:08x?}", &belt_words[13..26]);
                println!("{:08x?}\n\n", &belt_words[26..39]);
            }
        }

        impl StatefulHasher for $name {
            fn update(&mut self, mut bytes: &[u8]) {
                crate::compression_routine!(self.buffer, bytes, Self::BYTES_PER_INPUT, {
                    let words = self.make_input();
                    self.compress(words);
                });
            }

            fn finalize(mut self) -> Vec<u8> {
                self.buffer.push(0x01); // This is correct based on the reference code
                while self.buffer.len() % Self::BYTES_PER_INPUT != 0 {
                    self.buffer.push(0x00)
                }

                while !self.buffer.is_empty() {
                    let words = self.make_input();
                    self.compress(words);
                    self.buffer = self.buffer[(Self::BYTES_PER_INPUT)..].to_vec();
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
    test32_0, RadioGatun32::init(32), b"",
    "f30028b54afab6b3e55355d277711109a19beda7091067e9a492fb5ed9f20117";
    test32_1, RadioGatun32::init(32), b"1234",
    "9ebdd24f469993796c4aac6a821735a65a3cdef8a359944ce71f34e7a08e1182";
    test32_2, RadioGatun32::init(32), b"The quick brown fox jumps over the lazy dog",
    "191589005fec1f2a248f96a16e9553bf38d0aee1648ffa036655ce29c2e229ae";
    test32_3, RadioGatun32::init(32), b"The quick brown fox jumps over the lazy cog",
    "ebdc1c8dcd54deb47eeefc33ca0809ad23cd9ffc0b5254be0fdabb713477f2bd";
    test32_4, RadioGatun32::init(32), b"In response to the SHA-1 vulnerability that was announced in Feb. 2005, NIST held a Cryptographic Hash Workshop on Oct. 31-Nov. 1, 2005 to solicit public input on its cryptographic hash function policy and standards. NIST continues to recommend a transition from SHA-1 to the larger approved hash functions (SHA-224, SHA-256, SHA-384, and SHA-512). In response to the workshop, NIST has also decided that it would be prudent in the long-term to develop an additional hash function through a public competition, similar to the development process for the block cipher in the Advanced Encryption Standard (AES).",
    "4311d3cdc46efe38fdb5c3023a160c3069b26a2af0ce0ccaaffa3f3c61629ad6";
    test32_5, RadioGatun32::init(32), b"12345678", // correct loading into the belt and mill
    "e69e29ba139c20846116d8ad406e6197f1701d8243cc53bb86f2b72c62320a39";
    test32_6, RadioGatun32::init(32), b"1234567890123",
    "99F13E01DBF89E6BBF60C87E99F4F18C851D3385D9B5A1678C705E8F31F70B84";

    test64_0, RadioGatun64::init(32), b"",
    "64a9a7fa139905b57bdab35d33aa216370d5eae13e77bfcdd85513408311a584";
    test64_1, RadioGatun64::init(32), b"1234",
    "733e2b49a53fb166b6f3bd341919578b8c931880f8b8bd7c0fbbee1a538e7307";
    test64_2, RadioGatun64::init(32), b"The quick brown fox jumps over the lazy dog",
    "6219fb8dad92ebe5b2f7d18318f8da13cecbf13289d79f5abf4d253c6904c807";
    test64_3, RadioGatun64::init(32), b"The quick brown fox jumps over the lazy cog",
    "c06265cac961ea74912695ebf20f1c256a338bc0e980853a3eef188d4b06fce5";
    test64_4, RadioGatun64::init(32), b"In response to the SHA-1 vulnerability that was announced in Feb. 2005, NIST held a Cryptographic Hash Workshop on Oct. 31-Nov. 1, 2005 to solicit public input on its cryptographic hash function policy and standards. NIST continues to recommend a transition from SHA-1 to the larger approved hash functions (SHA-224, SHA-256, SHA-384, and SHA-512). In response to the workshop, NIST has also decided that it would be prudent in the long-term to develop an additional hash function through a public competition, similar to the development process for the block cipher in the Advanced Encryption Standard (AES).",
    "2c9ec1efc5d2feeffc2817cd571f394328111db8068fc79e2fb84a42416bf5d3";
);
