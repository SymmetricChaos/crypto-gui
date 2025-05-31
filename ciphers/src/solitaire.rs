use std::fmt::Display;

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

pub struct Solitaire {
    pub deck: Vec<Card>,
}

impl Default for Solitaire {
    fn default() -> Self {
        let mut deck = vec![Card::C(0); 54];
        deck[0] = Card::JA;
        deck[1] = Card::JB;
        for i in 0..52 {
            deck[i + 2] = Card::C((i + 1) as u8);
        }

        Self { deck }
    }
}

impl Solitaire {
    fn init(n: usize) -> Self {
        let mut deck = vec![Card::C(0); n + 2];
        deck[0] = Card::JA;
        deck[1] = Card::JB;
        for i in 0..n {
            deck[i + 2] = Card::C((i + 1) as u8);
        }
        Self { deck }
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

    fn move_a(&mut self) {
        let n = self.len();
        let p = self.position_a();
        if p == (n - 1) {
            self.deck.swap(n - 1, 0);
            self.deck.swap(0, 1);
        } else {
            self.deck.swap(p, p + 1);
        }
    }

    fn move_b(&mut self) {
        let n = self.len();
        let p = self.position_b();
        if p == (n - 1) {
            self.deck.swap(n - 1, 0);
            self.deck.swap(0, 1);
            self.deck.swap(1, 2);
        } else if p == (n - 2) {
            self.deck.swap(p, p + 1);
            self.deck.swap(n - 1, 0);
            self.deck.swap(0, 1);
        } else {
            self.deck.swap(p, p + 1);
            self.deck.swap(p + 1, p + 2);
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

        r.extend_from_slice(&m);
        r.extend_from_slice(&l);
        self.deck = r;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn move_test(v1: Vec<Card>, v2: Vec<Card>) {
        let mut cipher = Solitaire::init(v1.len());
        cipher.deck = v1;
        cipher.move_a();
        cipher.move_b();
        assert_eq!(cipher.deck, v2)
    }

    #[test]
    fn swaps() {
        move_test(
            vec![
                Card::JA,
                Card::C(1),
                Card::C(2),
                Card::JB,
                Card::C(3),
                Card::C(4),
                Card::C(5),
            ],
            vec![
                Card::C(1),
                Card::JA,
                Card::C(2),
                Card::C(3),
                Card::C(4),
                Card::JB,
                Card::C(5),
            ],
        );
        move_test(
            vec![
                Card::C(1),
                Card::JA,
                Card::JB,
                Card::C(2),
                Card::C(3),
                Card::C(4),
                Card::C(5),
            ],
            vec![
                Card::C(1),
                Card::JA,
                Card::C(2),
                Card::JB,
                Card::C(3),
                Card::C(4),
                Card::C(5),
            ],
        );
    }

    #[test]
    fn triple_cut() {
        let mut cipher = Solitaire::init(9);
        cipher.deck = vec![
            Card::C(1),
            Card::C(2),
            Card::C(3),
            Card::JB,
            Card::C(4),
            Card::C(5),
            Card::C(6),
            Card::C(7),
            Card::JA,
            Card::C(8),
            Card::C(9),
        ];
        cipher.triple_cut();
        assert_eq!(
            cipher.deck,
            vec![
                Card::C(8),
                Card::C(9),
                Card::JB,
                Card::C(4),
                Card::C(5),
                Card::C(6),
                Card::C(7),
                Card::JA,
                Card::C(1),
                Card::C(2),
                Card::C(3),
            ]
        );
    }
}
