use std::error::Error;
use std::fmt;

use super::super::ErrorKind;

#[derive(Debug)]
pub struct ActionError {
    kind: &'static ErrorKind,
    message: String,
}

impl ActionError {
    pub fn from_generation_error<T: Error>(generation_error: &T) -> ActionError {
        ActionError {
            kind: &ErrorKind::GenerationError,
            message: format!("{}", generation_error),
        }
    }

    pub fn from_error_kind(kind: &'static ErrorKind) -> ActionError {
        ActionError {
            kind,
            message: match kind {
                ErrorKind::PopOnEmptyStack => String::from("Tried to pop an empty state stack"),
                ErrorKind::UndefinedAtomType => {
                    String::from("The type of an atom is left undefined")
                }
                ErrorKind::GenerationError => {
                    String::from("General error while generating a MusicalElement")
                }
            },
        }
    }
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "There was an Error while interpreting the Axiom: {}.",
            self.message
        )
    }
}

impl Error for ActionError {}
