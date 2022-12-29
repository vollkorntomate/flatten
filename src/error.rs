use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FlattenError {
    pub message: String,
}

impl FlattenError {
    pub fn new(message: &str) -> FlattenError {
        FlattenError {
            message: String::from(message),
        }
    }
}

impl Display for FlattenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FlattenError {}
