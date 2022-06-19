use std::error::Error;
use std::fmt;

use crate::musical_notation::{Key, ScaleKind, Temperament};

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
    scale_kind: &'static ScaleKind,
}

impl PitchError {
    pub fn new<T: Temperament>(key: &Key<T>, scale_kind: &'static ScaleKind) -> Self {
        PitchError {
            key_msg: format!("{}", key),
            scale_kind,
        }
    }
}

impl fmt::Display for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "No pitches for a {:?} scale on a {} key.",
            self.scale_kind, self.key_msg
        )
    }
}

impl fmt::Debug for PitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PitchError (key: {}, scale_kind: {:?})",
            self.key_msg, self.scale_kind
        )
    }
}

impl Error for PitchError {}
