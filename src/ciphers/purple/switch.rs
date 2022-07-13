use super::wiring::*;
 
#[derive(PartialEq, Clone, Copy)]
pub enum SwitchSpeed {
    Slow,
    Middle,
    Fast
}
 
#[derive(Clone, Copy)]
pub struct Switch<const N: usize> {
    pub position: usize,
    pub speed: SwitchSpeed,
    wiring_enc: &'static [[usize; N]; 25],
    wiring_dec: &'static [[usize; N]; 25],
}
 
impl<const N: usize> Switch<N> {
 
    pub fn new(speed: SwitchSpeed, wiring_enc: &'static [[usize; N]; 25], wiring_dec: &'static [[usize; N]; 25]) -> Switch<N> {
        Self { position: 0, speed, wiring_enc, wiring_dec }
    }
 
    pub fn sixes() -> Switch<6_usize> {
        Switch::new(SwitchSpeed::Fast, &SIXES_ENC, &SIXES_DEC)
    }
 
    pub fn twenties() -> [Switch<20_usize>; 3] {
 
        let t1 = Switch::new(SwitchSpeed::Slow, &TWENTIES_1_ENC, &TWENTIES_1_DEC);
        let t2 = Switch::new(SwitchSpeed::Middle, &TWENTIES_2_ENC, &TWENTIES_2_DEC);
        let t3 = Switch::new(SwitchSpeed::Fast, &TWENTIES_3_ENC, &TWENTIES_3_DEC);
 
        [t1, t2, t3]
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % 25
    }
 
    pub fn encrypt(&self, n: usize) -> usize {
        self.wiring_enc[self.position][n]
    }
 
    pub fn decrypt(&self, n: usize) -> usize {
        self.wiring_dec[self.position][n]
    }
}