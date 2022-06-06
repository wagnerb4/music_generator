const OCTAVE_ADDITIVE: u8 = 12;
const OCTAVE_MULTIPLICATIVE: u8 = 2;

use std::rc::Rc;

pub mod temperament;

/**
 * Defines the pitch of a note in Herz.
 */
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pitch(pub f64);

impl Pitch {
    pub fn get_hz(&self) -> f64 {
        self.0
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

pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Note {
    fn get_index(&self) -> u8 {
        match self {
            Note::C => 0,
            Note::D => 1,
            Note::E => 2,
            Note::F => 3,
            Note::G => 4,
            Note::A => 5,
            Note::B => 6,
        }
    }
}

pub enum ScaleKind {
    Major,
    Minor,
    RelativeMinor,
    Chromatic,
}

pub struct Key<T>
where
    T: temperament::Temperament + Sized,
{
    note: &'static Note,
    accidental: &'static Accidental,
    temperament: Rc<T>,
}

impl<T> Key<T>
where
    T: temperament::Temperament,
{
    pub fn new(note: &'static Note, accidental: &'static Accidental, temperament: Rc<T>) -> Self {
        Key {
            note,
            accidental,
            temperament,
        }
    }

    /**
     * Get the key of the respective position in the twelve-tone system.
     * position - a position of 1 or 13 indicates the key of do
     * major is a boolean value indicating whether the key is intended to
     * to be used as a minor or major scale
     */
    fn key_by_position(&self, position: u8, major: bool) -> Option<Key<T>> {
        let mut position: u8 = position - 1;
        position %= OCTAVE_ADDITIVE;
        position += 1;

        let temperament: Rc<T> = Rc::clone(&self.temperament);

        let key = match position {
            1 => Some(Key::new(&Note::C, &Accidental::Natural, temperament)),
            2 => Some(match major {
                true => Key::new(&Note::C, &Accidental::Sharp, temperament),
                false => Key::new(&Note::D, &Accidental::Flat, temperament),
            }),
            3 => Some(Key::new(&Note::D, &Accidental::Natural, temperament)),
            4 => Some(match major {
                true => Key::new(&Note::D, &Accidental::Sharp, temperament),
                false => Key::new(&Note::E, &Accidental::Flat, temperament),
            }),
            5 => Some(Key::new(&Note::E, &Accidental::Natural, temperament)),
            6 => Some(Key::new(&Note::F, &Accidental::Natural, temperament)),
            7 => Some(match major {
                true => Key::new(&Note::F, &Accidental::Sharp, temperament),
                false => Key::new(&Note::G, &Accidental::Flat, temperament),
            }),
            8 => Some(Key::new(&Note::G, &Accidental::Natural, temperament)),
            9 => Some(match major {
                true => Key::new(&Note::G, &Accidental::Sharp, temperament),
                false => Key::new(&Note::A, &Accidental::Flat, temperament),
            }),
            10 => Some(Key::new(&Note::A, &Accidental::Natural, temperament)),
            11 => Some(match major {
                true => Key::new(&Note::A, &Accidental::Sharp, temperament),
                false => Key::new(&Note::B, &Accidental::Flat, temperament),
            }),
            12 => Some(Key::new(&Note::B, &Accidental::Natural, temperament)),
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

        let offset = SEMITONES_IN_MAJOR_SCALE[0..self.note.get_index() as usize]
            .iter()
            .sum::<u8>();
        position += offset;

        position = match self.accidental {
            Accidental::Flat => position - 1,
            Accidental::Natural => position,
            Accidental::Sharp => position + 1,
        };

        return position + 1;
    }

    /**
     * Calculate an array of consecutive pitches of the given scale using the given Temperament.
     * The Pitches will start in the given octave with the given scale-degree and comprise the given
     * number of pitches.
     */
    pub fn get_scale(
        &self,
        scale_kind: &'static ScaleKind,
        octave: i16,
        degree: u8,
        number_of_pitches: u8,
    ) -> Option<Vec<Pitch>> {
        match scale_kind {
            ScaleKind::Major => {
                let mut pitches: Vec<Pitch> = vec![];

                for degree in degree..(degree + number_of_pitches) {
                    match self
                        .temperament
                        .get_pitch(octave, self.get_position(degree) as i16)
                    {
                        Some(pitch) => pitches.push(pitch),
                        None => return None,
                    }
                }

                return Some(pitches);
            }
            ScaleKind::RelativeMinor => {
                let mut degree = degree - 1;
                degree -= 5;
                degree %= DEGREES_IN_SCALE;
                degree += 1;

                let submediant = self.get_position(1 + 5);

                match self.key_by_position(submediant, false) {
                    Some(relative_minor) => relative_minor.get_scale(
                        &ScaleKind::Major,
                        octave,
                        degree as u8,
                        number_of_pitches,
                    ),
                    None => None,
                }
            }
            ScaleKind::Minor => {
                let tonic = self.get_position(1);
                match self.key_by_position(tonic + 3, false) {
                    Some(minor) => {
                        let mapped_tonic_degree = minor.get_degree(tonic).unwrap();
                        let mapped_tonic = minor.get_position(mapped_tonic_degree);

                        let octave = octave
                            + ((tonic as i8 - mapped_tonic as i8) / OCTAVE_ADDITIVE as i8) as i16;

                        return minor.get_scale(
                            &ScaleKind::Major,
                            octave,
                            mapped_tonic_degree + (degree - 1),
                            number_of_pitches,
                        );
                    }
                    None => None,
                }
            }
            ScaleKind::Chromatic => {
                let mut pitches: Vec<Pitch> = vec![];

                for degree in degree..(degree + number_of_pitches) {
                    match self.temperament.get_pitch(octave, degree as i16) {
                        Some(pitch) => pitches.push(pitch),
                        None => return None,
                    }
                }

                return Some(pitches);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        temperament::EqualTemperament, temperament::Temperament, temperament::STUTTGART_PITCH,
        Accidental, Key, Note, ScaleKind,
    };

    use std::rc::Rc;

    #[test]
    fn test_get_position() {
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));

        let key = Key::new(&Note::C, &Accidental::Natural, Rc::clone(&temp));
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

        let key = Key::new(&Note::G, &Accidental::Natural, Rc::clone(&temp));
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
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
        let key = Key::new(&Note::C, &Accidental::Natural, temp);
        match key.get_scale(&ScaleKind::Major, 4, 1, 8) {
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
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
        let key = Key::new(&Note::G, &Accidental::Flat, temp);
        match key.get_scale(&ScaleKind::Minor, 4, 1, 8) {
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
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
        let key = Key::new(&Note::F, &Accidental::Sharp, temp);
        match key.get_scale(&ScaleKind::Minor, 4, 1, 8) {
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
