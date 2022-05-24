use crate::cipher_id::CipherID;

#[derive(Debug, PartialEq, Eq)]
pub enum CipherCategory {
    Substituion,
}

impl CipherCategory {
    pub fn description(&self) -> &'static str {
        match self {
            CipherCategory::Substituion => "Substitution ciphers are the simplest form of cipher.",
        }
    }

    pub fn ciphers(&self) -> &[CipherID] {
        match self {
            CipherCategory::Substituion => &[
                                            CipherID::Caesar,
                                            CipherID::Decoder,
                                            CipherID::Affine,
                                            CipherID::Substitution,],
        }
    }
}
