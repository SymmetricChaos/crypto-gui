#[derive(Clone, Debug)]
pub struct Wheel {
    pins: Vec<bool>,
}

impl Wheel {
    pub fn new(pins: Vec<bool>) -> Self {
        Self { pins }
    }
}

pub struct Lorenz {
    psi: [Wheel; 5],
    mu: [Wheel; 2],
    chi: [Wheel; 5],
}

impl Default for Lorenz {
    fn default() -> Self {
        Self {
            psi: [
                Wheel::new(vec![false; 41]),
                Wheel::new(vec![false; 31]),
                Wheel::new(vec![false; 29]),
                Wheel::new(vec![false; 26]),
                Wheel::new(vec![false; 23]),
            ],
            mu: [Wheel::new(vec![false; 61]), Wheel::new(vec![false; 37])],
            chi: [
                Wheel::new(vec![false; 43]),
                Wheel::new(vec![false; 47]),
                Wheel::new(vec![false; 51]),
                Wheel::new(vec![false; 53]),
                Wheel::new(vec![false; 59]),
            ],
        }
    }
}
