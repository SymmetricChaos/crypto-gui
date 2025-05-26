use super::lfsr::Lfsr64;

fn t2(x: u8) -> u8 {
    let x0 = x & 1;
    let x1 = (x >> 1) & 1;
    (x0 << 1) | (x0 ^ x1)
}

pub struct E0 {
    lfsrs: [Lfsr64; 4],
    delay: [u8; 2],
}

impl Default for E0 {
    fn default() -> Self {
        Self {
            lfsrs: [
                Lfsr64::from_taps(0b1000010000000100010000000),
                Lfsr64::from_taps(0b1000000100000001000100000000000),
                Lfsr64::from_taps(0b100001000100000000000000000001000),
                Lfsr64::from_taps(0b1001000000010000000000000000000000000001000),
            ],
            delay: [0; 2],
        }
    }
}

impl E0 {
    fn fsm(&self, y: u8) -> u8 {
        let s = (y + self.delay[0]) / 2;
        s ^ self.delay[0] ^ t2(self.delay[1])
    }

    fn next_bit(&mut self) -> u8 {
        let x1 = self.lfsrs[0].next_bit() as u8;
        let x2 = self.lfsrs[1].next_bit() as u8;
        let x3 = self.lfsrs[2].next_bit() as u8;
        let x4 = self.lfsrs[3].next_bit() as u8;
        let c0 = self.fsm(x1 + x2 + x3 + x4);

        x1 ^ x2 ^ x3 ^ x4 ^ c0
    }

    pub fn next_byte(&mut self) -> u8 {
        let mut out = 0;
        for _ in 0..8 {
            out <<= 1;
            out |= self.next_bit() as u8;
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_t2() {
        assert_eq!(0b00, t2(0b00));
        assert_eq!(0b11, t2(0b01));
        assert_eq!(0b01, t2(0b10));
        assert_eq!(0b10, t2(0b11));
    }
}
