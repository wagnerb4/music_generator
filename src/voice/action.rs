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
pub mod simple_action;

pub use simple_action::SimpleAction;
