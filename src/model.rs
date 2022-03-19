mod l_system;
pub mod musical_notation;

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

use musical_notation::pitch::Pitch;
use musical_notation::volume::Volume;
use musical_notation::MusicalElement;

use std::collections::HashMap;

use fundsp::audiounit::AudioUnit64;
use fundsp::math::bpm_hz;
use fundsp::sequencer::Sequencer;

enum AtomType {
    NoAction,
    HasAction,
    PushStack,
    PopStack,
}

pub struct Voice {
    musical_elements: Vec<MusicalElement>,
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

        for atom in axiom.atoms() {
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

    fn get_len(&self) -> u16 {
        let mut len: u16 = 0;

        for musical_element in &self.musical_elements {
            len += musical_element.get_duration().get_time_units();
        }

        return len;
    }

    pub fn sequence<T>(&self, sequencer: &mut Sequencer, bpm: f64, create_audio_unit: T)
    where
        T: Fn(Pitch, Volume) -> Box<dyn AudioUnit64>,
    {
        let length = self.get_len();
        let bpm_in_hz: f64 = bpm_hz(bpm);
        let mut last_time_unit: u16 = 0;

        for musical_element in &self.musical_elements {
            match musical_element {
                MusicalElement::Rest { duration } => {
                    last_time_unit += duration.get_time_units();
                }
                MusicalElement::Note {
                    pitch,
                    duration,
                    volume,
                } => {
                    let time_note_starts: f64 = last_time_unit as f64 / bpm_in_hz;
                    last_time_unit += duration.get_time_units();
                    let time_note_stops: f64 = last_time_unit as f64 / bpm_in_hz;
                    sequencer.add64(
                        time_note_starts,
                        time_note_stops,
                        0.2,
                        0.2,
                        create_audio_unit(*pitch, *volume),
                    );
                }
            }
        }
    }
}
