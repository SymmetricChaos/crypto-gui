use super::char_to_usize;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

// Specifically for SIGABA rotor
#[derive(Clone, Debug)]
pub struct Rotor<const N: usize> {
    wiring_rtl: [usize; N],
    wiring_ltr: [usize; N],
    pub position: usize,
    pub reversed: bool,
    pub wiring_str: &'static str,
    pub name: &'static str,
}
 
pub type IndexRotor = Rotor<10>;
pub type CipherRotor = Rotor<26>;
 
impl<const N: usize> Rotor<N> {
 
    pub fn new(name: &'static str, wiring_str: &'static str) -> Rotor<N> {
 
        let mut wiring_rtl = [0; N];
        let mut wiring_ltr = [0; N];
        for w in wiring_str.chars().map(|x| char_to_usize(x)).enumerate() {
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Rotor {
            wiring_rtl,
            wiring_ltr,
            position: 0,
            reversed: false,
            wiring_str,
            name,
        }
    }
 
    pub fn step(&mut self) {
        self.position = (self.position + 1) % N;
    }
 
    // Signal starts on the right and goes through the rotor then back
    // We will use and return usize instead of char to avoid constantly converting types
    // Need to logically confirm that a reversed rotor works
    pub fn rtl(&self, entry: usize) -> usize {
        let inner_position = (N + entry + self.position) % N;
        let inner = match self.reversed {
            true => self.wiring_ltr[inner_position],
            false => self.wiring_rtl[inner_position],
        };
        (inner + N - self.position) % N
    }
 
    pub fn ltr(&self, entry: usize) -> usize {
        let inner_position = (N + entry + self.position) % N;
        let inner = match self.reversed {
            true => self.wiring_rtl[inner_position],
            false => self.wiring_ltr[inner_position],
        };
        (inner + N - self.position) % N
    }
}
 
impl<const N: usize> PartialEq for Rotor<N> {
    fn eq(&self, other: &Self) -> bool {
        self.wiring_str == other.wiring_str
    }
}
 
impl<const N: usize> fmt::Display for Rotor<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::with_capacity(N);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        write!(f, "{}", out)
    }
}
 
lazy_static! {
    pub static ref BIG_ROTOR_VEC: Vec<CipherRotor> = {
        let mut v = Vec::with_capacity(10);
        v.push(CipherRotor::new("R-A", "YCHLQSUGBDIXNZKERPVJTAWFOM"));
        v.push(CipherRotor::new("R-B", "INPXBWETGUYSAOCHVLDMQKZJFR"));
        v.push(CipherRotor::new("R-C", "WNDRIOZPTAXHFJYQBMSVEKUCGL"));
        v.push(CipherRotor::new("R-D", "TZGHOBKRVUXLQDMPNFWCJYEIAS"));
        v.push(CipherRotor::new("R-E", "YWTAHRQJVLCEXUNGBIPZMSDFOK"));
        v.push(CipherRotor::new("R-F", "QSLRBTEKOGAICFWYVMHJNXZUDP"));
        v.push(CipherRotor::new("R-G", "CHJDQIGNBSAKVTUOXFWLEPRMZY"));
        v.push(CipherRotor::new("R-H", "CDFAJXTIMNBEQHSUGRYLWZKVPO"));
        v.push(CipherRotor::new("R-I", "XHFESZDNRBCGKQIJLTVMUOYAPW"));
        v.push(CipherRotor::new("R-J", "EZJQXMOGYTCSFRIUPVNADLHWBK"));
        v
    };
 
    pub static ref BIG_ROTOR_MAP: HashMap<&'static str, CipherRotor> = {
        let mut m = HashMap::new();
        for rtr in BIG_ROTOR_VEC.iter() {
            m.insert(rtr.name, rtr.clone());
        }
        m
    };
 
    pub static ref INDEX_ROTOR_VEC: Vec<IndexRotor> = {
        let mut v = Vec::with_capacity(5);
        v.push(IndexRotor::new("0","7591482630"));
        v.push(IndexRotor::new("1","3810592764"));
        v.push(IndexRotor::new("2","4086153297"));
        v.push(IndexRotor::new("3","3980526174"));
        v.push(IndexRotor::new("4","6497135280"));
        v
    };
}