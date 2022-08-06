use super::{
    char_to_usize, usize_to_char, EnigmaPlugboard, Reflector, Rotor, REFLECTORS, ROTOR_MAP,
};
use crate::{ciphers::Cipher, errors::Error, global_rng::get_global_rng, text_aux::PresetAlphabet};

pub fn prep_enigma_text(text: &str) -> Result<String, Error> {
    let mut out = String::with_capacity(text.len());
    for t in text.chars() {
        if PresetAlphabet::BasicLatin.slice().contains(t) {
            out.push(t)
        } else if t.is_whitespace() || t.is_ascii_punctuation() {
            // ignore any Unicode whitespace and
            // any ASCII punctuation
        } else if PresetAlphabet::BasicLatin
            .slice()
            .contains(t.to_ascii_uppercase())
        {
            out.push(t.to_ascii_uppercase())
        } else {
            match t {
                'Ä' | 'ä' => out.push_str("AE"),
                'Ö' | 'ö' => out.push_str("OE"),
                'Ü' | 'ü' => out.push_str("UE"),
                'ẞ' | 'ß' => out.push_str("SS"),
                _ => return Err(Error::invalid_input_char(t)),
            }
        }
    }
    Ok(out)
}

#[test]
fn enigma_text_prep() {
    //Twelve boxers chase Viktor across the large Sylter dike
    let pangram = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich";
    assert_eq!(
        prep_enigma_text(pangram).unwrap(),
        "ZWOELFBOXKAEMPFERJAGENVIKTORQUERUEBERDENGROSSENSYLTERDEICH"
    );
}

// This will be the mutating inner state of the Enigma machine. Each time we
// encrypt with Enigma this state is cloned and run.
// Cloning Rotors and Reflectors is cheap as they are Copy. Plugboard is
// small and so should be cheap to Clone.
#[derive(Clone, Debug)]
pub struct EnigmaState {
    pub plugboard_pairs: String,
    pub plugboard: EnigmaPlugboard,
    pub rotors: [Rotor; 3],
    pub reflector: Reflector,
}

impl EnigmaState {
    fn advance_rotors(&mut self) {
        let mut on_notch = self.rotors[2].position == self.rotors[2].notch.0
            || self.rotors[2].position == self.rotors[2].notch.1;
        self.rotors[2].step();
        if on_notch {
            on_notch = self.rotors[1].position == self.rotors[1].notch.0
                || self.rotors[1].position == self.rotors[1].notch.1;
            self.rotors[1].step();
            if on_notch {
                self.rotors[0].step();
            }
        }
    }

    // The message key
    pub fn set_rotors(&mut self, rotor_positions: (usize, usize, usize)) {
        self.rotors[0].position = rotor_positions.0;
        self.rotors[1].position = rotor_positions.1;
        self.rotors[2].position = rotor_positions.2;
    }

    pub fn set_rings(&mut self, rotor_ring_positions: (usize, usize, usize)) {
        self.rotors[0].ring = rotor_ring_positions.0;
        self.rotors[1].ring = rotor_ring_positions.1;
        self.rotors[2].ring = rotor_ring_positions.2;
    }

    pub fn set_plugboard(&mut self) -> Result<(), Error> {
        self.plugboard.set_plugboard(&self.plugboard_pairs)
    }

    // Notice that the signal goes through the rotors starting on the right with the 3rd rotor,
    // then through the reflector, and back through from left to right starting with the 1st rotor
    fn encrypt_char(&mut self, c: char) -> char {
        self.advance_rotors();
        let mut x = char_to_usize(self.plugboard.swap(c));
        x = self.rotors[2].encrypt_rtl(x);
        x = self.rotors[1].encrypt_rtl(x);
        x = self.rotors[0].encrypt_rtl(x);
        x = self.reflector.encrypt(x);
        x = self.rotors[0].encrypt_ltr(x);
        x = self.rotors[1].encrypt_ltr(x);
        x = self.rotors[2].encrypt_ltr(x);
        self.plugboard.swap(usize_to_char(x))
    }
}

impl Default for EnigmaState {
    fn default() -> Self {
        Self {
            plugboard_pairs: String::new(),
            plugboard: EnigmaPlugboard::default(),
            rotors: [ROTOR_MAP["I"], ROTOR_MAP["II"], ROTOR_MAP["III"]],
            reflector: REFLECTORS["B"],
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnigmaM3 {
    pub state: EnigmaState,
}

impl Cipher for EnigmaM3 {
    fn encrypt(&self, text: &str) -> Result<String, Error> {
        let mut inner_state = self.state.clone();
        inner_state.set_plugboard()?;
        Ok(text.chars().map(|c| inner_state.encrypt_char(c)).collect())
    }

    fn decrypt(&self, text: &str) -> Result<String, Error> {
        self.encrypt(text)
    }

    fn randomize(&mut self) {
        let rng = get_global_rng();
        todo!("{:?}", rng)
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for EnigmaM3 {
    fn default() -> Self {
        Self {
            state: Default::default(),
        }
    }
}

#[cfg(test)]
mod enigma_tests {
    use super::*;

    const PLAINTEXT: &'static str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    const CIPHERTEXT: &'static str = "BDZGOWCXLTKSBTMCDLPBMUQOFXYHCXTGYJFLINHNXSHIUNTHEO";

    #[test]
    fn encrypt_test() {
        let cipher = EnigmaM3::default();
        assert_eq!(cipher.encrypt(PLAINTEXT).unwrap(), CIPHERTEXT);
    }

    #[test]
    fn decrypt_test() {
        let cipher = EnigmaM3::default();
        assert_eq!(cipher.decrypt(CIPHERTEXT).unwrap(), PLAINTEXT);
    }
}
