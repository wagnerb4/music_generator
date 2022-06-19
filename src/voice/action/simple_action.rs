use super::{error::ActionError, Action, NeutralActionState};
use crate::musical_notation as notation;
use std::cell::RefMut;

pub mod error;

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
    ) -> Result<notation::MusicalElement, ActionError> {
        if let Some(pitches) = self.key.get_scale(self.scale_kind, 4, 1, 7 * 7) {
            let char_pos = symbol as u16;
            const CHAR_POS_CAP_A: u16 = 'A' as u16;
            const CHAR_POS_CAP_Z: u16 = 'Z' as u16;
            const CHAR_POS_LOW_A: u16 = 'a' as u16;
            const CHAR_POS_LOW_W: u16 = 'w' as u16;
            const CHAR_POS_LOW_X: u16 = 'x' as u16;

            match char_pos {
                CHAR_POS_LOW_X => Ok(notation::MusicalElement::Rest {
                    duration: notation::Duration(1),
                }),
                CHAR_POS_CAP_A..=CHAR_POS_CAP_Z => Ok(notation::MusicalElement::Note {
                    pitch: pitches[(char_pos - CHAR_POS_CAP_A) as usize],
                    duration: notation::Duration(1),
                    volume: notation::M,
                }),
                CHAR_POS_LOW_A..=CHAR_POS_LOW_W => Ok(notation::MusicalElement::Note {
                    pitch: pitches[(26 + char_pos - CHAR_POS_LOW_A) as usize],
                    duration: notation::Duration(1),
                    volume: notation::M,
                }),
                _ => Err(ActionError::from_generation_error(
                    &error::MappingError::new(symbol),
                )),
            }
        } else {
            Err(ActionError::from_generation_error(&error::PitchError::new(
                &self.key,
                &self.scale_kind,
            )))
        }
    }
}
