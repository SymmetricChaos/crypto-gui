use crate::{ciphers::{substitution::Plugboard, Cipher}, errors::CipherError};

use super::switch::{Switch, SwitchSpeed};


 
pub struct Switches {
    sixes: Switch,
    twenties: [Switch; 3],
}
 
impl Switches {
    pub fn step(&mut self) {
        let spos = self.sixes.position;
        let mpos = self.get_switch(SwitchSpeed::Middle).position;
 
        // Sixes always steps
        self.sixes.step();
 
        // Exactly one of the Twenties steps at a time
        if spos == 23 && mpos == 24 {
            self.get_switch(SwitchSpeed::Slow).step();
        } else if spos == 24 {
            self.get_switch(SwitchSpeed::Middle).step();
        } else {
            self.get_switch(SwitchSpeed::Fast).step();
        }
 
    }
 
    pub fn encrypt(&mut self, c: char) -> char {
        todo!("")
    }
 
    pub fn decrypt(&mut self, c: char) -> char {
        todo!("")
    }
 
    fn get_switch(&mut self, speed: SwitchSpeed) -> &mut Switch {
        for switch in self.twenties {
            if switch.speed == speed {
                return &mut switch
            }
        }
        &mut self.twenties[0]
    }
}
 
pub struct Purple {
    switches: Switches, // this will be cloned during execution and then mutated
    input_plugboard: Plugboard,
    output_plugboard: Plugboard,
}
 
impl Purple {
    const SIXES: &'static str = "AEIOUY";
    const TWENTIES: &'static str = "BCDFGHJKLMNPQRSTVWXZ";
}
 
impl Cipher for Purple {
    fn encrypt(&self, text: &str) -> Result<String,CipherError> {
        let mut out = String::with_capacity(text.len());
        let mut switches = self.switches.clone();
        for c in text.chars() {
            let x = self.input_plugboard.get(c);
            let x = switches.encrypt(x);
            let f = self.output_plugboard.get(x);
            out.push(f);
            switches.step();
        }
 
        out
    }
}