mod l_system;
mod musical_notation;

pub mod error {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub enum ErrorKind {
        UndefinedAtomType,
        PopOnEmptyStack,
        GenerationError,
    }

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
}

use error::{ActionError, ErrorKind};

pub trait ActionState {
    fn get_neutral_state() -> Self;
    fn push(&self);
    fn pop(&mut self) -> Result<(), ActionError>;
}

pub trait Action<T: ActionState> {
    fn gen_next_musical_element(
        symbol: char,
        state: &mut T,
    ) -> Result<musical_notation::MusicalElement, ActionError>;
}

use l_system::{Atom, Axiom};
use std::collections::HashMap;

enum AtomType {
    NoAction,
    HasAction,
    PushStack,
    PopStack,
}

pub struct Voice {
    musical_elements: Vec<musical_notation::MusicalElement>,
}

impl Voice {
    fn from<S: ActionState, A: Action<S>>(
        axiom: &Axiom,
        atom_types: HashMap<Atom, AtomType>,
    ) -> Result<Voice, ActionError> {
        let mut voice = Voice {
            musical_elements: vec![],
        };

        let mut current_state: S = S::get_neutral_state();

        for atom in &axiom.atom_list {
            match atom_types.get(&atom) {
                Some(atom_type) => match atom_type {
                    AtomType::HasAction => voice.musical_elements.push(
                        A::gen_next_musical_element(atom.symbol, &mut current_state)?,
                    ),
                    AtomType::PushStack => current_state.push(),
                    AtomType::PopStack => current_state.pop()?,
                    AtomType::NoAction => {}
                },
                None => return Err(ActionError::from_error_kind(&ErrorKind::UndefinedAtomType)),
            };
        }

        return Ok(voice);
    }
}
