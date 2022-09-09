use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::wiring::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SwitchSpeed {
    Slow,
    Middle,
    Fast,
}

#[derive(Clone, Copy)]
pub struct Switch<const N: usize> {
    pub position: usize,
    pub speed: SwitchSpeed,
    wiring_enc: &'static [[usize; N]; 25],
    wiring_dec: &'static [[usize; N]; 25],
}

impl Switch<6> {
    pub fn sixes() -> Switch<6> {
        Switch::new(8, SwitchSpeed::Fast, &SIXES_ENC, &SIXES_DEC)
    }
}

impl Switch<20> {
    pub fn twenties() -> [Rc<RefCell<Switch<20>>>; 3] {
        let t1 = Rc::new(RefCell::new(Switch::new(
            0,
            SwitchSpeed::Slow,
            &TWENTIES_1_ENC,
            &TWENTIES_1_DEC,
        )));
        let t2 = Rc::new(RefCell::new(Switch::new(
            23,
            SwitchSpeed::Fast,
            &TWENTIES_2_ENC,
            &TWENTIES_2_DEC,
        )));
        let t3 = Rc::new(RefCell::new(Switch::new(
            5,
            SwitchSpeed::Middle,
            &TWENTIES_3_ENC,
            &TWENTIES_3_DEC,
        )));

        [t1, t2, t3]
    }
}

impl<const N: usize> Switch<N> {
    pub fn new(
        position: usize,
        speed: SwitchSpeed,
        wiring_enc: &'static [[usize; N]; 25],
        wiring_dec: &'static [[usize; N]; 25],
    ) -> Switch<N> {
        Self {
            position,
            speed,
            wiring_enc,
            wiring_dec,
        }
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

impl<const N: usize> Display for Switch<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.wiring_enc[self.position])
    }
}

#[cfg(test)]
mod purple_switch_tests {
    use super::*;

    #[test]
    fn sixes_encrypt() {
        let mut sixes = Switch::sixes();
        sixes.position = 0;
        assert_eq!(sixes.encrypt(0), 1);
        assert_eq!(sixes.encrypt(1), 0);
        assert_eq!(sixes.encrypt(2), 2);
        assert_eq!(sixes.encrypt(3), 4);
        assert_eq!(sixes.encrypt(4), 3);
        assert_eq!(sixes.encrypt(5), 5);

        sixes.step();
        assert_eq!(sixes.encrypt(0), 4);
        assert_eq!(sixes.encrypt(1), 3);
        assert_eq!(sixes.encrypt(2), 1);
        assert_eq!(sixes.encrypt(3), 5);
        assert_eq!(sixes.encrypt(4), 2);
        assert_eq!(sixes.encrypt(5), 0);

        sixes.position = 24;
        assert_eq!(sixes.encrypt(0), 5);
        assert_eq!(sixes.encrypt(1), 1);
        assert_eq!(sixes.encrypt(2), 3);
        assert_eq!(sixes.encrypt(3), 2);
        assert_eq!(sixes.encrypt(4), 0);
        assert_eq!(sixes.encrypt(5), 4);
    }

    #[test]
    fn sixes_decrypt() {
        let mut sixes = Switch::sixes();
        sixes.position = 0;
        assert_eq!(sixes.decrypt(0), 1);
        assert_eq!(sixes.decrypt(1), 0);
        assert_eq!(sixes.decrypt(2), 2);
        assert_eq!(sixes.decrypt(3), 4);
        assert_eq!(sixes.decrypt(4), 3);
        assert_eq!(sixes.decrypt(5), 5);

        sixes.step();
        assert_eq!(sixes.decrypt(0), 5);
        assert_eq!(sixes.decrypt(1), 2);
        assert_eq!(sixes.decrypt(2), 4);
        assert_eq!(sixes.decrypt(3), 1);
        assert_eq!(sixes.decrypt(4), 0);
        assert_eq!(sixes.decrypt(5), 3);

        sixes.position = 24;
        assert_eq!(sixes.decrypt(0), 4);
        assert_eq!(sixes.decrypt(1), 1);
        assert_eq!(sixes.decrypt(2), 3);
        assert_eq!(sixes.decrypt(3), 2);
        assert_eq!(sixes.decrypt(4), 5);
        assert_eq!(sixes.decrypt(5), 0);
    }
}
