mod model;

use model::musical_notation::pitch::{
	temperament::EqualTemperament, temperament::Temperament, temperament::STUTTGART_PITCH,
	Accidental, Key, Pitch,
};

use model::musical_notation::volume::Volume;

use model::l_system::{
	Atom, Axiom, Rule, RuleSet,
};

use model::{
	Voice, AtomType,
	NeutralActionState, generate_simple_action,
};

use fundsp::hacker::*;

use std::collections::HashMap;
use std::cell::RefMut;
use std::rc::Rc;

fn main() {
	let temp = EqualTemperament::new(STUTTGART_PITCH);
	let key = Key::Fa(&Accidental::Sharp, &temp);
    
	let mut axiom: Axiom = Axiom::from("ABA").unwrap();
	let ruleset: RuleSet = RuleSet::from(vec![Rule::from("A->ABA").unwrap(), Rule::from("B->BAB").unwrap()]).unwrap();
    for _ in 0..5 {
		axiom.apply_ruleset(&ruleset);
	}
	
	let mut atom_types: HashMap<&Atom, AtomType<NeutralActionState>> = HashMap::new();
	let action: Rc<dyn Fn(char, RefMut<_>) -> Result<_, _>> = generate_simple_action(key).unwrap();
	
	for atom in axiom.atoms() {
		atom_types.insert(atom, match atom.symbol {
			_ => AtomType::HasAction { action: Rc::clone(&action) },
		});
	}
	
	let voice = Voice::from(&axiom, atom_types).unwrap();
	
	let sample_rate = 44100.0;
	let mut sequencer = Sequencer::new(sample_rate, 2);
	
	let env = || envelope(|t| cos(t));
    let magic = |pitch: f64| 200.0_f64 * sine_hz(pitch) * env();
    let magic = |pitch: Pitch, volume: Volume| -> Box<dyn AudioUnit64> {
		Box::new(volume.get() as f64 * magic(pitch.get_hz()) >> pan(0.0))
	};
	
	voice.sequence(&mut sequencer, 60, magic);
	
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
