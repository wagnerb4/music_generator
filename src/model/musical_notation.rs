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

        const OCTAVE_ADDITIVE: u8 = 12;
        const OCTAVE_MULTIPLICATIVE: u8 = 2;
        const REFERENCE_PITCH_OCTAVE: u8 = 4;
        const REFERENCE_PITCH_DEGREE: u8 = 10;

        pub trait Temperament {
            /**
             * Construct a new object of this Temperament
             * and use the given pitch standard as a reference
             * for the pitch creation.
             */
            fn new(pitch_standard: f64) -> Self;

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
                let octave_intervall =
                    (octave - REFERENCE_PITCH_OCTAVE as i16) * OCTAVE_ADDITIVE as i16;
                let relative_a = position - REFERENCE_PITCH_DEGREE as i16;
                let intervall_size = relative_a + octave_intervall;
                return Some(Pitch(
                    self.pitch_standard
                        * (OCTAVE_MULTIPLICATIVE as f64)
                            .powf(intervall_size as f64 / OCTAVE_ADDITIVE as f64),
                ));
            }
        }

        const DEGREES_IN_SCALE: u8 = 7;
        //                                                              c  d  e  f  g  a  b  c
        const SEMITONES_IN_MAJOR_SCALE: [u8; DEGREES_IN_SCALE as usize] = [2, 2, 1, 2, 2, 2, 1];

        #[derive(Debug)]
        pub enum Accidental {
            Flat,
            Natural,
            Sharp,
        }

        pub enum Key<'a, T>
        where
            T: Temperament,
        {
            Do(&'static Accidental, &'a T),
            Re(&'static Accidental, &'a T),
            Mi(&'static Accidental, &'a T),
            Fa(&'static Accidental, &'a T),
            Sol(&'static Accidental, &'a T),
            La(&'static Accidental, &'a T),
            Ti(&'static Accidental, &'a T),
        }

        impl<'a, T> Key<'a, T>
        where
            T: Temperament,
        {
            fn get_index(&self) -> u8 {
                match self {
                    Key::Do(_, _) => 0,
                    Key::Re(_, _) => 1,
                    Key::Mi(_, _) => 2,
                    Key::Fa(_, _) => 3,
                    Key::Sol(_, _) => 4,
                    Key::La(_, _) => 5,
                    Key::Ti(_, _) => 6,
                }
            }

            fn get_accidental(&self) -> &'static Accidental {
                match self {
                    Key::Do(a, _) => a,
                    Key::Re(a, _) => a,
                    Key::Mi(a, _) => a,
                    Key::Fa(a, _) => a,
                    Key::Sol(a, _) => a,
                    Key::La(a, _) => a,
                    Key::Ti(a, _) => a,
                }
            }

            fn get_temperament(&self) -> &T {
                match self {
                    Key::Do(_, t) => t,
                    Key::Re(_, t) => t,
                    Key::Mi(_, t) => t,
                    Key::Fa(_, t) => t,
                    Key::Sol(_, t) => t,
                    Key::La(_, t) => t,
                    Key::Ti(_, t) => t,
                }
            }

            /**
             * Get the key of the respective position in the twelve-tone system.
             * position a position of 1 or 13 indicates the key of do
             * major is a boolean value indicating whether the key is intended to
             * to be used as a minor or major scale
             */
            fn key_by_position(&self, position: u8, major: bool) -> Option<Key<T>> {
                let mut position: u8 = position - 1;
                position %= OCTAVE_ADDITIVE;
                position += 1;

                let temperament: &T = self.get_temperament();

                let key = match position {
                    1 => Some(Key::Do(&Accidental::Natural, temperament)),
                    2 => Some(match major {
                        true => Key::Do(&Accidental::Sharp, temperament),
                        false => Key::Re(&Accidental::Flat, temperament),
                    }),
                    3 => Some(Key::Re(&Accidental::Natural, temperament)),
                    4 => Some(match major {
                        true => Key::Re(&Accidental::Sharp, temperament),
                        false => Key::Mi(&Accidental::Flat, temperament),
                    }),
                    5 => Some(Key::Mi(&Accidental::Natural, temperament)),
                    6 => Some(Key::Fa(&Accidental::Natural, temperament)),
                    7 => Some(match major {
                        true => Key::Fa(&Accidental::Sharp, temperament),
                        false => Key::Sol(&Accidental::Flat, temperament),
                    }),
                    8 => Some(Key::Sol(&Accidental::Natural, temperament)),
                    9 => Some(match major {
                        true => Key::Sol(&Accidental::Sharp, temperament),
                        false => Key::La(&Accidental::Flat, temperament),
                    }),
                    10 => Some(Key::La(&Accidental::Natural, temperament)),
                    11 => Some(match major {
                        true => Key::La(&Accidental::Sharp, temperament),
                        false => Key::Ti(&Accidental::Flat, temperament),
                    }),
                    12 => Some(Key::Ti(&Accidental::Natural, temperament)),
                    _ => None,
                };

                return key;
            }

            fn get_degree(&self, position: u8) -> Option<u8> {
                let mut position = position - 1;
                position %= OCTAVE_ADDITIVE;
                position += 1;

                for degree in 1..(DEGREES_IN_SCALE + 1) {
                    let mut position_of_degree = self.get_position(degree) - 1;
                    position_of_degree %= OCTAVE_ADDITIVE;
                    position_of_degree += 1;

                    if position == position_of_degree {
                        return Some(degree);
                    }
                }

                return None;
            }

            /**
             * Get the position of the tone in the twelve-tone system based
             * on the given scale-degree of the major scale.
             * For the Key of Mi the positions for the
             * degrees from 1 to 7 would be the following.
             * degree:   1  2  3  4  5  6  7 |  8 / 1
             * position: 4  6  8  9 11 13 15 | 16 (-12 = 4)
             *             +2 +2 +1 +2 +2 +2 | +1
             */
            fn get_position(&self, degree: u8) -> u8 {
                let mut end: u8 = degree - 1;

                let mut position: u8 = 0;

                if end > DEGREES_IN_SCALE {
                    end -= DEGREES_IN_SCALE;
                    let octaves: u8 = end / DEGREES_IN_SCALE;
                    end %= DEGREES_IN_SCALE;
                    position += (octaves + 1) * OCTAVE_ADDITIVE;
                    position += SEMITONES_IN_MAJOR_SCALE[0..end as usize].iter().sum::<u8>();
                } else {
                    position = SEMITONES_IN_MAJOR_SCALE[0..end as usize].iter().sum::<u8>();
                }

                let offset = SEMITONES_IN_MAJOR_SCALE[0..self.get_index() as usize]
                    .iter()
                    .sum::<u8>();
                position += offset;

                position = match self.get_accidental() {
                    Accidental::Flat => position - 1,
                    Accidental::Natural => position,
                    Accidental::Sharp => position + 1,
                };

                return position + 1;
            }

            /**
             * Calculate an array of consecutive pitches of the major scale using the given Temperament.
             * The Pitches will start in the given octave with the given scale-degree and comprise the given
             * number of pitches.
             */
            pub fn get_major_scale(
                &self,
                octave: i16,
                degree: u8,
                number_of_pitches: u8,
            ) -> Option<Vec<Pitch>> {
                let temperament = self.get_temperament();

                let mut pitches: Vec<Pitch> = vec![];

                for degree in degree..(degree + number_of_pitches) {
                    match temperament.get_pitch(octave, self.get_position(degree) as i16) {
                        Some(pitch) => pitches.push(pitch),
                        None => return None,
                    }
                }

                return Some(pitches);
            }

            pub fn get_minor_scale(
                &self,
                octave: i16,
                degree: u8,
                number_of_pitches: u8,
            ) -> Option<Vec<Pitch>> {
                let tonic = self.get_position(1);
                match self.key_by_position(tonic + 3, false) {
                    Some(minor) => {
                        let mapped_tonic_degree = minor.get_degree(tonic).unwrap();
                        let mapped_tonic = minor.get_position(mapped_tonic_degree);

                        let octave = octave
                            + ((tonic as i8 - mapped_tonic as i8) / OCTAVE_ADDITIVE as i8) as i16;

                        return minor.get_major_scale(
                            octave,
                            mapped_tonic_degree + (degree - 1),
                            number_of_pitches,
                        );
                    }
                    None => None,
                }
            }

            pub fn get_relative_minor_scale(
                &self,
                octave: i16,
                degree: u8,
                number_of_pitches: u8,
            ) -> Option<Vec<Pitch>> {
                let mut degree = degree - 1;
                degree -= 5;
                degree %= DEGREES_IN_SCALE;
                degree += 1;

                let submediant = self.get_position(1 + 5);

                match self.key_by_position(submediant, false) {
                    Some(relative_minor) => {
                        relative_minor.get_major_scale(octave, degree as u8, number_of_pitches)
                    }
                    None => None,
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::{Accidental, EqualTemperament, Key, Pitch, Temperament, STUTTGART_PITCH};

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

            #[test]
            fn test_get_position() {
                let temp = EqualTemperament::new(STUTTGART_PITCH);
                let key = Key::Do(&Accidental::Natural, &temp);
                assert_eq!(key.get_position(1), 1); // c
                assert_eq!(key.get_position(2), 3); // d
                assert_eq!(key.get_position(3), 5); // e
                assert_eq!(key.get_position(4), 6); // f
                assert_eq!(key.get_position(5), 8); // g
                assert_eq!(key.get_position(6), 10); // a
                assert_eq!(key.get_position(7), 12); // b
                assert_eq!(key.get_position(8), 13); // c
                assert_eq!(key.get_position(9), 15); // d
                assert_eq!(key.get_position(10), 17); // e
                assert_eq!(key.get_position(11), 18); // f
                assert_eq!(key.get_position(12), 20); // g
                assert_eq!(key.get_position(13), 22); // a
                assert_eq!(key.get_position(14), 24); // b
                assert_eq!(key.get_position(15), 25); // c

                let key = Key::Sol(&Accidental::Natural, &temp);
                assert_eq!(key.get_position(1), 8); // g
                assert_eq!(key.get_position(2), 10); // a
                assert_eq!(key.get_position(3), 12); // b
                assert_eq!(key.get_position(4), 13); // c
                assert_eq!(key.get_position(5), 15); // d
                assert_eq!(key.get_position(6), 17); // e
                assert_eq!(key.get_position(7), 19); // f#
                assert_eq!(key.get_position(8), 20); // g
                assert_eq!(key.get_position(9), 22); // a
                assert_eq!(key.get_position(10), 24); // b
                assert_eq!(key.get_position(11), 25); // c
                assert_eq!(key.get_position(12), 27); // d
                assert_eq!(key.get_position(13), 29); // e
                assert_eq!(key.get_position(14), 31); // f#
                assert_eq!(key.get_position(15), 32); // g
            }

            #[test]
            fn test_key_c_natural_major() {
                let temp = EqualTemperament::new(STUTTGART_PITCH);
                let key = Key::Do(&Accidental::Natural, &temp);
                match key.get_major_scale(4, 1, 8) {
                    Some(pitches) => {
                        assert_eq!(pitches.len(), 8);
                        assert_eq!(format!("{:.3?}", pitches[0]), "Pitch(261.626)" /*C_4*/);
                        assert_eq!(format!("{:.3?}", pitches[1]), "Pitch(293.665)" /*D_4*/);
                        assert_eq!(format!("{:.3?}", pitches[2]), "Pitch(329.628)" /*E_4*/);
                        assert_eq!(format!("{:.3?}", pitches[3]), "Pitch(349.228)" /*F_4*/);
                        assert_eq!(format!("{:.3?}", pitches[4]), "Pitch(391.995)" /*G_4*/);
                        assert_eq!(format!("{:.3?}", pitches[5]), "Pitch(440.000)" /*A_4*/);
                        assert_eq!(format!("{:.3?}", pitches[6]), "Pitch(493.883)" /*B_4*/);
                        assert_eq!(format!("{:.3?}", pitches[7]), "Pitch(523.251)" /*C_5*/);
                    }
                    None => panic!("expected some pitches"),
                }
            }

            #[test]
            fn test_key_g_flat_minor() {
                let temp = EqualTemperament::new(STUTTGART_PITCH);
                let key = Key::Sol(&Accidental::Flat, &temp);
                match key.get_minor_scale(4, 1, 8) {
                    Some(pitches) => {
                        assert_eq!(pitches.len(), 8);

                        // major [2, 2, 1, 2, 2, 2, 1]
                        // minor [2, 1, 2, 2, 1, 2, 2]

                        assert_eq!(
                            format!("{:.3?}", pitches[0]),
                            "Pitch(369.994)" /*(+0=-3) Gb_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[1]),
                            "Pitch(415.305)" /*(+2=-1) Ab_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[2]),
                            "Pitch(440.000)" /*(+1=0) A_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[3]),
                            "Pitch(493.883)" /*(+2=2) B_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[4]),
                            "Pitch(554.365)" /*(+2=4) Db_5*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[5]),
                            "Pitch(587.330)" /*(+1=5) D_5*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[6]),
                            "Pitch(659.255)" /*(+2=7) E_5*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[7]),
                            "Pitch(739.989)" /*(+2=9) Gb_5*/
                        );
                    }
                    None => panic!("expected some pitches"),
                }
            }

            #[test]
            fn test_key_f_sharp_minor() {
                let temp = EqualTemperament::new(STUTTGART_PITCH);
                let key = Key::Fa(&Accidental::Sharp, &temp);
                match key.get_minor_scale(4, 1, 8) {
                    Some(pitches) => {
                        assert_eq!(pitches.len(), 8);

                        // major [2, 2, 1, 2, 2, 2, 1]
                        // minor [2, 1, 2, 2, 1, 2, 2]

                        assert_eq!(
                            format!("{:.3?}", pitches[0]),
                            "Pitch(369.994)" /*(+0=-3) F#_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[1]),
                            "Pitch(415.305)" /*(+2=-1) G#_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[2]),
                            "Pitch(440.000)" /*(+1=0) A_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[3]),
                            "Pitch(493.883)" /*(+2=2) B_4*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[4]),
                            "Pitch(554.365)" /*(+2=4) C#_5*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[5]),
                            "Pitch(587.330)" /*(+1=5) D_5*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[6]),
                            "Pitch(659.255)" /*(+2=7) E_5*/
                        );
                        assert_eq!(
                            format!("{:.3?}", pitches[7]),
                            "Pitch(739.989)" /*(+2=9) F#_5*/
                        );
                    }
                    None => panic!("expected some pitches"),
                }
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
