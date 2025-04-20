use crate::traits::StatefulHasher;

const WORDS: usize = 16;
const ROUNDS: usize = 80;
const SUBKEYS: usize = ROUNDS / 4 + 1;

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
            W(0x5102B6B8C1894A35),
            W(0xFEEBC9E3FE8AF11A),
            W(0x0C807F06E32BED71),
            W(0x60C13A52B41A91F6),
            W(0x9716D35DD4917C38),
            W(0xE780DF126FD31D3A),
            W(0x797846B6C898303A),
            W(0xB172C2A8B3572A3B),
            W(0xC9BC8203A6104A6C),
            W(0x65909338D75624F4),
            W(0x94BCC5684B3F81A0),
            W(0x3EBBF51E10ECFD46),
            W(0x2DF50F0BEEB08542),
            W(0x3B5A65300DBC6516),
            W(0x484B9CD2167BBCE1),
            W(0x2D136947D4CBAFEA),
        ])
    }

    pub fn init_512() -> Self {
        Self::init([
            W(0xCAEC0E5D7C1B1B18),
            W(0xA01B0E045F03E802),
            W(0x33840451ED912885),
            W(0x374AFB04EAEC2E1C),
            W(0xDF25A0E2813581F7),
            W(0xE40040938B12F9D2),
            W(0xA662D539C2ED39B6),
            W(0xFA8B85CF45D8C75A),
            W(0x8316ED8E29EDE796),
            W(0x053289C02E9F91B8),
            W(0xC3F8EF1D6D518B73),
            W(0xBDCEC3C4D5EF332E),
            W(0x549A7E5222974487),
            W(0x670708725B749816),
            W(0xB9CD28FBF0581BD1),
            W(0x0E2940B815804974),
        ])
    }

    pub fn init_1024() -> Self {
        Self::init([
            W(0xD593DA0741E72355),
            W(0x15B5E511AC73E00C),
            W(0x5180E5AEBAF2C4F0),
            W(0x03BD41D3FCBCAFAF),
            W(0x1CAEC6FD1983A898),
            W(0x6E510B8BCDD0589F),
            W(0x77E2BDFDC6394ADA),
            W(0xC11E1DB524DCB0A3),
            W(0xD6D14AF9C6329AB5),
            W(0x6A9B0BFC6EB67E0D),
            W(0x9243C60DCCFF1332),
            W(0x1A1F1DDE743F02D4),
            W(0x0996753C10ED0BB8),
            W(0x6572DD22F2B4969A),
            W(0x61FD3062D00A579A),
            W(0x1DE0536E8682E539),
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
