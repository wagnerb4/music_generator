use super::{Pitch, OCTAVE_ADDITIVE, OCTAVE_MULTIPLICATIVE};

/* Different pitch standards.
 * The number always referes to
 * the frequency of A_4 in Herz.
 * Definitions are taken form Oxford Composer Companion JS Bach,
 * pp. 369–372. Oxford University Press, 1999
 */
pub const STUTTGART_PITCH: f64 = 440.0;
pub const BAROQUE_PITCH: f64 = 415.0;
pub const CHORTON_PITCH: f64 = 466.0;
pub const CLASSICAL_PITCH: f64 = 429.5; // 427–430

const REFERENCE_PITCH_OCTAVE: u8 = 4;
const REFERENCE_PITCH_DEGREE: u8 = 10;

pub trait Temperament {
    /**
     * Construct a new object of this Temperament
     * and use the given pitch standard as a reference
     * for the pitch creation.
     */
    fn new(pitch_standard: f64) -> Self
    where
        Self: Sized;

    /**
     * Get the pitch of a given tone in a given octave by its position using this Temperament.
     * octave referes to the number of the octave in scientific pitch notation can theoretically be lower than 0 or higher than 9
     * position refers to the position of the tone whose pitch should be calculated. If the position is lower than 1 or greater than 12 the relative pitches in
     * the respective octaves will be calculated
     * pitch:    c c# d d# e f f# g g# a  a# h
     * position: 1 2  3 4  5 6 7  8 9  10 11 12
     */
    fn get_pitch(&self, octave: i16, position: i16) -> Option<Pitch>;
}

pub struct EqualTemperament {
    pitch_standard: f64,
}

impl Temperament for EqualTemperament {
    fn new(pitch_standard: f64) -> EqualTemperament {
        EqualTemperament { pitch_standard }
    }

    fn get_pitch(&self, octave: i16, position: i16) -> Option<Pitch> {
        let octave_intervall = (octave - REFERENCE_PITCH_OCTAVE as i16) * OCTAVE_ADDITIVE as i16;
        let relative_a = position - REFERENCE_PITCH_DEGREE as i16;
        let intervall_size = relative_a + octave_intervall;
        return Some(Pitch(
            self.pitch_standard
                * (OCTAVE_MULTIPLICATIVE as f64)
                    .powf(intervall_size as f64 / OCTAVE_ADDITIVE as f64),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{EqualTemperament, Temperament, STUTTGART_PITCH};

    #[test]
    fn equal_temperament_test() {
        let temp = EqualTemperament::new(STUTTGART_PITCH);
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, 10)),
            "Some(Pitch(440.000))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, 1)),
            "Some(Pitch(261.626))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, 12)),
            "Some(Pitch(493.883))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(5, 1)),
            "Some(Pitch(523.251))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(4, 13)),
            "Some(Pitch(523.251))"
        );
        assert_eq!(
            format!("{:.3?}", temp.get_pitch(5, -11)),
            "Some(Pitch(261.626))"
        );
    }
}
