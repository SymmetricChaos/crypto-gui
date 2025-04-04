use crate::{errors::CipherError, traits::Cipher};
use itertools::Itertools;
use std::{
    fmt::{self, Formatter},
    sync::LazyLock,
};
use utils::vecstring::VecString;

#[derive(Copy, Clone, Debug)]
pub struct Cage {
    lugs: [(usize, usize); 27],
}

impl Default for Cage {
    fn default() -> Self {
        Self { lugs: [(0, 0); 27] }
    }
}

impl Cage {
    pub fn set_lugs(&mut self, lugs: [(usize, usize); 27]) {
        self.lugs = lugs
    }
}

impl fmt::Display for Cage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = "Cage\n".to_string();
        for b in self.lugs.chunks(9).collect_vec() {
            for (lug0, lug1) in b {
                let entry = format!("{lug0}-{lug1}  ");
                s.push_str(&entry)
            }
            s.push('\n')
        }
        write!(f, "{s}")
    }
}

#[derive(Clone, Debug)]
pub struct Rotor {
    alphabet: VecString,
    pub pins: Vec<char>,
    pub active: usize,
}

impl Rotor {
    pub fn new(alphabet: &str, active: usize) -> Rotor {
        let alphabet = VecString::unique_from(alphabet);
        Rotor {
            alphabet,
            pins: Vec::new(),
            active,
        }
    }

    pub fn step(&mut self) {
        self.alphabet.rotate_left(1)
    }

    pub fn set_pins(&mut self, pins: &str) -> Result<(), CipherError> {
        for p in pins.chars() {
            if !self.alphabet.contains(p) {
                return Err(CipherError::key(
                    "effective pins must be in the Rotor's alphabet",
                ));
            }
        }
        self.pins = pins.chars().unique().collect();
        Ok(())
    }

    pub fn get_pins(&mut self) -> &mut Vec<char> {
        &mut self.pins
    }

    pub fn set_active(&mut self, c: char) {
        while self
            .alphabet
            .get_char(self.active)
            .expect("active character did not exist")
            != &c
        {
            self.alphabet.rotate_left(1)
        }
    }

    pub fn set_display(&mut self, c: char) {
        while *self.alphabet.front().unwrap() != c {
            self.alphabet.rotate_left(1)
        }
    }

    pub fn get_active(&self) -> char {
        *self.alphabet.get_char(self.active).unwrap()
    }

    pub fn active_is_effective(&self) -> bool {
        self.pins
            .contains(&self.alphabet.get_char(self.active).unwrap())
    }

    pub fn rotor_length(&self) -> usize {
        self.alphabet.len()
    }
}

// This could be simplified since all the real rotors used ASCII characters but this library tries to work with Unicode as much as possible
impl fmt::Display for Rotor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (pos, letter) in self.alphabet.iter().enumerate() {
            if pos == self.active {
                // bracket the active position
                s.push_str(&format!("[{letter}]"));
            } else {
                s.push(*letter)
            }
        }
        // s.push_str(&format!(" ({})", self.pins.iter().collect::<String>()));
        write!(f, "{}", s)
    }
}

//The rotor alphabets all have coprime lengths
pub static M209_ROTORS: LazyLock<[Rotor; 6]> = LazyLock::new(|| {
    [
        Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 15),
        Rotor::new("ABCDEFGHIJKLMNOPQRSTUVXYZ", 14),
        Rotor::new("ABCDEFGHIJKLMNOPQRSTUVX", 13),
        Rotor::new("ABCDEFGHIJKLMNOPQRSTU", 12),
        Rotor::new("ABCDEFGHIJKLMNOPQRS", 11),
        Rotor::new("ABCDEFGHIJKLMNOPQ", 10),
    ]
});

pub const M209_ALPHABETS: [&'static str; 6] = [
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "ABCDEFGHIJKLMNOPQRSTUVXYZ",
    "ABCDEFGHIJKLMNOPQRSTUVX",
    "ABCDEFGHIJKLMNOPQRSTU",
    "ABCDEFGHIJKLMNOPQRS",
    "ABCDEFGHIJKLMNOPQ",
];

fn char_to_usize(c: char) -> usize {
    (c as u8 as usize) - 65
}

fn usize_to_char(n: usize) -> char {
    (n + 65) as u8 as char
}

fn atbash_encrypt(n: usize, k: usize, l: usize) -> usize {
    ((l - 1) * (n + 1) + k) % l
}

pub struct M209 {
    wheels: [Rotor; 6],
    pub lugs: [(usize, usize); 27],
}

impl Default for M209 {
    fn default() -> Self {
        Self {
            wheels: M209_ROTORS.clone(),
            lugs: [(0, 0); 27],
        }
    }
}

impl M209 {
    pub fn set_pins(&mut self, pins: [&str; 6]) -> Result<(), CipherError> {
        for (r, p) in self.get_wheels().zip(pins) {
            r.set_pins(p)?
        }
        Ok(())
    }

    pub fn set_lugs(&mut self, lugs: [(usize, usize); 27]) {
        self.lugs = lugs
    }

    pub fn set_wheels(&mut self, settings: &str) {
        for (r, c) in self.wheels.iter_mut().zip(settings.chars()) {
            r.set_display(c)
        }
    }

    pub fn get_wheels(&mut self) -> std::slice::IterMut<'_, Rotor> {
        self.wheels.iter_mut()
    }

    pub fn step(&mut self) {
        for w in self.wheels.iter_mut() {
            w.step()
        }
    }

    pub fn print_cage(&self) -> String {
        let mut out = "Cage\n".to_string();
        for b in self.lugs.chunks(9).collect_vec() {
            for lug in b {
                let entry = format!("{}-{}  ", lug.0, lug.1);
                out.push_str(&entry)
            }
            out.push('\n')
        }
        out
    }

    pub fn print_wheels(&self) -> String {
        let mut out = String::new();
        for wheel in self.wheels.iter() {
            out.push_str(&wheel.to_string());
            out.push('\n');
        }
        out
    }
}

impl Cipher for M209 {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        let nums = text.chars().map(|x| char_to_usize(x)).collect_vec();
        let mut out = String::with_capacity(text.len());

        // The wheels move during encryption but we don't want the cipher to get into an unknown position so we just clone them all
        let mut wheels = self.wheels.clone();

        for n in nums {
            let mut sh = 0;

            // Each tuple represents the two lugs of a bar
            // A lug set to zero is inactive and is ignored
            // If either lug hits an active effective pin increase the shift by one
            for (lug_a, lug_b) in self.lugs {
                if lug_a != 0 {
                    if wheels[lug_a - 1].active_is_effective() {
                        sh += 1;
                        continue;
                    }
                }
                if lug_b != 0 {
                    if wheels[lug_b - 1].active_is_effective() {
                        sh += 1;
                        continue;
                    }
                }
            }

            let c = usize_to_char(atbash_encrypt(n, sh, 26));
            out.push(c);

            /*
            finally advance all the wheels by one step
            because the wheels all have coprime lengths this steps them through every possible permutation
            */
            for w in wheels.iter_mut() {
                w.step()
            }
        }
        Ok(out)
    }

    // The M209 is reciprocal
    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        self.encrypt(text)
    }
}

#[cfg(test)]
mod m209_tests {

    use super::*;

    const PLAINTEXT: &'static str = "AAAAAAAAAAAAAAAAAAAAAAAAAA";
    const CIPHERTEXT: &'static str = "TNJUWAUQTKCZKNUTOTBCWARMIO";

    #[test]
    fn encrypt_test() {
        let mut cipher = M209::default();
        cipher
            .set_pins([
                "ABDHIKMNSTVW",
                "ADEGJKLORSUX",
                "ABGHJLMNRSTUX",
                "CEFHIMNPSTU",
                "BDEFHIMNPS",
                "ABDHKNOQ",
            ])
            .expect("invalid pins");
        cipher.set_lugs([
            (3, 6),
            (0, 6),
            (1, 6),
            (1, 5),
            (4, 5),
            (0, 4),
            (0, 4),
            (0, 4),
            (0, 4),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 5),
            (2, 5),
            (0, 5),
            (0, 5),
            (0, 5),
            (0, 5),
            (0, 5),
            (0, 5),
        ]);
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let mut cipher = M209::default();
        cipher
            .set_pins([
                "ABDHIKMNSTVW",
                "ADEGJKLORSUX",
                "ABGHJLMNRSTUX",
                "CEFHIMNPSTU",
                "BDEFHIMNPS",
                "ABDHKNOQ",
            ])
            .expect("invalid pins");
        cipher.set_lugs([
            (3, 6),
            (0, 6),
            (1, 6),
            (1, 5),
            (4, 5),
            (0, 4),
            (0, 4),
            (0, 4),
            (0, 4),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 0),
            (2, 5),
            (2, 5),
            (0, 5),
            (0, 5),
            (0, 5),
            (0, 5),
            (0, 5),
            (0, 5),
        ]);
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
