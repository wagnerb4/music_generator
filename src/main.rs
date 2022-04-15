#![allow(dead_code)]

pub mod l_system;
pub mod musical_notation;
pub mod voice;

use musical_notation::{
    Accidental, EqualTemperament, Key, Note, Pitch, Temperament, Volume, STUTTGART_PITCH,
};

use l_system::{Atom, Axiom, Rule, RuleSet};

use voice::action::{Action, AtomType, NeutralActionState, SimpleAction};
use voice::Voice;

use fundsp::hacker::*;

use std::collections::HashMap;
use std::rc::Rc;

fn voice_one(temp: Rc<EqualTemperament>) -> Voice {
    let key = Key::new(&Note::F, &Accidental::Sharp, temp);

    let mut axiom: Axiom = Axiom::from("ABCD").unwrap();
    let ruleset: RuleSet = RuleSet::from(vec![
        Rule::from("A->AC").unwrap(),
        Rule::from("B->DD").unwrap(),
        Rule::from("C->CA").unwrap(),
        Rule::from("D->DB").unwrap(),
    ])
    .unwrap();

    for _ in 0..3 {
        axiom.apply_ruleset(&ruleset);
    }

    println!("{:?}", axiom);

    let mut atom_types: HashMap<&Atom, AtomType<NeutralActionState>> = HashMap::new();

    let action: Rc<dyn Action<_>> = Rc::new(SimpleAction::new(key, true));

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

    Voice::from(&axiom, atom_types).unwrap()
}

fn voice_two(temp: Rc<EqualTemperament>) -> Voice {
    let key = Key::new(&Note::F, &Accidental::Sharp, temp);

    let mut axiom: Axiom = Axiom::from("ABCD").unwrap();
    let ruleset: RuleSet = RuleSet::from(vec![
        Rule::from("A->CD").unwrap(),
        Rule::from("B->BA").unwrap(),
        Rule::from("C->AC").unwrap(),
        Rule::from("D->Bx").unwrap(),
        Rule::from("x->DC").unwrap(),
    ])
    .unwrap();

    for _ in 0..3 {
        axiom.apply_ruleset(&ruleset);
    }

    println!("{:?}", axiom);

    let mut atom_types: HashMap<&Atom, AtomType<NeutralActionState>> = HashMap::new();

    let action: Rc<dyn Action<_>> = Rc::new(SimpleAction::new(key, true));

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

    Voice::from(&axiom, atom_types).unwrap()
}

fn main() {
    let sample_rate = 44100.0;
    let mut sequencer = Sequencer::new(sample_rate, 2);

    let env = || envelope(|t| cos(t));
    let magic = |pitch: f64| 200.0_f64 * sine_hz(pitch) * env();
    let magic = |pitch: Pitch, volume: Volume| -> Box<dyn AudioUnit64> {
        Box::new(volume.get() as f64 * magic(pitch.get_hz()) >> pan(0.0))
    };

    let bpm = 120;

    let temp = Rc::new(EqualTemperament::new(STUTTGART_PITCH));
    //let v_one = voice_one();
    let v_two = voice_two(temp);

    v_two.sequence(&mut sequencer, bpm, magic);
    // v_two.sequence(&mut sequencer, bpm / 2, magic);

    let length = v_two.get_len();
    //let length = max(v_two.get_len(), v_one.get_len());
    let duration = length as f64 / bpm_hz(bpm as f64) + 2.0;
    println!("length: {:?}, duration: {:?}", length, duration);

    let wave = Wave64::render(sample_rate, duration, &mut sequencer);
    let wave = wave.filter(duration, &mut (reverb_stereo(0.1, 2.0) * 3.0));
    let wave = wave.filter_latency(duration, &mut (limiter_stereo((0.01, 0.1))));
    wave.save_wav16(std::path::Path::new("sequence.wav"))
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
