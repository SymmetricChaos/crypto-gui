#[derive(PartialEq, Clone, Copy)]
pub enum SwitchSpeed {
    Slow,
    Middle,
    Fast
}

#[derive(Clone, Copy)]
pub struct Switch {
    pub position: usize,
    pub speed: SwitchSpeed,
    pub wiring_enc: &'static [usize],
    pub wiring_dec: &'static [usize],
}

impl Switch {
    pub fn step(&mut self) {

    }

    pub fn encrypt(&self, n: usize) -> usize {

    }

    pub fn decrypt(&self, n: usize) -> usize {
        
    }
}