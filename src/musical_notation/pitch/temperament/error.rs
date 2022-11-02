use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TemperamentError {
    message: String,
}

impl Display for TemperamentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There was a problem creating the temperament. {}",
            self.message
        )
    }
}

impl Error for TemperamentError {}

impl From<TemperamentError> for String {
    fn from(temperament_error: TemperamentError) -> Self {
        format!(
            "There was a problem creating the temperament. {}",
            temperament_error.message
        )
    }
}

impl From<&str> for TemperamentError {
    fn from(message: &str) -> Self {
        TemperamentError {
            message: String::from(message),
        }
    }
}
