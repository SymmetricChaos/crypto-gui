use crypto_bigint::U1024;

pub struct Vsh {
    pub n: U1024,
}

impl Default for Vsh {
    fn default() -> Self {
        Self {
            n: Default::default(),
        }
    }
}
