use crate::traits::ClassicHasher;
use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be, xor_into_bytes};
use utils::{byte_formatting::ByteFormat, padding::zero_padding};

pub const GOST_R_34_12_2015: [u64; 8] = [
    0xc462a5b9e8d703f1,
    0x68239a5c1e47bd0f,
    0xb3582fade174c960,
    0xc821d4f670a53e9b,
    0x7f5a816d093eb42c,
    0x5df692cab78143e0,
    0x8e25691cf4b0da37,
    0x17ed05834fa69cb2,
];

pub const GOST_R_CRYPTO_PRO: [u64; 8] = [
    0xa4568137dce092bf,
    0x5f402db91763cea8,
    0x7fce94103b526a8d,
    0x4a7c0f28e165db93,
    0x764b9c2a180efd35,
    0x7624d9f0a15b8ec3,
    0xde41705a3c8f629b,
    0x13a95b4f867ed02c,
];

pub const GOST_R_TEST: [u64; 8] = [
    0x4a92d80e6b1c7f53,
    0xeb4c6dfa23810759,
    0x581da342efc7609b,
    0x7da1089fe46cb253,
    0x6c715fd84a9e03b2,
    0x4ba0721d36859cfe,
    0xdb413f590ae7682c,
    0x1fd057a4923e6b8c,
];

pub struct GostCipher {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub sboxes: [u64; 8],
    pub subkeys: [u32; 8],
}

impl Default for GostCipher {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            sboxes: GOST_R_34_12_2015.clone(),
            subkeys: [0; 8],
        }
    }
}

impl GostCipher {
    const ROUND_KEY_IDX: [usize; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 7, 6, 5, 4, 3, 2,
        1, 0,
    ];

    fn sbox(&self, n: u32) -> u32 {
        let mut out = 0;

        for i in 0..8 {
            let shift = 28 - (4 * i);
            let idx = (n >> shift) & 0x0f;
            let s = self.sboxes[i] >> (60 - idx * 4) & 0x0f;
            out |= (s as u32) << shift;
        }

        out
    }

    fn f(&self, n: u32, subkey: u32) -> u32 {
        let x = n.wrapping_add(subkey);
        let x = self.sbox(x);
        x.rotate_left(11)
    }

    pub fn with_sboxes(mut self, sboxes: [u64; 8]) -> Self {
        self.sboxes = sboxes;
        self
    }

    pub fn ksa(&mut self, bytes: [u8; 32]) {
        fill_u32s_be(&mut self.subkeys, &bytes)
    }

    pub fn with_key(mut self, bytes: [u8; 32]) -> Self {
        self.ksa(bytes);
        self
    }

    fn encrypt_block(&self, bytes: &mut [u8]) {
        let mut v = utils::byte_formatting::make_u32s_be::<2>(bytes);
        for idx in Self::ROUND_KEY_IDX {
            let t = v[0];
            // L_i+1 = R_i
            v[0] = v[1];

            // R_i+1 = L_i xor f(R_i)
            v[1] = t ^ self.f(v[1], self.subkeys[idx]);
        }
        v.swap(0, 1);
        u32s_to_bytes_be(bytes, &v);
    }
}

const C: [[u8; 32]; 3] = [
    [0; 32],
    [
        0xff, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0xff,
        0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00,
        0xff, 0x00,
    ],
    [0; 32],
];

fn a(mut y: [u8; 32]) -> [u8; 32] {
    y.rotate_right(8);
    for i in 0..8 {
        y[i] ^= y[24 + i]
    }
    y
}

fn p(y: [u8; 32]) -> [u8; 32] {
    let mut out = [0; 32];
    for i in 0..4 {
        for k in 0..8 {
            out[i + 4 * k] = y[8 * i + k]
        }
    }
    out
}

fn key_gen(h: [u8; 32], m: [u8; 32]) -> [[u8; 32]; 4] {
    let mut u = h;
    let mut v = m;
    let mut w = u.clone();
    xor_into_bytes(w, v);
    let mut ks = [[0; 32]; 4];

    for i in 1..4 {
        u = a(u);
        for j in 0..32 {
            u[j] ^= C[i - 1][j]
        }
        v = a(a(v));
        w = u.clone();
        for j in 0..32 {
            w[j] ^= v[j]
        }
        ks[i] = p(w)
    }

    ks
}

fn e(mut h: [u8; 32], ks: [[u8; 32]; 4], cipher: &mut GostCipher) -> [u8; 32] {
    cipher.ksa(ks[0]);
    cipher.encrypt_block(&mut h[0..8]);
    cipher.ksa(ks[1]);
    cipher.encrypt_block(&mut h[8..16]);
    cipher.ksa(ks[2]);
    cipher.encrypt_block(&mut h[16..24]);
    cipher.ksa(ks[3]);
    cipher.encrypt_block(&mut h[24..32]);
    h
}

fn shuffling(mut h: [u8; 32]) -> [u8; 32] {
    let t0 = h[31] ^ h[29] ^ h[27] ^ h[25] ^ h[7] ^ h[1];
    let t1 = h[30] ^ h[28] ^ h[26] ^ h[24] ^ h[6] ^ h[0];
    h.rotate_right(2);
    h[0] = t0;
    h[1] = t1;
    h
}

fn shuffling_transform(h: [u8; 32], s: [u8; 32], m: [u8; 32]) -> [u8; 32] {
    let mut t = s.clone();
    for _ in 0..12 {
        t = shuffling(t)
    }
    xor_into_bytes(t, m);
    t = shuffling(t);
    xor_into_bytes(t, h);
    for _ in 0..61 {
        t = shuffling(t)
    }
    t
}

fn compress(h: [u8; 32], m: [u8; 32], cipher: &mut GostCipher) -> [u8; 32] {
    let ks = key_gen(h, m);
    let s = e(h, ks, cipher);
    shuffling_transform(h, s, m)
}

#[derive(Debug, Clone)]
pub struct Gost {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub iv: [u8; 32],
    pub sboxes: [u64; 8],
}

impl Default for Gost {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            iv: [0; 32],
            sboxes: GOST_R_34_12_2015.clone(),
        }
    }
}

impl ClassicHasher for Gost {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        let mut input = bytes.to_vec();

        // Final block is padded with zeroes
        zero_padding(&mut input, 32);

        let mut h = self.iv;
        let mut ctrl = [0; 32];

        let mut cipher = GostCipher::default().with_sboxes(self.sboxes);

        // Take input in 256-bit blocks
        for block in input.chunks_exact(32) {
            for i in 0..32 {
                ctrl[i] ^= block[i]
            }
            h = compress(h, block.try_into().unwrap(), &mut cipher)
        }

        // Compress in the length of the input
        let mut l = [0; 32];
        for (i, b) in ((bytes.len() * 8) as u64).to_be_bytes().iter().enumerate() {
            l[i + 23] = *b
        }
        h = compress(h, l, &mut cipher);

        // Compress in the check value
        compress(h, ctrl, &mut cipher).to_vec()
    }

    crate::hash_bytes_from_string! {}
}

crate::basic_hash_tests!(
    test1, Gost::default(),
    "The quick brown fox jumps over the lazy dog",
    "77b7fa410c9ac58a25f49bca7d0468c9296529315eaca76bd1a10f376d1f4294";

    test2, Gost::default(),
    "This is message, length=32 bytes",
    "b1c466d37519b82e8319819ff32595e047a28cb6f83eff1c6916a815a637fffa";

    test3, Gost::default(),
    "Suppose the original message has length = 50 bytes",
    "471aba57a60a770d3a76130635c1fbea4ef14de51f78b4ae57dd893b62f55208";
);
