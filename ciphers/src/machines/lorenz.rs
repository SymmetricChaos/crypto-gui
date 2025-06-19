use crate::{Cipher, CipherError};

pub const ITA2_LETTERS: &'static str = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG␎MXV␏";
pub const ITA2_FIGURES: &'static str = "␀3␊- '87␍␅4␇,!:(5+)2£6019?&␎./=␏";

pub const CODES: [&'static str; 32] = [
    "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
    "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011",
    "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011", "11100", "11101",
    "11110", "11111",
];

#[derive(Clone, Debug)]
pub struct Wheel {
    pub pins: Vec<bool>,
    pub position: usize,
}

impl Wheel {
    pub fn new(string: &str) -> Result<Self, CipherError> {
        let mut pins = Vec::with_capacity(string.len());
        for c in string.chars() {
            if c == '.' {
                pins.push(false);
            } else if c == 'x' {
                pins.push(true);
            } else {
                return Err(CipherError::input(
                    "only the characters '.' and 'x' are used for setting the pins",
                ));
            }
        }
        Ok(Self { pins, position: 0 })
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.pins.len()
    }

    pub fn step_back(&mut self) {
        self.position = (self.position + self.pins.len() - 1) % self.pins.len()
    }

    pub fn bit(&self) -> bool {
        self.pins[self.position]
    }

    pub fn print_pins(&self) -> String {
        self.pins
            .iter()
            .map(|b| if *b { 'x' } else { '.' })
            .collect()
    }
}

pub struct Lorenz {
    pub wheels: [Wheel; 12],
}

impl Default for Lorenz {
    fn default() -> Self {
        // Equivalent to the KH setting
        Self {
            wheels: [
                // Psi Wheels
                Wheel::new(".x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x").unwrap(),
                Wheel::new(".xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x").unwrap(),
                Wheel::new(".x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x").unwrap(),
                Wheel::new(".xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.").unwrap(),
                Wheel::new("xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.").unwrap(),
                // Mu Wheels
                Wheel::new("x.x.x.x.x.x...x.x.x...x.x.x...x.x....").unwrap(),
                Wheel::new(".xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...")
                    .unwrap(),
                // Chi Wheels
                Wheel::new(".x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx..").unwrap(),
                Wheel::new("x..xxx...x.xxxx..xx..x..xx.xx..").unwrap(),
                Wheel::new("..xx..x.xxx...xx...xx..xx.xx.").unwrap(),
                Wheel::new("xx..x..xxxx..xx.xxx....x..").unwrap(),
                Wheel::new("xx..xx....xxxx.x..x.x..").unwrap(),
            ],
        }
    }
}

impl Lorenz {
    pub fn kh_setting(&mut self) {
        self.wheels = [
            // Psi Wheels
            Wheel::new(".x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x").unwrap(),
            Wheel::new(".xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x").unwrap(),
            Wheel::new(".x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x").unwrap(),
            Wheel::new(".xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.").unwrap(),
            Wheel::new("xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.").unwrap(),
            // Mu Wheels
            Wheel::new("x.x.x.x.x.x...x.x.x...x.x.x...x.x....").unwrap(),
            Wheel::new(".xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...").unwrap(),
            // Chi Wheels
            Wheel::new(".x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx..").unwrap(),
            Wheel::new("x..xxx...x.xxxx..xx..x..xx.xx..").unwrap(),
            Wheel::new("..xx..x.xxx...xx...xx..xx.xx.").unwrap(),
            Wheel::new("xx..x..xxxx..xx.xxx....x..").unwrap(),
            Wheel::new("xx..xx....xxxx.x..x.x..").unwrap(),
        ]
    }

    pub fn bream_setting(&mut self) {
        self.wheels = [
            // Psi Wheels
            Wheel::new("...xxx..xxx.xx..x.x.xx.xx.x..x..x.x.x.x.x..").unwrap(),
            Wheel::new("xx.x..xxx.....xxxx.x..x.xx..xx.x.x.x.x.x.xx.x..").unwrap(),
            Wheel::new("x..x..xx.xxx...xxx....xxxx.x.x.xx..x..x.x.x.x.x.x.x").unwrap(),
            Wheel::new(".x....x..x.xxxxx.xx..xx..xx....x.xx.x.x.x.x.x.xx..x.x").unwrap(),
            Wheel::new("x.x.x..xx..xx.xx..x...x....x.xx.xxxx.xxx..x.x...xx.x.x.x.x.").unwrap(),
            // Mu Wheels
            Wheel::new(".x.x.x.x.x.x.x.xxx.x.x..x.x.x.x.xxx.x").unwrap(),
            Wheel::new("x....xx...xx..xx.xxxx....xx...xx.xx.x.xxxx...xx..xx..xx.x.xxx").unwrap(),
            // Chi Wheels
            Wheel::new(".xxxx.x.xx.x.xx..x..xx.x....xx....xxxx...").unwrap(),
            Wheel::new(".xxx....x...xx.x.x...xx.xxx..xx").unwrap(),
            Wheel::new("xx..xx.xx..xxx....x..xx.xxx..").unwrap(),
            Wheel::new("xxxx..x..xx..x..xx.x..xx..").unwrap(),
            Wheel::new(".xxx.xxx...x..xx.x...x.").unwrap(),
        ]
    }

    pub fn zmug_setting(&mut self) {
        self.wheels = [
            // Psi Wheels
            Wheel::new("xx.x..xx...xxx..xx...xx...xxxx..xxx..xxx...").unwrap(),
            Wheel::new("...x...xxx..xx..xxx...xxxx...xx..xxx..xxx..x.xx").unwrap(),
            Wheel::new(".x..xx..xxx..xxx..x...xxxx...x...xxx...xx...xx..xxx").unwrap(),
            Wheel::new("..xxx..xx..xxx..xxxx...x...xx..xxx..x..xx...xx..xxx.x").unwrap(),
            Wheel::new("x..xxx...x...xxxx..xxx..x..xxxx...xx..xxx..xx..xxx..x...xx.").unwrap(),
            // Mu Wheels
            Wheel::new(".x.x.xx.x.xx.xxx.xxx.xx.x.xxx.xxx.xxx").unwrap(),
            Wheel::new("x.xx.x.xxx.xxx.x.x.xxx.xx.xx.xx.xx.xxx.xxx.xxx.x.x.xxxx.x.x.x").unwrap(),
            // Chi Wheels
            Wheel::new(".xx.xx...xx.xx..x....xxx..xxx....xxx..xx.").unwrap(),
            Wheel::new("xx.xx....xxx.xxxx.x...xx..xx...").unwrap(),
            Wheel::new("..x..xx...xx...xxx...xx.xxxx.").unwrap(),
            Wheel::new("x.x.x..xx...xx..x.xxx..x.x").unwrap(),
            Wheel::new(".x..xxxx...x.xxx....x.x").unwrap(),
        ]
    }

    pub fn step_sz40(&mut self) {
        // Step all of the Chi wheels once
        self.wheels[11].step();
        self.wheels[10].step();
        self.wheels[9].step();
        self.wheels[8].step();
        self.wheels[7].step();

        // Step Mu61 once
        self.wheels[6].step();

        // Step Mu37 once, if and only if Mu61 is set to an active pin
        if self.wheels[6].bit() {
            self.wheels[5].step();
        }

        // Step all of the Psi wheel once, if and only if M37 is set to an active pin
        if self.wheels[5].bit() {
            self.wheels[4].step();
            self.wheels[3].step();
            self.wheels[2].step();
            self.wheels[1].step();
            self.wheels[0].step();
        }
    }
}

impl Cipher for Lorenz {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}
