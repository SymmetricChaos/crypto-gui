use crate::traits::ClassicHasher;
use utils::byte_formatting::ByteFormat;

use super::KeccackState;

/// There are four domain separation values possible. Each starts with a different padding byte.
pub enum Domain {
    Keccak,
    Sha3,
    Shake,
    Cshake,
}

impl Domain {
    /// Domain separation value
    pub fn pad(&self) -> u8 {
        match self {
            Domain::Keccak => 0x01,
            Domain::Sha3 => 0x06,
            Domain::Shake => 0x1f,
            Domain::Cshake => 0x04,
        }
    }

    /// Domain separation value combine with with 0x80 byte
    pub fn pad_one(&self) -> u8 {
        match self {
            Domain::Keccak => 0x81,
            Domain::Sha3 => 0x86,
            Domain::Shake => 0x9f,
            Domain::Cshake => 0x84,
        }
    }
}

// https://chemejon.wordpress.com/2021/12/06/sha-3-explained-in-plain-english/
pub struct Keccack {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub rate: usize,     // rate in bytes, block size
    pub hash_len: usize, // output length in bytes, recommended to be half the capacity
    // pub function_name: Vec<u8>,
    // pub customization: Vec<u8>,
    pub domain: Domain,
}

/// Default to SHA3-256
impl Default for Keccack {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rate: 1152 / 8,
            hash_len: 224 / 8,
            // function_name: Vec::new(),
            // customization: Vec::new(),
            domain: Domain::Sha3,
        }
    }
}

impl Keccack {
    /// Input mode
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    /// Output mode
    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    /// Rate in bytes. Less than or equal to 200.
    pub fn rate(mut self, rate: usize) -> Self {
        assert!(rate <= 200);
        self.rate = rate;
        self
    }

    /// Capacity in bytes. Less than or equal to 200. Sets rate to be 200 - capacity.
    pub fn capacity(mut self, capacity: usize) -> Self {
        assert!(capacity <= 200);
        self.rate = 200 - capacity;
        self
    }

    /// Length of the output in bytes.
    pub fn hash_len(mut self, hash_len: usize) -> Self {
        self.hash_len = hash_len;
        self
    }

    pub fn sha3() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rate: 0,
            hash_len: 0,
            // function_name: Vec::new(),
            // customization: Vec::new(),
            domain: Domain::Sha3,
        }
    }

    pub fn shake() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rate: 0,
            hash_len: 0,
            // function_name: Vec::new(),
            // customization: Vec::new(),
            domain: Domain::Shake,
        }
    }

    pub fn cshake() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rate: 0,
            hash_len: 0,
            // function_name: Vec::new(),
            // customization: Vec::new(),
            domain: Domain::Cshake,
        }
    }

    pub fn keccak() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            rate: 0,
            hash_len: 0,
            // function_name: Vec::new(),
            // customization: Vec::new(),
            domain: Domain::Keccak,
        }
    }

    // NIST settings
    /// SHA3-224; rate of 1152 bits
    pub fn sha3_224() -> Self {
        Keccack::sha3().rate(1152 / 8).hash_len(224 / 8)
    }

    /// SHA3-256; rate of 1088 bits
    pub fn sha3_256() -> Self {
        Keccack::sha3().rate(1088 / 8).hash_len(256 / 8)
    }

    /// SHA3-382; rate of 832 bits
    pub fn sha3_384() -> Self {
        Keccack::sha3().rate(832 / 8).hash_len(384 / 8)
    }

    /// SHA3-512; rate of 576 bits
    pub fn sha3_512() -> Self {
        Keccack::sha3().rate(576 / 8).hash_len(512 / 8)
    }

    /// SHAKE128; rate of 1344 bits
    pub fn shake_128(hash_len: usize) -> Self {
        Keccack::shake().rate(1344 / 8).hash_len(hash_len)
    }

    /// SHAKE256; rate of 1088 bits
    pub fn shake_256(hash_len: usize) -> Self {
        Keccack::shake().rate(1088 / 8).hash_len(hash_len)
    }

    /// cSHAKE128; rate of 1344 bits
    pub fn cshake_128(hash_len: usize) -> Self {
        Keccack::cshake().rate(1344 / 8).hash_len(hash_len)
    }

    /// cSHAKE256; rate of 1088 bits
    pub fn cshake_256(hash_len: usize) -> Self {
        Keccack::cshake().rate(1088 / 8).hash_len(hash_len)
    }
}

impl ClassicHasher for Keccack {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Padding rules taken from NIST FIPS-202
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
        // This will never be zero since. Notably if the input length is equal to the rate we push an entire extra block
        let padding_len = self.rate - (input.len() % self.rate);

        if padding_len == 1 {
            // If only one padding bit is needed it combines the domain value and the 0x80 byte
            input.push(self.domain.pad_one());
        } else {
            // If multiple bits are needed append the domain value and then pad until ending with 0x80
            input.push(self.domain.pad());
            input.extend(vec![0x00; padding_len - 2]);
            input.push(0x80)
        }

        let mut state = KeccackState::new();
        state.absorb(&input, self.rate);
        state.squeeze(self.rate, self.hash_len)
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    empty_sha3_224, Keccack::sha3_224(), "",
    "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7";
    empty_sha3_256, Keccack::sha3_256(), "",
    "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a";
    empty_sha3_384, Keccack::sha3_384(), "",
    "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004";
    empty_sha3_512, Keccack::sha3_512(), "",
    "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26";
    empty_shake128, Keccack::shake_128(200), "",
    "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef263cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e235b8cc873c23dc62b8d260169afa2f75ab916a58d974918835d25e6a435085b2badfd6dfaac359a5efbb7bcc4b59d538df9a04302e10c8bc1cbf1a0b3a5120ea17cda7cfad765f5623474d368ccca8af0007cd9f5e4c849f167a580b14aabdefaee7eef47cb0fca9767be1fda69419dfb927e9df07348b196691abaeb580b32def58538b8d23f877";
    empty_shake256, Keccack::shake_256(200), "",
    "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be141e96616fb13957692cc7edd0b45ae3dc07223c8e92937bef84bc0eab862853349ec75546f58fb7c2775c38462c5010d846c185c15111e595522a6bcd16cf86f3d122109e3b1fdd943b6aec468a2d621a7c06c6a957c62b54dafc3be87567d677231395f6147293b68ceab7a9e0c58d864e8efde4e1b9a46cbe854713672f5caaae314ed9083dab";
    abc_sha3_224, Keccack::sha3_224().input(ByteFormat::Hex), "616263",
    "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf";
    abc_sha3_256, Keccack::sha3_256(), "abc",
    "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532";
    sha3_256_1600_bits, Keccack::sha3_256().input(ByteFormat::Hex), "a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3",
    "79f38adec5c20307a98ef76e8324afbfd46cfd81b22e3973c65fa1bd9de31787";
    sha3_256_2008_bits, Keccack::sha3_256().input(ByteFormat::Hex),"83af34279ccb5430febec07a81950d30f4b66f484826afee7456f0071a51e1bbc55570b5cc7ec6f9309c17bf5befdd7c6ba6e968cf218a2b34bd5cf927ab846e38a40bbd81759e9e33381016a755f699df35d660007b5eadf292feefb735207ebf70b5bd17834f7bfa0e16cb219ad4af524ab1ea37334aa66435e5d397fc0a065c411ebbce32c240b90476d307ce802ec82c1c49bc1bec48c0675ec2a6c6f3ed3e5b741d13437095707c565e10d8a20b8c20468ff9514fcf31b4249cd82dcee58c0a2af538b291a87e3390d737191a07484a5d3f3fb8c8f15ce056e5e5f8febe5e1fb59d6740980aa06ca8a0c20f5712b4cde5d032e92ab89f0ae1",
    "3298a95cfe59b9d6cab99c36dc1324194c09f97f08944a02d9574bbca3186b41";
);
