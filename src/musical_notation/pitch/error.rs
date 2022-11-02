use crate::musical_notation::pitch::temperament::error::TemperamentError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct KeyCreationError {
    message: String,
}

impl Display for KeyCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was an error creating the Key. {}", self.message)
    }
}

impl Error for KeyCreationError {}

impl From<TemperamentError> for KeyCreationError {
    fn from(temperament_error: TemperamentError) -> Self {
        KeyCreationError {
            message: String::from(temperament_error),
        }
    }
}

impl From<KeyCreationError> for String {
    fn from(error: KeyCreationError) -> Self {
        format!("There was an error creating the Key. {}", error.message)
    }
}
