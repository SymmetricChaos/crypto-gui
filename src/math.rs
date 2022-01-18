use rand::prelude::{ThreadRng, SliceRandom};
use num::{Integer, ToPrimitive};


pub fn mul_inv<N: Integer + Copy >(num: N, modulus: N) -> Option<N> {
    let egcd = num.extended_gcd(&modulus);
    if !egcd.gcd.is_one() {
        None 
    } else {
        Some( egcd.x.mod_floor(&modulus) )
    }
}

// We're not going to deal with big numbers so this thse crude factorizations are plenty
pub fn factors<N: Integer + Copy + ToPrimitive>(n: N) -> Vec<N> {
    let mut out = Vec::new();
    for f in num::range(N::one(), n) {
        if n.is_multiple_of(&f) {
            out.push(n);
            out.push(n.div(f));
        }
    }
    out.sort();
    out
}

pub fn prime_factorization<N: Integer + Copy>(n: N) -> Vec<N> {
    if n.is_zero() { return Vec::new() }
    let mut out = Vec::new();
    let mut n = n;
    let mut f = N::one();
    while !n.is_one() {
        f = f + N::one();
        while n.is_multiple_of(&f) {
            n = n/f;
            out.push(f);
        }
    }
    out.sort();
    out
}

pub fn prime_factors<N: Integer + Copy>(n: N) -> Vec<N> {
    let mut out = prime_factorization(n);
    out.dedup();
    out
}



pub fn shuffle_str(s: &str, rng: &mut ThreadRng) -> String {
    let mut characters = s.chars().collect::<Vec<char>>();
    let slice = characters.as_mut_slice();
    slice.shuffle(rng);
    slice.iter().map(|x| *x).collect::<String>()
}
