use super::GenError;
use std::fmt;
#[allow(dead_code)]
mod variables;
pub use variables::{VarDeclaration, VarDeclareAs};
#[allow(dead_code, unused_imports)]
mod functions;


#[allow(dead_code, clippy::upper_case_acronyms)]
#[derive(Eq, PartialEq)]
pub enum Visibility {
    PUB,
    PRIV,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Self::PUB { write!(f, "pub")? }
        Ok(())
    }
} 
