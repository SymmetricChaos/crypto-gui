pub struct Solitaire {
    pub deck: Vec<u8>,
}

impl Default for Solitaire {
    fn default() -> Self {
        let mut deck = vec![0; 54];
        for i in 0..54 {
            deck[i] = i as u8;
        }
        Self { deck }
    }
}

impl Solitaire {
    fn init(n: usize) -> Self {
        let mut deck = vec![0; n];
        for i in 0..n {
            deck[i] = i as u8;
        }
        Self { deck }
    }

    fn len(&self) -> usize {
        self.deck.len()
    }

    /// Joker A is the number 0
    fn move_a(&mut self) {
        let n = self.len();
        let p = self.deck.iter().position(|x| *x == 0).unwrap();
        if p == (n - 1) {
            self.deck.swap(n - 1, 0);
            self.deck.swap(0, 1);
        } else {
            self.deck.swap(p, p + 1);
        }
    }

    /// Joker B is the number largest value (default 53)
    fn move_b(&mut self) {
        let n = self.len();
        let p = self.deck.iter().position(|x| *x == (n - 1) as u8).unwrap();
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn swaps() {
        let mut cipher = Solitaire::init(7);
        cipher.deck = vec![0, 1, 2, 6, 3, 4, 5];
        cipher.move_a();
        cipher.move_b();
        println!("{:?}", cipher.deck);
        cipher.deck = vec![1, 0, 6, 2, 3, 4, 5];
        cipher.move_a();
        cipher.move_b();
        println!("{:?}", cipher.deck);
    }
}
