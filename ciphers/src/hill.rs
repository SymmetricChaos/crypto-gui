use nalgebra::Matrix6;

use crate::{Cipher, CipherError};

// https://patents.google.com/patent/US1845947

pub struct Hill {
    mat: Matrix6<f64>, // we will be using small integers so there is no loss of precision
    key: String,
}

impl Hill {
    fn key_inv(&self) -> Result<Matrix6<f64>, CipherError> {
        todo!()
    }
}

impl Cipher for Hill {
    fn encrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }

    fn decrypt(&self, text: &str) -> Result<String, CipherError> {
        todo!()
    }
}
