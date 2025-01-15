pub mod delta;
pub mod gamma;
pub mod omega;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EliasVariant {
    Delta,
    Gamma,
    Omega,
}
