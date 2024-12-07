use crate::traits::ClassicHasher;
use utils::byte_formatting::{fill_u32s_be, u32s_to_bytes_be, xor_into_bytes};
use utils::{byte_formatting::ByteFormat, padding::zero_padding};

pub const GOST_R_34_12_2015: [u64; 8] = [
    0xC462A5B9E8D703F1,
    0x68239A5C1E47BD0F,
    0xB3582FADE174C960,
    0xC821D4F670A53E9B,
    0x7F5A816D093EB42C,
    0x5DF692CAB78143E0,
    0x8E25691CF4B0DA37,
    0x17ED05834FA69CB2,
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
        for k in 1..9 {
            out[i] = y[8 * i + k]
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

    for i in 2..5 {
        u = a(u);
        for j in 0..32 {
            u[i] ^= C[i][j]
        }
        v = a(a(v));
        w = u.clone();
        for i in 0..32 {
            w[i] ^= v[i]
        }
        ks[i] = p(w)
    }

    ks
}

fn e(mut h: [u8; 32], ks: [[u8; 32]; 4], cipher: &mut GostCipher) -> [u8; 32] {
    for i in 0..4 {
        cipher.ksa(ks[i]);
        cipher.encrypt_block(&mut h[(i * 8)..(i * 8 + 8)]);
    }
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
