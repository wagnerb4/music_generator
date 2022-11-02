use super::{Accidental, NoteName, Pitch, Tone, DEGREES_IN_SCALE, OCTAVE_MULTIPLICATIVE};
use crate::musical_notation::pitch::temperament::error::TemperamentError;
use crate::musical_notation::pitch::temperament::proportionen::Proportion;
use std::cmp::Ordering;

pub mod error;
mod proportionen;

pub const STUTTGART_PITCH: f64 = 440.0;
pub const BAROQUE_PITCH: f64 = 415.0;
pub const CHORTON_PITCH: f64 = 466.0;
pub const CLASSICAL_PITCH: f64 = 429.5; // 427–430

const REFERENCE_PITCH_OCTAVE: u8 = 4;

/// twelve tone temperament
///
pub trait Temperament {
    /// Construct a new object of this Temperament and use the given
    /// pitch standard as a reference for the pitch creation.
    ///
    /// # Arguments
    /// * `pitch_standard` - refers to the frequency of A_4 in Herz
    ///
    fn new(
        pitch_standard: f64,
        scale: [Tone; DEGREES_IN_SCALE as usize],
    ) -> Result<Self, TemperamentError>
    where
        Self: Sized;

    /// Get the pitch of a given tone in a given octave using this Temperament.
    ///
    /// # Arguments
    /// * `octave`  - refers to the number of the octave in scientific pitch notation and can theoretically be lower than 0 or higher than 9
    /// * `tone` - refers to the tone of a note in the musical notation and consists of note name and its accidental
    ///
    fn get_pitch(&self, octave: i16, tone: Tone) -> Option<Pitch>;

    /// defines the number of notes in an octave
    ///
    fn get_octave_additive() -> u8 {
        12
    }
}

/// Returns the position of a tone in the twelve tone system.
///
fn get_position(tone: Tone) -> u8 {
    match (tone.note_name, tone.accidental) {
        (&NoteName::C, &Accidental::Flat) => 12,
        (&NoteName::C, &Accidental::Natural) => 1,
        (&NoteName::C, &Accidental::Sharp) => 2,
        (&NoteName::D, &Accidental::Flat) => 2,
        (&NoteName::D, &Accidental::Natural) => 3,
        (&NoteName::D, &Accidental::Sharp) => 4,
        (&NoteName::E, &Accidental::Flat) => 4,
        (&NoteName::E, &Accidental::Natural) => 5,
        (&NoteName::E, &Accidental::Sharp) => 6,
        (&NoteName::F, &Accidental::Flat) => 5,
        (&NoteName::F, &Accidental::Natural) => 6,
        (&NoteName::F, &Accidental::Sharp) => 7,
        (&NoteName::G, &Accidental::Flat) => 7,
        (&NoteName::G, &Accidental::Natural) => 8,
        (&NoteName::G, &Accidental::Sharp) => 9,
        (&NoteName::A, &Accidental::Flat) => 9,
        (&NoteName::A, &Accidental::Natural) => 10,
        (&NoteName::A, &Accidental::Sharp) => 11,
        (&NoteName::B, &Accidental::Flat) => 11,
        (&NoteName::B, &Accidental::Natural) => 12,
        (&NoteName::B, &Accidental::Sharp) => 1,
    }
}

pub struct EqualTemperament {
    pitch_standard: f64,
}

impl EqualTemperament {
    /// defines the degree of the reference pitch
    ///
    fn get_reference_pitch_degree() -> u8 {
        10
    }
}

impl Temperament for EqualTemperament {
    fn new(
        pitch_standard: f64,
        _: [Tone; DEGREES_IN_SCALE as usize],
    ) -> Result<EqualTemperament, TemperamentError> {
        Ok(EqualTemperament { pitch_standard })
    }

    fn get_pitch(&self, octave: i16, tone: Tone) -> Option<Pitch> {
        let position: i16 = get_position(tone) as i16;
        let octave_interval =
            (octave - REFERENCE_PITCH_OCTAVE as i16) * Self::get_octave_additive() as i16;
        let relative_a = position - Self::get_reference_pitch_degree() as i16;
        let interval_size = relative_a + octave_interval;
        return Some(Pitch(
            self.pitch_standard
                * (OCTAVE_MULTIPLICATIVE as f64)
                    .powf(interval_size as f64 / Self::get_octave_additive() as f64),
        ));
    }
}

/**
 * Creates a seven tone temperament based on whole
 * number rations by leveraging the idea of euler's tonnetz.
 */
pub struct JustIntonation {
    pitch_standard: f64,
    reference_pitch_degree: u8,
    proportionen: [Proportion; DEGREES_IN_SCALE as usize],
}

impl JustIntonation {
    fn calc_proportionen(
        scale: [Tone; DEGREES_IN_SCALE as usize],
    ) -> [Proportion; DEGREES_IN_SCALE as usize] {
        todo!()
    }
}

impl Temperament for JustIntonation {
    fn new(
        pitch_standard: f64,
        scale: [Tone; DEGREES_IN_SCALE as usize],
    ) -> Result<JustIntonation, TemperamentError>
    where
        Self: Sized,
    {
        match scale.binary_search_by(|tone: &Tone| -> Ordering {
            tone.note_name.get_index().cmp(&NoteName::A.get_index())
        }) {
            Ok(reference_pitch_degree) => {
                let tone = scale[reference_pitch_degree];
                let equal_temperament = EqualTemperament::new(pitch_standard, scale).unwrap();
                Ok(JustIntonation {
                    // adjust the pitch_standard if the scale doesn't contain an a natural
                    pitch_standard: equal_temperament
                        .get_pitch(REFERENCE_PITCH_OCTAVE as i16, tone)
                        .unwrap()
                        .get_hz(),
                    reference_pitch_degree: reference_pitch_degree as u8,
                    proportionen: JustIntonation::calc_proportionen(scale),
                })
            }
            Err(_) => Err(TemperamentError::from(
                "Couldn't find NoteName A in given scale.",
            )),
        }
    }

    fn get_pitch(&self, octave: i16, tone: Tone) -> Option<Pitch> {
        let mut position = get_position(tone) as i16;

        let relative_a = position - self.reference_pitch_degree as i16;
        let octave_proportion =
            proportionen::OCTAVE_UP.pow((octave - REFERENCE_PITCH_OCTAVE as i16) as i32);

        let mut position_proportion = proportionen::UNIT;

        if relative_a > 0 {
            // position > reference
            for i in (self.reference_pitch_degree - 1) as u16
                ..((self.reference_pitch_degree - 1) as u16 + relative_a as u16)
            {
                position_proportion = position_proportion.fusion(&self.proportionen[i as usize]);
            }
        } else if relative_a < 0 {
            // position < reference
            position = position - 1; // 1 -> 0; 5 -> 4; 4 -> 3
            for i in position..(4 + 1) {
                // i = 0, 1, 2, 3, 4; i = 4; i = 3, 4
                // position + 4 - i = 4, 3, 2, 1, 0; position + 4 - i = 4; position + 4 - i = 4, 3
                position_proportion =
                    position_proportion.fusion(&self.proportionen[(position + 4 - i) as usize]);
            }
            position_proportion = position_proportion.invert();
        }

        return Some(Pitch(
            octave_proportion
                .fusion(&position_proportion)
                .scale(self.pitch_standard),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{EqualTemperament, Temperament, Tone, STUTTGART_PITCH};
    use crate::musical_notation::pitch::temperament::{proportionen, JustIntonation};
    use crate::musical_notation::pitch::DEGREES_IN_SCALE;
    use crate::musical_notation::{Key, ScaleKind};

    #[test]
    fn equal_temperament_test() -> Result<(), String> {
        let temp = EqualTemperament::new(
            STUTTGART_PITCH,
            [Tone::from("C").unwrap(); DEGREES_IN_SCALE as usize],
        )?;

        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, Tone::from("A")?)),
            "Some(Pitch(440.000))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, Tone::from("C")?)),
            "Some(Pitch(261.626))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, Tone::from("B#")?)),
            "Some(Pitch(261.626))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, Tone::from("B")?)),
            "Some(Pitch(493.883))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(5, Tone::from("C")?)),
            "Some(Pitch(523.251))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(5, Tone::from("C")?)),
            "Some(Pitch(523.251))"
        );

        return Ok(());
    }

    #[test]
    fn just_intonation_test() -> Result<(), String> {
        let expected_proportionen: [proportionen::Proportion; 7] = [
            proportionen::Proportion::new(8, 9),   // D
            proportionen::Proportion::new(9, 10),  // E
            proportionen::Proportion::new(15, 16), // F
            proportionen::Proportion::new(8, 9),   // G
            proportionen::Proportion::new(8, 9),   // A
            proportionen::Proportion::new(9, 10),  // B
            proportionen::Proportion::new(15, 16), // C
        ];

        let c_natural_major = Key::new(
            Tone::from("C")?,
            &ScaleKind::Major,
            STUTTGART_PITCH,
            JustIntonation::new,
        )?;

        assert_eq!(
            c_natural_major.temperament.proportionen,
            expected_proportionen
        );

        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("C")?)
            ),
            "Some(Pitch(260.741))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("D")?)
            ),
            "Some(Pitch(293.333))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("E")?)
            ),
            "Some(Pitch(325.926))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("F")?)
            ),
            "Some(Pitch(347.654))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("G")?)
            ),
            "Some(Pitch(391.111))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("A")?)
            ),
            "Some(Pitch(440.000))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(4, Tone::from("B")?)
            ),
            "Some(Pitch(488.889))"
        );
        assert_eq!(
            format!(
                "{:.3?}",
                c_natural_major.temperament.get_pitch(5, Tone::from("C")?)
            ),
            "Some(Pitch(521.481))"
        );
        return Ok(());
    }
}
