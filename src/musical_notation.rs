mod pitch;
pub use pitch::temperament::{EqualTemperament, Temperament};
pub use pitch::temperament::{BAROQUE_PITCH, CHORTON_PITCH, CLASSICAL_PITCH, STUTTGART_PITCH};
pub use pitch::{Key, Pitch, ScaleKind, Tone};

mod duration;
pub use duration::Duration;

mod volume;
pub use volume::Volume;
pub use volume::{F, FF, FFF, M, MF, MP, P, PP, PPP, SILENT};

#[derive(Debug)]
pub enum MusicalElement {
    Rest {
        duration: Duration,
    },
    Note {
        pitch: Pitch,
        duration: Duration,
        volume: Volume,
    },
}

impl MusicalElement {
    pub fn get_duration(&self) -> Duration {
        match self {
            MusicalElement::Rest { duration } => *duration,
            MusicalElement::Note { duration, .. } => *duration,
        }
    }
}
