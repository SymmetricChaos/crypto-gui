use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::{errors::Error, global_rng::get_global_rng, text_aux::VecString};

#[derive(Clone, Debug)]
pub struct HebernRotor {
    wiring_rtl: Vec<usize>,
    wiring_ltr: Vec<usize>,
    pub position: usize,
    pub wiring_str: String,
    size: usize,
    pub editable: bool,
    pub error: String,
}

impl HebernRotor {
    pub fn new(wiring_str: &str, alphabet: &VecString) -> Result<HebernRotor, Error> {
        let size = wiring_str.chars().count();
        let mut wiring_rtl = vec![0; size];
        let mut wiring_ltr = vec![0; size];

        for (pos, c) in wiring_str.chars().enumerate() {
            let n = alphabet.get_pos_of(c).ok_or(Error::invalid_input_char(c))?;
            wiring_rtl[pos] = n;
            wiring_ltr[n] = pos;
        }
        Ok(HebernRotor {
            wiring_rtl,
            wiring_ltr,
            position: 0,
            wiring_str: wiring_str.to_string(),
            size,
            editable: false,
            error: String::new(),
        })
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % self.size
    }

    // We will use and return usize instead of char to avoid constantly converting types
    pub fn rtl(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_rtl[inner_position];
        (inner + self.size - self.position) % self.size
    }

    pub fn ltr(&self, entry: usize) -> usize {
        let inner_position = (self.size + entry + self.position) % self.size;
        let inner = self.wiring_ltr[inner_position];
        (inner + self.size - self.position) % self.size
    }

    pub fn set(&mut self, alphabet: &VecString) -> Result<(), Error> {
        let total_size = self.wiring_str.chars().count();
        if total_size != self.size {
            return Err(Error::General(format!(
                "must provide exactly {} characters",
                self.size
            )));
        }
        let unique_size = self.wiring_str.chars().unique().count();
        if unique_size != total_size {
            return Err(Error::General(String::from(
                "duplicate characters are not allowed",
            )));
        }

        let mut new_wiring_rtl = vec![0; self.size];
        let mut new_wiring_ltr = vec![0; self.size];
        for (pos, c) in self.wiring_str.chars().enumerate() {
            let n = alphabet.get_pos_of(c).ok_or(Error::invalid_input_char(c))?;

            new_wiring_rtl[pos] = n;
            new_wiring_ltr[n] = pos;
        }
        self.wiring_rtl = new_wiring_rtl;
        self.wiring_ltr = new_wiring_ltr;
        Ok(())
    }

    pub fn fill(&mut self, alphabet: &VecString) -> Result<(), Error> {
        for a in alphabet.chars() {
            if self.wiring_str.contains(a) {
                continue;
            } else {
                self.wiring_str.push(a)
            }
        }
        self.set(alphabet)
    }

    pub fn randomize(&mut self, alphabet: &VecString) -> Result<(), Error> {
        self.wiring_str = alphabet.shuffled(&mut get_global_rng()).to_string();
        self.set(alphabet)
    }
}

impl Display for HebernRotor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = String::with_capacity(self.size);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}
