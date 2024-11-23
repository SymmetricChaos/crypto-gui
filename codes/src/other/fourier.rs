use itertools::Itertools;
use num::{complex::Complex64, Complex, FromPrimitive, Zero};

use crate::traits::Code;
use core::f64;

fn dft(v: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let tau = Complex64::from_f64(f64::consts::TAU).unwrap();
    let mut out = Vec::with_capacity(v.len());
    let l = v.len();
    let lc = Complex64::from_usize(l).unwrap();
    let idxs = (0..l)
        .map(|i| Complex64::from_usize(i).unwrap())
        .collect_vec();
    for k in 0..l {
        out.push(Complex64::zero());
        for n in 0..l {
            out[k] += v[n] * (tau * -Complex64::i() * idxs[n] * (idxs[k] / lc)).exp()
        }
    }
    out
}

pub struct Fourier {}

impl Default for Fourier {
    fn default() -> Self {
        Self {}
    }
}

impl Code for Fourier {
    fn encode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        todo!()
    }

    fn decode(&self, text: &str) -> Result<String, crate::errors::CodeError> {
        todo!()
    }
}

#[cfg(test)]
mod fourier_tests {

    use nalgebra::ComplexField;
    use num::Num;

    use super::*;

    #[test]
    fn dft_test() {
        let v0 = ["1", "2-i", "-i", "-1+2i"]
            .map(|s| Complex64::from_str_radix(s, 10).unwrap())
            .to_vec();
        let v1 = ["2", "-2-2i", "-2i", "4+4i"]
            .map(|s| Complex64::from_str_radix(s, 10).unwrap())
            .to_vec();

        let ft: Vec<Complex<f64>> = dft(&v0).into_iter().map(|z| z.round()).collect_vec();

        assert_eq!(v1, ft);
    }
}
