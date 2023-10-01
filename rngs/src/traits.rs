pub trait ClassicRng {
    fn step(&mut self) -> u32;
}
