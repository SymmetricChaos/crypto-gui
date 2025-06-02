use crate::{Cipher, CipherError};
use itertools::Itertools;
use std::fmt::Display;
use utils::{preset_alphabet::Alphabet, vecstring::VecString};

/// Two character names of cards in order A-K and ♣♦♥♠
pub const CARD_NAMES: [&'static str; 52] = [
    "A♣", "2♣", "3♣", "4♣", "5♣", "6♣", "7♣", "8♣", "9♣", "T♣", "J♣", "Q♣", "K♣", "A♦", "2♦", "3♦",
    "4♦", "5♦", "6♦", "7♦", "8♦", "9♦", "T♦", "J♦", "Q♦", "K♦", "A♥", "2♥", "3♥", "4♥", "5♥", "6♥",
    "7♥", "8♥", "9♥", "T♥", "J♥", "Q♥", "K♥", "A♠", "2♠", "3♠", "4♠", "5♠", "6♠", "7♠", "8♠", "9♠",
    "T♠", "J♠", "Q♠", "K♠",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    JA,
    JB,
    C(u8),
}

impl Card {
    pub fn is_joker(&self) -> bool {
        match self {
            Card::JA => true,
            Card::JB => true,
            Card::C(_) => false,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Card::JA => 0,
            Card::JB => 0,
            Card::C(n) => *n,
        }
    }

    pub fn to_unicode(&self) -> &'static str {
        match self {
            Card::JA => "XA",
            Card::JB => "XB",
            Card::C(n) => CARD_NAMES[(n - 1) as usize],
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::JA => write!(f, "JA"),
            Card::JB => write!(f, "JB"),
            Card::C(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Solitaire {
    pub deck: Vec<Card>,
    pub alphabet: VecString,
}

impl Default for Solitaire {
    fn default() -> Self {
        let mut deck = Vec::with_capacity(54);

        for i in 0..52 {
            deck.push(Card::C((i + 1) as u8));
        }
        deck.push(Card::JA);
        deck.push(Card::JB);

        Self {
            deck,
            alphabet: VecString::from(Alphabet::BasicLatin),
        }
    }
}

impl Solitaire {
    pub fn from_keyword(keyword: &str) -> Result<Self, CipherError> {
        let mut deck = Vec::with_capacity(54);
        for i in 0..52 {
            deck.push(Card::C((i + 1) as u8));
        }
        deck.push(Card::JA);
        deck.push(Card::JB);
        let mut cipher = Self {
            deck,
            alphabet: VecString::from(Alphabet::BasicLatin),
        };

        // Convert the keyword to numbers
        let keyword_stream: Vec<Result<usize, CipherError>> = keyword
            .chars()
            .map(|c| {
                cipher
                    .alphabet
                    .get_pos(c)
                    .ok_or(CipherError::invalid_input_char(c))
            })
            .collect();
        for i in keyword_stream {
            if let Ok(n) = i {
                cipher.move_jokers();
                cipher.triple_cut();
                cipher.count_cut();
                cipher.count_cut_n(n + 1);
            } else {
                return Err(i.unwrap_err());
            }
        }
        Ok(cipher)
    }

    pub fn set_from_keyword(&mut self, keyword: &str) -> Result<(), CipherError> {
        // Reset the deck
        let n = self.len() - 2;
        self.deck.clear();

        for i in 0..n {
            self.deck.push(Card::C((i + 1) as u8));
        }
        self.deck.push(Card::JA);
        self.deck.push(Card::JB);

        // Convert the keyword to numbers
        let keyword_stream: Vec<Result<usize, CipherError>> = keyword
            .chars()
            .map(|c| {
                self.alphabet
                    .get_pos(c)
                    .ok_or(CipherError::invalid_input_char(c))
            })
            .collect();
        for i in keyword_stream {
            if let Ok(n) = i {
                self.move_jokers();
                self.triple_cut();
                self.count_cut();
                self.count_cut_n(n + 1);
            } else {
                return Err(i.unwrap_err());
            }
        }
        Ok(())
    }

    pub fn as_string(&self) -> String {
        self.deck.iter().map(|c| c.to_string()).join(" ")
    }

    pub fn as_unicode(&self) -> String {
        self.deck.iter().map(|c| c.to_unicode()).join(" ")
    }

    fn len(&self) -> usize {
        self.deck.len()
    }

    fn position_a(&self) -> usize {
        self.deck.iter().position(|x| *x == Card::JA).unwrap()
    }

    fn position_b(&self) -> usize {
        self.deck.iter().position(|x| *x == Card::JB).unwrap()
    }

    fn move_jokers(&mut self) {
        let n = self.len();

        let a = self.position_a();
        if a == (n - 1) {
            self.deck.swap(n - 1, 0);
            self.deck.swap(0, 1);
        } else {
            self.deck.swap(a, a + 1);
        }

        let b = self.position_b();
        if b == (n - 1) {
            self.deck.swap(n - 1, 0);
            self.deck.swap(0, 1);
            self.deck.swap(1, 2);
        } else if b == (n - 2) {
            self.deck.swap(b, b + 1);
            let j = self.deck.pop().unwrap();
            self.deck.insert(1, j);
        } else {
            self.deck.swap(b, b + 1);
            self.deck.swap(b + 1, b + 2);
        }
    }

    fn triple_cut(&mut self) {
        let pa = self.position_a();
        let pb = self.position_b();

        let (mut r, m, l) = if pa > pb {
            let r = self.deck.split_off(pa + 1);
            let m = self.deck.split_off(pb);
            let l = self.deck.clone();
            (r, m, l)
        } else {
            let r = self.deck.split_off(pb + 1);
            let m = self.deck.split_off(pa);
            let l = self.deck.clone();
            (r, m, l)
        };

        r.extend(m);
        r.extend(l);
        self.deck = r;
    }

    fn count_cut(&mut self) {
        let last_card = self.deck.pop().unwrap();
        if last_card.is_joker() {
            self.deck.push(last_card);
        } else {
            let p = last_card.value() as usize;
            let mut r = self.deck.split_off(p);
            r.extend_from_slice(&self.deck);
            r.push(last_card);
            self.deck = r;
        }
    }

    fn count_cut_n(&mut self, n: usize) {
        let last_card = self.deck.pop().unwrap();
        let mut r = self.deck.split_off(n);
        r.extend_from_slice(&self.deck);
        r.push(last_card);
        self.deck = r;
    }

    fn output_card(&self) -> Card {
        let first_card = self.deck[0];
        if first_card.is_joker() {
            self.deck.last().unwrap().clone()
        } else {
            let value = first_card.value() as usize;
            self.deck[value]
        }
    }

    fn next_value(&mut self) -> u8 {
        // Retry until a non-joker card is output
        loop {
            self.move_jokers();
            self.triple_cut();
            self.count_cut();
            let out = self.output_card();
            if !out.is_joker() {
                return out.value();
            }
        }
    }

    fn key_stream(&self, n: usize) -> Vec<u8> {
        let mut rng = self.clone();
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(rng.next_value());
        }
        out
    }

    fn encrypt_char(&self, c: char, k: usize) -> Result<char, CipherError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(CipherError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, k as i32).unwrap())
    }

    fn decrypt_char(&self, c: char, k: usize) -> Result<char, CipherError> {
        let p = self
            .alphabet
            .get_pos(c)
            .ok_or(CipherError::invalid_input_char(c))?;
        Ok(*self.alphabet.get_char_offset(p, -(k as i32)).unwrap())
    }
}

impl Cipher for Solitaire {
    fn encrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        let key_steam = self.key_stream(text.chars().count());
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(key_steam) {
            out.push(self.encrypt_char(c, n as usize)?)
        }
        Ok(out)
    }

    fn decrypt(&self, text: &str) -> Result<String, crate::CipherError> {
        let key_steam = self.key_stream(text.chars().count());
        let mut out = String::with_capacity(text.len());
        for (c, n) in text.chars().zip(key_steam) {
            out.push(self.decrypt_char(c, n as usize)?)
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn keystream() {
        let mut cipher = Solitaire::default();
        assert_eq!(4, cipher.next_value());
        assert_eq!(49, cipher.next_value());
        assert_eq!(10, cipher.next_value());
        assert_eq!(24, cipher.next_value());
        assert_eq!(8, cipher.next_value());
        assert_eq!(51, cipher.next_value());
        assert_eq!(44, cipher.next_value());
        assert_eq!(6, cipher.next_value());
        assert_eq!(4, cipher.next_value());
        assert_eq!(33, cipher.next_value());
    }

    #[test]
    fn encrypt() {
        let cipher = Solitaire::default();
        assert_eq!("EXKYIZSGEH", cipher.encrypt("AAAAAAAAAA").unwrap());
    }

    #[test]
    fn encrypt_with_key1() {
        let cipher = Solitaire::from_keyword("FOO").unwrap();
        assert_eq!(
            "ITHZUJIWGRFARMW",
            cipher.encrypt("AAAAAAAAAAAAAAA").unwrap()
        );
    }

    #[test]
    fn encrypt_with_key2() {
        let cipher = Solitaire::from_keyword("CRYPTONOMICON").unwrap();
        assert_eq!("KIRAKSFJAN", cipher.encrypt("SOLITAIREX").unwrap());
    }

    #[test]
    fn encrypt_with_key3() {
        let mut cipher = Solitaire::default();
        cipher.set_from_keyword("A").unwrap();
        assert_eq!(
            "XODALGSCULIQNSC",
            cipher.encrypt("AAAAAAAAAAAAAAA").unwrap()
        );
    }
}
