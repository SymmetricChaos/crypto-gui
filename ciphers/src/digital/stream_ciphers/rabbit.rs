use utils::byte_formatting::ByteFormat;

const A: [u32; 8] = [
    0x4D34D34D, 0xD34D34D3, 0x34D34D34, 0x4D34D34D, 0xD34D34D3, 0x34D34D34, 0x4D34D34D, 0xD34D34D3,
];

// Square a u32 to get a u64, then XOR the upper and lower 32 bits to get a u32
fn g_func(n: u32) -> u32 {
    let sq = (n as u64) * (n as u64);
    ((sq >> 32) ^ sq) as u32
}

pub struct Rabbit {
    pub input_format: ByteFormat,
    pub output_format: ByteFormat,
    pub state: [u32; 8],
    pub ctrs: [u32; 8],
    pub carry: u32, // only one bit is used by matching the type makes it easier to use, a bool would likely be aligned similarly anyway
}

impl Default for Rabbit {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            state: Default::default(),
            ctrs: Default::default(),
            carry: 0,
        }
    }
}

impl Rabbit {
    pub fn with_key(key: [u8; 16]) -> Self {
        let mut k = [0_u32; 8];
        for i in 0..8 {
            k[i] = (key[i] as u32) | ((key[i + 1] << 8) as u32)
        }

        let mut state = [0; 8];
        let mut ctrs = [0; 8];

        for i in 0..8 {
            if i % 2 == 0 {
                state[i] = (k[(i + 1) % 8] << 16) | k[i];
                ctrs[i] = (k[(i + 4) % 8] << 16) | k[(i + 5) % 8];
            } else {
                state[i] = (k[(i + 5) % 8] << 16) | k[(i + 4) % 8];
                ctrs[i] = (k[i] << 16) | k[(i + 1) % 8];
            }
        }

        let mut out = Self {
            input_format: ByteFormat::Hex,
            output_format: ByteFormat::Hex,
            state,
            ctrs,
            carry: 0,
        };

        for _ in 0..4 {
            out.step();
        }

        for i in 0..8 {
            out.ctrs[i] ^= out.ctrs[(i + 4) & 0x07]
        }

        out
    }

    fn step(&mut self) {
        // Update all the counters and the carry
        let old_ctrs = self.ctrs.clone();
        self.ctrs[0] = A[0].wrapping_add(self.ctrs[0]).wrapping_add(self.carry);
        for i in 1..8 {
            self.ctrs[i] = A[i]
                .wrapping_add(self.ctrs[i])
                .wrapping_add((self.ctrs[i - 1] < old_ctrs[i - 1]) as u32);
        }
        self.carry = (self.ctrs[7] < old_ctrs[7]) as u32;

        // Calculate g-values
        let mut g = [0_u32; 8];
        for i in 0..8 {
            g[i] = g_func(self.ctrs[i].wrapping_add(self.state[i]));
        }

        // Update the state
        self.state[0] = g[0]
            .wrapping_add(g[7].rotate_left(16))
            .wrapping_add(g[6].rotate_left(16));
        self.state[1] = g[1].wrapping_add(g[0].rotate_left(8)).wrapping_add(g[7]);
        self.state[2] = g[2]
            .wrapping_add(g[1].rotate_left(16))
            .wrapping_add(g[0].rotate_left(16));
        self.state[3] = g[3].wrapping_add(g[2].rotate_left(8)).wrapping_add(g[1]);
        self.state[4] = g[4]
            .wrapping_add(g[3].rotate_left(16))
            .wrapping_add(g[2].rotate_left(16));
        self.state[5] = g[5].wrapping_add(g[4].rotate_left(8)).wrapping_add(g[3]);
        self.state[6] = g[6]
            .wrapping_add(g[5].rotate_left(16))
            .wrapping_add(g[4].rotate_left(16));
        self.state[7] = g[7].wrapping_add(g[6].rotate_left(8)).wrapping_add(g[5]);
    }
}
