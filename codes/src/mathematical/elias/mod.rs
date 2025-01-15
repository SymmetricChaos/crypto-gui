pub mod delta;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EliasVariant {
    Delta,
    Gamma,
    Omega,
}
