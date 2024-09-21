pub mod chacha;
pub mod chacha20poly1305;
pub mod chacha_ietf;
pub mod xchacha;
pub mod xchacha_itef;

const DEBUG: bool = false;

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
        writeln!(
            f,
            "{:08x?}\n{:08x?}\n{:08x?}\n{:08x?}",
            &self.0[0..4],
            &self.0[4..8],
            &self.0[8..12],
            &self.0[12..16]
        )
    }
}

impl ChaChaState {
    pub fn new(state: [u32; 16]) -> Self {
        if DEBUG {
            let s = Self(state);
            println!("initial:\n{s}");
            s
        } else {
            Self(state)
        }
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
        if DEBUG {
            println!("column:\n{self}")
        }
    }

    pub fn diag_round(&mut self) {
        self.quarter_round(0, 5, 10, 15);
        self.quarter_round(1, 6, 11, 12);
        self.quarter_round(2, 7, 8, 13);
        self.quarter_round(3, 4, 9, 14);
        if DEBUG {
            println!("diagon:\n{self}")
        }
    }

    pub fn double_round(&mut self) {
        self.column_round();
        self.diag_round();
    }
}
