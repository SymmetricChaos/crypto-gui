use crate::traits::StatefulHasher;

use super::KeccackState;

pub(crate) fn left_encode(val: u64, b: &mut [u8; 9]) -> &[u8] {
    b[1..].copy_from_slice(&val.to_be_bytes());
    let i = b[1..8].iter().take_while(|&&a| a == 0).count();
    b[i] = (8 - i) as u8;
    &b[i..]
}

pub(crate) fn right_encode(val: u64, b: &mut [u8; 9]) -> &[u8] {
    b[0..8].copy_from_slice(&val.to_be_bytes());
    let i = b[0..7].iter().take_while(|&&a| a == 0).count();
    b[8] = (8 - i) as u8;
    &b[i..]
}

#[test]
fn test_encoding() {
    let x = 0;
    let mut b = [0; 9];
    assert_eq!(&[0x01, 0x00], left_encode(x, &mut b));
    assert_eq!(&[0x00, 0x01], right_encode(x, &mut b));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeccackMode {
    Keccak,
    Sha3,
    Shake,
    Cshake,
    Kmac,
    TupleHash,
}

impl KeccackMode {
    /// Domain separation value
    pub fn pad(&self) -> u8 {
        match self {
            KeccackMode::Keccak => 0x01,
            KeccackMode::Sha3 => 0x06,
            KeccackMode::Shake => 0x1f,
            KeccackMode::Cshake => 0x04,
            KeccackMode::Kmac => 0x04,
            KeccackMode::TupleHash => 0x04,
        }
    }

    /// Domain separation value combined with with 0x80 byte
    pub fn pad_one(&self) -> u8 {
        match self {
            KeccackMode::Keccak => 0x81,
            KeccackMode::Sha3 => 0x86,
            KeccackMode::Shake => 0x9f,
            KeccackMode::Cshake => 0x84,
            KeccackMode::Kmac => 0x84,
            KeccackMode::TupleHash => 0x84,
        }
    }
}

// https://chemejon.wordpress.com/2021/12/06/sha-3-explained-in-plain-english/
pub struct Keccack {
    state: KeccackState,
    buffer: Vec<u8>,
    rate: u64,     // rate in bytes, block size
    hash_len: u64, // output length in bytes, recommended to be half the capacity
    domain: KeccackMode,
    bits_taken: u64,
}

impl Keccack {
    // Rate in bytes. Less than or equal to 200.
    fn with_rate(mut self, rate: u64) -> Self {
        assert!(rate <= 200);
        self.rate = rate;
        self
    }

    // Length of the output in bytes.
    fn with_hash_len(mut self, hash_len: u64) -> Self {
        self.hash_len = hash_len;
        self
    }

    fn sha3() -> Self {
        Self {
            state: KeccackState::new(),
            buffer: Vec::new(),
            rate: 0,
            hash_len: 0,
            domain: KeccackMode::Sha3,
            bits_taken: 0,
        }
    }

    fn shake() -> Self {
        Self {
            state: KeccackState::new(),
            buffer: Vec::new(),
            rate: 0,
            hash_len: 0,
            domain: KeccackMode::Shake,
            bits_taken: 0,
        }
    }

    fn cshake() -> Self {
        Self {
            state: KeccackState::new(),
            buffer: Vec::new(),
            rate: 0,
            hash_len: 0,
            domain: KeccackMode::Cshake,
            bits_taken: 0,
        }
    }

    pub fn keccak(rate: u64, hash_len: u64) -> Self {
        Self {
            state: KeccackState::new(),
            buffer: Vec::new(),
            rate,
            hash_len,
            domain: KeccackMode::Keccak,
            bits_taken: 0,
        }
    }

    // NIST settings
    /// SHA3-224; rate of 1152 bits
    pub fn sha3_224() -> Self {
        Keccack::sha3().with_rate(1152 / 8).with_hash_len(224 / 8)
    }

    /// SHA3-256; rate of 1088 bits
    pub fn sha3_256() -> Self {
        Keccack::sha3().with_rate(1088 / 8).with_hash_len(256 / 8)
    }

    /// SHA3-382; rate of 832 bits
    pub fn sha3_384() -> Self {
        Keccack::sha3().with_rate(832 / 8).with_hash_len(384 / 8)
    }

    /// SHA3-512; rate of 576 bits
    pub fn sha3_512() -> Self {
        Keccack::sha3().with_rate(576 / 8).with_hash_len(512 / 8)
    }

    /// SHAKE128; rate of 1344 bits
    pub fn shake_128(hash_len: u64) -> Self {
        Keccack::shake().with_rate(1344 / 8).with_hash_len(hash_len)
    }

    /// SHAKE256; rate of 1088 bits
    pub fn shake_256(hash_len: u64) -> Self {
        Keccack::shake().with_rate(1088 / 8).with_hash_len(hash_len)
    }

    /// cSHAKE128; rate of 1344 bits
    /// This function is intended to be defined by NIST and not used directly
    pub fn cshake_128(hash_len: u64, function_name: &[u8], customization: &[u8]) -> Self {
        if function_name.is_empty() && customization.is_empty() {
            return Keccack::shake_128(hash_len);
        }
        let mut k = Keccack::cshake()
            .with_rate(1344 / 8)
            .with_hash_len(hash_len);
        let mut b = [0u8; 9];
        // Rate in bits
        k.update(left_encode(k.rate as u64, &mut b));
        // Length of the function name in bits followed by the function name
        k.update(left_encode((function_name.len() * 8) as u64, &mut b));
        k.update(function_name);
        // Length of the customization string in bits followed by the customization string
        k.update(left_encode((customization.len() * 8) as u64, &mut b));
        k.update(customization);
        for _ in 0..(k.rate as usize - k.buffer.len()) {
            k.buffer.push(0);
        }
        println!("buffer: {:02x?}", k.buffer);
        k.update(&[]);
        println!("buffer: {:02x?}", k.buffer);
        println!("state: {}", k.state);
        k
    }

    /// cSHAKE256; rate of 1088 bits
    /// This function is intended to be defined by NIST and not used directly
    pub fn cshake_256(hash_len: u64, function_name: &[u8], customization: &[u8]) -> Self {
        if function_name.is_empty() && customization.is_empty() {
            return Keccack::shake_256(hash_len);
        }
        let mut k = Keccack::cshake()
            .with_rate(1088 / 8)
            .with_hash_len(hash_len);
        let mut b = [0u8; 9];
        // Rate in bits
        k.update(left_encode(k.rate as u64, &mut b));
        // Length of the function name in bits followed by the function name
        k.update(left_encode((function_name.len() * 8) as u64, &mut b));
        k.update(function_name);
        // Length of the customization string in bits followed by the customization string
        k.update(left_encode((customization.len() * 8) as u64, &mut b));
        k.update(customization);
        for _ in 0..(k.rate as usize - k.buffer.len()) {
            k.buffer.push(0);
        }
        k.update(&[]);
        k
    }

    fn pad_and_absorb(&mut self) {
        self.bits_taken += (self.buffer.len() * 8) as u64;
        let padding_len = self.rate as usize - (self.buffer.len() % self.rate as usize);

        if padding_len == 1 {
            // If only one padding bit is needed it combines the domain value and the 0x80 byte
            self.buffer.push(self.domain.pad_one());
        } else {
            // If multiple bits are needed append the domain value and then pad until ending with 0x80
            self.buffer.push(self.domain.pad());
            self.buffer.extend(vec![0x00; padding_len - 2]);
            self.buffer.push(0x80)
        }

        if self.domain == KeccackMode::Kmac {
            todo!("KMAC extra padding")
        }

        self.state.absorb(&self.buffer, self.rate as usize);
    }

    /// Keccack can be squeezed repeatedly to produce additional bits.
    /// No additional data should be absorbed after squeezing.
    pub fn squeeze(&mut self, n_bytes: usize) -> Vec<u8> {
        if !self.buffer.is_empty() {
            self.pad_and_absorb();
        }
        self.state.squeeze(self.rate as usize, n_bytes)
    }
}

impl StatefulHasher for Keccack {
    fn update(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        let chunks = self.buffer.chunks_exact(self.rate as usize);
        let rem = chunks.remainder().to_vec();
        for chunk in chunks {
            self.state.absorb(chunk, self.rate as usize);
            self.bits_taken += self.rate * 8;
        }
        self.buffer = rem;
    }

    fn finalize(mut self) -> Vec<u8> {
        self.pad_and_absorb();
        self.state
            .squeeze(self.rate as usize, self.hash_len as usize)
    }

    crate::stateful_hash_helpers!();
}

crate::stateful_hash_tests!(
    empty_sha3_224, Keccack::sha3_224(), b"",
    "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7";
    abc_sha3_224, Keccack::sha3_224(), b"abc",
    "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf";
    long_sha3_224, Keccack::sha3_224(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "543e6868e1666c1a643630df77367ae5a62a85070a51c14cbf665cbc";
    very_long_sha3_224, Keccack::sha3_224(), &[0x61; 1_000_000],
    "d69335b93325192e516a912e6d19a15cb51c6ed5c15243e7a7fd653c";

    empty_sha3_256, Keccack::sha3_256(), b"",
    "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a";
    abc_sha3_256, Keccack::sha3_256(), b"abc",
    "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532";
    long_sha3_256, Keccack::sha3_256(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "916f6061fe879741ca6469b43971dfdb28b1a32dc36cb3254e812be27aad1d18";
    very_long_sha3_256, Keccack::sha3_256(), &[0x61; 1_000_000],
    "5c8875ae474a3634ba4fd55ec85bffd661f32aca75c6d699d0cdcb6c115891c1";
    sha3_256_1600_bits, Keccack::sha3_256(), &utils::byte_formatting::ByteFormat::Hex.text_to_bytes("a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3").unwrap(),
    "79f38adec5c20307a98ef76e8324afbfd46cfd81b22e3973c65fa1bd9de31787";
    sha3_256_2008_bits, Keccack::sha3_256(), &utils::byte_formatting::ByteFormat::Hex.text_to_bytes("83af34279ccb5430febec07a81950d30f4b66f484826afee7456f0071a51e1bbc55570b5cc7ec6f9309c17bf5befdd7c6ba6e968cf218a2b34bd5cf927ab846e38a40bbd81759e9e33381016a755f699df35d660007b5eadf292feefb735207ebf70b5bd17834f7bfa0e16cb219ad4af524ab1ea37334aa66435e5d397fc0a065c411ebbce32c240b90476d307ce802ec82c1c49bc1bec48c0675ec2a6c6f3ed3e5b741d13437095707c565e10d8a20b8c20468ff9514fcf31b4249cd82dcee58c0a2af538b291a87e3390d737191a07484a5d3f3fb8c8f15ce056e5e5f8febe5e1fb59d6740980aa06ca8a0c20f5712b4cde5d032e92ab89f0ae1").unwrap(),
    "3298a95cfe59b9d6cab99c36dc1324194c09f97f08944a02d9574bbca3186b41";

    empty_sha3_384, Keccack::sha3_384(), b"",
    "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004";
    long_sha3_384, Keccack::sha3_384(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "79407d3b5916b59c3e30b09822974791c313fb9ecc849e406f23592d04f625dc8c709b98b43b3852b337216179aa7fc7";
    very_long_sha3_384, Keccack::sha3_384(), &[0x61; 1_000_000],
    "eee9e24d78c1855337983451df97c8ad9eedf256c6334f8e948d252d5e0e76847aa0774ddb90a842190d2c558b4b8340";

    empty_sha3_512, Keccack::sha3_512(), b"",
    "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26";
    long_sha3_512, Keccack::sha3_512(), b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
    "afebb2ef542e6579c50cad06d2e578f9f8dd6881d7dc824d26360feebf18a4fa73e3261122948efcfd492e74e82e2189ed0fb440d187f382270cb455f21dd185";
    very_long_sha3_512, Keccack::sha3_512(), &[0x61; 1_000_000],
    "3c3a876da14034ab60627c077bb98f7e120a2a5370212dffb3385a18d4f38859ed311d0a9d5141ce9cc5c66ee689b266a8aa18ace8282a0e0db596c90b0a7b87";

    empty_shake128, Keccack::shake_128(200), b"",
    "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef263cb1eea988004b93103cfb0aeefd2a686e01fa4a58e8a3639ca8a1e3f9ae57e235b8cc873c23dc62b8d260169afa2f75ab916a58d974918835d25e6a435085b2badfd6dfaac359a5efbb7bcc4b59d538df9a04302e10c8bc1cbf1a0b3a5120ea17cda7cfad765f5623474d368ccca8af0007cd9f5e4c849f167a580b14aabdefaee7eef47cb0fca9767be1fda69419dfb927e9df07348b196691abaeb580b32def58538b8d23f877";
    empty_shake256, Keccack::shake_256(200), b"",
    "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be141e96616fb13957692cc7edd0b45ae3dc07223c8e92937bef84bc0eab862853349ec75546f58fb7c2775c38462c5010d846c185c15111e595522a6bcd16cf86f3d122109e3b1fdd943b6aec468a2d621a7c06c6a957c62b54dafc3be87567d677231395f6147293b68ceab7a9e0c58d864e8efde4e1b9a46cbe854713672f5caaae314ed9083dab";

    nist_test_4_cshake128, Keccack::cshake_128(32, b"", b"Email Signature"), &[0, 1, 2, 3],
    "c1c36925b6409a04f1b504fcbca9d82b4017277cb5ed2b2065fc1d3814d5aaf5";
    nist_test_1600_cshake128, Keccack::cshake_128(32, b"", b"Email Signature"),
    &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F,
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F,
    0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF,
    0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF,
    0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7],
    "c5221d50e4f822d96a2e8881a961420f294b7b24fe3d2094baed2c6524cc166b";

    nist_test_4_cshake256, Keccack::cshake_256(64, b"", b"Email Signature"), &[0, 1, 2, 3],
    "d008828e2b80ac9d2218ffee1d070c48b8e4c87bff32c9699d5b6896eee0edd164020e2be0560858d9c00c037e34a96937c561a74c412bb4c746469527281c8c";
    nist_test_1600_cshake256, Keccack::cshake_256(64, b"", b"Email Signature"),
    &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F,
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F,
    0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF,
    0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF,
    0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7],
    "07dc27b11e51fbac75bc7b3c1d983e8b4b85fb1defaf218912ac86430273091727f42b17ed1df63e8ec118f04b23633c1dfb1574c8fb55cb45da8e25afb092bb";
);
