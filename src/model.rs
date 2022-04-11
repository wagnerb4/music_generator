pub mod l_system;
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

pub struct NeutralActionState {}

impl ActionState for NeutralActionState {
    fn get_neutral_state() -> NeutralActionState {
        NeutralActionState {}
    }
    fn push(&self) {}
    fn pop(&mut self) -> Result<(), ActionError> {
        Ok(())
    }
}

use musical_notation::pitch::temperament::EqualTemperament;
use musical_notation::pitch::Key;
use musical_notation::MusicalElement;

pub trait Action<T: ActionState> {
    fn gen_next_musical_element(
        &self,
        symbol: char,
        state: &mut T,
    ) -> Result<MusicalElement, ActionError>;
}

pub struct SimpleAction {}

/*
impl Action<NeutralActionState> for SimpleAction {
    fn gen_next_musical_element(
        symbol: char,
        ,
    ) -> Result<MusicalElement, ActionError> {
        match symbol {
            '0'
        }
    }
}
*/

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

/**
 * Generates a simple action, that maps the 26 upper case
 * letters A to Z and the 23 lower case letters a to w in that
 * order to the notes of seven consecutive octaves of the given key.
 * The letter x will be mapped to a rest.
 * key can be an Integer from 1 to 12
 */
pub fn generate_simple_action<'a>(
    key: Key<'a, EqualTemperament>,
) -> Result<Rc<dyn Fn(char, RefMut<NeutralActionState>) -> Result<MusicalElement, ActionError>>, ActionError> {
    if let Some(pitches) = key.get_major_scale(4, 1, 7 * 7) {
		Ok(
			Rc::new(
				move |symbol: char, _state: RefMut<NeutralActionState>| -> Result<MusicalElement, ActionError> {
					let mut char_pos = symbol as u16;
					let char_pos_cap_a = 'A' as u16;
					if char_pos < char_pos_cap_a {
						Err(ActionError::from_error_kind(&ErrorKind::GenerationError))
					} else {
						char_pos = char_pos - char_pos_cap_a + 1;
						if char_pos > 7*7 {
							Err(ActionError::from_error_kind(&ErrorKind::GenerationError))
						} else {
							Ok(
								MusicalElement::Note {
									pitch: pitches[char_pos as usize],
									duration: musical_notation::duration::Duration(1),
									volume: musical_notation::volume::M
								}
							)
						}
					}
				},
			)
		)
	} else {
		Err(ActionError::from_error_kind(&ErrorKind::GenerationError))
	}
}

pub enum AtomType<S: ActionState> {
    NoAction,
    HasAction {
        action: Rc<dyn Fn(char, RefMut<S>) -> Result<MusicalElement, ActionError>>,
    },
    PushStack,
    PopStack,
}

use l_system::{Atom, Axiom};

use musical_notation::pitch::Pitch;
use musical_notation::volume::Volume;

use std::collections::HashMap;

use fundsp::audiounit::AudioUnit64;
use fundsp::math::bpm_hz;
use fundsp::sequencer::Sequencer;

pub struct Voice {
    musical_elements: Vec<MusicalElement>,
}

impl Voice {
    pub fn from<S: ActionState>(
        axiom: &Axiom,
        atom_types: HashMap<&Atom, AtomType<S>>,
    ) -> Result<Voice, ActionError> {
        let mut voice = Voice {
            musical_elements: vec![],
        };

        let current_state: RefCell<S> = RefCell::new(S::get_neutral_state());

        for atom in axiom.atoms() {
            match atom_types.get(&atom) {
                Some(atom_type) => match atom_type {
                    AtomType::HasAction { action } => voice
                        .musical_elements
                        .push(action(atom.symbol, current_state.borrow_mut())?),
                    AtomType::PushStack => current_state.borrow().push(),
                    AtomType::PopStack => current_state.borrow_mut().pop()?,
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

    pub fn sequence<T>(&self, sequencer: &mut Sequencer, bpm: u16, create_audio_unit: T)
    where
        T: Fn(Pitch, Volume) -> Box<dyn AudioUnit64>,
    {
        let bpm_in_hz: f64 = bpm_hz(bpm as f64);
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
