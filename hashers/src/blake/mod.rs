pub mod blake256;
pub mod blake2b;
pub mod blake3;
pub mod blake512;
pub use blake2b::Blake2b;

pub mod blake2s;
pub use blake2s::Blake2s;

// Message permutation schedule shared by BLAKE and BLAKE2
const SIGMA: [[usize; 16]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

// These are the constants for how BLAKE and BLAKE2 mix along columns then rows
const A: [usize; 8] = [0, 1, 2, 3, 0, 1, 2, 3];
const B: [usize; 8] = [4, 5, 6, 7, 5, 6, 7, 4];
const C: [usize; 8] = [8, 9, 10, 11, 10, 11, 8, 9];
const D: [usize; 8] = [12, 13, 14, 15, 15, 12, 13, 14];

// This is the mixing function (quarter round) used by BLAKE and BLAKE2 written in a type agnostic form
// information in $v (the working vector) is spread across four of its indexes ($a, $b, $c, $d) which
// are chosen to represent columns and rows of $v when it is viewed as a 4x4 array
#[macro_export]
macro_rules! blake_mix {
    ($v: expr, $a: expr, $b: expr, $c: expr, $d: expr, $x: expr, $y: expr, $r: expr) => {
        $v[$a] = $v[$a].wrapping_add($v[$b]).wrapping_add($x);
        $v[$d] = ($v[$d] ^ $v[$a]).rotate_right($r[0]);

        $v[$c] = $v[$c].wrapping_add($v[$d]);
        $v[$b] = ($v[$b] ^ $v[$c]).rotate_right($r[1]);

        $v[$a] = $v[$a].wrapping_add($v[$b]).wrapping_add($y);
        $v[$d] = ($v[$d] ^ $v[$a]).rotate_right($r[2]);

        $v[$c] = $v[$c].wrapping_add($v[$d]);
        $v[$b] = ($v[$b] ^ $v[$c]).rotate_right($r[3]);
    };
}

// The double round in type agnosic form.
// Applies the quarter round eight times.
// The second variant of the macro XORs in values from a constant as in the original BLAKE
#[macro_export]
macro_rules! blake_double_round {
    ($v: expr, $chunk: expr, $r: expr, $s: ident) => {
        for j in 0..8 {
            let x = $chunk[$s[2 * j]];
            let y = $chunk[$s[2 * j + 1]];
            crate::blake_mix!(
                $v,
                super::A[j],
                super::B[j],
                super::C[j],
                super::D[j],
                x,
                y,
                $r
            );
        }
    };
    ($v: expr, $chunk: ident, $r: expr, $s: ident, $c: ident) => {
        for j in 0..8 {
            let x = $chunk[$s[2 * j]] ^ $c[$s[2 * j + 1]];
            let y = $chunk[$s[2 * j + 1]] ^ $c[$s[2 * j]];
            crate::blake_mix!(
                $v,
                super::A[j],
                super::B[j],
                super::C[j],
                super::D[j],
                x,
                y,
                $r
            );
        }
    };
}

// A sequence of compression rounds
#[macro_export]
macro_rules! blake_compress {
    ($v: expr, $chunk: ident, $r: expr, $n: literal) => {
        for i in 0..$n {
            let s = super::SIGMA[i % 10];
            crate::blake_double_round!($v, $chunk, $r, s);
        }
    };
    ($v: expr, $chunk: ident, $r: expr, $c: ident, $n: literal) => {
        for i in 0..$n {
            let s = super::SIGMA[i % 10];
            crate::blake_double_round!($v, $chunk, $r, s, $c);
        }
    };
}
