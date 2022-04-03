pub trait Rotor {
    fn right_to_left(&self, n: usize) -> usize;
    fn left_ro_right(&self, n: usize) -> usize;

    fn name(&self) -> String;
    fn wiring_string(&self) -> String;

    fn size(&self) -> usize;
    fn position(&mut self) -> &mut usize;

    fn step_left(&mut self) {
        *self.position() += 1;
        *self.position() %= self.size()
    }

    fn step_right(&mut self) {
        *self.position() += self.size();
        *self.position() -= 1;
        *self.position() %= self.size()
    }
}