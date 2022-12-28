use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FlattenError {
    pub message: String,
}

impl Display for FlattenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FlattenError {}
