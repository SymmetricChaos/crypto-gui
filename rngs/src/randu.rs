use crate::ClassicRng;

pub struct Randu {
    state: u32,
}

impl Default for Randu {
    fn default() -> Self {
        Self { state: 1 }
    }
}

impl ClassicRng for Randu {
    fn next_u32(&mut self) -> u32 {
        let out = self.state;
        self.state = ((self.state as u64 * 65539 as u64) % 0x80000000) as u32;
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sequence() {
        let mut rng = Randu::default();
        assert_eq!(1, rng.next_u32());
        assert_eq!(65539, rng.next_u32());
        assert_eq!(393225, rng.next_u32());
        assert_eq!(1769499, rng.next_u32());
        assert_eq!(7077969, rng.next_u32());
        assert_eq!(26542323, rng.next_u32());
        assert_eq!(95552217, rng.next_u32());
        assert_eq!(334432395, rng.next_u32());
        assert_eq!(1146624417, rng.next_u32());
        assert_eq!(1722371299, rng.next_u32());
        assert_eq!(14608041, rng.next_u32());
        assert_eq!(1766175739, rng.next_u32());
    }
}
