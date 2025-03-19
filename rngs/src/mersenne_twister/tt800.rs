use crate::traits::ClassicRng;

pub const N: usize = 25;
pub const M: usize = 7;
pub const A: [u32; 2] = [0, 0x8ebfd028];

const DEFAULT_ARRAY: [u32; 25] = [
    0x95f24dab, 0x0b685215, 0xe76ccae7, 0xaf3ec239, 0x715fad23, 0x24a590ad, 0x69e4b5ef, 0xbf456141,
    0x96bc1b7b, 0xa7bdf825, 0xc1de75b7, 0x8858a9c9, 0x2da87693, 0xb657f9dd, 0xffdc8a9f, 0x8121da71,
    0x8b823ecb, 0x885d05f5, 0x4e20cd47, 0x5a9ad5d9, 0x512c0c03, 0xea857ccd, 0x4cc1d30f, 0x8891a8a1,
    0xa6b7aadb,
];

pub struct Tt800 {
    pub index: usize,
    pub arr: [u32; N],
}

impl Default for Tt800 {
    fn default() -> Self {
        Self {
            index: 0,
            arr: DEFAULT_ARRAY,
        }
    }
}

impl Tt800 {
    pub fn ksa_default(&mut self) {
        self.index = 0;
        self.arr = DEFAULT_ARRAY;
    }

    // Not defined by Matsumoto but copied from MT19937-32
    pub fn from_u32(key: u32) -> Self {
        let mut arr = [0u32; N];
        let index = 0;
        arr[0] = key; // default key
        for i in 1..N {
            arr[i] = 1812433253_u32
                .wrapping_mul(arr[i - 1] ^ (arr[i - 1] >> 30))
                .wrapping_add(i as u32)
        }
        Self { index, arr }
    }

    pub fn from_array(key: &[u32]) -> Self {
        let mut rng = Self {
            index: 0,
            arr: [0; 25],
        };
        for (i, word) in key.iter().take(25).enumerate() {
            rng.arr[i] = *word;
        }
        rng
    }

    pub fn twist(&mut self) {
        for i in 0..(N - M) {
            let x = self.arr[i];
            self.arr[i] = self.arr[i + M] ^ (x >> 1) ^ A[(x % 2) as usize];
        }
        for i in (N - M)..N {
            let x = self.arr[i];
            self.arr[i] = self.arr[i + M - N] ^ (x >> 1) ^ A[(x % 2) as usize];
        }
    }

    pub fn temper(mut x: u32) -> u32 {
        x ^= (x << 7) & 0x2b5b2500;
        x ^= (x << 15) & 0xdb8b0000;
        x ^= x >> 16; // added in 1996, the original 1994 definition did not include this
        x
    }

    // pub fn untemper(mut x: u32) -> u32 {
    //     x ^= x >> 18;

    //     x ^= (x << 15) & 0x2fc60000;
    //     x ^= (x << 15) & 0xc0000000;

    //     x ^= (x << 7) & 0x00001680;
    //     x ^= (x << 7) & 0x000c4000;
    //     x ^= (x << 7) & 0x0d200000;
    //     x ^= (x << 7) & 0x90000000;

    //     x ^= x >> 11;
    //     x ^= x >> 22;

    //     x
    // }
}

impl ClassicRng for Tt800 {
    fn next_u32(&mut self) -> u32 {
        // index should never be zero here but if it is use the default key schedule
        if self.index == 0 {
            self.ksa_default()
        }
        if self.index >= N {
            self.twist();
            self.index = 0;
        }
        let y = Self::temper(self.arr[self.index]);
        self.index += 1;
        y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_fifty() {
        let fifty: [u32; 50] = [
            3169973338, 2724982910, 347012937, 1735893326, 2282497071, 3975116866, 62755666,
            500522132, 129776071, 1978109378, 4040131704, 3800592193, 3057303977, 1468369496,
            370579849, 3630178833, 51910867, 819270944, 476180518, 190380673, 1370447020,
            1620916304, 663482756, 1354889312, 4000276916, 868393086, 1441698743, 1086138563,
            1899869374, 3717419747, 2455034041, 2617437696, 1595651084, 4148285605, 1860328467,
            928897371, 263340857, 4091726170, 2359987311, 1669697327, 1882626857, 1635656338,
            897501559, 3233276032, 373770970, 2950632840, 2706386845, 3294066568, 3819538748,
            1902519841,
        ];
        let mut rng = Tt800::default();
        for i in 0..50 {
            assert_eq!(fifty[i], rng.next_u32(), "{}", i)
        }
    }

    #[ignore = "long test, determines that default setting is not a u32"]
    #[test]
    fn derive_default_from_u32() {
        for i in 0..=u32::MAX {
            let a = Tt800::from_u32(i).arr;
            if a == DEFAULT_ARRAY {
                println!("default key is {i}");
                return;
            }
        }
        println!("default key is not a u32");
    }
}
