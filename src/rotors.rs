use itertools::Itertools;

// An array backed Rotor with a fixed size known at compile time
#[derive(Copy, Clone, Debug)]
pub struct Rotor<const N: usize> {
    wiring_rtl: [usize; N],
    wiring_ltr: [usize; N],
    pub position: usize,
    pub reversed: bool,
    pub wiring_str: &'static str,
    pub name: &'static str,
}

impl<const N: usize> Rotor<N> {
    pub fn new(
        name: &'static str,
        wiring_str: &'static str,
        char_to_usize: &dyn Fn(char) -> usize,
    ) -> Result<Rotor<N>, String> {
        let count = wiring_str.chars().count();
        if count != N {
            return Err(format!(
                "Rotor<{N}> requires wiring_st to have exactly {N} character, found {count}"
            ));
        }

        if count != wiring_str.chars().unique().count() {
            return Err(format!("found repeated characters in {wiring_str}"));
        }

        let mut wiring_rtl = [0; N];
        let mut wiring_ltr = [0; N];
        for w in wiring_str.chars().map(|x| char_to_usize(x)).enumerate() {
            if w.1 >= N {
                return Err(format!(
                    "invalid char_to_usize() produced value {} when rotor size is {}",
                    w.1, N
                ));
            }
            wiring_rtl[w.0] = w.1;
            wiring_ltr[w.1] = w.0;
        }
        Ok(Rotor {
            wiring_rtl,
            wiring_ltr,
            position: 0,
            reversed: false,
            wiring_str,
            name,
        })
    }

    // Step forward one position
    pub fn step(&mut self) {
        self.position = (self.position + 1) % N;
    }

    // Step forward n positions
    pub fn step_n(&mut self, n: usize) {
        self.position = (self.position + n) % N;
    }

    // Step backward one position
    pub fn step_back(&mut self) {
        self.position = (self.position + N - 1) % N;
    }

    // Step forward n positions
    pub fn step_back_n(&mut self, n: usize) {
        self.position = (self.position + N - (n % N)) % N;
    }

    // View the writing array for right to left signals
    pub fn wiring_rtl(&self) -> [usize; N] {
        self.wiring_rtl.clone()
    }

    // View the writing array for left to right signals
    pub fn wiring_ltr(&self) -> [usize; N] {
        self.wiring_ltr.clone()
    }

    // Pass a signal through the rotor from right to left
    pub fn signal_rtl(&self, entry: usize) -> usize {
        let inner_position = (N + entry + self.position) % N;
        let inner = match self.reversed {
            true => self.wiring_ltr[inner_position],
            false => self.wiring_rtl[inner_position],
        };
        (inner + N - self.position) % N
    }

    // Pass a signal through the rotor from left to right
    pub fn signal_ltr(&self, entry: usize) -> usize {
        let inner_position = (N + entry + self.position) % N;
        let inner = match self.reversed {
            true => self.wiring_rtl[inner_position],
            false => self.wiring_ltr[inner_position],
        };
        (inner + N - self.position) % N
    }

    // Show the Rotor in its current position
    pub fn show(&self) -> String {
        let mut out = String::with_capacity(N);
        let p = self.position;
        out.push_str(&self.wiring_str[p..]);
        out.push_str(&self.wiring_str[0..p]);
        out
    }

    // Show the Rotor as it would be in the zero position
    pub fn show_zeroed(&self) -> String {
        self.wiring_str.to_string()
    }
}
