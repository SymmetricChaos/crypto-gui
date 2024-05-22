use itertools::Itertools;
use nalgebra::Matrix6;
use utils::vecstring::VecString;

use crate::{Cipher, CipherError};

// https://patents.google.com/patent/US1845947

pub struct Hill {
    pub alphabet: VecString,
    pub mat: Matrix6<f64>, // we will be using small integers so there is no loss of precision
    pub key1: String,
    pub key2: String,
}

impl Hill {
    fn mat_inv(&self) -> Result<Matrix6<f64>, CipherError> {
        todo!()
    }

    fn encrypt_chunk(&self, chunk: &str) -> Result<String, CipherError> {
        let nums = chunk
            .chars()
            .map(|c| self.alphabet.get_pos(c).unwrap() as f64);
        let message = Matrix6::from_iterator(nums);

        let prod = self.mat * message;

        Ok(prod
            .into_iter()
            .map(|n| *self.alphabet.get_char(*n as usize).unwrap())
            .collect())
    }
}

impl Cipher for Hill {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        // Vigenere step?
        // Matrix step
        // Vigenere step?
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}

// #[cfg(test)]
// mod hill_test {
//     use rand::{thread_rng, Rng};
//     use utils::math_functions::mul_inv;

//     use super::*;

//     #[test]
//     fn test_inverse() {
//         let mut rng = thread_rng();
//         let i = (0..36).map(|_| f64::from(rng.gen_range(0..26)));
//         let m = Matrix6::from_iterator(i);
//         let ident = Matrix6::identity();
//         let inv = m.solve_lower_triangular(&ident);

//         println!("{m:?}");
//         println!("{inv:?}");
//         println!("{:?}", inv.unwrap() * m);
//     }
// }
