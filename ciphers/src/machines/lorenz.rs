use crate::{Cipher, CipherError};
use bimap::BiMap;
use std::ops::Not;
use utils::text_functions::string_chunks;

const WIDTH: usize = 5;

const LETTERS: &'static str = "\0E\nA SIU\rDRJNFCKTZLWHYPQOBG␎MXV␏";
const FIGURES: &'static str = "\03\n- '87\r␅4␇,!:(5+)2£6019?&␎./=␏";
const CYBER_CHEF: &'static str = "/E3A9SIU4DRJNFCKTZLWHYPQOBG5MXV8"; // I only know of this mapping from the GCHQ Cyber Chef

const CODES: [&'static str; 32] = [
    "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
    "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010", "10011",
    "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011", "11100", "11101",
    "11110", "11111",
];

// const CODES_INV: [&'static str; 32] = [
//     "00000", "10000", "01000", "11000", "00100", "10100", "01100", "11100", "00010", "10010",
//     "01010", "11010", "00110", "10110", "01110", "11110", "00001", "10001", "01001", "11001",
//     "00101", "10101", "01101", "11101", "00011", "10011", "01011", "11011", "00111", "10111",
//     "01111", "11111",
// ];

static LETTER_MAP: std::sync::LazyLock<BiMap<char, &'static str>> =
    std::sync::LazyLock::new(|| {
        utils::text_functions::bimap_from_iter(LETTERS.chars().zip(CODES.into_iter()))
    });
static FIGURE_MAP: std::sync::LazyLock<BiMap<char, &'static str>> =
    std::sync::LazyLock::new(|| {
        utils::text_functions::bimap_from_iter(FIGURES.chars().zip(CODES.into_iter()))
    });
static CYBER_CHEF_MAP: std::sync::LazyLock<BiMap<char, &'static str>> =
    std::sync::LazyLock::new(|| {
        utils::text_functions::bimap_from_iter(CYBER_CHEF.chars().zip(CODES.into_iter()))
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

fn map_inv_cyber_chef(k: &str) -> Option<char> {
    CYBER_CHEF_MAP.get_by_right(k).cloned()
}

/// Uses doubled codes for figure and letter shift, following Cyber Chef
pub fn encode_ita2(text: &str) -> Result<String, CipherError> {
    let mut mode = Mode::Letters;
    let mut out = String::with_capacity(text.len() * WIDTH);
    for s in text.chars().map(|c| c.to_ascii_uppercase()) {
        if s == '␎' {
            out.push_str("11011");
            out.push_str("11011");
            mode = Mode::Figures;
            continue;
        }
        if s == '␏' {
            out.push_str("11111");
            out.push_str("11111");
            mode = Mode::Letters;
            continue;
        }
        if mode == Mode::Figures && s == ' ' {
            out.push_str("11111");
            out.push_str("11111");
            out.push_str("00100");
            mode = Mode::Letters;
            continue;
        }
        match map(&s, mode) {
            Some(code_group) => out.push_str(code_group),
            None => match map(&s, !mode) {
                Some(code_group) => {
                    out.push_str(mode.shift());
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

pub fn decode_ita2(text: &str) -> Result<String, CipherError> {
    let mut mode = Mode::Letters;
    let mut out = String::with_capacity(text.len() / WIDTH);
    for group in string_chunks(&text.replace(' ', ""), WIDTH) {
        // Note that repeated shifts of the same kind are the same as a single shift
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
                    "The code group `{}` is not valid in ITA2",
                    group
                )))
            }
        }
    }

    Ok(out)
}

pub fn decode_ita2_cyber_chef(text: &str) -> Result<String, CipherError> {
    let mut out = String::with_capacity(text.len() / WIDTH);
    for group in string_chunks(&text.replace(' ', ""), WIDTH) {
        match map_inv_cyber_chef(&group) {
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

    /// Reverses pins but not position
    pub fn reverse(&mut self) {
        self.pins = self.pins.iter().copied().rev().collect();
    }

    /// Clone with reversed pins but same position
    pub fn reversed(&self) -> Self {
        Self {
            pins: self.pins.iter().copied().rev().collect(),
            position: self.position,
        }
    }

    pub fn print_pins(&self) -> String {
        self.pins
            .iter()
            .map(|b| if *b { 'x' } else { '.' })
            .collect()
    }

    pub fn print_current_pins(&self) -> String {
        let t: String = self
            .pins
            .iter()
            .map(|b| if *b { 'x' } else { '.' })
            .collect();
        let mut out = t[self.position..].to_string();
        out.push_str(&t[..self.position]);
        out
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
        Self::new_kh()
    }
}

impl Lorenz {
    pub fn new_kh() -> Self {
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

    pub fn new_bream() -> Self {
        Self {
            psi: [
                Wheel::new("...xxx..xxx.xx..x.x.xx.xx.x..x..x.x.x.x.x..").unwrap(),
                Wheel::new("xx.x..xxx.....xxxx.x..x.xx..xx.x.x.x.x.x.xx.x..").unwrap(),
                Wheel::new("x..x..xx.xxx...xxx....xxxx.x.x.xx..x..x.x.x.x.x.x.x").unwrap(),
                Wheel::new(".x....x..x.xxxxx.xx..xx..xx....x.xx.x.x.x.x.x.xx..x.x").unwrap(),
                Wheel::new("x.x.x..xx..xx.xx..x...x....x.xx.xxxx.xxx..x.x...xx.x.x.x.x.").unwrap(),
            ],
            mu: [
                Wheel::new(".x.x.x.x.x.x.x.xxx.x.x..x.x.x.x.xxx.x").unwrap(),
                Wheel::new("x....xx...xx..xx.xxxx....xx...xx.xx.x.xxxx...xx..xx..xx.x.xxx")
                    .unwrap(),
            ],
            chi: [
                Wheel::new(".xxxx.x.xx.x.xx..x..xx.x....xx....xxxx...").unwrap(),
                Wheel::new(".xxx....x...xx.x.x...xx.xxx..xx").unwrap(),
                Wheel::new("xx..xx.xx..xxx....x..xx.xxx..").unwrap(),
                Wheel::new("xxxx..x..xx..x..xx.x..xx..").unwrap(),
                Wheel::new(".xxx.xxx...x..xx.x...x.").unwrap(),
            ],
        }
    }

    pub fn new_zmug() -> Self {
        Self {
            psi: [
                Wheel::new("xx.x..xx...xxx..xx...xx...xxxx..xxx..xxx...").unwrap(),
                Wheel::new("...x...xxx..xx..xxx...xxxx...xx..xxx..xxx..x.xx").unwrap(),
                Wheel::new(".x..xx..xxx..xxx..x...xxxx...x...xxx...xx...xx..xxx").unwrap(),
                Wheel::new("..xxx..xx..xxx..xxxx...x...xx..xxx..x..xx...xx..xxx.x").unwrap(),
                Wheel::new("x..xxx...x...xxxx..xxx..x..xxxx...xx..xxx..xx..xxx..x...xx.").unwrap(),
            ],
            mu: [
                Wheel::new(".x.x.xx.x.xx.xxx.xxx.xx.x.xxx.xxx.xxx").unwrap(),
                Wheel::new("x.xx.x.xxx.xxx.x.x.xxx.xx.xx.xx.xx.xxx.xxx.xxx.x.x.xxxx.x.x.x")
                    .unwrap(),
            ],
            chi: [
                Wheel::new(".xx.xx...xx.xx..x....xxx..xxx....xxx..xx.").unwrap(),
                Wheel::new("xx.xx....xxx.xxxx.x...xx..xx...").unwrap(),
                Wheel::new("..x..xx...xx...xxx...xx.xxxx.").unwrap(),
                Wheel::new("x.x.x..xx...xx..x.xxx..x.x").unwrap(),
                Wheel::new(".x..xxxx...x.xxx....x.x").unwrap(),
            ],
        }
    }

    pub fn _zmug_setting2(&mut self) {
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
        for c in self.chi.iter_mut() {
            c.step_back();
        }

        // Step all of the Psi wheels once, if and only if M37 is set to an active pin
        if self.mu[0].bit() {
            for p in self.psi.iter_mut() {
                p.step_back();
            }
        }

        // Step Mu61 once
        self.mu[1].step_back();

        // Step Mu37 once, if and only if Mu61 is set to an active pin
        if self.mu[1].bit() {
            self.mu[0].step_back();
        }
    }

    // // Used during testing for settings provided in reverse order
    // pub fn step_sz40_reverse(&mut self) {
    //     // Step all of the Chi wheels once
    //     for w in self.chi.iter_mut() {
    //         w.step();
    //     }

    //     // Step Mu61 once
    //     self.mu[1].step();

    //     // Step all of the Psi wheels once, if and only if M37 is set to an active pin
    //     if self.mu[0].bit() {
    //         for w in self.psi.iter_mut() {
    //             w.step();
    //         }
    //     }

    //     // Step Mu37 once, if and only if Mu61 is set to an active pin
    //     if self.mu[1].bit() {
    //         self.mu[0].step();
    //     }
    // }

    pub fn print_state(&self) {
        println!("p43 {}", self.psi[0].print_current_pins());
        println!("p47 {}", self.psi[1].print_current_pins());
        println!("p51 {}", self.psi[2].print_current_pins());
        println!("p53 {}", self.psi[3].print_current_pins());
        println!("p59 {}", self.psi[4].print_current_pins());
        println!("m37 {}", self.mu[0].print_current_pins());
        println!("m61 {}", self.mu[1].print_current_pins());
        println!("c41 {}", self.chi[0].print_current_pins());
        println!("c31 {}", self.chi[1].print_current_pins());
        println!("c29 {}", self.chi[2].print_current_pins());
        println!("c26 {}", self.chi[3].print_current_pins());
        println!("c23 {}\n", self.chi[4].print_current_pins());
    }

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
        let bits = encode_ita2(text)?;
        let mut out = Vec::new();
        for group in string_chunks(&bits, WIDTH) {
            // self.print_state();
            self.encrypt_group(&group, &mut out);
            self.step_sz40();
        }
        Ok(out
            .into_iter()
            .map(|b| if b == false { '0' } else { '1' })
            .collect())
    }

    fn decrypt_mut(&mut self, text: &str) -> Result<String, CipherError> {
        let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
        let mut out = Vec::new();
        if text.chars().count() % 5 != 0 {
            return Err(CipherError::input("input must be groups of five bits"));
        }
        if text.chars().any(|c| c != '0' && c != '1') {
            return Err(CipherError::input("invalid bit found"));
        }
        for group in string_chunks(&text, WIDTH) {
            self.encrypt_group(&group, &mut out);
            self.step_sz40();
        }
        decode_ita2(
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
        assert_eq!("GIBR", decode_ita2_cyber_chef(&ciphertext).unwrap());
    }

    #[test]
    fn test_short_encrypt_decrypt() {
        let plaintext = "TEST";
        let cipher = Lorenz::default();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        assert_eq!(plaintext, cipher.decrypt(&ciphertext).unwrap())
    }

    #[test]
    fn test_a() {
        // Why does this show such a high rate of matching characters?
        let plaintext = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let cipher = Lorenz::default();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        assert_eq!(
            "D98BJ4TYCGDOTDGOHQMPKEVK9FGA/AJELR4498WZMKRVXQXSRW5I84TLRNFFLNJ9MGAB4I",
            decode_ita2_cyber_chef(&ciphertext).unwrap()
        );
    }

    #[test]
    fn test_long_kh() {
        let plaintext =
            "THIS IS A TEST TRANSMISSION, FROM A LORENZ SZ42 CIPHER ATTACHMENT, USING CYBERCHEF.";
        let cipher = Lorenz::new_kh();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        assert_eq!("GWG8NUPLCXGGPGXJXQWTT9ODEPY5ONQXY9JB5OWPFHAMLSGAOJAKKVNYUDOGTSN9KDAHYBP5U8MIYPMSAFTHWFGZJXTM5SR4L", decode_ita2_cyber_chef(&ciphertext).unwrap());
    }

    #[test]
    fn test_long_zmug() {
        let plaintext =
            "THIS IS A TEST TRANSMISSION, FROM A LORENZ SZ42 CIPHER ATTACHMENT, USING CYBERCHEF.";
        let cipher = Lorenz::new_zmug();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        assert_eq!("JXFCVJS/YWVJ5Y44FUBLHYCHY8LS/MLUPWHTVNCGFG38MLYAG4BLUJATPTV9/EPGYVHTOE5ECZOLB4YOAVKUXD/9YVRLYSARP", decode_ita2_cyber_chef(&ciphertext).unwrap());
    }

    #[test]
    fn test_long_bream() {
        let plaintext =
            "THIS IS A TEST TRANSMISSION, FROM A LORENZ SZ42 CIPHER ATTACHMENT, USING CYBERCHEF.";
        let cipher = Lorenz::new_bream();
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        assert_eq!("R/OSBCINF9QQBHHFPXQ9XYQPLXXOWXD8AXFYEQXWZBDLIMRUSMBP5WAWOMC8XZGPOU4MKW4MBBRKLRFTTKLL3UWQNE4UY8PIC", decode_ita2_cyber_chef(&ciphertext).unwrap());
    }

    #[test]
    fn test_baudot_encode_decode() {
        assert_eq!(
            "ABC123DEF.",
            decode_ita2(&encode_ita2("ABC123DEF.").unwrap()).unwrap()
        );
        assert_eq!(
            "ABC55QWE88DEF55M",
            decode_ita2_cyber_chef(&encode_ita2("ABC123DEF.").unwrap()).unwrap()
        );
        assert_eq!(
            "THIS9IS9A9TEST9TRANSMISSION55N889FROM9A9LORENZ9SZ55RW889CIPHER9ATTACHMENT55N889USING9CYBERCHEF55M",
            decode_ita2_cyber_chef(&encode_ita2("THIS IS A TEST TRANSMISSION, FROM A LORENZ SZ42 CIPHER ATTACHMENT, USING CYBERCHEF.").unwrap()).unwrap()
        );
    }

    #[test]
    fn print_pairs() {
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            println!("{}: {}", c, LETTER_MAP.get_by_left(&c).unwrap())
        }
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
