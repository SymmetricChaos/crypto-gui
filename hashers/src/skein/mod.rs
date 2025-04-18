pub mod skein1024;
pub mod skein256;
pub mod skein512;

// The number 240 encrypted with AES with an all zero key
pub const C240: u64 = 0x1BD11BDAA9FC1A22;

pub const PERM_256: [usize; 4] = [0, 3, 2, 1];
pub const PERM_512: [usize; 8] = [2, 1, 4, 7, 6, 5, 0, 3];
pub const PERM_1024: [usize; 16] = [0, 9, 2, 13, 6, 11, 4, 15, 10, 7, 12, 3, 14, 5, 8, 1];

macro_rules! skein_mix {
    ($a: expr, $b: expr, $r: literal) => {
        $a = $a.wrapping_add($b);
        $b = $b.rotate_left($r) ^ $a;
    };
}

pub fn eight_rounds_256(w: &mut [u64; 4], subkeys0: [u64; 4], subkeys1: [u64; 4]) {
    for i in 0..4 {
        w[i] = w[i].wrapping_add(subkeys0[i])
    }

    skein_mix!(w[0], w[1], 14);
    skein_mix!(w[2], w[3], 16);

    skein_mix!(w[0], w[3], 52);
    skein_mix!(w[2], w[1], 57);

    skein_mix!(w[0], w[1], 23);
    skein_mix!(w[2], w[3], 40);

    skein_mix!(w[0], w[3], 5);
    skein_mix!(w[2], w[1], 37);

    for i in 0..4 {
        w[i] = w[i].wrapping_add(subkeys1[i])
    }

    skein_mix!(w[0], w[1], 25);
    skein_mix!(w[2], w[3], 33);

    skein_mix!(w[0], w[3], 46);
    skein_mix!(w[2], w[1], 12);

    skein_mix!(w[0], w[1], 58);
    skein_mix!(w[2], w[3], 22);

    skein_mix!(w[0], w[3], 32);
    skein_mix!(w[2], w[1], 32);
}

pub fn eight_rounds_512(w: &mut [u64; 8], subkeys0: [u64; 8], subkeys1: [u64; 8]) {
    for i in 0..8 {
        w[i] = w[i].wrapping_add(subkeys0[i])
    }

    skein_mix!(w[0], w[1], 46);
    skein_mix!(w[2], w[3], 36);
    skein_mix!(w[4], w[5], 19);
    skein_mix!(w[6], w[7], 37);

    skein_mix!(w[2], w[1], 33);
    skein_mix!(w[4], w[7], 27);
    skein_mix!(w[6], w[5], 14);
    skein_mix!(w[0], w[3], 42);

    skein_mix!(w[4], w[1], 17);
    skein_mix!(w[6], w[3], 49);
    skein_mix!(w[0], w[5], 36);
    skein_mix!(w[2], w[7], 39);

    skein_mix!(w[6], w[1], 44);
    skein_mix!(w[0], w[7], 9);
    skein_mix!(w[2], w[5], 54);
    skein_mix!(w[4], w[3], 56);

    for i in 0..8 {
        w[i] = w[i].wrapping_add(subkeys1[i])
    }

    skein_mix!(w[0], w[1], 39);
    skein_mix!(w[2], w[3], 30);
    skein_mix!(w[4], w[5], 34);
    skein_mix!(w[6], w[7], 24);

    skein_mix!(w[2], w[1], 13);
    skein_mix!(w[4], w[7], 50);
    skein_mix!(w[6], w[5], 10);
    skein_mix!(w[0], w[3], 17);

    skein_mix!(w[4], w[1], 25);
    skein_mix!(w[6], w[3], 29);
    skein_mix!(w[0], w[5], 39);
    skein_mix!(w[2], w[7], 43);

    skein_mix!(w[6], w[1], 8);
    skein_mix!(w[0], w[7], 35);
    skein_mix!(w[2], w[5], 56);
    skein_mix!(w[4], w[3], 22);
}
