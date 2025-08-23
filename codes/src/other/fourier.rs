use crate::traits::Code;
use core::f64;
use itertools::Itertools;
use nalgebra::ComplexField;
use num::{complex::Complex64, Complex, FromPrimitive, Num, Zero};
use utils::errors::GeneralError;

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

fn dft_inv(v: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
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
            out[k] += v[n] * (tau * Complex64::i() * idxs[n] * (idxs[k] / lc)).exp()
        }
        out[k] = out[k] / lc;
    }
    out
}

pub struct Fourier {
    rounded: bool,
}

impl Default for Fourier {
    fn default() -> Self {
        Self { rounded: true }
    }
}

impl Code for Fourier {
    fn encode(&self, text: &str) -> Result<String, GeneralError> {
        let mut v = Vec::new();
        for s in text.split(",") {
            v.push(
                Complex64::from_str_radix(s.trim(), 10)
                    .or_else(|e| Err(GeneralError::input(e.to_string())))?,
            );
        }
        if self.rounded {
            Ok(dft(&v)
                .into_iter()
                .map(|z| z.round().to_string())
                .join(", "))
        } else {
            Ok(dft(&v).into_iter().map(|z| z.to_string()).join(", "))
        }
    }

    fn decode(&self, text: &str) -> Result<String, GeneralError> {
        let mut v = Vec::new();
        for s in text.split(",") {
            v.push(
                Complex64::from_str_radix(s.trim(), 10)
                    .or_else(|e| Err(GeneralError::input(e.to_string())))?,
            );
        }
        if self.rounded {
            Ok(dft_inv(&v)
                .into_iter()
                .map(|z| z.round().to_string())
                .join(", "))
        } else {
            Ok(dft_inv(&v).into_iter().map(|z| z.to_string()).join(", "))
        }
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

        // Do note the rounding
        let ft: Vec<Complex<f64>> = dft(&v0).into_iter().map(|z| z.round()).collect_vec();

        assert_eq!(v1, ft);
    }

    // Input and output are written according to the simplest formatting rules
    // For this test we write the third element of the input as -0-1i because when decoding the zero has a negative sign
    const PLAINTEXT: &'static str = "1+0i, 2-1i, -0-1i, -1+2i";
    const CODETEXT: &'static str = "2+0i, -2-2i, 0-2i, 4+4i";

    #[test]
    fn encode_test() {
        let code = Fourier::default();
        assert_eq!(CODETEXT, code.encode(PLAINTEXT).unwrap());
    }

    #[test]
    fn decode_test() {
        let code = Fourier::default();
        assert_eq!(PLAINTEXT, code.decode(CODETEXT).unwrap());
    }
}
