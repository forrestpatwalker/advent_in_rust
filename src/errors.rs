use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AdventError {
    #[error("Failed with an error: {0}")]
    SomeError(String)
}