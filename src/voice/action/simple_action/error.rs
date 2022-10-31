use std::error::Error;
use std::fmt;

use crate::musical_notation::{Key, Temperament};

#[derive(Debug)]
pub struct MappingError {
    symbol: char,
}

impl MappingError {
    pub fn new(symbol: char) -> Self {
        MappingError { symbol }
    }
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected symbol: '{}'.", self.symbol)
    }
}

impl Error for MappingError {}

pub struct PitchError {
    key_msg: String,
}

impl PitchError {
    pub fn new<T: Temperament>(key: &Key<T>) -> Self {
        PitchError {
            key_msg: format!("{}", key),
        }
    }
}

impl fmt::Display for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No pitches for a {} key.", self.key_msg)
    }
}

impl fmt::Debug for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PitchError (key: {})", self.key_msg)
    }
}

impl Error for PitchError {}
