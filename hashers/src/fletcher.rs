use crate::traits::{ResettableHasher, StatefulHasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FletcherhWidth {
    W16,
    W32,
    W64,
}

pub struct Fletcher {
    state: [u32; 2],
    width: FletcherhWidth,
}

impl Fletcher {
    pub fn init(width: FletcherhWidth) -> Self {
        Self {
            state: [0; 2],
            width,
        }
    }

    pub fn w16() -> Self {
        Self::init(FletcherhWidth::W16)
    }

    pub fn w32() -> Self {
        Self::init(FletcherhWidth::W32)
    }

    pub fn w64() -> Self {
        Self::init(FletcherhWidth::W64)
    }
}

impl StatefulHasher for Fletcher {
    fn update(&mut self, bytes: &[u8]) {
        let m = match self.width {
            FletcherhWidth::W16 => u8::MAX as u32,
            FletcherhWidth::W32 => u16::MAX as u32,
            FletcherhWidth::W64 => u32::MAX,
        };

        for byte in bytes {
            self.state[0] = self.state[0].wrapping_add(*byte as u32);
            self.state[1] = self.state[1].wrapping_add(self.state[0]);
        }

        self.state[0] = self.state[0] % m;
        self.state[1] = self.state[1] % m;
    }

    fn finalize(self) -> Vec<u8> {
        match self.width {
            FletcherhWidth::W16 => vec![self.state[1] as u8, self.state[0] as u8],
            FletcherhWidth::W32 => [self.state[1] as u16, self.state[0] as u16]
                .iter()
                .flat_map(|w| w.to_be_bytes())
                .collect(),
            FletcherhWidth::W64 => [self.state[1], self.state[0]]
                .iter()
                .flat_map(|w| w.to_be_bytes())
                .collect(),
        }
    }

    crate::stateful_hash_helpers!();
}

impl ResettableHasher for Fletcher {
    fn finalize_and_reset(&mut self) -> Vec<u8> {
        let out = match self.width {
            FletcherhWidth::W16 => vec![self.state[1] as u8, self.state[0] as u8],
            FletcherhWidth::W32 => [self.state[1] as u16, self.state[0] as u16]
                .iter()
                .flat_map(|w| w.to_be_bytes())
                .collect(),
            FletcherhWidth::W64 => [self.state[1], self.state[0]]
                .iter()
                .flat_map(|w| w.to_be_bytes())
                .collect(),
        };
        self.state = [0; 2];
        out
    }
}

crate::stateful_hash_tests!(
    test1,
    Fletcher::w16(),
    b"abcde",
    "c8f0";

    test2,
    Fletcher::w16(),
    b"abcdef",
    "2057";

    test3,
    Fletcher::w16(),
    b"abcdefgh",
    "0627";
);
