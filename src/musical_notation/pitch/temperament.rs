use super::{Pitch, OCTAVE_MULTIPLICATIVE};

mod proportionen;

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

/*
 * twelve tone temperament
 */
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

    /**
     * returns the number of notes in an octave
     */
    fn get_octave_additive() -> u8 {
        12
    }

    /**
     * returns the degree of the reference pitch
     */
    fn get_reference_pitch_degree() -> u8 {
        10
    }
}

/*
 * seven tone temperament
 */
pub trait SevenToneTemperament {
    /**
     * Construct a new object of this Temperament
     * and use the given pitch standard as a reference
     * for the pitch creation.
     */
    fn new(
        pitch_standard: f64,
        reference_pitch_degree: u8,
        proportionen: [proportionen::Proportion; 7],
    ) -> Self
    where
        Self: Sized;

    /**
     * Get the pitch of a given tone in a given octave by its position using this Temperament.
     * octave referes to the number of the octave in scientific pitch notation can theoretically be lower than 0 or higher than 9
     * position refers to the position of the tone whose pitch should be calculated. If the position is lower than 1 or greater than 7 the relative pitches in
     * the respective octaves will be calculated
     * pitch:    c d e f g a h
     * position: 1 2 3 4 5 6 7
     */
    fn get_pitch(&self, octave: i16, position: i16) -> Option<Pitch>;

    /**
     * returns the number of notes in an octave
     */
    fn get_octave_additive() -> u8 {
        7
    }
}

/**
 * Creates a seven tone temperament based on whole
 * number rations by leveraging the idea of euler's tonnetz.
 */
pub struct JustIntonation {
    pitch_standard: f64,
    reference_pitch_degree: u8,
    proportionen: [proportionen::Proportion; 7],
}

impl SevenToneTemperament for JustIntonation {
    fn new(
        pitch_standard: f64,
        reference_pitch_degree: u8,
        proportionen: [proportionen::Proportion; 7],
    ) -> JustIntonation {
        JustIntonation {
            pitch_standard,
            reference_pitch_degree,
            proportionen,
        }
    }

    fn get_pitch(&self, octave: i16, position: i16) -> Option<Pitch> {
        let relative_a = position - self.reference_pitch_degree as i16;
        let octave_proportion =
            proportionen::OCTAVE_UP.pow((octave - REFERENCE_PITCH_OCTAVE as i16) as i32);

        let mut position_proportion = proportionen::UNIT;

        if relative_a > 0 {
            for i in 0..relative_a {
                position_proportion = position_proportion.fusion(&self.proportionen[i as usize]);
            }
        } else if relative_a < 0 {
            let relative_a = relative_a + 7; // -6 -> 1
            for i in relative_a..(6 + 1) {
                // 1, 2, 3, 4, 5, 6
                // 6, 5, 4, 3, 2, 1
                position_proportion =
                    position_proportion.fusion(&self.proportionen[(relative_a + 6 - i) as usize]);
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

pub struct EqualTemperament {
    pitch_standard: f64,
}

impl Temperament for EqualTemperament {
    fn new(pitch_standard: f64) -> EqualTemperament {
        EqualTemperament { pitch_standard }
    }

    fn get_pitch(&self, octave: i16, position: i16) -> Option<Pitch> {
        let octave_intervall =
            (octave - REFERENCE_PITCH_OCTAVE as i16) * Self::get_octave_additive() as i16;
        let relative_a = position - Self::get_reference_pitch_degree() as i16;
        let intervall_size = relative_a + octave_intervall;
        return Some(Pitch(
            self.pitch_standard
                * (OCTAVE_MULTIPLICATIVE as f64)
                    .powf(intervall_size as f64 / Self::get_octave_additive() as f64),
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
