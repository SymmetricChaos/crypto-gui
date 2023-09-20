pub trait ClassicRng {
    // fn next(&mut self) -> u32;
    fn step(&mut self);
}
