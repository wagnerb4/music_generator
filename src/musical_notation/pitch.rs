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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn get_by_index(index: u8) -> Result<&'static Self, ()> {
        match index {
            0 => Ok(&Note::C),
            1 => Ok(&Note::D),
            2 => Ok(&Note::E),
            3 => Ok(&Note::F),
            4 => Ok(&Note::G),
            5 => Ok(&Note::A),
            6 => Ok(&Note::B),
            7.. => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ScaleKind {
    Major,
    Minor,
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

    /// Returns the notes with accidentials in the current major key.
    ///
    fn get_key_signature(
        &self,
        note: &'static Note,
        accidental: &'static Accidental,
    ) -> (&'static Accidental, Vec<&'static Note>) {
        let helper = |index: i8| -> (&'static Accidental, Vec<&'static Note>) {
            let accidentals_sharp: [&'static Note; 7] = [
                &Note::F,
                &Note::C,
                &Note::G,
                &Note::D,
                &Note::A,
                &Note::E,
                &Note::B,
            ];
            let accidentals_flat: [&'static Note; 7] = [
                &Note::B,
                &Note::E,
                &Note::A,
                &Note::D,
                &Note::G,
                &Note::C,
                &Note::F,
            ];
            return (
                match index {
                    0 => &Accidental::Natural,
                    -7..=-1 => &Accidental::Flat,
                    1..=8 => &Accidental::Sharp,
                    _ => panic!("logischer Fehler"),
                },
                match index {
                    0 => vec![],
                    -7..=-1 => accidentals_flat[0..(index.abs() as usize)].to_vec(),
                    1..=8 => accidentals_sharp[0..(index as usize)].to_vec(),
                    _ => panic!("logischer Fehler"),
                },
            );
        };

        match (note, accidental) {
            (&Note::C, &Accidental::Flat) => helper(-7),
            (&Note::G, &Accidental::Flat) => helper(-6),
            (&Note::D, &Accidental::Flat) => helper(-5),
            (&Note::A, &Accidental::Flat) => helper(-4),
            (&Note::G, &Accidental::Sharp) => helper(-4),
            (&Note::E, &Accidental::Flat) => helper(-3),
            (&Note::D, &Accidental::Sharp) => helper(-3),
            (&Note::B, &Accidental::Flat) => helper(-2),
            (&Note::A, &Accidental::Sharp) => helper(-2),
            (&Note::F, &Accidental::Natural) => helper(-1),
            (&Note::E, &Accidental::Sharp) => helper(-1),
            (&Note::C, &Accidental::Natural) => helper(0),
            (&Note::B, &Accidental::Sharp) => helper(0),
            (&Note::G, &Accidental::Natural) => helper(1),
            (&Note::D, &Accidental::Natural) => helper(2),
            (&Note::A, &Accidental::Natural) => helper(3),
            (&Note::E, &Accidental::Natural) => helper(4),
            (&Note::F, &Accidental::Flat) => helper(4),
            (&Note::B, &Accidental::Natural) => helper(5),
            (&Note::F, &Accidental::Sharp) => helper(6),
            (&Note::C, &Accidental::Sharp) => helper(7),
        }
    }

    /// Returns the tonic of the major scale, whose
    /// relative minor scale has the tonic of this key.
    ///
    fn get_major_of_minor(&self) -> (&'static Note, &'static Accidental) {
        match (self.note, self.accidental) {
            (&Note::C, &Accidental::Flat) => (&Note::D, &Accidental::Natural),
            (&Note::G, &Accidental::Flat) => (&Note::A, &Accidental::Natural),
            (&Note::D, &Accidental::Flat) => (&Note::E, &Accidental::Natural),
            (&Note::A, &Accidental::Flat) => (&Note::C, &Accidental::Flat),
            (&Note::G, &Accidental::Sharp) => (&Note::C, &Accidental::Flat),
            (&Note::E, &Accidental::Flat) => (&Note::G, &Accidental::Flat),
            (&Note::D, &Accidental::Sharp) => (&Note::G, &Accidental::Flat),
            (&Note::B, &Accidental::Flat) => (&Note::D, &Accidental::Flat),
            (&Note::A, &Accidental::Sharp) => (&Note::D, &Accidental::Flat),
            (&Note::F, &Accidental::Natural) => (&Note::A, &Accidental::Flat),
            (&Note::E, &Accidental::Sharp) => (&Note::A, &Accidental::Flat),
            (&Note::C, &Accidental::Natural) => (&Note::E, &Accidental::Flat),
            (&Note::B, &Accidental::Sharp) => (&Note::E, &Accidental::Flat),
            (&Note::G, &Accidental::Natural) => (&Note::B, &Accidental::Flat),
            (&Note::D, &Accidental::Natural) => (&Note::F, &Accidental::Natural),
            (&Note::A, &Accidental::Natural) => (&Note::C, &Accidental::Natural),
            (&Note::E, &Accidental::Natural) => (&Note::G, &Accidental::Natural),
            (&Note::F, &Accidental::Flat) => (&Note::G, &Accidental::Natural),
            (&Note::B, &Accidental::Natural) => (&Note::D, &Accidental::Natural),
            (&Note::F, &Accidental::Sharp) => (&Note::A, &Accidental::Natural),
            (&Note::C, &Accidental::Sharp) => (&Note::E, &Accidental::Natural),
        }
    }

    /// Returns the notes and accidentals of the current key.
    ///
    fn get_scale(
        &self,
        scale_kind: &'static ScaleKind,
    ) -> [(&'static Note, &'static Accidental); DEGREES_IN_SCALE as usize] {
        let helper = |note: &'static Note,
                      accidental: &'static Accidental|
         -> [(&'static Note, &'static Accidental); DEGREES_IN_SCALE as usize] {
            let key_signature = self.get_key_signature(note, accidental);

            let mut scale = [(&Note::C, &Accidental::Natural); DEGREES_IN_SCALE as usize];
            let start_index = note.get_index();

            for index in 0..(DEGREES_IN_SCALE as usize) {
                let note =
                    Note::get_by_index((start_index + index as u8) % DEGREES_IN_SCALE).unwrap();

                scale[index] = (
                    note,
                    if key_signature.1.contains(&note) {
                        key_signature.0
                    } else {
                        &Accidental::Natural
                    },
                );
            }

            return scale;
        };

        match scale_kind {
            ScaleKind::Major => helper(self.note, self.accidental),
            ScaleKind::Minor => {
                // get the tonic of the major scale whose
                // relative minor scale has the tonic of this key
                let major_of_minor = self.get_major_of_minor();

                // get the major scale of that tonic
                let mut scale = helper(major_of_minor.0, major_of_minor.1);

                // shift right three times

                let shift_by = 2;
                let mut tmp1 = scale[0];
                let mut tmp2;
                let mut shift_to: i8 = 0;

                for index in 0..(DEGREES_IN_SCALE as usize) {
                    /*
                     * shifting a by three to b will need the following steps
                     *
                     * a: 0 1 2 3 4 5 6
                     * b: 4 5 6 0 1 2 3
                     *
                     * 4 1 2 3 4 5 6 | 0
                     * 4 1 2 0 4 5 6 | 3
                     * 4 1 2 0 4 5 3 | 6
                     * 4 1 6 0 4 5 3 | 2
                     * 4 1 6 0 4 2 3 | 5
                     * 4 5 6 0 4 2 3 | 1
                     * 4 5 6 0 1 2 3 | 4
                     */

                    if index == 0 {
                        let shift_from: i8 =
                            (shift_to - shift_by).rem_euclid(DEGREES_IN_SCALE as i8);
                        tmp1 = scale[shift_to as usize];
                        scale[shift_to as usize] = scale[shift_from as usize];
                    } else {
                        tmp2 = scale[shift_to as usize];
                        scale[shift_to as usize] = tmp1;
                        tmp1 = tmp2;
                    }

                    shift_to = (shift_to + shift_by).rem_euclid(DEGREES_IN_SCALE as i8);
                }

                return scale;
            }
        }
    }

    /// Calculate an array of consecutive pitches of the given scale using the given Temperament.
    /// The Pitches will start in the given octave with the given scale-degree and comprise the given
    /// number of pitches.
    ///
    /// # Arguments
    ///
    /// * `scale_kind` - the kind of the scale
    /// * `octave` - the octave where the pitches should start in
    /// * `degree` - the starting degree of the scale to generate, a number between 1 and 7
    /// * `number_of_pitches` - the number of pitches to generate
    ///
    pub fn get_scale_pitches(
        &self,
        scale_kind: &'static ScaleKind,
        octave: i16,
        degree: u8,
        number_of_pitches: u8,
    ) -> Option<Vec<Pitch>> {
        if degree < 1 || degree > 7 {
            return None;
        }

        let mut pitches: Vec<Pitch> = vec![];
        let scale: [(&'static Note, &'static Accidental); DEGREES_IN_SCALE as usize] =
            self.get_scale(scale_kind);

        let mut octaves: i16 = 0;
        let mut pitches_in_octave = 0;

        for degree in degree..(degree + number_of_pitches) {
            let tone = scale[(degree as i8 - 1).rem_euclid(DEGREES_IN_SCALE as i8) as usize];
            if degree > 1
                && octaves == 0
                && ((tone.0 == &Note::C && tone.1 == &Accidental::Natural)
                    || (tone.0 == &Note::B && tone.1 == &Accidental::Sharp)
                    || (tone.0 == &Note::C && tone.1 == &Accidental::Sharp)
                    || (tone.0 == &Note::D && tone.1 == &Accidental::Flat))
            {
                octaves += 1;
            }

            if octaves > 0 {
                pitches_in_octave += 1;

                if pitches_in_octave == (DEGREES_IN_SCALE + 1) {
                    octaves += 1;
                    pitches_in_octave = 1;
                }
            }

            match self
                .temperament
                .get_pitch(octave + octaves, tone)
            {
                Some(pitch) => pitches.push(pitch),
                None => return None,
            }
        }

        return Some(pitches);
    }
}

impl<T> std::fmt::Display for Key<T>
where
    T: temperament::Temperament,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.accidental {
            Accidental::Flat => write!(f, "{:?}b", self.note),
            Accidental::Natural => write!(f, "{:?}", self.note),
            Accidental::Sharp => write!(f, "{:?}#", self.note),
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
    fn test_key_c_natural_major() {
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
        let key = Key::new(&Note::C, &Accidental::Natural, temp);
        match key.get_scale_pitches(&ScaleKind::Major, 4, 1, 8) {
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
    fn test_key_g_natural_major() {
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
        let key = Key::new(&Note::G, &Accidental::Natural, temp);
        match key.get_scale_pitches(&ScaleKind::Major, 4, 1, 8) {
            Some(pitches) => {
                assert_eq!(pitches.len(), 8);
                assert_eq!(
                    format!("{:.3?}", pitches[0]),
                    "Pitch(391.995)" /*G_4 - 2*/
                );
                assert_eq!(format!("{:.3?}", pitches[1]), "Pitch(440.000)" /*A_4*/);
                assert_eq!(
                    format!("{:.3?}", pitches[2]),
                    "Pitch(493.883)" /*B_4 + 2*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[3]),
                    "Pitch(523.251)" /*C_5 + 3*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[4]),
                    "Pitch(587.330)" /*D_5 + 5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[5]),
                    "Pitch(659.255)" /*E_5 + 7*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[6]),
                    "Pitch(739.989)" /*F#_5 + 9*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[7]),
                    "Pitch(783.991)" /*G_5 + 10*/
                );
            }
            None => panic!("expected some pitches"),
        }
    }

    #[test]
    fn test_key_d_flat_major() {
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));

        let key_a = Key::new(&Note::D, &Accidental::Flat, Rc::clone(&temp));
        match key_a.get_scale_pitches(&ScaleKind::Major, 4, 1, 15) {
            Some(pitches) => {
                assert_eq!(pitches.len(), 15);
                assert_eq!(
                    format!("{:.3?}", pitches[0]),
                    "Pitch(277.183)" /*(+0=-8) Db_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[1]),
                    "Pitch(311.127)" /*(+2=-6) Eb_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[2]),
                    "Pitch(349.228)" /*(+2=-4) F_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[3]),
                    "Pitch(369.994)" /*(+1=-3) Gb_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[4]),
                    "Pitch(415.305)" /*(+2=-1) Ab_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[5]),
                    "Pitch(466.164)" /*(+2=1) Bb_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[6]),
                    "Pitch(523.251)" /*(+2=3) C_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[7]),
                    "Pitch(554.365)" /*(+1=4) Db_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[8]),
                    "Pitch(622.254)" /*(+2=6) Eb_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[9]),
                    "Pitch(698.456)" /*(+2=8) F_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[10]),
                    "Pitch(739.989)" /*(+1=9) Gb_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[11]),
                    "Pitch(830.609)" /*(+2=11) Ab_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[12]),
                    "Pitch(932.328)" /*(+2=13) Bb_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[13]),
                    "Pitch(1046.502)" /*(+2=15) C_6*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[14]),
                    "Pitch(1108.731)" /*(+1=16) Db_6*/
                );
            }
            None => panic!("expected some pitches"),
        }

        let key_b = Key::new(&Note::C, &Accidental::Sharp, Rc::clone(&temp));
        match key_b.get_scale_pitches(&ScaleKind::Major, 4, 1, 15) {
            Some(pitches) => {
                assert_eq!(pitches.len(), 15);
                assert_eq!(
                    format!("{:.3?}", pitches[0]),
                    "Pitch(277.183)" /*(+0=-8) Db_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[1]),
                    "Pitch(311.127)" /*(+2=-6) Eb_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[2]),
                    "Pitch(349.228)" /*(+2=-4) F_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[3]),
                    "Pitch(369.994)" /*(+1=-3) Gb_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[4]),
                    "Pitch(415.305)" /*(+2=-1) Ab_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[5]),
                    "Pitch(466.164)" /*(+2=1) Bb_4*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[6]),
                    "Pitch(523.251)" /*(+2=3) C_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[7]),
                    "Pitch(554.365)" /*(+1=4) Db_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[8]),
                    "Pitch(622.254)" /*(+2=6) Eb_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[9]),
                    "Pitch(698.456)" /*(+2=8) F_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[10]),
                    "Pitch(739.989)" /*(+1=9) Gb_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[11]),
                    "Pitch(830.609)" /*(+2=11) Ab_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[12]),
                    "Pitch(932.328)" /*(+2=13) Bb_5*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[13]),
                    "Pitch(1046.502)" /*(+2=15) C_6*/
                );
                assert_eq!(
                    format!("{:.3?}", pitches[14]),
                    "Pitch(1108.731)" /*(+1=16) Db_6*/
                );
            }
            None => panic!("expected some pitches"),
        }
    }

    #[test]
    fn test_key_g_flat_minor() {
        let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
        let key = Key::new(&Note::G, &Accidental::Flat, temp);
        match key.get_scale_pitches(&ScaleKind::Minor, 4, 1, 8) {
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
        match key.get_scale_pitches(&ScaleKind::Minor, 4, 1, 8) {
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
