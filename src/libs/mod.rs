use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct QecError(String);

impl Error for QecError {}


impl fmt::Display for QecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

