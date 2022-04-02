pub trait Rotor {
    pub fn right_to_left(&self, n: usize) -> usize;
    pub fn left_ro_right(&self, n: usize) -> usize;
    pub fn name(&self) -> String;
    pub fn wiring_string(&self) -> String;
    pub fn size(&self) -> usize;
    pub fn step_left(&mut self);
    pub fn step_right(&mut self);
}