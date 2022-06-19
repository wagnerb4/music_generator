use music_generator::musical_notation::{
    Accidental, Duration, EqualTemperament, Key, MusicalElement, Note, Pitch, ScaleKind,
    Temperament, Volume, M, STUTTGART_PITCH,
};

use music_generator::voice::action::{Action, AtomType, NeutralActionState, SimpleAction};
use music_generator::voice::Voice;

use music_generator::l_system::{Atom, Axiom};

use std::collections::HashMap;
use std::rc::Rc;

use fundsp::hacker::*;

fn mff(frequency: f64) -> MusicalElement {
    MusicalElement::Note {
        duration: Duration(1),
        volume: M,
        pitch: Pitch(frequency),
    }
}

fn sequence_helper(voice: Voice) {
    let sample_rate = 44100.0;
    let mut sequencer = Sequencer::new(sample_rate, 2);

    let env = || envelope(|t| cos(t));
    let magic = |pitch: f64| 200.0_f64 * sine_hz(pitch) * env();
    let magic = |pitch: Pitch, volume: Volume| -> Box<dyn AudioUnit64> {
        Box::new(volume.get() as f64 * magic(pitch.get_hz()) >> pan(0.0))
    };

    let bpm = 120;
    voice.sequence(&mut sequencer, bpm, magic);

    let duration = voice.get_duration(bpm);

    let wave = Wave64::render(sample_rate, duration, &mut sequencer);
    // let wave = wave.filter(duration, &mut (reverb_stereo(0.1, 2.0) * 3.0));
    let wave = wave.filter_latency(duration, &mut (limiter_stereo((0.01, 0.1))));
    wave.save_wav16(std::path::Path::new("target/gen/sequence.wav"))
        .unwrap()

    /*
    let sample_rate = 44100.0;
    let env = || envelope(|t| cos(t));
    // let test = |pitch: f64| 200.0_f64 * sine_hz(pitch) * env();
    // let test = |pitch: f64| test(pitch) >> pan(0.0);
    let test = |pitch: f64| brown() * pitch >> sine() >> pan(0.0);
    let mut sequencer = Sequencer::new(sample_rate, 2);
    let duration = length as f64 / bpm_hz(bpm) + 2.0;
    let wave = Wave64::render(sample_rate, duration, &mut sequencer);
    let wave = wave.filter(duration, &mut (reverb_stereo(0.1, 2.0) * 3.0));
    let wave = wave.filter_latency(duration, &mut (limiter_stereo((0.01, 0.1))));
    wave.save_wav16(std::path::Path::new("sequence.wav")).unwrap()
    */
}

/* Tests the simple action, which mapps the 49 letters A-Za-w to the notes of seven octaves
 * ABCDEFG HIJKLMN OPQRSTU VWXYZab cdefghi jklmnop qrstuvw
 * 1234567 1234567 1234567 1234567 1234567 1234567 1234567
 */
 
#[test]
fn voice_of_c_major_seven_octaves() {
    let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
    let key = Key::new(&Note::C, &Accidental::Natural, temp);
    let axiom: Axiom = Axiom::from("AHOVcjqBIPWdkrCJQXelsDKRYfmtELSZgnuFMTahovGNUbipw").unwrap();

    let mut atom_types: HashMap<&Atom, AtomType<NeutralActionState>> = HashMap::new();

    let action: Rc<dyn Action<_>> = Rc::new(SimpleAction::new(key, &ScaleKind::Major));

    for atom in axiom.atoms() {
        atom_types.insert(
            atom,
            match atom.symbol {
                _ => AtomType::HasAction {
                    action: Rc::clone(&action),
                },
            },
        );
    }

    let voice_actual = Voice::from(&axiom, atom_types).unwrap();

    let voice_expected = Voice::from_musical_elements(vec![
        mff(261.626),  /*-9 C_4*/
		mff(523.251),  /*3  C_5*/
		mff(1046.502),  /*15 C_6*/
		mff(2093.005),  /*27 C_7*/
		mff(4186.009),  /*39 C_8*/
		mff(8372.018),  /*51 C_9*/
		mff(16744.036),  /*63 C_10*/
		mff(277.183),  /*-7 D_4*/
		mff(277.183),  /*5  D_5*/
		mff(277.183),  /*17 D_6*/
		mff(277.183),  /*29 D_7*/
		mff(277.183),  /*41 D_8*/
		mff(277.183),  /*53 D_9*/
		mff(277.183),  /*65 D_10*/
		mff(277.183),  /*-5 E_4*/
		mff(277.183),  /*7  E_5*/
		mff(277.183),  /*19 E_6*/
		mff(277.183),  /*31 E_7*/
		mff(277.183),  /*43 E_8*/
		mff(277.183),  /*55 E_9*/
		mff(277.183),  /*67 E_10*/
		mff(277.183),  /*-4 F_4*/
		mff(277.183),  /*8  F_5*/
		mff(277.183),  /*20 F_6*/
		mff(277.183),  /*32 F_7*/
		mff(277.183),  /*44 F_8*/
		mff(277.183),  /*56 F_9*/
		mff(277.183),  /*68 F_10*/
		mff(277.183),  /*-2 G_4*/
		mff(277.183),  /*10 G_5*/
		mff(277.183),  /*22 G_6*/
		mff(277.183),  /*34 G_7*/
		mff(277.183),  /*46 G_8*/
		mff(277.183),  /*58 G_9*/
		mff(277.183),  /*70 G_10*/
		mff(277.183),  /*0  A_4*/
		mff(277.183),  /*12 A_5*/
		mff(277.183),  /*24 A_6*/
		mff(277.183),  /*36 A_7*/
		mff(277.183),  /*48 A_8*/
		mff(277.183),  /*60 A_9*/
		mff(277.183),  /*72 A_10*/
		mff(277.183),  /*2  B_4*/
		mff(277.183),  /*14 B_5*/
		mff(277.183),  /*26 B_6*/
		mff(277.183),  /*38 B_7*/
		mff(277.183),  /*50 B_8*/
		mff(277.183),  /*62 B_9*/
		mff(277.183),  /*74 B_10*/
    ]);

    assert_eq!(
        format!("{:.3?}", voice_actual),
        format!("{:.3?}", voice_expected)
    );

    sequence_helper(voice_actual);
}

#[test]
fn voice_of_d_flat_major_two_octave_scale() {
    let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
    let key = Key::new(&Note::C, &Accidental::Sharp, temp);
    let axiom: Axiom = Axiom::from("ABCDEFGHIJKLMNO").unwrap();

    let mut atom_types: HashMap<&Atom, AtomType<NeutralActionState>> = HashMap::new();

    let action: Rc<dyn Action<_>> = Rc::new(SimpleAction::new(key, &ScaleKind::Major));

    for atom in axiom.atoms() {
        atom_types.insert(
            atom,
            match atom.symbol {
                _ => AtomType::HasAction {
                    action: Rc::clone(&action),
                },
            },
        );
    }

    let voice_actual = Voice::from(&axiom, atom_types).unwrap();

    let voice_expected = Voice::from_musical_elements(vec![
        mff(277.183),  /*(+0=-8) Db_4*/
        mff(311.127),  /*(+2=-6) Eb_4*/
        mff(349.228),  /*(+2=-4) F_4*/
        mff(369.994),  /*(+1=-3) Gb_4*/
        mff(415.305),  /*(+2=-1) Ab_4*/
        mff(466.164),  /*(+2=1) Bb_4*/
        mff(523.251),  /*(+2=3) C_5*/
        mff(554.365),  /*(+1=4) Db_5*/
        mff(622.254),  /*(+2=6) Eb_5*/
        mff(698.456),  /*(+2=8) F_5*/
        mff(739.989),  /*(+1=9) Gb_5*/
        mff(830.609),  /*(+2=11) Ab_5*/
        mff(932.328),  /*(+2=13) Bb_5*/
        mff(1046.502), /*(+2=15) C_6*/
        mff(1108.731), /*(+1=16) Db_6*/
    ]);

    assert_eq!(
        format!("{:.3?}", voice_actual),
        format!("{:.3?}", voice_expected)
    );

    sequence_helper(voice_actual);
}
