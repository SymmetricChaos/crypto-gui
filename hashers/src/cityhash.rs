use utils::byte_formatting::ByteFormat;

use crate::traits::ClassicHasher;

// 64-bit primes
const P0: u64 = 0xc3a5c85c97cb3127;
const P1: u64 = 0xb492b66fbe98f273;
const P2: u64 = 0x9ae16a3b2f90404f;

// 32-bit constants
const C0: u32 = 0xcc9e2d51;
const C1: u32 = 0x1b873593;
const C2: u32 = 0xe6546b64;
const C3: u32 = 0x85ebca6b;
const C4: u32 = 0xc2b2ae35;

// Function taken from Murmur3
fn final_mix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(C3);
    x ^= x >> 13;
    x = x.wrapping_mul(C4);
    x ^= x >> 16;
    x
}

fn compress(mut x: u32, mut y: u32) -> u32 {
    x = x.wrapping_mul(C0);
    x = x.rotate_right(17);
    x = x.wrapping_mul(C1);
    y ^= x;
    y = y.rotate_right(19);
    y.wrapping_mul(5).wrapping_add(C2)
}

fn fetch_u32(bytes: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap())
}

fn hash32_0_to_4(bytes: &[u8]) -> u32 {
    let l = bytes.len() as u32;
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    dbg!(bytes);
    for byte in bytes {
        b = b.wrapping_mul(C0).wrapping_add(*byte as i8 as u32); // yes really, conversion to i8 then to u32 is the intended transformation
        c ^= b;
    }
    final_mix(compress(b, compress(l, c)))
}

fn hash32_5_to_12(bytes: &[u8]) -> u32 {
    let l = bytes.len();
    let mut a = bytes.len() as u32;
    let mut b = a.wrapping_mul(5);
    let mut c: u32 = 9;
    let d: u32 = b;
    a = a.wrapping_add(fetch_u32(bytes, 0));
    b = b.wrapping_add(fetch_u32(bytes, l - 4));
    c = c.wrapping_add(fetch_u32(bytes, (l >> 1) & 4));
    final_mix(compress(c, compress(b, compress(a, d))))
}

fn hash32_13_to_24(bytes: &[u8]) -> u32 {
    let l = bytes.len();
    let a = fetch_u32(bytes, (l >> 1) - 4);
    let b = fetch_u32(bytes, 4);
    let c = fetch_u32(bytes, l - 8);
    let d = fetch_u32(bytes, l >> 1);
    let e = fetch_u32(bytes, 0);
    let f = fetch_u32(bytes, l - 4);
    let h = bytes.len() as u32;
    final_mix(compress(
        f,
        compress(e, compress(d, compress(c, compress(b, compress(a, h))))),
    ))
}

fn hash32_25(bytes: &[u8]) -> u32 {
    let l = bytes.len();
    let mut h = l as u32;
    let mut g = C0.wrapping_mul(h);
    let mut f = g;
    let a0 = fetch_u32(bytes, l - 4)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a1 = fetch_u32(bytes, l - 8)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a2 = fetch_u32(bytes, l - 16)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a3 = fetch_u32(bytes, l - 12)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    let a4 = fetch_u32(bytes, l - 20)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C1);
    h ^= a0;
    h = h.rotate_right(19).wrapping_mul(5).wrapping_add(C2);
    h ^= a2;
    h = h.rotate_right(19).wrapping_mul(5).wrapping_add(C2);
    g ^= a1;
    g = g.rotate_right(19).wrapping_mul(5).wrapping_add(C2);
    g ^= a3;
    g = g.rotate_right(19).wrapping_mul(5).wrapping_add(C2);
    f ^= a4;
    f = f.rotate_right(19).wrapping_mul(5).wrapping_add(C2);

    let mut offset = 0;

    for _ in 0..((l - 1) / 20) {
        let a0 = fetch_u32(bytes, offset)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a1 = fetch_u32(bytes, offset + 4);
        let a2 = fetch_u32(bytes, 8)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a3 = fetch_u32(bytes, offset + 12)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a4 = fetch_u32(bytes, offset + 16);

        h ^= a0;
        h = h.rotate_right(18).wrapping_mul(5).wrapping_add(C2);

        f = f.wrapping_add(a1).rotate_right(19).wrapping_mul(C0);

        g = g
            .wrapping_add(a2)
            .rotate_right(18)
            .wrapping_mul(5)
            .wrapping_add(C2);

        h ^= a3.wrapping_add(a1);
        h = h.rotate_right(19).wrapping_mul(5).wrapping_add(C2);

        g ^= a4;
        g = g.swap_bytes().wrapping_mul(5);

        h = h.wrapping_add(a4.wrapping_mul(5));
        h = h.swap_bytes();

        f = f.wrapping_add(a0);

        std::mem::swap(&mut f, &mut h);
        std::mem::swap(&mut f, &mut g);

        offset += 20
    }

    g = g
        .rotate_right(11)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C0);
    f = f
        .rotate_right(11)
        .wrapping_mul(C0)
        .rotate_right(17)
        .wrapping_mul(C0);

    h.wrapping_add(g)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2)
        .rotate_right(17)
        .wrapping_mul(C0)
        .wrapping_add(f)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2)
        .rotate_right(17)
        .wrapping_mul(C0)
}

pub struct CityHash32 {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
}

impl Default for CityHash32 {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
        }
    }
}

impl CityHash32 {
    pub fn input(mut self, input: ByteFormat) -> Self {
        self.input_format = input;
        self
    }

    pub fn output(mut self, output: ByteFormat) -> Self {
        self.output_format = output;
        self
    }
}

impl ClassicHasher for CityHash32 {
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            0..=4 => hash32_0_to_4(bytes),
            5..=12 => hash32_5_to_12(bytes),
            13..=24 => hash32_13_to_24(bytes),
            _ => hash32_25(bytes),
        }
        .to_be_bytes()
        .to_vec()
    }

    crate::hash_bytes_from_string! {}
}

#[cfg(test)]
mod cityhash_tests {
    use super::*;

    #[test]
    fn test_fetching_does_not_panic() {
        let mut v = Vec::new();
        for i in 0..=4 {
            hash32_0_to_4(&v);
            v.push(i);
        }
        for i in 5..=12 {
            hash32_5_to_12(&v);
            v.push(i);
        }
        for i in 13..=24 {
            hash32_13_to_24(&v);
            v.push(i);
        }
    }

    #[test]
    fn generate_test_inputs() {
        const N: usize = 1 << 12;
        let mut data = [0_u8; N];
        let mut a: u64 = 9;
        let mut b: u64 = 777;
        for i in 0..N {
            a = a.wrapping_add(b);
            b = b.wrapping_add(a);
            a = (a ^ (a >> 41)).wrapping_mul(P0);
            b = (b ^ (b >> 41)).wrapping_mul(P0).wrapping_add(i as u64);
            data[i] = (b >> 37) as u8;
        }
        // Confirmed from a REPL that this matches the goland data
        // println!("{:02x?}", data);
        // Again confirmed from a REPL that this matches the goland data
        for i in 0..60 {
            let s = i * i;
            let e = s + i;
            let mut st = String::new();
            for byte in &data[s..e] {
                st.push_str(&format!("{:02x}", byte));
            }
            println!("{st}");
        }
        // 6a, e4, 4a, 82, 3b, e7, aa, 66, 42, aa, b0, ce, 6a, 47, ea, 25,
        // 3b, 30, a7, 2c, 53, d8, 1f, c5, a8, 64, fc, f2, 8f, 05, 91, 42,
        // be, b4, ae, 38, ee, e8, 84, 54, 99, 9e, c6, 63, f0, c3, 37, 6d,
        // c3, f8, 59, f1, 67, 27, 4d, c2, 15, 4f, 80, 19, ed, 8c, cd, d1
    }
}

crate::basic_hash_tests!(
    test1, CityHash32::default().input(ByteFormat::Hex), "", "dc56d17a";
    test2, CityHash32::default().input(ByteFormat::Hex), "e4", "99929334";
    test3, CityHash32::default().input(ByteFormat::Hex), "3be7", "4252edb7";
);
