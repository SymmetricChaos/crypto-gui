use crate::traits::StatefulHasher;

use super::KeccackState;

// This encode functions are defined as encoding n: 0 <= n < 2^2040
// but values of n: 0 <= n < 2^64 cover all real world cases for the forseeable future
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
    /// Keccack can be squeezed repeatedly to produce additional bits.
    /// No additional data should be absorbed after squeezing.
    pub fn squeeze(&mut self, n_bytes: usize) -> Vec<u8> {
        if !self.buffer.is_empty() {
            self.pad_and_absorb();
        }
        self.state.squeeze(self.rate as usize, n_bytes)
    }

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
        k.update(&vec![0x00; k.rate as usize - k.buffer.len()]);
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
        k.update(&vec![0x00; k.rate as usize - k.buffer.len()]);
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
