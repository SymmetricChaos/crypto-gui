use crate::Cipher;

pub struct Lfsr {
    pub bits: Vec<Bit>,
    pub taps: Vec<bool>,
}

impl Lfsr {
    pub fn new(taps: Vec<bool>) -> Self {
        Self {
            bits: vec![0; taps.len()],
            taps,
        }
    }

    pub fn next_bit(&mut self) -> Bit {
        let mut next_bit = Bit::zero();
        for (bit, tap) in self.bits.iter().zip(self.taps.iter()) {
            if *tap {
                next_bit ^= *bit;
            }
        }
        self.bits.pop();
        self.bits.insert(0, next_bit);
        next_bit
    }
}

pub struct A51 {
    lfsr1: Lfsr,
    lfsr2: Lfsr,
    lfsr3: Lfsr,
}

impl Default for A51 {
    fn default() -> Self {
        Self {
            lfsr1: Default::default(),
            lfsr2: Default::default(),
            lfsr3: Default::default(),
        }
    }
}

impl Cipher for A51 {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        todo!()
    }
}
