pub mod pitch {
    /**
     * Defines the pitch of a note in Herz.
     */
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Pitch(f64);

    impl Pitch {
        pub fn get_hz(&self) -> f64 {
            self.0
        }
    }

    pub mod temperament {
        use super::Pitch;

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

        pub trait Temperament {
            /**
             * Construct a new object of this Temperament
             * and use the given pitch standard as a reference
             * for the pitch creation.
             */
            fn new(pitch_standard: f64) -> Self;

            /**
             * Get the pitch of a given scale-degree in a given octave using this Temperament.
             * octave referes to the number of the octave in scientific pitch notation can theoretically be lower than 0 or higher than 9
             * degree refers to the scale-degree of the note whose pitch should be calculated if the degree is lower than 1 or greater than 12 the relative pitches in
             * the respective octaves will be calculated
             * c c# d d# e f f# g g# a  a# h
             * 1 2  3 4  4 6 7  8 9  10 11 12
             */
            fn get_pitch(&self, octave: i16, degree: i16) -> Option<Pitch>;
        }

        pub struct EqualTemperament {
            pitch_standard: f64,
        }

        impl Temperament for EqualTemperament {
            fn new(pitch_standard: f64) -> EqualTemperament {
                EqualTemperament { pitch_standard }
            }

            fn get_pitch(&self, octave: i16, degree: i16) -> Option<Pitch> {
                let octave_intervall = (octave - 4) * 12;
                let relative_a = degree - 10;
                let intervall_size = relative_a + octave_intervall;
                return Some(Pitch(
                    self.pitch_standard * 2_f64.powf(intervall_size as f64 / 12.0_f64),
                ));
            }
        }

        #[cfg(test)]
        mod tests {
            use super::{EqualTemperament, Pitch, Temperament, STUTTGART_PITCH};

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
    }
}

pub mod duration {
    /**
     * Defines the duration of a MusicalElement using the
     * [time unit box system](https://en.wikipedia.org/wiki/Time_unit_box_system).
     * The number that Duration contains refers the the number of boxes of a fixed unit of time
     * that the MusicalElement is played for.
     */
    #[derive(Debug, Copy, Clone)]
    pub struct Duration(u16);

    impl Duration {
        pub fn get_time_units(&self) -> u16 {
            self.0
        }
    }
}

pub mod volume {
    #[derive(Debug, Copy, Clone)]
    pub struct Volume(u8);

    const STEP_SIZE: u8 = 28;
    pub const SILENT: Volume = Volume(0);
    pub const PPP: Volume = Volume(1 * STEP_SIZE);
    pub const PP: Volume = Volume(2 * STEP_SIZE);
    pub const P: Volume = Volume(3 * STEP_SIZE);
    pub const MP: Volume = Volume(4 * STEP_SIZE);
    pub const M: Volume = Volume(5 * STEP_SIZE);
    pub const MF: Volume = Volume(6 * STEP_SIZE);
    pub const F: Volume = Volume(7 * STEP_SIZE);
    pub const FF: Volume = Volume(8 * STEP_SIZE);
    pub const FFF: Volume = Volume(9 * STEP_SIZE);
}

pub enum MusicalElement {
    Rest {
        duration: duration::Duration,
    },
    Note {
        pitch: pitch::Pitch,
        duration: duration::Duration,
        volume: volume::Volume,
    },
}

impl MusicalElement {
    pub fn get_duration(&self) -> duration::Duration {
        match self {
            MusicalElement::Rest { duration } => *duration,
            MusicalElement::Note { duration, .. } => *duration,
        }
    }
}
