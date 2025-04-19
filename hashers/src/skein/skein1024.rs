use crate::traits::StatefulHasher;

const WORDS: usize = 16;
const ROUNDS: usize = 80;

pub struct Skein1024 {
    state: [u64; WORDS],
    key: [u64; WORDS],
    tweak: [u64; 2],
    bytes_taken: u64,
}

impl Default for Skein1024 {
    fn default() -> Self {
        Self::init_512()
    }
}

impl Skein1024 {
    fn init(iv: [u64; WORDS]) -> Self {
        Self {
            state: iv,
            key: todo!(),
            tweak: todo!(),
            bytes_taken: 0,
        }
    }

    pub fn init_384() -> Self {
        Self::init([
            0x5102B6B8C1894A35,
            0xFEEBC9E3FE8AF11A,
            0x0C807F06E32BED71,
            0x60C13A52B41A91F6,
            0x9716D35DD4917C38,
            0xE780DF126FD31D3A,
            0x797846B6C898303A,
            0xB172C2A8B3572A3B,
            0xC9BC8203A6104A6C,
            0x65909338D75624F4,
            0x94BCC5684B3F81A0,
            0x3EBBF51E10ECFD46,
            0x2DF50F0BEEB08542,
            0x3B5A65300DBC6516,
            0x484B9CD2167BBCE1,
            0x2D136947D4CBAFEA,
        ])
    }

    pub fn init_512() -> Self {
        Self::init([
            0xCAEC0E5D7C1B1B18,
            0xA01B0E045F03E802,
            0x33840451ED912885,
            0x374AFB04EAEC2E1C,
            0xDF25A0E2813581F7,
            0xE40040938B12F9D2,
            0xA662D539C2ED39B6,
            0xFA8B85CF45D8C75A,
            0x8316ED8E29EDE796,
            0x053289C02E9F91B8,
            0xC3F8EF1D6D518B73,
            0xBDCEC3C4D5EF332E,
            0x549A7E5222974487,
            0x670708725B749816,
            0xB9CD28FBF0581BD1,
            0x0E2940B815804974,
        ])
    }

    pub fn init_1024() -> Self {
        Self::init([
            0xD593DA0741E72355,
            0x15B5E511AC73E00C,
            0x5180E5AEBAF2C4F0,
            0x03BD41D3FCBCAFAF,
            0x1CAEC6FD1983A898,
            0x6E510B8BCDD0589F,
            0x77E2BDFDC6394ADA,
            0xC11E1DB524DCB0A3,
            0xD6D14AF9C6329AB5,
            0x6A9B0BFC6EB67E0D,
            0x9243C60DCCFF1332,
            0x1A1F1DDE743F02D4,
            0x0996753C10ED0BB8,
            0x6572DD22F2B4969A,
            0x61FD3062D00A579A,
            0x1DE0536E8682E539,
        ])
    }
}

impl StatefulHasher for Skein1024 {
    fn update(&mut self, bytes: &[u8]) {
        todo!()
    }

    fn finalize(self) -> Vec<u8> {
        todo!()
    }

    crate::stateful_hash_helpers!();
}
