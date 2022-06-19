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
            let mut char_pos = symbol as u16;
            let char_pos_cap_a = 'A' as u16;
            if char_pos < char_pos_cap_a {
                Err(ActionError::from_generation_error(
                    &error::MappingError::new(symbol),
                ))
            } else {
                char_pos = char_pos - char_pos_cap_a;
                if char_pos >= 7 * 7 {
                    if symbol == 'x' {
                        Ok(notation::MusicalElement::Rest {
                            duration: notation::Duration(1),
                        })
                    } else {
                        Err(ActionError::from_generation_error(
                            &error::MappingError::new(symbol),
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
            Err(ActionError::from_generation_error(&error::PitchError::new(
                &self.key,
                &self.scale_kind,
            )))
        }
    }
}
