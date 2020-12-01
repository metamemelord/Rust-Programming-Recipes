use failure::*;

#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display = "Could not load the file: {:?}", 0)]
    LoadError(std::io::Error),
    #[fail(display = "Could not load the file: {:?}", 0)]
    ParseError(serde_json::Error),
    #[fail(display = "Could not load the file: {:?}", 0)]
    Error(&'static str),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Error(e)
    }
}
