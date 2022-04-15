use crate::musical_notation as notation;

use fundsp::audiounit::AudioUnit64;
use fundsp::math::bpm_hz;
use fundsp::sequencer::Sequencer;

pub mod action;

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedAtomType,
    PopOnEmptyStack,
    GenerationError,
}

pub struct Voice {
    musical_elements: Vec<notation::MusicalElement>,
}

impl Voice {
    pub fn get_len(&self) -> u16 {
        let mut len: u16 = 0;

        for musical_element in &self.musical_elements {
            len += musical_element.get_duration().get_time_units();
        }

        return len;
    }

    pub fn sequence<T>(&self, sequencer: &mut Sequencer, bpm: u16, create_audio_unit: T)
    where
        T: Fn(notation::Pitch, notation::Volume) -> Box<dyn AudioUnit64>,
    {
        let bpm_in_hz: f64 = bpm_hz(bpm as f64);
        let mut last_time_unit: u16 = 0;

        for musical_element in &self.musical_elements {
            match musical_element {
                notation::MusicalElement::Rest { duration } => {
                    last_time_unit += duration.get_time_units();
                }
                notation::MusicalElement::Note {
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
