const NLFSR_LEN: [usize; 13] = [21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33];
const NLFSR_MASK: [u64; 13] = [
    0x001FFFFF,
    0x003FFFFF,
    0x007FFFFF,
    0x00FFFFFF,
    0x01FFFFFF,
    0x03FFFFFF,
    0x07FFFFFF,
    0x0FFFFFFF,
    0x1FFFFFFF,
    0x3FFFFFFF,
    0x7FFFFFFF,
    0xFFFFFFFF,
    0x1FFFFFFFF,
];

macro_rules! and3 {
    ($a: expr, $b: expr, $c: expr) => {
        a & b & c
    };
}

macro_rules! and2 {
    ($a: expr, $b: expr) => {
        a & b
    };
}

macro_rules! xor3 {
    ($a: expr, $b: expr, $c: expr) => {
        a ^ b ^ c
    };
}

macro_rules! xor2 {
    ($a: expr, $b: expr) => {
        a ^ b
    };
}

macro_rules! mux3 {
    ($a: expr, $b: expr, $c: expr) => {
        (c & (a ^ b)) ^ a
    };
}

macro_rules! maj3 {
    ($a: expr, $b: expr, $c: expr) => {
        (a & (b ^ c)) ^ (b & c)
    };
}

pub struct Achterbahn128 {}
