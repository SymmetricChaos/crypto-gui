fn egcd(a: i64, b: i64) -> (i64,i64,i64) {
    if a == 0 {
        (b,0,1)
    } else {
        let (g, y, x) = egcd(b%a, a);
        (g,x-(b/a)*y,y)
    }
}

pub fn mul_inv(num: usize, modulus: usize) -> Option<usize> {
    let (g, x, _) = egcd(num  as i64, modulus as i64);
    if g != 1 {
        None 
    } else {
        let t = x as usize;
        Some( t.rem_euclid(modulus) )
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