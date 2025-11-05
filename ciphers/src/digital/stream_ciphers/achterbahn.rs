const LENS: [usize; 13] = [21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33];
const MASKS: [u64; 13] = [
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

fn step_a0(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 15, xor3!(x >> 3, x >> 2, x)),
                xor3!(
                    and2!(x >> 4, x >> 7),
                    xor3!(x >> 5, x >> 6, x >> 8),
                    mux3!(x >> 4, x >> 5, x >> 6)
                ),
                mux3!(
                    mux3!(x >> 11, x >> 12, x >> 2),
                    and3!(x >> 6, x >> 2, x >> 13),
                    mux3!(x >> 1, x >> 10, x >> 9)
                )
            )))
            << 20)
}

fn step_a1(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 15, xor3!(x >> 8, x >> 5, x)),
                xor3!(
                    and2!(x >> 5, x >> 11),
                    mux3!(x >> 13, x >> 3, x >> 1),
                    mux3!(x >> 6, x >> 4, x >> 12)
                ),
                mux3!(
                    mux3!(x >> 1, x >> 9, x >> 7),
                    mux3!(x >> 4, x >> 12, x >> 10),
                    and3!(x >> 1, x >> 11, x >> 14)
                )
            )))
            << 21)
}

fn step_a2(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 16, xor3!(x >> 13, x >> 4, x)),
                xor3!(
                    and2!(x >> 12, x >> 14),
                    mux3!(x >> 1, x >> 9, x >> 7),
                    mux3!(x >> 1, x >> 4, x >> 6)
                ),
                mux3!(
                    mux3!(x >> 5, x >> 8, x >> 11),
                    mux3!(x >> 10, x >> 3, x >> 11),
                    and3!(x >> 1, x >> 9, x >> 15)
                )
            )))
            << 22)
}

fn step_a3(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 18, xor3!(x >> 8, x >> 3, x)),
                xor3!(
                    and2!(x >> 1, x >> 11),
                    mux3!(x >> 2, x >> 14, x >> 13),
                    mux3!(x >> 12, x >> 4, x >> 13)
                ),
                mux3!(
                    mux3!(x >> 6, x >> 1, x >> 15),
                    mux3!(x >> 14, x >> 16, x >> 9),
                    maj3!(x >> 2, x >> 5, x >> 7)
                )
            )))
            << 23)
}

fn step_a4(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 20, xor3!(x >> 11, x >> 1, x)),
                xor3!(
                    and2!(x >> 4, x >> 12),
                    mux3!(x >> 1, x >> 3, x >> 5),
                    mux3!(x >> 6, x >> 7, x >> 16)
                ),
                mux3!(
                    maj3!(x >> 8, x >> 15, x >> 17),
                    mux3!(x >> 14, x >> 13, x >> 12),
                    mux3!(x >> 5, x >> 3, x >> 2)
                )
            )))
            << 24)
}

fn step_a5(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 21, xor3!(x >> 17, x >> 16, x >> 15)),
                xor3!(
                    xor3!(x >> 5, x >> 4, x),
                    and2!(x >> 3, x >> 6),
                    mux3!(x >> 4, x >> 18, x >> 2)
                ),
                mux3!(
                    mux3!(x >> 4, x >> 12, x >> 13),
                    mux3!(x >> 14, x >> 11, x >> 7),
                    maj3!(x >> 3, x >> 10, x >> 15)
                )
            )))
            << 25)
}

fn step_a6(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 25, xor3!(x >> 15, x >> 4, x)),
                xor3!(
                    and2!(x >> 1, x >> 12),
                    mux3!(x >> 10, x >> 6, x >> 17),
                    mux3!(x >> 3, x >> 8, x >> 1)
                ),
                mux3!(
                    mux3!(x >> 10, x >> 14, x >> 13),
                    maj3!(x >> 17, x >> 2, x >> 16),
                    and3!(x >> 18, x >> 11, x >> 5)
                )
            )))
            << 26)
}

fn step_a7(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 25, xor3!(x >> 18, x >> 5, x)),
                xor3!(
                    and2!(x >> 4, x >> 12),
                    mux3!(x >> 1, x >> 17, x >> 2),
                    mux3!(x >> 20, x >> 14, x >> 16)
                ),
                mux3!(
                    mux3!(x >> 18, x >> 15, x >> 10),
                    and3!(x >> 1, x >> 2, x >> 13),
                    and3!(x >> 7, x >> 9, x >> 19)
                )
            )))
            << 27)
}

fn step_a8(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 24, xor3!(x >> 21, x >> 18, x >> 17)),
                xor3!(
                    and2!(x >> 1, x >> 4),
                    xor3!(x >> 11, x >> 2, x),
                    mux3!(x >> 10, x >> 8, x >> 21)
                ),
                mux3!(
                    and3!(x >> 8, x >> 18, x >> 9),
                    mux3!(x >> 13, x >> 6, x >> 15),
                    mux3!(x >> 19, x >> 16, x >> 14)
                )
            )))
            << 28)
}
fn step_a9(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 28, xor3!(x >> 18, x >> 1, x)),
                xor3!(
                    and2!(x >> 2, x >> 8),
                    mux3!(x >> 12, x >> 19, x >> 10),
                    mux3!(x >> 10, x >> 14, x >> 22)
                ),
                mux3!(
                    mux3!(x >> 7, x >> 18, x >> 4),
                    maj3!(x >> 21, x >> 9, x >> 1),
                    maj3!(x >> 8, x >> 5, x >> 3)
                )
            )))
            << 29)
}
fn step_a10(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 25, xor3!(x >> 18, x >> 15, x >> 6)),
                xor3!(
                    xor3!(x >> 5, x >> 2, x),
                    and2!(x >> 19, x >> 14),
                    mux3!(x >> 17, x >> 12, x >> 21)
                ),
                mux3!(
                    mux3!(x >> 20, x >> 18, x >> 8),
                    maj3!(x >> 4, x >> 12, x >> 19),
                    mux3!(x >> 22, x >> 7, x >> 21)
                )
            )))
            << 30)
}

fn step_a11(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 28, xor3!(x >> 22, x >> 17, x >> 8)),
                xor3!(
                    and2!(x >> 13, x >> 15),
                    xor3!(x >> 5, x >> 3, x),
                    mux3!(x >> 5, x >> 7, x >> 19)
                ),
                mux3!(
                    mux3!(x >> 8, x >> 2, x >> 13),
                    and3!(x >> 4, x >> 11, x >> 24),
                    mux3!(x >> 12, x >> 14, x >> 7)
                )
            )))
            << 31)
}

fn step_a12(x: u64, feedin: u64) -> u64 {
    (x >> 1)
        | ((1
            & (xor3!(
                xor3!(feedin, x >> 30, xor3!(x >> 23, x >> 10, x >> 9)),
                xor3!(
                    xor3!(x >> 7, x >> 2, x),
                    and2!(x >> 15, x >> 16),
                    mux3!(x >> 25, x >> 15, x >> 13)
                ),
                mux3!(
                    mux3!(x >> 15, x >> 12, x >> 16),
                    maj3!(x >> 14, x >> 1, x >> 18),
                    mux3!(x >> 8, x >> 24, x >> 17)
                )
            )))
            << 32)
}

pub struct Achterbahn128 {
    nlfsrs: [u64; 13],
}

impl Achterbahn128 {
    pub fn step_all(&mut self, feedin: u64) {
        self.nlfsrs[0] = step_a0(self.nlfsrs[0], feedin);
        self.nlfsrs[1] = step_a1(self.nlfsrs[1], feedin);
        self.nlfsrs[2] = step_a2(self.nlfsrs[2], feedin);
        self.nlfsrs[3] = step_a3(self.nlfsrs[3], feedin);
        self.nlfsrs[4] = step_a4(self.nlfsrs[4], feedin);
        self.nlfsrs[5] = step_a5(self.nlfsrs[5], feedin);
        self.nlfsrs[6] = step_a6(self.nlfsrs[6], feedin);
        self.nlfsrs[7] = step_a7(self.nlfsrs[7], feedin);
        self.nlfsrs[8] = step_a8(self.nlfsrs[8], feedin);
        self.nlfsrs[9] = step_a9(self.nlfsrs[9], feedin);
        self.nlfsrs[10] = step_a10(self.nlfsrs[10], feedin);
        self.nlfsrs[11] = step_a11(self.nlfsrs[11], feedin);
        self.nlfsrs[12] = step_a12(self.nlfsrs[12], feedin);
    }

    pub fn ksa(&mut self, key: [u8; 16], iv: [u8; 16]) {
        // 1: Load all NLFSRs with the first key bits
        let key33 = (key[0] as u64)
            | (key[1] as u64) << 8
            | (key[2] as u64) << 16
            | (key[3] as u64) << 24
            | (key[4] as u64) << 32;
        self.nlfsrs[0] = step_a0(self.nlfsrs[0], key33 & MASKS[0]);
        self.nlfsrs[1] = step_a1(self.nlfsrs[1], key33 & MASKS[1]);
        self.nlfsrs[2] = step_a2(self.nlfsrs[2], key33 & MASKS[2]);
        self.nlfsrs[3] = step_a3(self.nlfsrs[3], key33 & MASKS[3]);
        self.nlfsrs[4] = step_a4(self.nlfsrs[4], key33 & MASKS[4]);
        self.nlfsrs[5] = step_a5(self.nlfsrs[5], key33 & MASKS[5]);
        self.nlfsrs[6] = step_a6(self.nlfsrs[6], key33 & MASKS[6]);
        self.nlfsrs[7] = step_a7(self.nlfsrs[7], key33 & MASKS[7]);
        self.nlfsrs[8] = step_a8(self.nlfsrs[8], key33 & MASKS[8]);
        self.nlfsrs[9] = step_a9(self.nlfsrs[9], key33 & MASKS[9]);
        self.nlfsrs[10] = step_a10(self.nlfsrs[10], key33 & MASKS[10]);
        self.nlfsrs[11] = step_a11(self.nlfsrs[11], key33 & MASKS[11]);
        self.nlfsrs[12] = step_a12(self.nlfsrs[12], key33 & MASKS[12]);

        // 2: For each NLFSRS feed-in the key bits not loaded in step 1

        // 3: for each NLFSR feed-in all IV bits

        // 4: for each NLFSR feed-in the keystream output

        // 5: set the least significant bit of each NLFSR to 1
        for nlfsr in self.nlfsrs.iter_mut() {
            *nlfsr |= 1;
        }

        // 6: warm up
        for _ in 0..64 {
            self.step_all(0);
        }
    }
}
