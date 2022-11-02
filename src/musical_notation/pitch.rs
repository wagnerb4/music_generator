use crate::musical_notation::pitch::error::KeyCreationError;
use crate::musical_notation::pitch::temperament::error::TemperamentError;
use crate::musical_notation::Temperament;

const OCTAVE_MULTIPLICATIVE: u8 = 2;

pub mod error;
/// Defines the temperaments that can be used to determine the
/// frequency of a specific musical tone like 'c natural' or 'a flat'.
///
/// # Constants
///
/// Exports constants for different pitch standards.
/// The number of such a constant always refers to
/// the frequency of A_4 in Herz.
///
/// Definitions are taken form Oxford Composer Companion JS Bach,
/// pp. 369–372. Oxford University Press, 1999
///
pub mod temperament;

/// Defines the pitch of a note in Herz.
///
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pitch(pub f64);

impl Pitch {
    pub fn get_hz(&self) -> f64 {
        self.0
    }
}

const DEGREES_IN_SCALE: u8 = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteName {
    fn get_index(&self) -> u8 {
        match self {
            NoteName::C => 0,
            NoteName::D => 1,
            NoteName::E => 2,
            NoteName::F => 3,
            NoteName::G => 4,
            NoteName::A => 5,
            NoteName::B => 6,
        }
    }

    fn get_by_index(index: u8) -> Result<&'static Self, ()> {
        match index {
            0 => Ok(&NoteName::C),
            1 => Ok(&NoteName::D),
            2 => Ok(&NoteName::E),
            3 => Ok(&NoteName::F),
            4 => Ok(&NoteName::G),
            5 => Ok(&NoteName::A),
            6 => Ok(&NoteName::B),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tone {
    note_name: &'static NoteName,
    accidental: &'static Accidental,
}

impl Tone {
    pub fn new(note_name: &'static NoteName, accidental: &'static Accidental) -> Self {
        Tone {
            note_name,
            accidental,
        }
    }

    pub fn from(string: &str) -> Result<Self, String> {
        match string {
            "Cb" => Ok(Self::new(&NoteName::C, &Accidental::Flat)),
            "C" => Ok(Self::new(&NoteName::C, &Accidental::Natural)),
            "C#" => Ok(Self::new(&NoteName::C, &Accidental::Sharp)),
            "Db" => Ok(Self::new(&NoteName::D, &Accidental::Flat)),
            "D" => Ok(Self::new(&NoteName::D, &Accidental::Natural)),
            "D#" => Ok(Self::new(&NoteName::D, &Accidental::Sharp)),
            "Eb" => Ok(Self::new(&NoteName::E, &Accidental::Flat)),
            "E" => Ok(Self::new(&NoteName::E, &Accidental::Natural)),
            "E#" => Ok(Self::new(&NoteName::E, &Accidental::Sharp)),
            "Fb" => Ok(Self::new(&NoteName::F, &Accidental::Flat)),
            "F" => Ok(Self::new(&NoteName::F, &Accidental::Natural)),
            "F#" => Ok(Self::new(&NoteName::F, &Accidental::Sharp)),
            "Gb" => Ok(Self::new(&NoteName::G, &Accidental::Flat)),
            "G" => Ok(Self::new(&NoteName::G, &Accidental::Natural)),
            "G#" => Ok(Self::new(&NoteName::G, &Accidental::Sharp)),
            "Ab" => Ok(Self::new(&NoteName::A, &Accidental::Flat)),
            "A" => Ok(Self::new(&NoteName::A, &Accidental::Natural)),
            "A#" => Ok(Self::new(&NoteName::A, &Accidental::Sharp)),
            "Bb" => Ok(Self::new(&NoteName::B, &Accidental::Flat)),
            "B" => Ok(Self::new(&NoteName::B, &Accidental::Natural)),
            "B#" => Ok(Self::new(&NoteName::B, &Accidental::Sharp)),
            _ => Err(
                "Please provide a valid tonic. Examples of correct values are 'C', 'F#', 'Gb'."
                    .to_string(),
            ),
        }
    }

    pub fn get_note_name(&self) -> &NoteName {
        self.note_name
    }

    pub fn get_accidental(&self) -> &Accidental {
        self.accidental
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScaleKind {
    Major,
    Minor,
}

pub struct Key<T>
where
    T: Temperament + Sized,
{
    tone: Tone,
    scale_kind: &'static ScaleKind,
    temperament: T,
    scale: [Tone; DEGREES_IN_SCALE as usize],
}

impl<T> Key<T>
where
    T: Temperament,
{
    pub fn new<F>(
        tone: Tone,
        scale_kind: &'static ScaleKind,
        pitch_standard: f64,
        func: F,
    ) -> Result<Self, KeyCreationError>
    where
        F: Fn(f64, [Tone; DEGREES_IN_SCALE as usize]) -> Result<T, TemperamentError>,
    {
        let scale: [Tone; DEGREES_IN_SCALE as usize] = Self::get_scale(tone, scale_kind);
        let temperament: T = func(pitch_standard, scale)?;
        Ok(Key {
            tone,
            scale_kind,
            temperament,
            scale,
        })
    }

    /// Returns the note names with accidentals in the current major key.
    ///
    fn get_key_signature(tone: Tone) -> (&'static Accidental, Vec<&'static NoteName>) {
        let helper = |index: i8| -> (&'static Accidental, Vec<&'static NoteName>) {
            let accidentals_sharp: [&'static NoteName; 7] = [
                &NoteName::F,
                &NoteName::C,
                &NoteName::G,
                &NoteName::D,
                &NoteName::A,
                &NoteName::E,
                &NoteName::B,
            ];
            let accidentals_flat: [&'static NoteName; 7] = [
                &NoteName::B,
                &NoteName::E,
                &NoteName::A,
                &NoteName::D,
                &NoteName::G,
                &NoteName::C,
                &NoteName::F,
            ];
            return (
                match index {
                    0 => &Accidental::Natural,
                    -7..=-1 => &Accidental::Flat,
                    1..=8 => &Accidental::Sharp,
                    _ => panic!("logic error in code"),
                },
                match index {
                    0 => vec![],
                    -7..=-1 => accidentals_flat[0..(index.abs() as usize)].to_vec(),
                    1..=8 => accidentals_sharp[0..(index as usize)].to_vec(),
                    _ => panic!("logic error in code"),
                },
            );
        };

        match (tone.note_name, tone.accidental) {
            (&NoteName::C, &Accidental::Flat) => helper(-7),
            (&NoteName::G, &Accidental::Flat) => helper(-6),
            (&NoteName::D, &Accidental::Flat) => helper(-5),
            (&NoteName::A, &Accidental::Flat) => helper(-4),
            (&NoteName::G, &Accidental::Sharp) => helper(-4),
            (&NoteName::E, &Accidental::Flat) => helper(-3),
            (&NoteName::D, &Accidental::Sharp) => helper(-3),
            (&NoteName::B, &Accidental::Flat) => helper(-2),
            (&NoteName::A, &Accidental::Sharp) => helper(-2),
            (&NoteName::F, &Accidental::Natural) => helper(-1),
            (&NoteName::E, &Accidental::Sharp) => helper(-1),
            (&NoteName::C, &Accidental::Natural) => helper(0),
            (&NoteName::B, &Accidental::Sharp) => helper(0),
            (&NoteName::G, &Accidental::Natural) => helper(1),
            (&NoteName::D, &Accidental::Natural) => helper(2),
            (&NoteName::A, &Accidental::Natural) => helper(3),
            (&NoteName::E, &Accidental::Natural) => helper(4),
            (&NoteName::F, &Accidental::Flat) => helper(4),
            (&NoteName::B, &Accidental::Natural) => helper(5),
            (&NoteName::F, &Accidental::Sharp) => helper(6),
            (&NoteName::C, &Accidental::Sharp) => helper(7),
        }
    }

    /// Returns the tonic of the major scale, whose
    /// relative minor scale has the tonic of this key.
    ///
    fn get_major_of_minor(tone: Tone) -> Tone {
        match (tone.note_name, tone.accidental) {
            (&NoteName::C, &Accidental::Flat) => Tone::new(&NoteName::D, &Accidental::Natural),
            (&NoteName::G, &Accidental::Flat) => Tone::new(&NoteName::A, &Accidental::Natural),
            (&NoteName::D, &Accidental::Flat) => Tone::new(&NoteName::E, &Accidental::Natural),
            (&NoteName::A, &Accidental::Flat) => Tone::new(&NoteName::C, &Accidental::Flat),
            (&NoteName::G, &Accidental::Sharp) => Tone::new(&NoteName::C, &Accidental::Flat),
            (&NoteName::E, &Accidental::Flat) => Tone::new(&NoteName::G, &Accidental::Flat),
            (&NoteName::D, &Accidental::Sharp) => Tone::new(&NoteName::G, &Accidental::Flat),
            (&NoteName::B, &Accidental::Flat) => Tone::new(&NoteName::D, &Accidental::Flat),
            (&NoteName::A, &Accidental::Sharp) => Tone::new(&NoteName::D, &Accidental::Flat),
            (&NoteName::F, &Accidental::Natural) => Tone::new(&NoteName::A, &Accidental::Flat),
            (&NoteName::E, &Accidental::Sharp) => Tone::new(&NoteName::A, &Accidental::Flat),
            (&NoteName::C, &Accidental::Natural) => Tone::new(&NoteName::E, &Accidental::Flat),
            (&NoteName::B, &Accidental::Sharp) => Tone::new(&NoteName::E, &Accidental::Flat),
            (&NoteName::G, &Accidental::Natural) => Tone::new(&NoteName::B, &Accidental::Flat),
            (&NoteName::D, &Accidental::Natural) => Tone::new(&NoteName::F, &Accidental::Natural),
            (&NoteName::A, &Accidental::Natural) => Tone::new(&NoteName::C, &Accidental::Natural),
            (&NoteName::E, &Accidental::Natural) => Tone::new(&NoteName::G, &Accidental::Natural),
            (&NoteName::F, &Accidental::Flat) => Tone::new(&NoteName::G, &Accidental::Natural),
            (&NoteName::B, &Accidental::Natural) => Tone::new(&NoteName::D, &Accidental::Natural),
            (&NoteName::F, &Accidental::Sharp) => Tone::new(&NoteName::A, &Accidental::Natural),
            (&NoteName::C, &Accidental::Sharp) => Tone::new(&NoteName::E, &Accidental::Natural),
        }
    }

    /// Returns the notes and accidentals of the current key.
    ///
    fn get_scale(tone: Tone, scale_kind: &'static ScaleKind) -> [Tone; DEGREES_IN_SCALE as usize] {
        let helper = |tone: Tone| -> [Tone; DEGREES_IN_SCALE as usize] {
            let key_signature = Self::get_key_signature(tone);

            let mut scale =
                [Tone::new(&NoteName::C, &Accidental::Natural); DEGREES_IN_SCALE as usize];
            let start_index = tone.note_name.get_index();

            for index in 0..(DEGREES_IN_SCALE as usize) {
                let note =
                    NoteName::get_by_index((start_index + index as u8) % DEGREES_IN_SCALE).unwrap();

                scale[index] = Tone::new(
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
            ScaleKind::Major => helper(tone),
            ScaleKind::Minor => {
                // get the tonic of the major scale whose
                // relative minor scale has the tonic of this key
                let major_of_minor: Tone = Self::get_major_of_minor(tone);

                // get the major scale of that tonic
                let mut scale = helper(major_of_minor);

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
        octave: i16,
        degree: u8,
        number_of_pitches: u8,
    ) -> Option<Vec<Pitch>> {
        if degree < 1 || degree > 7 {
            return None;
        }

        let mut pitches: Vec<Pitch> = vec![];

        let mut octaves: i16 = 0;
        let mut pitches_in_octave = 0;

        for degree in degree..(degree + number_of_pitches) {
            let tone = self.scale[(degree as i8 - 1).rem_euclid(DEGREES_IN_SCALE as i8) as usize];
            if degree > 1
                && octaves == 0
                && ((tone.note_name == &NoteName::C && tone.accidental == &Accidental::Natural)
                    || (tone.note_name == &NoteName::B && tone.accidental == &Accidental::Sharp)
                    || (tone.note_name == &NoteName::C && tone.accidental == &Accidental::Sharp)
                    || (tone.note_name == &NoteName::D && tone.accidental == &Accidental::Flat))
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

            match self.temperament.get_pitch(octave + octaves, tone) {
                Some(pitch) => pitches.push(pitch),
                None => return None,
            }
        }

        return Some(pitches);
    }
}

impl<T> std::fmt::Display for Key<T>
where
    T: Temperament,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tone.accidental {
            Accidental::Flat => write!(f, "{:?}b", self.tone.note_name),
            Accidental::Natural => write!(f, "{:?}", self.tone.note_name),
            Accidental::Sharp => write!(f, "{:?}#", self.tone.note_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        temperament::EqualTemperament, temperament::Temperament, temperament::STUTTGART_PITCH,
        Accidental, Key, NoteName, ScaleKind, Tone,
    };

    #[test]
    fn test_key_c_natural_major() -> Result<(), String> {
        let c_natural = Tone::new(&NoteName::C, &Accidental::Natural);
        let c_natural_major = Key::new(
            c_natural,
            &ScaleKind::Major,
            STUTTGART_PITCH,
            EqualTemperament::new,
        )?;
        match c_natural_major.get_scale_pitches(4, 1, 8) {
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
                return Ok(());
            }
            None => Err(String::from("expected some pitches")),
        }
    }

    #[test]
    fn test_key_g_natural_major() -> Result<(), String> {
        let g_natural = Tone::new(&NoteName::G, &Accidental::Natural);
        let g_natural_major = Key::new(
            g_natural,
            &ScaleKind::Major,
            STUTTGART_PITCH,
            EqualTemperament::new,
        )?;
        match g_natural_major.get_scale_pitches(4, 1, 8) {
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
                return Ok(());
            }
            None => Err(String::from("expected some pitches")),
        }
    }

    #[test]
    fn test_key_d_flat_major() -> Result<(), String> {
        let d_flat = Tone::new(&NoteName::D, &Accidental::Flat);
        let c_sharp = Tone::new(&NoteName::C, &Accidental::Sharp);

        let d_flat_major = Key::new(
            d_flat,
            &ScaleKind::Major,
            STUTTGART_PITCH,
            EqualTemperament::new,
        )?;
        match d_flat_major.get_scale_pitches(4, 1, 15) {
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
            None => return Err(String::from("expected some pitches")),
        }

        let c_sharp_major = Key::new(
            c_sharp,
            &ScaleKind::Major,
            STUTTGART_PITCH,
            EqualTemperament::new,
        )?;
        match c_sharp_major.get_scale_pitches(4, 1, 15) {
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
                return Ok(());
            }
            None => Err(String::from("expected some pitches")),
        }
    }

    #[test]
    fn test_key_g_flat_minor() -> Result<(), String> {
        let g_flat = Tone::new(&NoteName::G, &Accidental::Flat);
        let g_flat_minor = Key::new(
            g_flat,
            &ScaleKind::Minor,
            STUTTGART_PITCH,
            EqualTemperament::new,
        )?;
        match g_flat_minor.get_scale_pitches(4, 1, 8) {
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
                return Ok(());
            }
            None => Err(String::from("expected some pitches")),
        }
    }

    #[test]
    fn test_key_f_sharp_minor() -> Result<(), String> {
        let f_sharp = Tone::new(&NoteName::F, &Accidental::Sharp);
        let f_sharp_minor = Key::new(
            f_sharp,
            &ScaleKind::Minor,
            STUTTGART_PITCH,
            EqualTemperament::new,
        )?;
        match f_sharp_minor.get_scale_pitches(4, 1, 8) {
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
                return Ok(());
            }
            None => Err(String::from("expected some pitches")),
        }
    }
}
