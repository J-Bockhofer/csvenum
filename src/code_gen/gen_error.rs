use thiserror::Error as Error;


#[derive(Error, Debug, PartialEq)]
pub enum GenError {

    #[error("Malformed declaration: {0}")]
    DeclarationMalformed(String),

}