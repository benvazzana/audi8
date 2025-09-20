use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct InsufficientInputError;

impl fmt::Display for InsufficientInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "not enough input samples have been provided")
    }
}

impl Error for InsufficientInputError {}

