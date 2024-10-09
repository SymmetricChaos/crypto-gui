use utils::byte_formatting::ByteFormat;

const R: u128 = 0xE1000000000000000000000000000000;

// Multiplication in the Galois field used for GHASH. Addition in the same is XOR.
// This implementation is not optimized at all because this project seeks clarity not real world use.
pub fn mult_gf(x: u128, y: u128) -> u128 {
    let mut out = 0;
    let mut v = x;
    for i in 0..128 {
        // Take bits of y from the left
        if y >> (127 - i) & 1 == 1 {
            out ^= v
        }
        // Check rightmost bit of v
        if v & 1 == 0 {
            v >>= 1;
        } else {
            v >>= 1;
            v ^= R;
        }
    }
    out
}

// Add the bytes of block to the accumulator (this is XOR in the Galois Field) then multiply by the value h
// This is used to implement Horner's Rule for evaluating a polynomial
pub fn add_mul(acc: &mut u128, block: &[u8], h: u128) {
    for (i, byte) in block.iter().enumerate() {
        *acc ^= (*byte as u128) << (15 - i) * 8
    }
    *acc = mult_gf(*acc, h);
}

#[derive(Debug, Clone)]
pub struct Ghash {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub h: u128,     // usually determined by a cipher
    pub c: u128,     // constant term, usually determined by a cipher
    pub ad_len: u64, // how many bytes of input to treat as the additional data
}

impl Default for Ghash {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            h: 0,
            c: 0,
            ad_len: 0,
        }
    }
}

impl Ghash {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }

    pub fn h(mut self, h: u128) -> Self {
        self.h = h;
        self
    }

    pub fn h_bytes(mut self, h: [u8; 16]) -> Self {
        self.h = u128::from_be_bytes(h);
        self
    }

    pub fn c(mut self, c: u128) -> Self {
        self.c = c;
        self
    }

    pub fn c_bytes(mut self, c: [u8; 16]) -> Self {
        self.c = u128::from_be_bytes(c);
        self
    }

    pub fn ad_len(mut self, ad_len: u64) -> Self {
        self.ad_len = ad_len;
        self
    }

    pub fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut acc: u128 = 0;

        // In an AEAD cipher the input would be treated as Addition Data and Ciphertext
        let (ad, ctext) = bytes.split_at(self.ad_len as usize);

        // Process each AD block
        for block in ad.chunks(16) {
            add_mul(&mut acc, block, self.h);
        }

        // Process each CT block
        for block in ctext.chunks(16) {
            add_mul(&mut acc, block, self.h);
        }

        // The length of the AD and CT form the term x^1
        acc ^= ((ad.len() * 8) as u128) << 64;
        acc ^= (ctext.len() * 8) as u128;
        acc = mult_gf(acc, self.h);

        // XOR in the constant term, x^0, this is the key when used securely
        acc ^= self.c;

        acc.to_be_bytes().into()
    }
}
