pub trait ClassicRng {
    fn next_u32(&mut self) -> u32;
    // Default method provided for generators that cannot make a u64 directly
    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
    }
}
