use crate::{Cipher, CipherError};
use bimap::BiMap;
use std::ops::Not;
use utils::text_functions::string_chunks;

const WIDTH: usize = 5;

const LETTERS: &'static str = "␀E␊A SIU␍DRJNFCKTZLWHYPQOBG␎MXV␏";
const FIGURES: &'static str = "␀3␊- '87␍␅4␇,!:(5+)2£6019?&␎./=␏";

const CODES: [&'static str; 32] = [
    "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
    "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011",
    "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011", "11100", "11101",
    "11110", "11111",
];

static LETTER_MAP: std::sync::LazyLock<BiMap<char, &'static str>> =
    std::sync::LazyLock::new(|| {
        utils::text_functions::bimap_from_iter(LETTERS.chars().zip(CODES.into_iter()))
    });
static FIGURE_MAP: std::sync::LazyLock<BiMap<char, &'static str>> =
    std::sync::LazyLock::new(|| {
        utils::text_functions::bimap_from_iter(FIGURES.chars().zip(CODES.into_iter()))
    });

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Letters,
    Figures,
}

impl Not for Mode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Mode::Letters => Mode::Figures,
            Mode::Figures => Mode::Letters,
        }
    }
}

impl Mode {
    pub fn shift(&self) -> &str {
        match self {
            Mode::Letters => "11011",
            Mode::Figures => "11111",
        }
    }
}

fn map(k: &char, mode: Mode) -> Option<&str> {
    let map = match mode {
        Mode::Letters => &LETTER_MAP,
        Mode::Figures => &FIGURE_MAP,
    };
    map.get_by_left(k).cloned()
}

fn map_inv(k: &str, mode: Mode) -> Option<char> {
    let map = match mode {
        Mode::Letters => &LETTER_MAP,
        Mode::Figures => &FIGURE_MAP,
    };
    map.get_by_right(k).cloned()
}

pub fn encode(text: &str) -> Result<String, CipherError> {
    let mut mode = Mode::Letters;
    let mut out = String::with_capacity(text.len() * WIDTH);
    for s in text.chars() {
        if s == '␎' {
            mode = Mode::Figures;
            continue;
        }
        if s == '␏' {
            mode = Mode::Letters;
            continue;
        }
        match map(&s, mode) {
            Some(code_group) => out.push_str(code_group),
            None => match map(&s, !mode) {
                Some(code_group) => {
                    out.push_str(mode.shift());
                    out.push_str(code_group);
                    mode = !mode;
                }
                None => return Err(CipherError::invalid_input_char(s)),
            },
        }
    }
    Ok(out)
}

pub fn decode(text: &str) -> Result<String, CipherError> {
    let mut mode = Mode::Letters;
    let mut out = String::with_capacity(text.len() / WIDTH);
    for group in string_chunks(&text.replace(' ', ""), WIDTH) {
        if group == "11011" {
            mode = Mode::Figures;
            continue;
        }
        if group == "11111" {
            mode = Mode::Letters;
            continue;
        }
        match map_inv(&group, mode) {
            Some(code_group) => out.push(code_group),
            None => {
                return Err(CipherError::Input(format!(
                    "The code group `{}` is not valid",
                    group
                )))
            }
        }
    }

    Ok(out)
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug)]
pub struct Lorenz {
    pub psi: [Wheel; 5],
    pub mu: [Wheel; 2],
    pub chi: [Wheel; 5],
}

impl Default for Lorenz {
    fn default() -> Self {
        // Equivalent to the KH setting
        Self {
            psi: [
                Wheel::new(".x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x").unwrap(),
                Wheel::new(".xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x").unwrap(),
                Wheel::new(".x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x").unwrap(),
                Wheel::new(".xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.").unwrap(),
                Wheel::new("xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.").unwrap(),
            ],
            mu: [
                Wheel::new("x.x.x.x.x.x...x.x.x...x.x.x...x.x....").unwrap(),
                Wheel::new(".xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...")
                    .unwrap(),
            ],
            chi: [
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
    // pub fn kh_setting(&mut self) {
    //     self.wheels = [
    //         // Psi Wheels
    //         Wheel::new(".x...xx.x.x..xxx.x.x.xxxx.x.x.x.x.x..x.xx.x").unwrap(),
    //         Wheel::new(".xx.x.xxx..x.x.x..x.xx.x.xxx.x....x.xx.x.x.x..x").unwrap(),
    //         Wheel::new(".x.x.x..xxx....x.x.xx.x.x.x..xxx.x.x..x.x.xx..x.x.x").unwrap(),
    //         Wheel::new(".xx...xxxxx.x.x.xx...x.xx.x.x..x.x.xx.x..x.x.x.x.x.x.").unwrap(),
    //         Wheel::new("xx...xx.x..x.xx.x...x.x.x.x.x.x.x.x.xx..xxxx.x.x...xx.x..x.").unwrap(),
    //         // Mu Wheels
    //         Wheel::new("x.x.x.x.x.x...x.x.x...x.x.x...x.x....").unwrap(),
    //         Wheel::new(".xxxx.xxxx.xxx.xxxx.xx....xxx.xxxx.xxxx.xxxx.xxxx.xxx.xxxx...").unwrap(),
    //         // Chi Wheels
    //         Wheel::new(".x...xxx.x.xxxx.x...x.x..xxx....xx.xxxx..").unwrap(),
    //         Wheel::new("x..xxx...x.xxxx..xx..x..xx.xx..").unwrap(),
    //         Wheel::new("..xx..x.xxx...xx...xx..xx.xx.").unwrap(),
    //         Wheel::new("xx..x..xxxx..xx.xxx....x..").unwrap(),
    //         Wheel::new("xx..xx....xxxx.x..x.x..").unwrap(),
    //     ]
    // }

    // pub fn bream_setting(&mut self) {
    //     self.wheels = [
    //         // Psi Wheels
    //         Wheel::new("...xxx..xxx.xx..x.x.xx.xx.x..x..x.x.x.x.x..").unwrap(),
    //         Wheel::new("xx.x..xxx.....xxxx.x..x.xx..xx.x.x.x.x.x.xx.x..").unwrap(),
    //         Wheel::new("x..x..xx.xxx...xxx....xxxx.x.x.xx..x..x.x.x.x.x.x.x").unwrap(),
    //         Wheel::new(".x....x..x.xxxxx.xx..xx..xx....x.xx.x.x.x.x.x.xx..x.x").unwrap(),
    //         Wheel::new("x.x.x..xx..xx.xx..x...x....x.xx.xxxx.xxx..x.x...xx.x.x.x.x.").unwrap(),
    //         // Mu Wheels
    //         Wheel::new(".x.x.x.x.x.x.x.xxx.x.x..x.x.x.x.xxx.x").unwrap(),
    //         Wheel::new("x....xx...xx..xx.xxxx....xx...xx.xx.x.xxxx...xx..xx..xx.x.xxx").unwrap(),
    //         // Chi Wheels
    //         Wheel::new(".xxxx.x.xx.x.xx..x..xx.x....xx....xxxx...").unwrap(),
    //         Wheel::new(".xxx....x...xx.x.x...xx.xxx..xx").unwrap(),
    //         Wheel::new("xx..xx.xx..xxx....x..xx.xxx..").unwrap(),
    //         Wheel::new("xxxx..x..xx..x..xx.x..xx..").unwrap(),
    //         Wheel::new(".xxx.xxx...x..xx.x...x.").unwrap(),
    //     ]
    // }

    pub fn zmug_setting(&mut self) {
        self.psi = [
            Wheel::new("xx.x..xx...xxx..xx...xx...xxxx..xxx..xxx...").unwrap(),
            Wheel::new("...x...xxx..xx..xxx...xxxx...xx..xxx..xxx..x.xx").unwrap(),
            Wheel::new(".x..xx..xxx..xxx..x...xxxx...x...xxx...xx...xx..xxx").unwrap(),
            Wheel::new("..xxx..xx..xxx..xxxx...x...xx..xxx..x..xx...xx..xxx.x").unwrap(),
            Wheel::new("x..xxx...x...xxxx..xxx..x..xxxx...xx..xxx..xx..xxx..x...xx.").unwrap(),
        ];
        self.mu = [
            Wheel::new(".x.x.xx.x.xx.xxx.xxx.xx.x.xxx.xxx.xxx").unwrap(),
            Wheel::new("x.xx.x.xxx.xxx.x.x.xxx.xx.xx.xx.xx.xxx.xxx.xxx.x.x.xxxx.x.x.x").unwrap(),
        ];
        self.chi = [
            Wheel::new(".xx.xx...xx.xx..x....xxx..xxx....xxx..xx.").unwrap(),
            Wheel::new("xx.xx....xxx.xxxx.x...xx..xx...").unwrap(),
            Wheel::new("..x..xx...xx...xxx...xx.xxxx.").unwrap(),
            Wheel::new("x.x.x..xx...xx..x.xxx..x.x").unwrap(),
            Wheel::new(".x..xxxx...x.xxx....x.x").unwrap(),
        ];
    }

    pub fn zmug_setting2(&mut self) {
        self.psi = [
            Wheel::new("x...xxx..xxx..xxxx...xx...xx..xxx...xx..x.x").unwrap(),
            Wheel::new(".xx.x..xxx..xxx..xx...xxxx...xxx..xx..xxx...x..").unwrap(),
            Wheel::new(".xxx..xx...xx...xxx...x...xxxx...x..xxx..xxx..xx..x").unwrap(),
            Wheel::new(".x.xxx..xx...xx..x..xxx..xx...x...xxxx..xxx..xx..xxx.").unwrap(),
            Wheel::new("x.xx...x..xxx..xx..xxx..xx...xxxx..x..xxx..xxxx...x...xxx..").unwrap(),
        ];
        self.mu = [
            Wheel::new(".xxx.xxx.xxx.x.xx.xxx.xxx.xx.x.xx.x.x").unwrap(),
            Wheel::new("xx.x.x.xxxx.x.x.xxx.xxx.xxx.xx.xx.xx.xx.xxx.x.x.xxx.xxx.x.xx.").unwrap(),
        ];
        self.chi = [
            Wheel::new("..xx..xxx....xxx..xxx....x..xx.xx...xx.xx").unwrap(),
            Wheel::new("x...xx..xx...x.xxxx.xxx....xx.x").unwrap(),
            Wheel::new("..xxxx.xx...xxx...xx...xx..x.").unwrap(),
            Wheel::new("xx.x..xxx.x..xx...xx..x.x.").unwrap(),
            Wheel::new(".x.x....xxx.x...xxxx..x").unwrap(),
        ];
    }

    pub fn step_sz40(&mut self) {
        // Step all of the Chi wheels once
        for w in self.chi.iter_mut() {
            w.step_back();
        }

        // Step Mu61 once
        self.mu[1].step_back();

        // Step all of the Psi wheels once, if and only if M37 is set to an active pin
        if self.mu[0].bit() {
            for w in self.psi.iter_mut() {
                w.step_back();
            }
        }

        // Step Mu37 once, if and only if Mu61 is set to an active pin
        if self.mu[1].bit() {
            self.mu[0].step_back();
        }
    }

    // pub fn print_masks(&self) {
    //     println!(
    //         "{}{}{}{}{} ^ {}{}{}{}{}",
    //         self.chi[4].bit() as u32,
    //         self.chi[3].bit() as u32,
    //         self.chi[2].bit() as u32,
    //         self.chi[1].bit() as u32,
    //         self.chi[0].bit() as u32,
    //         self.psi[4].bit() as u32,
    //         self.psi[3].bit() as u32,
    //         self.psi[2].bit() as u32,
    //         self.psi[1].bit() as u32,
    //         self.psi[0].bit() as u32
    //     )
    // }

    // fn keystream()

    fn encrypt_group(&self, group: &str, out: &mut Vec<bool>) {
        for (n, bit) in group
            .chars()
            .map(|c| if c == '0' { false } else { true })
            .enumerate()
        {
            out.push(bit ^ self.chi[4 - n].bit() ^ self.psi[4 - n].bit())
        }
    }

    fn encrypt_mut(&mut self, text: &str) -> Result<String, CipherError> {
        let bits = encode(text)?;
        let mut out = Vec::new();
        for group in string_chunks(&bits, WIDTH) {
            self.encrypt_group(&group, &mut out);
            self.step_sz40();
        }
        Ok(out
            .into_iter()
            .map(|b| if b == false { '0' } else { '1' })
            .collect())
    }

    fn decrypt_mut(&mut self, text: &str) -> Result<String, CipherError> {
        let mut out = Vec::new();
        if text.replace(" ", "").chars().count() % 5 != 0 {
            return Err(CipherError::input("input must be groups of five bits"));
        }
        if text.chars().any(|c| c != '0' && c != '1' && c != ' ') {
            return Err(CipherError::input("invalid bit found"));
        }
        for group in string_chunks(&text.replace(" ", ""), WIDTH) {
            for (n, bit) in group
                .chars()
                .map(|c| if c == '0' { false } else { true })
                .enumerate()
            {
                out.push(bit ^ self.chi[4 - n].bit() ^ self.psi[4 - n].bit())
            }
            self.step_sz40();
        }
        decode(
            &out.into_iter()
                .map(|b| if b == false { '0' } else { '1' })
                .collect::<String>(),
        )
    }
}

impl Cipher for Lorenz {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut cipher = self.clone();
        cipher.encrypt_mut(text)
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        let mut cipher = self.clone();
        cipher.decrypt_mut(text)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_short() {
        let plaintext = "TEST";
        let cipher = Lorenz::default();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        println!("{ciphertext}");
        println!("{}", decode(&ciphertext).unwrap());
        assert_eq!("GIBR", decode(&ciphertext).unwrap());
    }

    #[test]
    fn test_long() {
        let plaintext = "THIS IS A TEST TRANSMISSION FROM A LORENZ SZ";
        let cipher = Lorenz::default();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        println!("{ciphertext}");
        println!("{}", decode(&ciphertext).unwrap());
        assert_eq!(
            "GWG8NUPFZ4VKAKD8DCCXXWKOL3NLZ9VYZTUC4JZVCQ44",
            decode(&ciphertext).unwrap()
        );
    }

    #[test]
    fn test_encrypt_decrypt_short() {
        let plaintext = "TEST";
        let cipher = Lorenz::default();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        assert_eq!(plaintext, cipher.decrypt(&ciphertext).unwrap())
    }

    #[test]
    fn test_auto_baudot_shift() {
        println!("{}", encode("ABC123DEF.").unwrap());
        println!(
            "{}",
            decode("00011110010111011011101111001100001111110100100001011011101111100").unwrap()
        );
    }

    // #[test]
    // fn test_short_null() {
    //     let plaintext = "␀␀␀␀␀";
    //     let mut cipher = Lorenz::default();
    //     cipher.zmug_setting2();
    //     let ciphertext = cipher.encrypt(plaintext).unwrap();
    //     assert_eq!(
    //         ciphertext,
    //         format!(
    //             "{:05b}{:05b}{:05b}{:05b}{:05b}",
    //             0b01010 ^ 0b10001,
    //             0b11000 ^ 0b10001,
    //             0b00101 ^ 0b01110,
    //             0b11101 ^ 0b10110,
    //             0b00110 ^ 0b11100
    //         )
    //     );
    //     // print!("{:05b}", 0b01010 ^ 0b10001);
    //     // print!("{:05b}", 0b11000 ^ 0b10001);
    //     // print!("{:05b}", 0b00101 ^ 0b01110);
    //     // print!("{:05b}", 0b11101 ^ 0b10110);
    //     // print!("{:05b}", 0b00110 ^ 0b11100);
    // }
}
