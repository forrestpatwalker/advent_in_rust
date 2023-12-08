use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AdventError {
    #[error("Failed with an error: {0}")]
    SomeError(String)
}

impl From<std::io::Error> for AdventError {
    fn from(err: std::io::Error) -> AdventError {
        AdventError::SomeError(err.to_string())
    }
}