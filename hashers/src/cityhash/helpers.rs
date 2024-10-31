use num::Integer;
use std::ops::BitXor;

// 64-bit primes
pub(super) const P0: u64 = 0xc3a5c85c97cb3127;
pub(super) const P1: u64 = 0xb492b66fbe98f273;
pub(super) const P2: u64 = 0x9ae16a3b2f90404f;
pub(super) const P3: u64 = 0x9ddfea08eb382d69;

// 32-bit constants from Murmur3
pub(super) const C0: u32 = 0xcc9e2d51;
pub(super) const C1: u32 = 0x1b873593;
pub(super) const C2: u32 = 0xe6546b64;
pub(super) const C3: u32 = 0x85ebca6b;
pub(super) const C4: u32 = 0xc2b2ae35;

pub(super) fn fetch_u32(bytes: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap())
}

pub(super) fn fetch_u64(bytes: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap())
}

pub(super) fn shift_mix(a: u64) -> u64 {
    a ^ (a >> 47)
}

pub(super) fn final_mix(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(C3);
    x ^= x >> 13;
    x = x.wrapping_mul(C4);
    x ^= x >> 16;
    x
}

pub(super) fn compress(mut x: u32, mut y: u32) -> u32 {
    x = x.wrapping_mul(C0);
    x = x.rotate_right(17);
    x = x.wrapping_mul(C1);
    y ^= x;
    y = y.rotate_right(19);
    y.wrapping_mul(5).wrapping_add(C2)
}

pub(super) fn hash32_0_to_4(bytes: &[u8]) -> u32 {
    let l = bytes.len() as u32;
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    for byte in bytes {
        b = b.wrapping_mul(C0).wrapping_add(*byte as i8 as u32); // yes really, conversion to i8 then to u32 is the intended transformation
        c ^= b;
    }
    final_mix(compress(b, compress(l, c)))
}

pub(super) fn hash32_5_to_12(bytes: &[u8]) -> u32 {
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

pub(super) fn hash32_13_to_24(bytes: &[u8]) -> u32 {
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

pub(super) fn hash32_25(bytes: &[u8]) -> u32 {
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

    h = h
        .bitxor(a0)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    h = h
        .bitxor(a2)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    g = g
        .bitxor(a1)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    g = g
        .bitxor(a3)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);
    f = f
        .wrapping_add(a4)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(C2);

    let mut offset = 0;

    for _ in 0..((l - 1) / 20) {
        let a0 = fetch_u32(bytes, offset)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a1 = fetch_u32(bytes, offset + 4);
        let a2 = fetch_u32(bytes, offset + 8)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a3 = fetch_u32(bytes, offset + 12)
            .wrapping_mul(C0)
            .rotate_right(17)
            .wrapping_mul(C1);
        let a4 = fetch_u32(bytes, offset + 16);

        h = h
            .bitxor(a0)
            .rotate_right(18)
            .wrapping_mul(5)
            .wrapping_add(C2);

        f = f.wrapping_add(a1).rotate_right(19).wrapping_mul(C0);

        g = g
            .wrapping_add(a2)
            .rotate_right(18)
            .wrapping_mul(5)
            .wrapping_add(C2);

        h ^= a3.wrapping_add(a1);
        h = h.rotate_right(19).wrapping_mul(5).wrapping_add(C2);

        g = g.bitxor(a4).swap_bytes().wrapping_mul(5);

        h = h.wrapping_add(a4.wrapping_mul(5)).swap_bytes();

        f = f.wrapping_add(a0);

        (f, g, h) = (g, h, f);

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

pub(super) fn hash64_0_to_16(bytes: &[u8]) -> u64 {
    let l = bytes.len();
    match l {
        0 => P2,
        1..=3 => {
            let a = bytes[0] as u32;
            let b = bytes[l >> 1] as u32;
            let c = bytes[l - 1] as u32;
            let y = a.wrapping_add(b << 8) as u64;
            let z = (l as u32).wrapping_add(c << 2) as u64;
            shift_mix(y.wrapping_mul(P2) ^ z.wrapping_mul(P0)).wrapping_mul(P2)
        }
        4..=7 => {
            let mul = P2.wrapping_add((l as u64) * 2); // no change that l will overflow so no point in wrapping mul, but should make no difference
            let a = fetch_u32(bytes, 0) as u64;
            let u = (l as u64).wrapping_add(a << 3);
            let v = fetch_u32(bytes, l - 4) as u64;
            hash128_64_mul(u, v, mul)
        }
        _ => {
            let mul = P2.wrapping_add((l as u64) * 2);
            let a = fetch_u64(bytes, 0).wrapping_add(P2);
            let b = fetch_u64(bytes, l - 8);
            let u = b.rotate_right(37).wrapping_mul(mul).wrapping_add(a);
            let v = a.rotate_right(25).wrapping_add(b).wrapping_mul(mul);
            hash128_64_mul(u, v, mul)
        }
    }
}

pub(super) fn hash64_17_to_32(bytes: &[u8]) -> u64 {
    let l = bytes.len();
    let mul = P2.wrapping_add((l as u64) * 2);
    let a = fetch_u64(bytes, 0).wrapping_mul(P1);
    let b = fetch_u64(bytes, 8);
    let c = fetch_u64(bytes, l - 8).wrapping_mul(mul);
    let d = fetch_u64(bytes, l - 16).wrapping_mul(P2);
    let u = a
        .wrapping_add(b)
        .rotate_right(43)
        .wrapping_add(c.rotate_right(30))
        .wrapping_add(d);
    let v = a
        .wrapping_add(b.wrapping_add(P2).rotate_right(18))
        .wrapping_add(c);
    hash128_64_mul(u, v, mul)
}

pub(super) fn hash64_33_to_64(bytes: &[u8]) -> u64 {
    let l = bytes.len();
    let mul = P2.wrapping_add((l as u64) * 2);
    let mut a = fetch_u64(bytes, 0).wrapping_mul(P2);
    let mut b = fetch_u64(bytes, 8);
    let c = fetch_u64(bytes, l - 24);
    let d = fetch_u64(bytes, l - 32);
    let e = fetch_u64(bytes, 16).wrapping_mul(P2);
    let f = fetch_u64(bytes, 24).wrapping_mul(9);
    let g = fetch_u64(bytes, l - 8);
    let h = fetch_u64(bytes, l - 16).wrapping_mul(mul);
    let u = a
        .wrapping_add(g)
        .rotate_right(43)
        .wrapping_add(b.rotate_right(30).wrapping_add(c).wrapping_mul(9));
    let v = a.wrapping_add(g).bitxor(d).wrapping_add(f).wrapping_add(1);
    let w = u
        .wrapping_add(v)
        .wrapping_mul(mul)
        .swap_bytes()
        .wrapping_add(h);
    let x = e.wrapping_add(f).rotate_right(42).wrapping_add(c);
    let y = v
        .wrapping_add(w)
        .wrapping_mul(mul)
        .swap_bytes()
        .wrapping_add(g)
        .wrapping_mul(mul);
    let z = e.wrapping_add(f).wrapping_add(c);
    a = x
        .wrapping_add(z)
        .wrapping_mul(mul)
        .wrapping_add(y)
        .swap_bytes()
        .wrapping_add(b);
    b = shift_mix(
        z.wrapping_add(a)
            .wrapping_mul(mul)
            .wrapping_add(d)
            .wrapping_add(h),
    )
    .wrapping_mul(mul);
    b.wrapping_add(x)
}

// Hash 128 bits down to 64 with a variable multiplier
pub(super) fn hash128_64_mul(u: u64, v: u64, mul: u64) -> u64 {
    let a = u.bitxor(v).wrapping_mul(mul);
    let b = (v ^ shift_mix(a)).wrapping_mul(mul);
    shift_mix(b).wrapping_mul(mul)
}

// Hash 128 bits down to 64 with a fixed multiplier
pub(super) fn hash128_64(u: u64, v: u64) -> u64 {
    hash128_64_mul(u, v, P3)
}

pub(super) fn weak_hash_128_with_seeds(bytes: &[u8], mut a: u64, mut b: u64) -> (u64, u64) {
    let w = fetch_u64(bytes, 0);
    let x = fetch_u64(bytes, 8);
    let y = fetch_u64(bytes, 16);
    let z = fetch_u64(bytes, 24);

    a = a.wrapping_add(w);
    b = b.wrapping_add(a).wrapping_add(z).rotate_right(21);
    let c = a;
    a = a.wrapping_add(x).wrapping_add(y);
    b = b.wrapping_add(a.rotate_right(44));

    (a.wrapping_add(z), b.wrapping_add(c))
}

pub(super) fn hash64_65(bytes: &[u8]) -> u64 {
    let l = bytes.len();

    let mut x = fetch_u64(bytes, l - 40);
    let mut y = fetch_u64(bytes, l - 16).wrapping_add(fetch_u64(bytes, l - 56));
    let mut z = hash128_64(
        fetch_u64(bytes, l - 48).wrapping_add(l as u64),
        fetch_u64(bytes, l - 24),
    );

    let (mut v1, mut v2) = weak_hash_128_with_seeds(&bytes[l - 64..], l as u64, z);
    let (mut w1, mut w2) = weak_hash_128_with_seeds(&bytes[l - 32..], y.wrapping_add(P1), x);

    x = x.wrapping_mul(P1).wrapping_add(fetch_u64(bytes, 0));

    let mut n = (l - 1).prev_multiple_of(&64);

    let mut offset = 0;
    loop {
        let block = &bytes[offset..];
        x = x
            .wrapping_add(y)
            .wrapping_add(v1)
            .wrapping_add(fetch_u64(block, 8))
            .rotate_right(37)
            .wrapping_mul(P1);
        y = y
            .wrapping_add(v2)
            .wrapping_add(fetch_u64(block, 48))
            .rotate_right(42)
            .wrapping_mul(P1);

        x ^= w2;

        y = y.wrapping_add(v1).wrapping_add(fetch_u64(block, 40));

        z = z.wrapping_add(w1).rotate_right(33).wrapping_mul(P1);

        (v1, v2) = weak_hash_128_with_seeds(block, v2.wrapping_mul(P1), x.wrapping_add(w1));
        (w1, w2) = weak_hash_128_with_seeds(
            &block[32..],
            z.wrapping_add(w2),
            y.wrapping_add(fetch_u64(block, 16)),
        );

        (z, x) = (x, z);
        n -= 64;
        offset += 64;
        if n == 0 {
            break;
        }
    }

    hash128_64(
        hash128_64(v1, w1)
            .wrapping_add(shift_mix(y).wrapping_mul(P1))
            .wrapping_add(z),
        hash128_64(v2, w2).wrapping_add(x),
    )
}

pub(super) fn city_mur(bytes: &[u8], seed0: u64, seed1: u64) -> Vec<u8> {
    let l = bytes.len();
    let mut a = seed0;
    let mut b = seed1;
    let mut c: u64;
    let mut d: u64;

    if l <= 16 {
        a = shift_mix(a.wrapping_mul(P1)).wrapping_mul(P1);
        c = b.wrapping_mul(P1).wrapping_add(hash64_0_to_16(bytes));
        if l >= 8 {
            d = shift_mix(a.wrapping_add(fetch_u64(bytes, 0)))
        } else {
            d = shift_mix(a.wrapping_add(c))
        }
    } else {
        c = hash128_64(fetch_u64(bytes, l - 8).wrapping_add(P1), a);
        d = hash128_64(
            b.wrapping_add(l as u64),
            c.wrapping_add(fetch_u64(bytes, l - 16)),
        );
        a = a.wrapping_add(d);
        let mut n = l - 16;
        let mut offset = 0;
        while n > 0 {
            a ^= shift_mix(fetch_u64(bytes, offset + 0).wrapping_mul(P1)).wrapping_mul(P1);
            a = a.wrapping_mul(P1);
            b ^= a;
            c ^= shift_mix(fetch_u64(bytes, offset + 8).wrapping_mul(P1)).wrapping_mul(P1);
            c = c.wrapping_mul(P1);
            d ^= c;
            n -= 16;
            offset += 16;
        }
    }

    a = hash128_64(a, c);
    b = hash128_64(d, b);
    [a ^ b, hash128_64(b, a)]
        .into_iter()
        .flat_map(|w| w.to_be_bytes())
        .collect()
}
