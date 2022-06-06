use music_generator::musical_notation::{
    Accidental, Duration, EqualTemperament, Key, MusicalElement, Note, Pitch, Temperament, Volume,
    M, STUTTGART_PITCH, ScaleKind,
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

#[test]
fn voice_of_c_scale() {
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
        mff(311.127),
        mff(349.228),
        mff(369.994),
        mff(415.305),
        mff(466.164),
        mff(523.251),
        mff(554.365),
        mff(622.254),
        mff(698.456),
        mff(739.989),
        mff(830.609),
        mff(932.328),
        mff(1046.502),
        mff(1108.731),
        mff(1244.508),
    ]);
	
    assert_eq!(
        format!("{:.3?}", voice_actual),
        format!("{:.3?}", voice_expected)
    );
	
	sequence_helper(voice_actual);
}
