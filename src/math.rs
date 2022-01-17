use rand::prelude::{ThreadRng, SliceRandom};
use num::Integer;

fn egcd<N: Integer + Copy>(a: N, b: N) -> (N,N,N) {
    if a.is_zero() {
        (b,N::zero(),N::one())
    } else {
        let (g, y, x) = egcd(b.mod_floor(&a), a);
        (g,x-(b/a)*y,y)
    }
}

pub fn mul_inv<N: Integer + Copy>(num: N, modulus: N) -> Option<N> {
    let (g, x, _) = egcd(num, modulus);
    if !g.is_one() {
        None 
    } else {
        Some( x.mod_floor(&modulus) )
    }
}

// We're not going to deal with big numbers so this thse crude factorizations are plenty
pub fn factors(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for f in 1..n/2 {
        if n%f == 0 {
            out.push(n);
            out.push(n/f);
        }
    }
    out.sort();
    out
}

pub fn prime_factorization(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut n = n;
    let mut f = 1;
    while n != 1 {
        f += 1;
        while n % f == 0 {
            n = n/f;
            out.push(f);
        }
    }
    out.sort();
    out
}

pub fn prime_factors(n: usize) -> Vec<usize> {
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