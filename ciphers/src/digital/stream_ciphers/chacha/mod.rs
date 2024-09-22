use itertools::Itertools;

pub mod chacha;
pub mod chacha20poly1305;
pub mod chacha_ietf;
pub mod xchacha;
pub mod xchacha_ietf;

const DEBUG: bool = false;
macro_rules! debug_state {
    ($s:literal, $v:ident) => {
        if DEBUG {
            print!($s);
            println!("\n{}", $v);
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChaChaState([u32; 16]);

// Shortcut indexing
impl std::ops::Index<usize> for ChaChaState {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for ChaChaState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::fmt::Display for ChaChaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(144);
        for line in self.0.chunks_exact(4) {
            out.push_str(&line.iter().map(|word| format!("{:08x?}", word)).join(" "));
            out.push('\n')
        }
        writeln!(f, "{}", out)
    }
}

impl ChaChaState {
    pub fn new(state: [u32; 16]) -> Self {
        let s = Self(state);
        debug_state!("initial", s);
        s
    }

    pub fn quarter_round(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self[a] = self[a].wrapping_add(self[b]);
        self[d] ^= self[a];
        self[d] = self[d].rotate_left(16);

        self[c] = self[c].wrapping_add(self[d]);
        self[b] ^= self[c];
        self[b] = self[b].rotate_left(12);

        self[a] = self[a].wrapping_add(self[b]);
        self[d] ^= self[a];
        self[d] = self[d].rotate_left(8);

        self[c] = self[c].wrapping_add(self[d]);
        self[b] ^= self[c];
        self[b] = self[b].rotate_left(7);
    }

    pub fn column_round(&mut self) {
        self.quarter_round(0, 4, 8, 12);
        self.quarter_round(1, 5, 9, 13);
        self.quarter_round(2, 6, 10, 14);
        self.quarter_round(3, 7, 11, 15);
        debug_state!("column", self);
    }

    pub fn diag_round(&mut self) {
        self.quarter_round(0, 5, 10, 15);
        self.quarter_round(1, 6, 11, 12);
        self.quarter_round(2, 7, 8, 13);
        self.quarter_round(3, 4, 9, 14);
        debug_state!("diagonal", self);
    }

    pub fn double_round(&mut self) {
        self.column_round();
        self.diag_round();
    }
}
