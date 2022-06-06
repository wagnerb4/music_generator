/* This module models the actions
 * that the Atoms of an Axiom, used to
 * build a Voice, can do.
 */

use crate::l_system::{Atom, Axiom};
use crate::musical_notation as notation;

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

pub mod error;

/**
 * An ActionState is used to create a Voice.
 */
pub trait ActionState {
    fn get_neutral_state() -> Self;
    fn push(&self);
    fn pop(&mut self) -> Result<(), error::ActionError>;
}

/**
 * An Action is used to create a MusicalElement from
 * an Atom defined by its representative symbol. An Action
 * can modify the ActionState used to create a Voice.
 */
pub trait Action<S: ActionState> {
    fn gen_next_musical_element(
        &self,
        symbol: char,
        state: RefMut<S>,
    ) -> Result<notation::MusicalElement, error::ActionError>;
}

pub enum AtomType<S: ActionState> {
    NoAction,
    HasAction { action: Rc<dyn Action<S>> },
    PushStack,
    PopStack,
}

impl super::Voice {
    pub fn from<S: ActionState>(
        axiom: &Axiom,
        atom_types: HashMap<&Atom, AtomType<S>>,
    ) -> Result<super::Voice, error::ActionError> {
        let mut voice = super::Voice {
            musical_elements: vec![],
        };

        let current_state: RefCell<S> = RefCell::new(S::get_neutral_state());

        for atom in axiom.atoms() {
            match atom_types.get(&atom) {
                Some(atom_type) => match atom_type {
                    AtomType::HasAction { action } => voice.musical_elements.push(
                        action.gen_next_musical_element(atom.symbol, current_state.borrow_mut())?,
                    ),
                    AtomType::PushStack => current_state.borrow().push(),
                    AtomType::PopStack => current_state.borrow_mut().pop()?,
                    AtomType::NoAction => {}
                },
                None => {
                    return Err(error::ActionError::from_error_kind(
                        &super::ErrorKind::UndefinedAtomType,
                    ))
                }
            };
        }

        return Ok(voice);
    }
}

/**
 * This is an ActionState that does
 * nothing. Used in the creation of very
 * simple Voices.
 *
 * Actions, that could use this ActionState are
 * one-to-one mappings Actions.
 */
pub struct NeutralActionState {}

impl ActionState for NeutralActionState {
    fn get_neutral_state() -> NeutralActionState {
        NeutralActionState {}
    }
    fn push(&self) {}
    fn pop(&mut self) -> Result<(), error::ActionError> {
        Ok(())
    }
}

/**
 * A SimpleAction is an Action, that maps the 26 upper case
 * letters A to Z and the 23 lower case letters a to w in that
 * order to the notes of seven consecutive octaves of the given key.
 * The letter x will be mapped to a rest.
 */
pub struct SimpleAction<T: notation::Temperament> {
    key: notation::Key<T>,
    scale_kind: &'static notation::ScaleKind,
}

impl<T: notation::Temperament> SimpleAction<T> {
    pub fn new(key: notation::Key<T>, scale_kind: &'static notation::ScaleKind) -> Self {
        SimpleAction { key, scale_kind }
    }
}

impl<T: notation::Temperament> Action<NeutralActionState> for SimpleAction<T> {
    fn gen_next_musical_element(
        &self,
        symbol: char,
        _state: RefMut<NeutralActionState>,
    ) -> Result<notation::MusicalElement, error::ActionError> {
        if let Some(pitches) = self.key.get_scale(self.scale_kind, 4, 1, 7 * 7) {
            let mut char_pos = symbol as u16;
            let char_pos_cap_a = 'A' as u16;
            if char_pos < char_pos_cap_a {
                Err(error::ActionError::from_error_kind(
                    &super::ErrorKind::GenerationError,
                ))
            } else {
                char_pos = char_pos - char_pos_cap_a + 1;
                if char_pos > 7 * 7 {
                    if symbol == 'x' {
                        Ok(notation::MusicalElement::Rest {
                            duration: notation::Duration(1),
                        })
                    } else {
                        Err(error::ActionError::from_error_kind(
                            &super::ErrorKind::GenerationError,
                        ))
                    }
                } else {
                    Ok(notation::MusicalElement::Note {
                        pitch: pitches[char_pos as usize],
                        duration: notation::Duration(1),
                        volume: notation::M,
                    })
                }
            }
        } else {
            Err(error::ActionError::from_error_kind(
                &super::ErrorKind::GenerationError,
            ))
        }
    }
}
