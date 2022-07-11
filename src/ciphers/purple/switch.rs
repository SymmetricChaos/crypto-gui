#[derive(PartialEq)]
pub enum SwitchSpeed {
    Slow,
    Middle,
    Fast
}
 
pub struct Switch {
    pub position: usize,
    pub speed: SwitchSpeed,
    pub wiring: Vec<usize>,
}

impl Switch {
    pub fn step(&mut self) {
        
    }
}