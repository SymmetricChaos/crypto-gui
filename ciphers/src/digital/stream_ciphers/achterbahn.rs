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
        $a & $b & $c
    };
}

macro_rules! and2 {
    ($a: expr, $b: expr) => {
        $a & $b
    };
}

macro_rules! xor3 {
    ($a: expr, $b: expr, $c: expr) => {
        $a ^ $b ^ $c
    };
}

macro_rules! xor2 {
    ($a: expr, $b: expr) => {
        $a ^ $b
    };
}

macro_rules! mux3 {
    ($a: expr, $b: expr, $c: expr) => {
        ($c & ($a ^ $b)) ^ $a
    };
}

macro_rules! maj3 {
    ($a: expr, $b: expr, $c: expr) => {
        ($a & ($b ^ $c)) ^ ($b & $c)
    };
}

// #define A0_cycle(x,feedin)\
//    (x = (x >> ONE) | ((ONE & (XOR3(XOR3(feedin,\
//                                          x >>15,\
//                                         XOR3( x >>3, x >>2, x )),\
//                                    XOR3(AND2( x >>4, x >>7),\
//                                         XOR3( x >>5, x >>6, x >>8),\
//                                         MUX3( x >>4, x >>5, x >>6)),\
//                                    MUX3(MUX3( x >>11, x >>12, x >>2),\
//                                         AND3( x >>6, x >>2, x >>13),\
//                                         MUX3( x >>1, x >>10, x >>9)))\
//                      )) << 20))

fn step_a0(x: u64, feedin: u64) -> u64 {

    (x >> 1) | 1 & 
    xor3!(
        xor3!(
            feedin, 
            x >> 15,
            xor3!(x >> 3, x >> 2, x)
            ),
        xor3!(
            and2!(x >> 4, x >> 7),
            xor3!(x >> 5, x >> 6, x >> 8),
            mux3!(x >> 4, x >> 5, x >> 6)
            ),
        mux3!(
            mux3!(x >>11, x >>12, x >>2),
            and3!(x >>6, x >>2, x >>13),
            mux3!( x >>1, x >>10, x >>9)
            )
        )
}

pub struct Achterbahn128 {}
