use std::num::Wrapping;

pub mod chacha;
pub mod chacha20poly1305;
pub mod chacha_ietf;
pub mod xchacha;

pub fn quarter_round(state: &mut [Wrapping<u32>; 16], a: usize, b: usize, c: usize, d: usize) {
    state[a] += state[b];
    state[d] ^= state[a];
    state[d] = Wrapping(state[d].0.rotate_left(16));

    state[c] += state[d];
    state[b] ^= state[c];
    state[b] = Wrapping(state[b].0.rotate_left(12));

    state[a] += state[b];
    state[d] ^= state[a];
    state[d] = Wrapping(state[d].0.rotate_left(8));

    state[c] += state[d];
    state[b] ^= state[c];
    state[b] = Wrapping(state[b].0.rotate_left(7));
}

pub fn column_round(state: &mut [Wrapping<u32>; 16]) {
    quarter_round(state, 0, 4, 8, 12);
    quarter_round(state, 1, 5, 9, 13);
    quarter_round(state, 2, 6, 10, 14);
    quarter_round(state, 3, 7, 11, 15);
}

pub fn diag_round(state: &mut [Wrapping<u32>; 16]) {
    quarter_round(state, 0, 5, 10, 15);
    quarter_round(state, 1, 6, 11, 12);
    quarter_round(state, 2, 7, 8, 13);
    quarter_round(state, 3, 4, 9, 14);
}

pub fn double_round(state: &mut [Wrapping<u32>; 16]) {
    column_round(state);
    diag_round(state);
}
